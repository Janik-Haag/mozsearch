use rnix::SyntaxKind;
use rowan::{ast::AstNode, WalkEvent};
use std::fs;

#[derive(Debug)]
#[allow(non_snake_case)]
struct Source {
    syntax: String,
    pretty: String,
    no_crossref: Option<String>,
    nestingRange: Option<String>, // upstream doesn't have consistent casing

    // Experimental fields
    r#type: Option<String>,
    typesym: Option<String>,
}

#[derive(Debug)]
#[allow(non_snake_case)]
struct Target {
    kind: String,
    pretty: String,
    context: Option<String>,
    contextsym: Option<String>,
    peekRange: Option<String>, // upstream doesn't have consistent casing
}

fn main() {
    let file = "/dev/stdin";
    let src = fs::read_to_string(file).unwrap();
    let ast = rnix::Root::parse(&src);
    let root = ast.ok().expect("failed to parse input");
    let mut preordered = root.syntax().preorder();

    let mut entries = vec![];

    while let Some(ev) = preordered.next() {
        match ev {
            WalkEvent::Enter(n) if n.kind() == SyntaxKind::NODE_LET_IN => entries.push(n),
            _ => (),
        }
    }
    println!("{:#?}", entries);
}
