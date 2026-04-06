use crate::{
  error::{JangError, JangResult},
  interpreter::{
    bytecode::{
      instruction::{JitCompiledFunction, JitInstruction, JitTerminalInstruction},
      local_table::LocalTable,
    },
    value::Value,
  },
  parser::{ast::binary_expression::BinaryOp, token::ident::Ident},
};

#[derive(Default)]
struct MachineStack<'a> {
  items: Vec<Value<'a>>,
}

impl<'a> MachineStack<'a> {
  fn from_args(mut args: Vec<Value<'a>>) -> Self {
    args.reverse();
    Self { items: args }
  }

  fn pop_value(&mut self) -> JangResult<Value<'a>> {
    match self.items.pop() {
      Some(v) => Ok(v),
      None => Err(JangError::interpret_error("bad stack: empty")),
    }
  }

  fn push_value(&mut self, value: Value<'a>) {
    self.items.push(value);
  }
}

pub trait JitFunctionContext<'a> {
  fn resolve_ident(&'a self, name: &'_ Ident) -> JangResult<Value<'a>>;
}

pub fn evaluate_function<'a>(
  jit_fn: &'a JitCompiledFunction<'a>,
  args: Vec<Value<'a>>,
  context: &'a impl JitFunctionContext<'a>,
) -> JangResult<Option<Value<'a>>> {
  let mut locals = LocalTable::<Value<'a>>::new();
  let mut stack = MachineStack::from_args(args);
  let mut pc = jit_fn.entrypoint();

  loop {
    let block = jit_fn.block(pc);
    for instr in block.instructions() {
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
          stack.push_value(context.resolve_ident(ident)?);
        }
        JitInstruction::LoadLocal(local_id) => {
          stack.push_value(locals.read(*local_id)?.clone());
        }
        JitInstruction::StoreLocal(local_id) => {
          locals.write(*local_id, stack.pop_value()?);
        }
        JitInstruction::Call(call_instr) => {
          let target_fn = stack.pop_value()?.as_jit_function()?;
          let mut args = Vec::new();
          for _ in 0..call_instr.arity {
            args.push(stack.pop_value()?);
          }
          let value = evaluate_function(target_fn, args, context)?;
          if let Some(value) = value {
            stack.push_value(value);
          }
        }
      }
    }

    match block.terminator() {
      JitTerminalInstruction::Jump(block_id) => {
        pc = *block_id;
      }
      JitTerminalInstruction::ConditionalJump(cond) => {
        let condition = stack.pop_value()?;
        pc = if condition.is_truthy()? {
          cond.true_target
        } else {
          cond.false_target
        };
      }
      JitTerminalInstruction::RetWithValue => return Ok(Some(stack.pop_value()?)),
      JitTerminalInstruction::Ret => return Ok(None),
    }
  }
}
