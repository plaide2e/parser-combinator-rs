use crate::parser::base::Parser;
use crate::parser::map;
use crate::parser::{ExtractMap, Many, Seq};

/**
 *  Trim "a" by "-":
 *    "--a-" -> "a"
 */
pub fn build<T: Clone + 'static, P: Parser<T>, B: Parser<T>>(parser: &P, by: &B) -> map::Map<T> {
  ExtractMap(&Seq(&Many(by)).and(parser).and(&Many(by)), 1)
}
