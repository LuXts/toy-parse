use crate::parse::s;

mod parse;
mod token;

fn main() {
    let input = "1-2*(-2)/2333";
    let re = crate::token::parse_token(input);
    println!("{:#?}", re);
    if let Ok(mut t) = re {
        let re = s(&mut t);
        println!("{:#?}", re);
    }
}
