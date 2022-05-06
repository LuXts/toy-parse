use crate::token::*;

#[derive(Debug)]
pub struct ParseErr {
    pub reason: String,
    pub position: usize,
}

pub fn s(input: &mut Vec<Token>) -> Result<(), ParseErr> {
    let re = a(input);
    if re.is_err() {
        return re;
    }
    if input.is_empty() {
        return Ok(());
    } else {
        return Err(ParseErr {
            reason: "有额外的输入！".to_owned(),
            position: 0,
        });
    }
}

fn a(input: &mut Vec<Token>) -> Result<(), ParseErr> {
    let re = m(input);
    if re.is_err() {
        return re;
    }
    a1(input)
}

fn a1(input: &mut Vec<Token>) -> Result<(), ParseErr> {
    if input.is_empty() {
        return Ok(());
    }
    let re = o1(input);
    if re.is_err() {
        return Ok(());
    }
    let re = m(input);
    if re.is_err() {
        return re;
    }
    a1(input)
}

fn o1(input: &mut Vec<Token>) -> Result<(), ParseErr> {
    if let Some(f) = input.first() {
        if let TokenInfo::Symbol(SymbolType::Add) = f.info {
            input.remove(0);
            return Ok(());
        } else if let TokenInfo::Symbol(SymbolType::Sub) = f.info {
            input.remove(0);
            return Ok(());
        } else {
            return Err(ParseErr {
                reason: format!("期望获得 + 或 - ，却得到了 '{}' 。", f.info).to_owned(),
                position: f.position,
            });
        }
    } else {
        return Err(ParseErr {
            reason: "期望获得 + 或 - ，却意外终止。".to_owned(),
            position: 0,
        });
    }
}

fn m(input: &mut Vec<Token>) -> Result<(), ParseErr> {
    let re = at(input);
    if re.is_err() {
        return re;
    }
    m1(input)
}

fn m1(input: &mut Vec<Token>) -> Result<(), ParseErr> {
    if input.is_empty() {
        return Ok(());
    }
    let re = o2(input);
    if re.is_err() {
        return Ok(());
    }
    let re = at(input);
    if re.is_err() {
        return re;
    }
    m1(input)
}

fn o2(input: &mut Vec<Token>) -> Result<(), ParseErr> {
    if let Some(f) = input.first() {
        if let TokenInfo::Symbol(SymbolType::Mul) = f.info {
            input.remove(0);
            return Ok(());
        } else if let TokenInfo::Symbol(SymbolType::Div) = f.info {
            input.remove(0);
            return Ok(());
        } else {
            return Err(ParseErr {
                reason: format!("期望获得 * 或 / ，却得到了 '{}' 。", f.info).to_owned(),
                position: f.position,
            });
        }
    } else {
        return Err(ParseErr {
            reason: "期望获得 * 或 / ，却意外终止。".to_owned(),
            position: 0,
        });
    }
}

fn at(input: &mut Vec<Token>) -> Result<(), ParseErr> {
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
                        return Ok(());
                    } else {
                        return Err(ParseErr {
                            reason: format!("期望获得 ) ，却得到了 '{}' 。", f.info).to_owned(),
                            position: f.position,
                        });
                    }
                } else {
                    return Err(ParseErr {
                        reason: "期望获得 ) ，却意外终止。".to_owned(),
                        position: 0,
                    });
                }
            } else {
                return Err(ParseErr {
                    reason: format!("期望获得 ( 或数字，却得到了 '{}' 。", f.info).to_owned(),
                    position: f.position,
                });
            }
        }
    }
    return re;
}

fn num(input: &mut Vec<Token>) -> Result<(), ParseErr> {
    if let Some(f) = input.get(0) {
        match f.info {
            TokenInfo::Number(_) => {
                input.remove(0);
                return Ok(());
            }
            TokenInfo::Symbol(SymbolType::Add) => {
                if let Some(f) = input.get(1) {
                    if let TokenInfo::Number(_) = f.info {
                        input.remove(0);
                        input.remove(0);
                        return Ok(());
                    }
                }
                return Err(ParseErr {
                    reason: "期望获得数字，却意外终止。".to_owned(),
                    position: 0,
                });
            }
            TokenInfo::Symbol(SymbolType::Sub) => {
                if let Some(f) = input.get(1) {
                    if let TokenInfo::Number(_) = f.info {
                        input.remove(0);
                        input.remove(0);
                        return Ok(());
                    }
                }
                return Err(ParseErr {
                    reason: "期望获得数字，却意外终止。".to_owned(),
                    position: 0,
                });
            }
            _ => {
                return Err(ParseErr {
                    reason: format!("期望获得（+/-）数字，却得到了 '{}' 。", f.info).to_owned(),
                    position: f.position,
                });
            }
        }
    } else {
        return Err(ParseErr {
            reason: "期望获得（+/-）数字，却意外终止。".to_owned(),
            position: 0,
        });
    }
}
