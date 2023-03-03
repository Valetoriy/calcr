use calcr::lexer;

fn main() {
    let s = "((12 + 47) / (16 * 1 - 4^3))";
    let res = lexer::lex(&s);
    println!("{res:#?}");
}
