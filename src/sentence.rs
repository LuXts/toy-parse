use crate::token::*;
use bigdecimal::BigDecimal;
use std::rc::Rc;

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

#[derive(Debug)]
pub struct ASTRoot {
    pub data: Rc<ASTNode>,
}
#[derive(Debug)]
pub enum ASTNode {
    Expression(Rc<ASTNode>, OperatorType, Rc<ASTNode>),
    Number(bool, Num),
}

#[derive(Debug, Clone)]
pub enum OperatorType {
    Add, // 加
    Sub, // 减
    Mul, // 乘
    Div, // 除
}

pub fn parse_sentence(input: &mut Vec<Token>) -> Result<ASTRoot, ParseErr> {
    let re = a(input)?;
    if input.is_empty() {
        return Ok(ASTRoot { data: re });
    } else {
        return Err(ParseErr {
            reason: "有未能解析的输入".to_owned(),
            err_type: ParseErrType::Redundant(input[0].position),
        });
    }
}

// parse_sentence => a#
// a1 => m1 o1 a | m1
// m1 => at1 o2 m | at1
// at1 => (a1) | -num | num
// o1 => + | -
// o2 => * | /
// a => m o1 a | m
// m => at o2 m | at
// at => (a) | num

fn a(input: &mut Vec<Token>) -> Result<Rc<ASTNode>, ParseErr> {
    let mut re = m(input, true)?;

    while !input.is_empty() {
        let op = o1(input);
        if op.is_err() {
            break;
        }
        let right_re = m(input, false);
        if let Ok(right) = right_re {
            re = Rc::new(ASTNode::Expression(re, op.unwrap(), right));
        } else {
            return right_re;
        }
    }
    Ok(re)
}

fn o1(input: &mut Vec<Token>) -> Result<OperatorType, ()> {
    if let Some(f) = input.first() {
        if let TokenInfo::Symbol(SymbolType::Add) = f.info {
            input.remove(0);
            return Ok(OperatorType::Add);
        } else if let TokenInfo::Symbol(SymbolType::Sub) = f.info {
            input.remove(0);
            return Ok(OperatorType::Sub);
        }
    }
    return Err(());
}

fn m(input: &mut Vec<Token>, is_first: bool) -> Result<Rc<ASTNode>, ParseErr> {
    let mut re = at(input, is_first)?;

    while !input.is_empty() {
        let op = o2(input);
        if op.is_err() {
            break;
        }
        let right_re = at(input, false);
        if let Ok(right) = right_re {
            re = Rc::new(ASTNode::Expression(re, op.unwrap(), right));
        } else {
            return right_re;
        }
    }

    Ok(re)
}

fn o2(input: &mut Vec<Token>) -> Result<OperatorType, ()> {
    if let Some(f) = input.first() {
        if let TokenInfo::Symbol(SymbolType::Mul) = f.info {
            input.remove(0);
            return Ok(OperatorType::Mul);
        } else if let TokenInfo::Symbol(SymbolType::Div) = f.info {
            input.remove(0);
            return Ok(OperatorType::Div);
        }
    }
    return Err(());
}

fn at(input: &mut Vec<Token>, is_first: bool) -> Result<Rc<ASTNode>, ParseErr> {
    let re = num(input, is_first);
    if let Ok(result) = re {
        return Ok(result);
    } else {
        if let Some(f) = input.get(0) {
            if let TokenInfo::Symbol(SymbolType::LeftBracket) = f.info {
                input.remove(0);
                let re = a(input)?;
                if let Some(f) = input.get(0) {
                    if let TokenInfo::Symbol(SymbolType::RightBracket) = f.info {
                        input.remove(0);
                        return Ok(re);
                    } else {
                        return Err(ParseErr {
                            reason: format!("期望获得 ) ，却得到了 '{}' ", f.info).to_owned(),
                            err_type: ParseErrType::Unexpected(f.to_owned()),
                        });
                    }
                } else {
                    return Err(ParseErr {
                        reason: "期望获得 ) ，却意外终止".to_owned(),
                        err_type: ParseErrType::Insufficient,
                    });
                }
            } else {
                return Err(ParseErr {
                    reason: format!("期望获得 ( 或数字，却得到了 '{}' ", f.info).to_owned(),
                    err_type: ParseErrType::Unexpected(f.to_owned()),
                });
            }
        }
        return Err(ParseErr {
            reason: "期望获得 ( 或数字，却意外终止".to_owned(),
            err_type: ParseErrType::Insufficient,
        });
    }
}

fn num(input: &mut Vec<Token>, is_first: bool) -> Result<Rc<ASTNode>, ()> {
    if let Some(f) = input.get(0) {
        match &f.info {
            TokenInfo::Number(n) => {
                let n = n.to_owned();
                input.remove(0);
                return Ok(Rc::new(ASTNode::Number(true, n)));
            }
            TokenInfo::Symbol(SymbolType::Sub) => {
                if is_first {
                    if input.len() >= 2 {
                        if let TokenInfo::Number(n) = &input[1].info {
                            let n = n.to_owned();
                            input.remove(0);
                            input.remove(0);
                            return Ok(Rc::new(ASTNode::Number(false, n)));
                        }
                    }
                }
            }
            _ => {}
        }
    }
    return Err(());
}

#[cfg(test)]
mod test {
    use crate::sentence::parse_sentence;

    use super::parse_token;

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
        ];
        for i in 0..input_vec.len() {
            let re = parse_token(input_vec[i]);
            assert!(re.is_ok());
            let re = parse_sentence(&mut re.unwrap());
            assert!(re.is_ok(), "input: {}", input_vec[i]);
        }
    }

    #[test]
    fn parse_test_fail() {
        // 测试不符合语法的内容
        let input_vec = vec![
            "56+", "1e9-", "*1.0", "(", ")", "()", "(((2)", "3**3", "4-*2", "45(+6)", "4 5", "++",
            "--15", "-(5)", "-(+5)", "++++++1", "+1-", "+3", "3++2", "3--2", "-(2)",
        ];
        for i in 0..input_vec.len() {
            let re = parse_token(input_vec[i]);
            assert!(re.is_ok());
            let re = parse_sentence(&mut re.unwrap());
            assert!(re.is_err(), "input: {}", input_vec[i]);
        }
    }
}
