use cknittel_util::peekable_stream::PeekableStream;

pub struct CharStream<I: Iterator<Item = char>> {
  iter: PeekableStream<I>,
}
