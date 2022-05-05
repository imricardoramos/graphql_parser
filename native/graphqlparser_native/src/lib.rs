use rustler::types::atom;
use rustler::{Atom, Error};

mod datatypes;

pub use datatypes::Document;

#[rustler::nif(schedule = "DirtyCpu")]
fn parse_query<'a>(doc: String) -> Result<(Atom, Document), Error> {
    let ast = match graphql_parser::parse_query::<String>(doc.as_str()) {
        Ok(ast) => Ok((atom::ok(), Document::new(ast))),
        Err(e) => Err(Error::Term(Box::new(e.to_string()))),
    };
    return ast;
}

rustler::init!("Elixir.GraphQLParser.Native", [parse_query]);
