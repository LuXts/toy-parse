use std::fmt;

use crate::token::*;
use crate::token_render::*;
use bigdecimal::BigDecimal;
use bigdecimal::Zero;

type Num = BigDecimal;
type Position = usize;

#[derive(Debug, Clone)]
pub struct ParseErr {
    pub reason: String,
    pub err_type: ParseErrType,
}

#[derive(Debug, Clone)]
pub enum ParseErrType {
    Unexpected(Token),
    Insufficient,
    Redundant(Position),
}

pub enum TraverseItem {
    Operator(Operator),
    Number(Num),
}

pub enum Operator {
    Add,   // 加
    Sub,   // 减
    Mul,   // 乘
    Div,   // 除
    Minus, // 负号
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
            Operator::Minus => write!(f, "@"),
        }
    }
}

impl fmt::Display for TraverseItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TraverseItem::Operator(op) => write!(f, "{}", op),
            TraverseItem::Number(n) => {
                write!(f, "{}", n.normalized().to_string())
            }
        }
    }
}

pub fn parse_sentence(render: &mut TokenRender) -> Result<Vec<TraverseItem>, ParseErr> {
    let mut output = vec![];
    a(render, &mut output)?;
    if render.is_empty() {
        return Ok(output);
    } else {
        return Err(ParseErr {
            reason: "有未能解析的输入".to_owned(),
            err_type: ParseErrType::Redundant(render.peek().position),
        });
    }
}

// parse_sentence => a1#
// a1 => m1 o1 a | m1
// m1 => at1 o2 m | at1
// at1 => -(a1) | (a1) | -num | num
// o1 => + | -
// o2 => * | /
// a => m o1 a | m
// m => at o2 m | at
// at => (a) | num

fn a(render: &mut TokenRender, output: &mut Vec<TraverseItem>) -> Result<(), ParseErr> {
    m(render, output, true)?;

    while !render.is_empty() {
        let op = o1(render);
        if op.is_err() {
            break;
        }
        m(render, output, false)?;
        output.push(TraverseItem::Operator(op.unwrap()));
    }
    return Ok(());
}

fn o1(render: &mut TokenRender) -> Result<Operator, ()> {
    if !render.is_empty() {
        if render.try_token(TokenInfo::Symbol(SymbolType::Add)) {
            return Ok(Operator::Add);
        }
        if render.try_token(TokenInfo::Symbol(SymbolType::Sub)) {
            return Ok(Operator::Sub);
        }
    }
    return Err(());
}

fn m(
    render: &mut TokenRender,
    output: &mut Vec<TraverseItem>,
    is_first: bool,
) -> Result<(), ParseErr> {
    at(render, output, is_first)?;

    while !render.is_empty() {
        let op = o2(render);
        if op.is_err() {
            break;
        }
        at(render, output, false)?;
        output.push(TraverseItem::Operator(op.unwrap()));
    }

    return Ok(());
}

fn o2(input: &mut TokenRender) -> Result<Operator, ()> {
    if !input.is_empty() {
        if input.try_token(TokenInfo::Symbol(SymbolType::Mul)) {
            return Ok(Operator::Mul);
        }
        if input.try_token(TokenInfo::Symbol(SymbolType::Div)) {
            return Ok(Operator::Div);
        }
    }
    return Err(());
}

fn at(
    render: &mut TokenRender,
    output: &mut Vec<TraverseItem>,
    is_first: bool,
) -> Result<(), ParseErr> {
    let mut is_neg = false;
    if !render.is_empty() {
        if is_first && render.try_token(TokenInfo::Symbol(SymbolType::Sub)) {
            is_neg = true;
        }
    }

    if !num(render, output) {
        if !render.is_empty() {
            render.expect(TokenInfo::Symbol(SymbolType::LeftBracket), |token| {
                return format!("期望获得 ( 或数字，却得到了 '{}' ", token.info).to_owned();
            })?;
            render.next();
            a(render, output)?;
            if !render.is_empty() {
                render.expect(TokenInfo::Symbol(SymbolType::RightBracket), |token| {
                    return format!("期望获得 )，却得到了 '{}' ", token.info).to_owned();
                })?;
                render.next();
            } else {
                return Err(ParseErr {
                    reason: "期望获得 ) ，却意外终止".to_owned(),
                    err_type: ParseErrType::Insufficient,
                });
            }
        } else {
            return Err(ParseErr {
                reason: "期望获得 ( 或数字，却意外终止".to_owned(),
                err_type: ParseErrType::Insufficient,
            });
        }
    }
    if is_neg {
        output.push(TraverseItem::Operator(Operator::Minus));
    }
    return Ok(());
}

fn num(render: &mut TokenRender, output: &mut Vec<TraverseItem>) -> bool {
    if !render.is_empty() {
        if let Ok(temp) = render.expect(TokenInfo::Number(Num::zero()), |_| {
            return "".to_owned();
        }) {
            if let TokenInfo::Number(n) = temp.info {
                render.next();
                output.push(TraverseItem::Number(n));
                return true;
            }
        }
    }
    return false;
}

#[cfg(test)]
mod test {
    use super::parse_token;
    use crate::sentence::parse_sentence;
    use crate::token_render::*;

    #[test]
    fn parse_test_success() {
        // 测试符合语法的内容
        let input_vec = vec![
            "56+88-9999",
            "1e9",
            "1.0",
            ".0",
            ".0+1",
            "3-2",
            "3-(-2)",
            "(-2)+3",
            "(12)+1",
            "2*3",
            "2*3-1",
            "3-2*1",
            "3*4*5/2",
            "3*4*5/(-2)",
            "-2",
            "-3+.5",
            "(-2)",
            "-(2)",
        ];
        for i in 0..input_vec.len() {
            let re = parse_token(input_vec[i]);
            assert!(re.is_ok());
            let re = parse_sentence(&mut TokenRender::new_with_tokens(re.unwrap()));
            assert!(re.is_ok(), "input: {}", input_vec[i]);
        }
    }

    #[test]
    fn parse_test_fail() {
        // 测试不符合语法的内容
        let input_vec = vec![
            "56+", "1e9-", "*1.0", "(", ")", "()", "(((2)", "3**3", "4-*2", "45(+6)", "4 5", "++",
            "--15", "-(+5)", "++++++1", "+1-", "+3", "3++2", "3--2",
        ];
        for i in 0..input_vec.len() {
            let re = parse_token(input_vec[i]);
            assert!(re.is_ok());
            let re = parse_sentence(&mut TokenRender::new_with_tokens(re.unwrap()));
            assert!(re.is_err(), "input: {}", input_vec[i]);
        }
    }
}
