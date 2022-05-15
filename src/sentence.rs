use std::rc::Rc;

use crate::token::*;

use bigdecimal::BigDecimal;
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
    let re = a(input);
    if re.is_err() {
        return Err(re.err().unwrap());
    }
    if input.is_empty() {
        return Ok(ASTRoot { data: re.unwrap() });
    } else {
        return Err(ParseErr {
            reason: "有未能解析的输入".to_owned(),
            err_type: ParseErrType::Redundant(input[0].position),
        });
    }
}

fn a(input: &mut Vec<Token>) -> Result<Rc<ASTNode>, ParseErr> {
    let re = m(input);
    if re.is_err() {
        return re;
    }
    let mut re = re.unwrap();
    while !input.is_empty() {
        let temp = input[0].to_owned();
        let op = o1(input);
        if op.is_err() {
            return Ok(re);
        }
        if input.is_empty() {
            return Err(ParseErr {
                reason: "期望获得数字 ，却意外终止".to_owned(),
                err_type: ParseErrType::Insufficient,
            });
        }
        let right = m(input);
        if right.is_err() {
            input.insert(0, temp);
            return Ok(re);
        }
        re = Rc::new(ASTNode::Expression(re, op.unwrap(), right.unwrap()));
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
        } else {
            return Err(());
        }
    } else {
        return Err(());
    }
}

fn m(input: &mut Vec<Token>) -> Result<Rc<ASTNode>, ParseErr> {
    let re = at(input);
    if re.is_err() {
        return re;
    }
    let mut re = re.unwrap();
    while !input.is_empty() {
        let temp = input[0].to_owned();
        let op = o2(input);
        if op.is_err() {
            return Ok(re);
        }
        if input.is_empty() {
            return Err(ParseErr {
                reason: "期望获得数字 ，却意外终止".to_owned(),
                err_type: ParseErrType::Insufficient,
            });
        }
        let right = at(input);
        if right.is_err() {
            input.insert(0, temp);
            return Ok(re);
        }
        re = Rc::new(ASTNode::Expression(re, op.unwrap(), right.unwrap()));
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
        } else {
            return Err(());
        }
    } else {
        return Err(());
    }
}

fn at(input: &mut Vec<Token>) -> Result<Rc<ASTNode>, ParseErr> {
    let re = num(input);
    if re.is_err() {
        if let Some(f) = input.get(0) {
            if let TokenInfo::Symbol(SymbolType::LeftBracket) = f.info {
                input.remove(0);
                let re = a(input);
                if re.is_err() {
                    return re;
                }
                if let Some(f) = input.get(0) {
                    if let TokenInfo::Symbol(SymbolType::RightBracket) = f.info {
                        input.remove(0);
                        let n = re.unwrap();
                        return Ok(n);
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
    }
    return re;
}

fn num(input: &mut Vec<Token>) -> Result<Rc<ASTNode>, ParseErr> {
    if let Some(f) = input.get(0) {
        match &f.info {
            TokenInfo::Number(n) => {
                let n = n.to_owned();
                input.remove(0);
                return Ok(Rc::new(ASTNode::Number(true, n)));
            }
            TokenInfo::Symbol(SymbolType::LeftBracket) => {
                if input.len() >= 4 {
                    if let TokenInfo::Symbol(SymbolType::Sub) = input[1].info {
                        if let TokenInfo::Number(n) = &input[2].info {
                            if let TokenInfo::Symbol(SymbolType::RightBracket) = input[3].info {
                                let n = n.to_owned();
                                input.remove(0);
                                input.remove(0);
                                input.remove(0);
                                input.remove(0);
                                return Ok(Rc::new(ASTNode::Number(false, n)));
                            }
                        }
                    }
                }

                return Err(ParseErr {
                    reason: "期望获得数字，却意外终止".to_owned(),
                    err_type: ParseErrType::Insufficient,
                });
            }
            _ => {
                return Err(ParseErr {
                    reason: format!("期望获得数字，却得到了 '{}' ", f.info).to_owned(),
                    err_type: ParseErrType::Unexpected(f.to_owned()),
                });
            }
        }
    } else {
        return Err(ParseErr {
            reason: "期望获得数字，却意外终止".to_owned(),
            err_type: ParseErrType::Insufficient,
        });
    }
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
            "--15", "-(5)", "-(+5)", "++++++1", "+1-", "-2", "+3", "3++2", "3--2",
        ];
        for i in 0..input_vec.len() {
            let re = parse_token(input_vec[i]);
            assert!(re.is_ok());
            let re = parse_sentence(&mut re.unwrap());
            assert!(re.is_err(), "input: {}", input_vec[i]);
        }
    }
}
