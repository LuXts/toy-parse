mod token;

fn main() {
    let input = "   a";
    let re = crate::token::parse_token(input);
    println!("{:#?}", re);
}
