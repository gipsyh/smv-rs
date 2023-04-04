use crate::{ast::Expr, Smv};
use cudd::{Cudd, DdNode};
use std::collections::HashMap;

pub struct SmvBdd {
    pub symbols: HashMap<String, usize>,
    pub cudd: Cudd,
    pub trans: DdNode,
    pub init: DdNode,
}

impl SmvBdd {
    pub fn expr_to_bdd(&mut self, expr: &Expr) -> DdNode {
        let ans = match expr {
            Expr::Ident(ident) => self.cudd.ith_var(self.symbols[ident]),
            Expr::LitExpr(lit) => self.cudd.constant(*lit),
            Expr::PrefixExpr(op, sub_expr) => {
                let expr_bdd = self.expr_to_bdd(sub_expr);
                match op {
                    crate::ast::Prefix::Not => !expr_bdd,
                    crate::ast::Prefix::Next => {
                        let num_vars = self.symbols.len();
                        self.cudd.swap_vars(
                            &expr_bdd,
                            (0..num_vars).map(|x| x * 2),
                            (0..num_vars).map(|x| x * 2 + 1),
                        )
                    }
                    _ => todo!(),
                }
            }
            Expr::InfixExpr(op, left, right) => {
                let left_bdd = self.expr_to_bdd(left);
                let right_bdd = self.expr_to_bdd(right);
                match op {
                    crate::ast::Infix::And => left_bdd & right_bdd,
                    crate::ast::Infix::Or => left_bdd | right_bdd,
                    crate::ast::Infix::Imply => !left_bdd | right_bdd,
                    crate::ast::Infix::Iff => !(left_bdd ^ right_bdd),
                    _ => todo!(),
                }
            }
            Expr::CaseExpr(case_expr) => {
                let mut ans = self.expr_to_bdd(&case_expr.branchs.last().unwrap().1);
                for i in (0..case_expr.branchs.len() - 1).rev() {
                    let cond = self.expr_to_bdd(&case_expr.branchs[i].0);
                    let res = self.expr_to_bdd(&case_expr.branchs[i].1);
                    ans = self.cudd.if_then_else(&cond, &res, &ans);
                }
                ans
            }
        };
        ans
    }

    pub fn new(smv: &Smv) -> Self {
        let mut cudd = Cudd::new();
        let mut symbols = HashMap::new();
        let trans = cudd.constant(true);
        let init = cudd.constant(true);
        for i in 0..smv.vars.len() {
            let current = i * 2;
            let next = current + 1;
            assert!(symbols.insert(smv.vars[i].ident.clone(), current).is_none());
            cudd.ith_var(next);
        }
        let mut ret = Self {
            symbols,
            cudd,
            trans,
            init,
        };
        for i in 0..smv.trans.len() {
            let expr_ddnode = ret.expr_to_bdd(&smv.trans[i]);
            ret.trans = &ret.trans & expr_ddnode;
        }
        for i in 0..smv.invariants.len() {
            let expr_ddnode = ret.expr_to_bdd(&smv.invariants[i]);
            ret.trans = &ret.trans & expr_ddnode;
        }
        for i in 0..smv.inits.len() {
            let expr_ddnode = ret.expr_to_bdd(&smv.inits[i]);
            ret.init = &ret.init & expr_ddnode;
        }
        ret
    }
}
