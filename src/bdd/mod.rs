mod trans;
pub use trans::*;

use crate::{ast::Expr, Prefix, Smv};
use cudd::{Cudd, DdNode};
use std::collections::HashMap;

#[derive(Clone)]
pub struct SmvBdd {
    pub symbols: HashMap<String, usize>,
    pub cudd: Cudd,
    pub trans: SmvTransBdd,
    pub init: DdNode,
}

pub fn expr_to_bdd(cudd: &Cudd, symbols: &HashMap<String, usize>, expr: &Expr) -> DdNode {
    let ans = match expr {
        Expr::Ident(ident) => cudd.ith_var(symbols[ident]),
        Expr::LitExpr(lit) => cudd.constant(*lit),
        Expr::PrefixExpr(op, sub_expr) => {
            let expr_bdd = expr_to_bdd(cudd, symbols, sub_expr);
            match op {
                crate::ast::Prefix::Not => !expr_bdd,
                crate::ast::Prefix::Next => expr_bdd.next_state(),
                _ => todo!(),
            }
        }
        Expr::InfixExpr(op, left, right) => {
            let left_bdd = expr_to_bdd(cudd, symbols, left);
            let right_bdd = expr_to_bdd(cudd, symbols, right);
            match op {
                crate::ast::Infix::And => left_bdd & right_bdd,
                crate::ast::Infix::Or => left_bdd | right_bdd,
                crate::ast::Infix::Imply => !left_bdd | right_bdd,
                crate::ast::Infix::Iff => !(left_bdd ^ right_bdd),
                _ => todo!(),
            }
        }
        Expr::CaseExpr(case_expr) => {
            let mut ans = expr_to_bdd(cudd, symbols, &case_expr.branchs.last().unwrap().1);
            for i in (0..case_expr.branchs.len() - 1).rev() {
                let cond = expr_to_bdd(cudd, symbols, &case_expr.branchs[i].0);
                let res = expr_to_bdd(cudd, symbols, &case_expr.branchs[i].1);
                ans = cond.if_then_else(&res, &ans);
            }
            ans
        }
    };
    ans
}

impl SmvBdd {
    pub fn new(smv: &Smv, method: SmvTransBddMethod) -> Self {
        let cudd = Cudd::new();
        let mut symbols = HashMap::new();
        let mut init = cudd.constant(true);
        for i in 0..smv.vars.len() {
            let current = i * 2;
            let next = current + 1;
            assert!(symbols.insert(smv.vars[i].ident.clone(), current).is_none());
            cudd.ith_var(next);
        }
        let mut trans = vec![];
        for i in 0..smv.invariants.len() {
            let expr_ddnode = expr_to_bdd(&cudd, &symbols, &smv.invariants[i]);
            let expr_next_ddnode = expr_to_bdd(
                &cudd,
                &symbols,
                &Expr::PrefixExpr(Prefix::Next, Box::new(smv.invariants[i].clone())),
            );
            trans.push(expr_ddnode);
            trans.push(expr_next_ddnode);
        }
        trans.extend(
            smv.trans
                .iter()
                .map(|expr| expr_to_bdd(&cudd, &symbols, expr)),
        );
        let trans = SmvTransBdd::new(cudd.clone(), trans, method);
        for i in 0..smv.inits.len() {
            let expr_ddnode = expr_to_bdd(&cudd, &symbols, &smv.inits[i]);
            init &= expr_ddnode;
        }
        let ret = Self {
            cudd,
            symbols,
            trans,
            init,
        };
        ret
    }
}
