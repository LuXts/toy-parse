use std::{collections::VecDeque, fmt, rc::Rc};

use crate::sentence::*;
use bigdecimal::{BigDecimal, ToPrimitive};
type Num = BigDecimal;

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
            TraverseItem::Number(n) => write!(f, "{}", n.to_f64().unwrap()),
        }
    }
}

#[derive(Debug, Clone)]
enum TraverseASTNode {
    Expression(Rc<ASTNode>),
    Operator(OperatorType),
    Number(bool, Num),
}

pub fn translate_ast(root: ASTRoot) -> Vec<TraverseItem> {
    let root = root;
    let mut unprocessed_node = VecDeque::new();
    let mut traverse_vec = Vec::new();
    match root.data.as_ref() {
        ASTNode::Expression(left, op, right) => {
            unprocessed_node.push_back(TraverseASTNode::Operator(op.to_owned()));
            unprocessed_node.push_back(TraverseASTNode::Expression(right.clone()));
            unprocessed_node.push_back(TraverseASTNode::Expression(left.clone()));
        }
        ASTNode::Number(mark, n) => {
            traverse_vec.push(TraverseItem::Number(n.to_owned()));
            if !mark {
                traverse_vec.push(TraverseItem::Operator(Operator::Minus));
            }
        }
    }
    while !unprocessed_node.is_empty() {
        let temp = unprocessed_node.pop_back().unwrap();
        match temp {
            TraverseASTNode::Number(mark, n) => {
                traverse_vec.push(TraverseItem::Number(n));
                if !mark {
                    traverse_vec.push(TraverseItem::Operator(Operator::Minus));
                }
            }
            TraverseASTNode::Expression(node) => match node.as_ref() {
                ASTNode::Expression(left, op, right) => {
                    unprocessed_node.push_back(TraverseASTNode::Operator(op.to_owned()));
                    unprocessed_node.push_back(TraverseASTNode::Expression(right.clone()));
                    unprocessed_node.push_back(TraverseASTNode::Expression(left.clone()));
                }
                ASTNode::Number(mark, n) => {
                    unprocessed_node
                        .push_back(TraverseASTNode::Number(mark.to_owned(), n.to_owned()));
                }
            },
            TraverseASTNode::Operator(op) => match op {
                OperatorType::Add => traverse_vec.push(TraverseItem::Operator(Operator::Add)),
                OperatorType::Sub => traverse_vec.push(TraverseItem::Operator(Operator::Sub)),
                OperatorType::Mul => traverse_vec.push(TraverseItem::Operator(Operator::Mul)),
                OperatorType::Div => traverse_vec.push(TraverseItem::Operator(Operator::Div)),
            },
        }
    }
    traverse_vec
}
