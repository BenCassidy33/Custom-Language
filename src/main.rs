#![allow(warnings)]
use custom_language::{ast::generation::Ast, lexer::Lexer};

fn main() {
    let test_input3: String = r#"
// Test program for lexer/parser

type Color = enum : int {
    Red = 0,
    Green = 1,
    Blue = 2
}

type Point = struct {
    x: Float,
    y: Float
}

type Metadata = DuckType {
    id: String,
    tags: Map[String: String]
}

fun distance(p1: Point, p2: Point): Float do
    let dx: Float = p1.x - p2.x;
    let dy: Float = p1.y - p2.y;
    return dx * dx + dy * dy;
end

fun print_if_close(p1: Point, p2: Point) do
    if distance(p1, p2) < 10.0 do
        print("Close points!");
    end
end

let origin: Point = {
    x: 0.0,
    y: 0.0
}

let info: Metadata = {
    id: "abc123",
    tags: {
        "type": "test",
        "status": "active"
    }
}
"#
    .into();

    let test_input1: String = r#"
type Color = enum : int {
    Red = 0,
    Green = 1,
    Blue = 2
}
"#
    .into();

    let mut lex = Lexer::new(test_input1);
    let tokens = lex.generate_tokens();
    let mut ast = Ast::new(tokens.to_vec());
    println!("Group: {:?}", ast.consume_group());
}
