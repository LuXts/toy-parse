use crate::{sentence::parse_sentence, translation::translate_ast};

mod sentence;
mod token;
mod translation;

fn main() {
    let input = "34/31*(2)--3";
    let re = crate::token::parse_token(input);
    println!("{:#?}", re);
    if let Ok(mut t) = re {
        let re = parse_sentence(&mut t);
        println!("{:#?}", re);
        if let Ok(root) = re {
            let v = translate_ast(root);
            for item in v {
                print!("{} ", item);
            }
        }
    }
}
