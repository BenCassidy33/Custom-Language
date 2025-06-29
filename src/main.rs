#![allow(warnings)]
use custom_language::lexer::Lexer;

fn main() {
    let test_input: String = "let x = 5".into();
    let test_input2: String = r"
fun print(message: String) do 
    for char in message do
        putchar(char)
    end
end"
    .into();

    let mut lex = Lexer::new(test_input);
    //lex.generate_tokens();
    println!("{:?}", lex.generate_tokens());

    lex = Lexer::new(test_input2);
    //println!("{:?}", lex.generate_tokens());

    //println!("{:#?}\n\n", Lexer::new(test_input).generate_tokens());
    //println!("{:#?}", Lexer::new(test_input2).generate_tokens());
}
