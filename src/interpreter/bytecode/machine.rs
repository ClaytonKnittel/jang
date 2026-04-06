use crate::{
  error::{JangError, JangResult},
  interpreter::{
    bytecode::instruction::{JitCompiledFunction, JitInstruction, LocalId},
    value::Value,
  },
  parser::{ast::binary_expression::BinaryOp, token::ident::Ident},
};

enum StackEntry<'a> {
  Value(Value<'a>),
  BasePointer(usize),
}

#[derive(Default)]
struct MachineStack<'a> {
  items: Vec<Value<'a>>,
}

impl<'a> MachineStack<'a> {
  fn from_args(args: Vec<Value<'a>>) -> Self {
    Self { items: args }
  }

  fn pop_value(&mut self) -> JangResult<Value<'a>> {
    match self.items.pop() {
      Some(v) => Ok(v),
      _ => Err(JangError::interpret_error("bad stack: empty")),
    }
  }

  fn push_value(&mut self, value: Value<'a>) {
    self.items.push(value);
  }

  fn set_local(&mut self, local_id: &LocalId, value: Value<'a>) -> JangResult {
    self.extend(local_id);
    match self.items.get_mut(local_id.as_index()) {
      Some(entry) => {
        *entry = value;
        Ok(())
      }
      _ => Err(JangError::interpret_error("bad stack: empty")),
    }
  }

  fn get_local(&self, local_id: &LocalId) -> JangResult<&Value<'a>> {
    match self.items.get(local_id.as_index()) {
      Some(value) => Ok(value),
      _ => Err(JangError::interpret_error("bad stack: empty")),
    }
  }

  fn extend(&mut self, local_id: &LocalId) {
    self.items.extend(std::iter::repeat_n(
      Value::Uninitialized,
      (1 + local_id.as_index()).saturating_sub(self.items.len()),
    ));
  }
}

pub trait JitFunctionContext<'a> {
  fn resolve_ident(&'a self, name: &'_ Ident) -> JangResult<Value<'a>>;
}

pub fn evaluate_function<'a>(
  jit_fn: &'a JitCompiledFunction<'a>,
  args: Vec<Value<'a>>,
  context: &'a impl JitFunctionContext<'a>,
) -> JangResult<Value<'a>> {
  let mut stack = MachineStack::<'a>::from_args(args);
  let mut pc = jit_fn.entrypoint;

  loop {
    let block = &jit_fn.blocks[pc.as_index()];
    for instr in &block.instructions {
      match instr {
        JitInstruction::BinaryOp(binary_op) => {
          let lhs = stack.pop_value()?;
          let rhs = stack.pop_value()?;
          stack.push_value(match binary_op {
            BinaryOp::Add => lhs.add(&rhs)?,
            BinaryOp::Sub => lhs.subtract(&rhs)?,
            BinaryOp::Mul => lhs.multiply(&rhs)?,
            BinaryOp::Div => lhs.divide(&rhs)?,
            BinaryOp::Mod => lhs.modulo(&rhs)?,
          });
        }
        JitInstruction::LoadLiteral(literal) => {
          stack.push_value(Value::from_literal(literal)?);
        }
        JitInstruction::LoadGlobal(ident) => {
          let value = context.resolve_ident(ident)?;
          stack.push_value(value);
        }
        JitInstruction::LoadLocal(local_id) => {
          let value = stack.get_local(local_id)?.clone();
          stack.push_value(value);
        }
        JitInstruction::StoreLocal(local_id) => {
          let value = stack.pop_value()?;
          stack.set_local(local_id, value)?;
        }
        JitInstruction::Call(call_instr) => {
          let target_fn = match stack.pop_value()? {
            Value::JitCompiledFunctionRef(jit_compiled_function) => Ok(jit_compiled_function),
            _ => Err(JangError::interpret_error(
              "invocation target is not a function",
            )),
          }?;
          let mut args = Vec::new();
          for _ in 0..call_instr.arity {
            args.push(stack.pop_value()?);
          }
          stack.push_value(evaluate_function(target_fn, args, context)?);
        }
        JitInstruction::Jump(block_id) => {
          pc = *block_id;
          break;
        }
        JitInstruction::ConditionalJump(conditional_jump_targets) => {
          let condition = stack.pop_value()?;
          let truthy = match condition {
            Value::Int32(x) => x != 0,
            Value::Float32(x) => x != 0.,
            _ => false,
          };
          pc = if truthy {
            conditional_jump_targets.true_target
          } else {
            conditional_jump_targets.false_target
          };
          break;
        }
        JitInstruction::Ret => return stack.pop_value(),
      }
    }
  }
}
