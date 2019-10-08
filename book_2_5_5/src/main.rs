use book_2_5_5::parser::Parser;

fn main() {
    let mut parser = Parser::init("9-3+2-2+9");
    println!("{:?}", parser);
    match parser.expr() {
        Err(e) => println!("Error : {:?}", e),
        _ => {}
    };
}
