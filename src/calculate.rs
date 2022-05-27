use crate::sentence::{self, RPNItem};
use bigdecimal::{BigDecimal, Zero};
use std::collections::VecDeque;

type Num = BigDecimal;

/**
计算函数。

* `exp` 输入的逆波兰式数组
 */
pub fn calculate(exp: &Vec<RPNItem>) -> Result<Num, String> {
    // 运算栈
    let mut stack: VecDeque<Num> = VecDeque::new();

    // 遍历输入
    for item in exp {
        // 检查 `item` 的类型
        match item {
            RPNItem::Operator(op) => {
                // 如果是运算符，就根据运算符的类型取出运算栈中的数字进行操作
                // 操作完成之后把结果 push_back 回运算栈
                match op {
                    sentence::Operator::Add => {
                        if let (Some(right), Some(left)) = (stack.pop_back(), stack.pop_back()) {
                            let temp = left + right;
                            stack.push_back(temp);
                            continue;
                        }
                    }
                    sentence::Operator::Sub => {
                        if let (Some(right), Some(left)) = (stack.pop_back(), stack.pop_back()) {
                            let temp = left - right;
                            stack.push_back(temp);
                            continue;
                        }
                    }
                    sentence::Operator::Mul => {
                        if let (Some(right), Some(left)) = (stack.pop_back(), stack.pop_back()) {
                            let temp = left * right;
                            stack.push_back(temp);
                            continue;
                        }
                    }
                    sentence::Operator::Div => {
                        if let (Some(right), Some(left)) = (stack.pop_back(), stack.pop_back()) {
                            if right == BigDecimal::zero() {
                                return Err("出现除 0 错误".to_owned());
                            }
                            let temp = left / right;
                            stack.push_back(temp);
                            continue;
                        }
                    }
                    sentence::Operator::Minus => {
                        if let Some(temp) = stack.pop_back() {
                            let temp: BigDecimal = -temp;
                            stack.push_back(temp);
                            continue;
                        }
                    }
                };
                // 如果运行到这里了，证明输入的逆波兰式有问题，应该排查上一步的语法分析。

                return Err("出现未知错误！栈内数字数量不符".to_owned());
            }
            RPNItem::Number(n) => {
                // 如果是数字类型，直接 push_back 进入运算栈。
                stack.push_back(n.to_owned());
            }
        }
    }

    // 检查运算栈中剩下的数字
    if stack.len() == 1 {
        // 取出最后一个数字返回
        let first = stack.pop_back().unwrap();

        return Ok(first);
    } else {
        // 如果运行到这里了，证明输入的逆波兰式有问题，应该排查上一步的语法分析。

        return Err("出现未知错误，运算栈里面剩余的数字不对".to_owned());
    }
}
