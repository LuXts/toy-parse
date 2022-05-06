use crate::sentence::parse_sentence;

mod sentence;
mod token;

fn main() {
    let input = "2 2";
    let re = crate::token::parse_token(input);
    println!("{:#?}", re);
    if let Ok(mut t) = re {
        let re = parse_sentence(&mut t);
        println!("{:#?}", re);
    }
}
