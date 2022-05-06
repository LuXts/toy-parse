use crate::token::*;

use bigdecimal::BigDecimal;
type Num = BigDecimal;
type Position = usize;
#[derive(Debug)]
pub struct ParseErr {
    pub reason: String,
    pub err_type: ParseErrType,
}
#[derive(Debug)]
pub enum ParseErrType {
    Unexpected(Position),
    Insufficient,
    Redundant(Position),
}

#[derive(Debug)]
pub struct ASTRoot {
    pub root: Box<ASTNode>,
}
#[derive(Debug)]
pub enum ASTNode {
    Expression(Box<ASTNode>, OperatorType, Box<ASTNode>),
    Number(bool, Num),
}

#[derive(Debug)]
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
        return Ok(ASTRoot { root: re.unwrap() });
    } else {
        return Err(ParseErr {
            reason: "有额外的输入！".to_owned(),
            err_type: ParseErrType::Redundant(input[0].position),
        });
    }
}

fn a(input: &mut Vec<Token>) -> Result<Box<ASTNode>, ParseErr> {
    let re = m(input);
    if re.is_err() {
        return re;
    }
    let mut re = re.unwrap();
    while !input.is_empty() {
        let op = o1(input);
        if op.is_err() {
            return Ok(re);
        }
        let right = m(input);
        if right.is_err() {
            return Ok(re);
        }
        re = Box::new(ASTNode::Expression(re, op.unwrap(), right.unwrap()));
    }
    Ok(re)
}

fn o1(input: &mut Vec<Token>) -> Result<OperatorType, ParseErr> {
    if let Some(f) = input.first() {
        if let TokenInfo::Symbol(SymbolType::Add) = f.info {
            input.remove(0);
            return Ok(OperatorType::Add);
        } else if let TokenInfo::Symbol(SymbolType::Sub) = f.info {
            input.remove(0);
            return Ok(OperatorType::Sub);
        } else {
            return Err(ParseErr {
                reason: format!("期望获得 + 或 - ，却得到了 '{}' 。", f.info).to_owned(),
                err_type: ParseErrType::Unexpected(f.position),
            });
        }
    } else {
        return Err(ParseErr {
            reason: "期望获得 + 或 - ，却意外终止。".to_owned(),
            err_type: ParseErrType::Insufficient,
        });
    }
}

fn m(input: &mut Vec<Token>) -> Result<Box<ASTNode>, ParseErr> {
    let re = at(input);
    if re.is_err() {
        return re;
    }
    let mut re = re.unwrap();
    while !input.is_empty() {
        let op = o2(input);
        if op.is_err() {
            return Ok(re);
        }
        let right = at(input);
        if right.is_err() {
            return Ok(re);
        }
        re = Box::new(ASTNode::Expression(re, op.unwrap(), right.unwrap()));
    }
    Ok(re)
}

fn o2(input: &mut Vec<Token>) -> Result<OperatorType, ParseErr> {
    if let Some(f) = input.first() {
        if let TokenInfo::Symbol(SymbolType::Mul) = f.info {
            input.remove(0);
            return Ok(OperatorType::Mul);
        } else if let TokenInfo::Symbol(SymbolType::Div) = f.info {
            input.remove(0);
            return Ok(OperatorType::Div);
        } else {
            return Err(ParseErr {
                reason: format!("期望获得 * 或 / ，却得到了 '{}' 。", f.info).to_owned(),
                err_type: ParseErrType::Unexpected(f.position),
            });
        }
    } else {
        return Err(ParseErr {
            reason: "期望获得 * 或 / ，却意外终止。".to_owned(),
            err_type: ParseErrType::Insufficient,
        });
    }
}

fn at(input: &mut Vec<Token>) -> Result<Box<ASTNode>, ParseErr> {
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
                            reason: format!("期望获得 ) ，却得到了 '{}' 。", f.info).to_owned(),
                            err_type: ParseErrType::Unexpected(f.position),
                        });
                    }
                } else {
                    return Err(ParseErr {
                        reason: "期望获得 ) ，却意外终止。".to_owned(),
                        err_type: ParseErrType::Insufficient,
                    });
                }
            } else {
                return Err(ParseErr {
                    reason: format!("期望获得 ( 或数字，却得到了 '{}' 。", f.info).to_owned(),
                    err_type: ParseErrType::Unexpected(f.position),
                });
            }
        }
    }
    return re;
}

fn num(input: &mut Vec<Token>) -> Result<Box<ASTNode>, ParseErr> {
    if let Some(f) = input.get(0) {
        match &f.info {
            TokenInfo::Number(n) => {
                let n = n.to_owned();
                input.remove(0);
                return Ok(Box::new(ASTNode::Number(true, n)));
            }
            TokenInfo::Symbol(SymbolType::Add) => {
                if let Some(f) = input.get(1) {
                    if let TokenInfo::Number(n) = &f.info {
                        let n = n.to_owned();
                        input.remove(0);
                        input.remove(0);
                        return Ok(Box::new(ASTNode::Number(true, n)));
                    }
                }
                return Err(ParseErr {
                    reason: "期望获得数字，却意外终止。".to_owned(),
                    err_type: ParseErrType::Insufficient,
                });
            }
            TokenInfo::Symbol(SymbolType::Sub) => {
                if let Some(f) = input.get(1) {
                    if let TokenInfo::Number(n) = &f.info {
                        let n = n.to_owned();
                        input.remove(0);
                        input.remove(0);
                        return Ok(Box::new(ASTNode::Number(false, n)));
                    }
                }
                return Err(ParseErr {
                    reason: "期望获得数字，却意外终止。".to_owned(),
                    err_type: ParseErrType::Insufficient,
                });
            }
            _ => {
                return Err(ParseErr {
                    reason: format!("期望获得数字，却得到了 '{}' 。", f.info).to_owned(),
                    err_type: ParseErrType::Unexpected(f.position),
                });
            }
        }
    } else {
        return Err(ParseErr {
            reason: "期望获得数字，却意外终止。".to_owned(),
            err_type: ParseErrType::Insufficient,
        });
    }
}
