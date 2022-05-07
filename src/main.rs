use translation::TraverseItem;

use crate::{sentence::parse_sentence, translation::translate_ast};

mod sentence;
mod token;
mod translation;

use bigdecimal::{BigDecimal, Zero};
type Num = BigDecimal;

use std::collections::VecDeque;

fn calculate(exp: Vec<TraverseItem>) -> Result<Num, String> {
    let mut stack: VecDeque<Num> = VecDeque::new();

    for item in exp {
        match item {
            TraverseItem::Operator(op) => match op {
                translation::Operator::Add => {
                    if let Some(right) = stack.pop_back() {
                        if let Some(left) = stack.pop_back() {
                            let temp = left + right;
                            stack.push_back(temp);
                            continue;
                        }
                    }
                    return Err("+ 出现未知错误！栈内数字数量不符！".to_owned());
                }
                translation::Operator::Sub => {
                    if let Some(right) = stack.pop_back() {
                        if let Some(left) = stack.pop_back() {
                            let temp = left - right;
                            stack.push_back(temp);
                            continue;
                        }
                    }
                    return Err("- 出现未知错误！栈内数字数量不符！".to_owned());
                }
                translation::Operator::Mul => {
                    if let Some(right) = stack.pop_back() {
                        if let Some(left) = stack.pop_back() {
                            let temp = left * right;
                            stack.push_back(temp);
                            continue;
                        }
                    }
                    return Err("* 出现未知错误！栈内数字数量不符！".to_owned());
                }
                translation::Operator::Div => {
                    if let Some(right) = stack.pop_back() {
                        if let Some(left) = stack.pop_back() {
                            if right == BigDecimal::zero() {
                                return Err("出现除 0 错误！".to_owned());
                            }
                            let temp = left / right;
                            stack.push_back(temp);
                            continue;
                        }
                    }
                    return Err("/ 出现未知错误！栈内数字数量不符！".to_owned());
                }
                translation::Operator::Minus => {
                    if let Some(temp) = stack.pop_back() {
                        let temp: BigDecimal = -temp;
                        stack.push_back(temp);
                        continue;
                    }
                    return Err("@ 出现未知错误！栈内数字数量不符！".to_owned());
                }
            },
            TraverseItem::Number(n) => stack.push_back(n),
        }
    }
    if stack.len() == 1 {
        let first = stack.pop_back().unwrap();
        Ok(first)
    } else {
        Err("出现未知错误！".to_owned())
    }
}

fn main() {
    let input = "2/(0+1)";
    let re = crate::token::parse_token(input);
    println!("{:#?}", re);
    if let Ok(mut t) = re {
        let re = parse_sentence(&mut t);
        println!("{:#?}", re);
        if let Ok(root) = re {
            let v = translate_ast(root);
            for item in &v {
                print!("{} ", item);
            }

            let re = calculate(v);
            println!("{:#?}", re);
        }
    }
}
