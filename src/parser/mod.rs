pub mod base;
pub mod char;
pub mod choice;
pub mod exclude;
pub mod exclude_empty;
pub mod extract;
pub mod filter;
pub mod flatten;
pub mod identity_map;
pub mod kind;
pub mod kind_ignore;
pub mod lazy;
pub mod many;
pub mod many1;
pub mod map;
pub mod opt;
pub mod regexp;
pub mod sep;
pub mod seq;
pub mod token;
pub mod trim;
pub mod type_map;
pub mod unwrap;
mod utils;
pub mod wrap;

pub use self::base::{DefaultType, Node, Parser, State, Type};
pub use self::char::build as char;
pub use self::choice::build as choice;
pub use self::exclude::build as exclude;
pub use self::exclude_empty::build as exclude_empty;
pub use self::extract::build as extract;
pub use self::filter::build as filter;
pub use self::flatten::build as flatten;
pub use self::identity_map::build as identity_map;
pub use self::kind::build as kind;
pub use self::kind_ignore::build as kind_ignore;
pub use self::lazy::build as lazy;
pub use self::many::build as many;
pub use self::many1::build as many1;
pub use self::map::build as map;
pub use self::opt::build as opt;
pub use self::regexp::build as regexp;
pub use self::sep::build as sep;
pub use self::seq::build as seq;
pub use self::token::build as token;
pub use self::trim::build as trim;
pub use self::type_map::build as type_map;
pub use self::unwrap::build as unwrap;
pub use self::wrap::build as wrap;

pub struct ParserCombinator<T: Clone = DefaultType> {
  pub parser: Box<Parser<T>>,
}

impl<T: Clone + Sized + 'static> ParserCombinator<T> {
  pub fn new<P: Parser<T>>(parser: &P) -> Self {
    ParserCombinator {
      parser: parser.box_clone(),
    }
  }

  pub fn parse(&self, target: &str) -> Result<Node<T>, String> {
    parse(self, target)
  }
}

impl<T: Clone + 'static> Parser<T> for ParserCombinator<T> {
  fn box_clone(&self) -> Box<Parser<T>> {
    Box::new(ParserCombinator {
      parser: self.parser.box_clone(),
    })
  }

  fn parse(&self, target: &str, position: usize) -> State<T> {
    self.parser.as_ref().parse(target, position)
  }
}

pub fn parse<T: Clone, P: Parser<T>>(parser: &P, target: &str) -> Result<Node<T>, String> {
  let result = parser.parse(target, 0);
  if result.success {
    if result.position == target.len() {
      if let Some(node) = result.node {
        return Ok(node);
      }
    } else {
      let (col, row) = utils::specify_position(target, result.position);
      let lines = utils::specify_lines(target, (col, row));
      return Err(format!("Parse Error: failed at {}:{}\n{}", col, row, lines));
    }
  }

  Err("Parse Error: failed at 1".to_string())
}

pub fn debug_parse<T: Clone, P: Parser<T>>(parser: &P, target: &str, position: usize) -> State<T> {
  parser.parse(target, position)
}
