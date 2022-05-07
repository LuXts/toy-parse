use std::collections::VecDeque;

use crate::translation::{self, TraverseItem};

use bigdecimal::{BigDecimal, Zero};
type Num = BigDecimal;

pub fn calculate(exp: &Vec<TraverseItem>) -> Result<Num, String> {
    let mut stack: VecDeque<Num> = VecDeque::new();

    for item in exp {
        match item {
            TraverseItem::Operator(op) => match op {
                translation::Operator::Add => match (stack.pop_back(), stack.pop_back()) {
                    (Some(right), Some(left)) => {
                        let temp = left + right;
                        stack.push_back(temp);
                        continue;
                    }
                    _ => {
                        return Err("出现未知错误！栈内数字数量不符".to_owned());
                    }
                },
                translation::Operator::Sub => match (stack.pop_back(), stack.pop_back()) {
                    (Some(right), Some(left)) => {
                        let temp = left - right;
                        stack.push_back(temp);
                        continue;
                    }
                    _ => {
                        return Err("出现未知错误！栈内数字数量不符".to_owned());
                    }
                },
                translation::Operator::Mul => match (stack.pop_back(), stack.pop_back()) {
                    (Some(right), Some(left)) => {
                        let temp = left * right;
                        stack.push_back(temp);
                        continue;
                    }
                    _ => {
                        return Err("出现未知错误！栈内数字数量不符".to_owned());
                    }
                },
                translation::Operator::Div => match (stack.pop_back(), stack.pop_back()) {
                    (Some(right), Some(left)) => {
                        if right == BigDecimal::zero() {
                            return Err("出现除 0 错误".to_owned());
                        }
                        let temp = left / right;
                        stack.push_back(temp);
                        continue;
                    }
                    _ => {
                        return Err("出现未知错误！栈内数字数量不符".to_owned());
                    }
                },
                translation::Operator::Minus => {
                    if let Some(temp) = stack.pop_back() {
                        let temp: BigDecimal = -temp;
                        stack.push_back(temp);
                        continue;
                    }
                    return Err("出现未知错误！栈内数字数量不符".to_owned());
                }
            },
            TraverseItem::Number(n) => stack.push_back(n.to_owned()),
        }
    }
    if stack.len() == 1 {
        let first = stack.pop_back().unwrap();
        Ok(first)
    } else {
        Err("出现未知错误".to_owned())
    }
}
