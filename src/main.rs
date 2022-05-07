use crate::{sentence::parse_sentence, token::parse_token, translation::translate_ast};

mod calculate;
mod sentence;
mod token;
mod translation;

use bigdecimal::ToPrimitive;

fn parse_and_run(input: &str) -> Result<String, String> {
    println!("输入字符串为    ||    '{}'", &input);
    match parse_token(input) {
        Ok(mut t) => match parse_sentence(&mut t) {
            Ok(root) => {
                let v = translate_ast(root);
                print!("逆波兰式为      ||    [ ");
                for item in &v {
                    print!("{} ", item);
                }
                println!("]");
                let re = calculate::calculate(&v);
                match re {
                    Ok(n) => {
                        return Ok(
                            format!("计算结果为      ||    '{}' ", n.to_f64().unwrap()).to_owned()
                        )
                    }
                    Err(e) => return Err(format!("计算结果为      ||    {}!", e)),
                }
            }
            Err(e) => match e.err_type {
                sentence::ParseErrType::Unexpected(e2) => {
                    return Err(format!(
                        "语法分析阶段    ||    在输入的第 {} 个字符处遇到错误: {}，输入处原字符串为: '{}' 。",
                        e2.position + 1,
                        e.reason,
                        e2.original
                    )
                    .to_owned());
                }
                sentence::ParseErrType::Insufficient => {
                    return Err(
                        format!("语法分析阶段    ||    输入末尾遇到错误: {} !", e.reason)
                            .to_owned(),
                    );
                }
                sentence::ParseErrType::Redundant(e2) => {
                    return Err(format!(
                            "语法分析阶段    ||    在输入的第 {} 个字符处遇到错误: {}，未能解析的输入：'{}' 。",
                            e2+1,
                            e.reason,
                            &input[e2..]
                        )
                        .to_owned());
                }
            },
        },
        Err(e) => {
            return Err(format!(
                "词法分析阶段    ||    在输入的第 {} 个字符处：{}",
                e.position + 1,
                e.reason
            )
            .to_owned());
        }
    }
}

fn main() {
    let input_vec = vec![
        "(1+1)*3   ",
        "-1e8+3/   .7",
        "1e",
        "3 + 0.005a",
        "2 + 3.3.3",
        "2 2",
        "*1",
        "1*****1",
        "1）",
        "()",
        "(1",
        "(1+",
        "-(1)",
        "1/0",
    ];

    for input in input_vec {
        println!("");
        match parse_and_run(input) {
            Ok(e) => println!("{}", e),
            Err(e) => println!("{}", e),
        }
    }
}
