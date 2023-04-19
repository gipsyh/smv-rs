mod trans;
pub use trans::*;

use crate::{ast::Expr, Prefix, Smv};
use bdds::{Bdd, BddManager};
use std::{
    collections::HashMap,
    ops::{BitAnd, BitOr, BitXor, Not},
};

#[derive(Clone)]
pub struct SmvBdd<BM: BddManager>
where
    for<'a, 'b> &'a BM::Bdd: Not<Output = BM::Bdd>
        + BitAnd<BM::Bdd, Output = BM::Bdd>
        + BitAnd<&'b BM::Bdd, Output = BM::Bdd>
        + BitOr<BM::Bdd, Output = BM::Bdd>
        + BitOr<&'b BM::Bdd, Output = BM::Bdd>
        + BitXor<BM::Bdd, Output = BM::Bdd>
        + BitXor<&'b BM::Bdd, Output = BM::Bdd>,
{
    pub symbols: HashMap<String, usize>,
    pub manager: BM,
    pub trans: SmvTransBdd<BM>,
    pub init: BM::Bdd,
}

pub fn expr_to_bdd<BM: BddManager>(
    manager: &BM,
    symbols: &HashMap<String, usize>,
    expr: &Expr,
) -> BM::Bdd
where
    for<'a, 'b> &'a BM::Bdd: Not<Output = BM::Bdd>
        + BitAnd<BM::Bdd, Output = BM::Bdd>
        + BitAnd<&'b BM::Bdd, Output = BM::Bdd>
        + BitOr<BM::Bdd, Output = BM::Bdd>
        + BitOr<&'b BM::Bdd, Output = BM::Bdd>
        + BitXor<BM::Bdd, Output = BM::Bdd>
        + BitXor<&'b BM::Bdd, Output = BM::Bdd>,
{
    let ans = match expr {
        Expr::Ident(ident) => manager.ith_var(symbols[ident]),
        Expr::LitExpr(lit) => manager.constant(*lit),
        Expr::PrefixExpr(op, sub_expr) => {
            let expr_bdd = expr_to_bdd(manager, symbols, sub_expr);
            match op {
                crate::ast::Prefix::Not => !expr_bdd,
                crate::ast::Prefix::Next => expr_bdd.next_state(),
                _ => todo!(),
            }
        }
        Expr::InfixExpr(op, left, right) => {
            let left_bdd = expr_to_bdd(manager, symbols, left);
            let right_bdd = expr_to_bdd(manager, symbols, right);
            match op {
                crate::ast::Infix::And => left_bdd & right_bdd,
                crate::ast::Infix::Or => left_bdd | right_bdd,
                crate::ast::Infix::Imply => !left_bdd | right_bdd,
                crate::ast::Infix::Iff => !(left_bdd ^ right_bdd),
                _ => todo!(),
            }
        }
        Expr::CaseExpr(case_expr) => {
            let mut ans = expr_to_bdd(manager, symbols, &case_expr.branchs.last().unwrap().1);
            for i in (0..case_expr.branchs.len() - 1).rev() {
                let cond = expr_to_bdd(manager, symbols, &case_expr.branchs[i].0);
                let res = expr_to_bdd(manager, symbols, &case_expr.branchs[i].1);
                ans = cond.if_then_else(&res, &ans);
            }
            ans
        }
    };
    ans
}

impl<BM: BddManager> SmvBdd<BM>
where
    for<'a, 'b> &'a BM::Bdd: Not<Output = BM::Bdd>
        + BitAnd<BM::Bdd, Output = BM::Bdd>
        + BitAnd<&'b BM::Bdd, Output = BM::Bdd>
        + BitOr<BM::Bdd, Output = BM::Bdd>
        + BitOr<&'b BM::Bdd, Output = BM::Bdd>
        + BitXor<BM::Bdd, Output = BM::Bdd>
        + BitXor<&'b BM::Bdd, Output = BM::Bdd>,
{
    pub fn new(manager: &BM, smv: &Smv, method: SmvTransBddMethod) -> Self {
        let mut symbols = HashMap::new();
        let mut init = manager.constant(true);
        for i in 0..smv.vars.len() {
            let current = i * 2;
            let next = current + 1;
            assert!(symbols.insert(smv.vars[i].ident.clone(), current).is_none());
            manager.ith_var(next);
        }
        let mut trans = vec![];
        for i in 0..smv.invariants.len() {
            let expr_ddnode = expr_to_bdd(manager, &symbols, &smv.invariants[i]);
            let expr_next_ddnode = expr_to_bdd(
                manager,
                &symbols,
                &Expr::PrefixExpr(Prefix::Next, Box::new(smv.invariants[i].clone())),
            );
            trans.push(expr_ddnode);
            trans.push(expr_next_ddnode);
        }
        trans.extend(
            smv.trans
                .iter()
                .map(|expr| expr_to_bdd(manager, &symbols, expr)),
        );
        let trans = SmvTransBdd::new(manager.clone(), trans, method);
        for i in 0..smv.inits.len() {
            let expr_ddnode = expr_to_bdd(manager, &symbols, &smv.inits[i]);
            init &= expr_ddnode;
        }
        let ret = Self {
            manager: manager.clone(),
            symbols,
            trans,
            init,
        };
        ret
    }
}
