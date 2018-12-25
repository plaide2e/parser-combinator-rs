mod base;
mod char;
mod choice;
mod flatten_map;
mod lazy;
mod many;
mod map;
mod opt;
mod regex;
mod seq;
mod token;
mod wrap_map;

pub use self::base::{Node, Parser, State};
pub use self::char::build as Char;
pub use self::choice::build as Choice;
pub use self::flatten_map::build as FlattenMap;
pub use self::lazy::build as Lazy;
pub use self::many::build as Many;
pub use self::map::build as Map;
pub use self::opt::build as Opt;
pub use self::regex::build as RegExp;
pub use self::seq::build as Seq;
pub use self::token::build as Token;
pub use self::wrap_map::build as WrapMap;

pub fn parse<P: Parser>(parser: &P, target: &str) -> Result<Node, String> {
  let result = parser.parse(target, 0);
  if result.success {
    if let Some(node) = result.node {
      return Ok(node);
    }
  }

  Err("Couldn't parse".to_string())
}
