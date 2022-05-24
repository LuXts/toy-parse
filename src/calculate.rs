use crate::translation::{self, TraverseItem};
use bigdecimal::{BigDecimal, Zero};
use std::collections::VecDeque;

type Num = BigDecimal;

pub fn calculate(exp: &Vec<TraverseItem>) -> Result<Num, String> {
    let mut stack: VecDeque<Num> = VecDeque::new();

    for item in exp {
        match item {
            TraverseItem::Operator(op) => {
                match op {
                    translation::Operator::Add => {
                        if let (Some(right), Some(left)) = (stack.pop_back(), stack.pop_back()) {
                            let temp = left + right;
                            stack.push_back(temp);
                            continue;
                        }
                    }
                    translation::Operator::Sub => {
                        if let (Some(right), Some(left)) = (stack.pop_back(), stack.pop_back()) {
                            let temp = left - right;
                            stack.push_back(temp);
                            continue;
                        }
                    }
                    translation::Operator::Mul => {
                        if let (Some(right), Some(left)) = (stack.pop_back(), stack.pop_back()) {
                            let temp = left * right;
                            stack.push_back(temp);
                            continue;
                        }
                    }
                    translation::Operator::Div => {
                        if let (Some(right), Some(left)) = (stack.pop_back(), stack.pop_back()) {
                            if right == BigDecimal::zero() {
                                return Err("出现除 0 错误".to_owned());
                            }
                            let temp = left / right;
                            stack.push_back(temp);
                            continue;
                        }
                    }
                    translation::Operator::Minus => {
                        if let Some(temp) = stack.pop_back() {
                            let temp: BigDecimal = -temp;
                            stack.push_back(temp);
                            continue;
                        }
                    }
                };
                return Err("出现未知错误！栈内数字数量不符".to_owned());
            }
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
