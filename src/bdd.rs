use crate::{ast::Expr, Smv};
use cudd::{Cudd, DdNode};
use std::collections::HashMap;

pub struct SmvBdd {
    pub symbols: HashMap<String, usize>,
    pub cudd: Cudd,
    pub vars: Vec<DdNode>,
    pub trans: DdNode,
    pub init: DdNode,
    pub cache: HashMap<Expr, DdNode>,
}

impl SmvBdd {
    pub fn symbol_to_bdd(&mut self, symbol: &String) -> DdNode {
        self.vars[self.symbols[symbol]].clone()
    }

    pub fn expr_to_bdd(&mut self, expr: &Expr) -> DdNode {
        if let Some(bdd) = self.cache.get(expr) {
            return bdd.clone();
        }
        let ans = match expr {
            Expr::Ident(ident) => self.symbol_to_bdd(ident),
            Expr::LitExpr(lit) => self.cudd.constant(*lit),
            Expr::PrefixExpr(op, sub_expr) => {
                let expr_bdd = self.expr_to_bdd(sub_expr);
                match op {
                    crate::ast::Prefix::Not => !expr_bdd,
                    crate::ast::Prefix::Next => {
                        let num_vars = self.vars.len();
                        self.cudd
                            .swap_vars(&expr_bdd, 0..num_vars, num_vars..2 * num_vars)
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
                if case_expr.branchs.len() == 2 {
                    let cond0 = self.expr_to_bdd(&case_expr.branchs[0].0);
                    let res0 = self.expr_to_bdd(&case_expr.branchs[0].1);
                    let res1 = self.expr_to_bdd(&case_expr.branchs[1].1);
                    let ans1 = &cond0 & res0;
                    let ans2 = !cond0 & res1;
                    let ans = ans1 | ans2;
                    ans
                } else {
                    let mut previous_all_false = self.cudd.constant(true);
                    let mut ans = self.cudd.constant(false);
                    for (cond, res) in case_expr.branchs.iter() {
                        let cond = self.expr_to_bdd(cond);
                        let res = self.expr_to_bdd(res);
                        ans = ans | (&previous_all_false & &cond & res);
                        previous_all_false = &previous_all_false & !cond;
                    }
                    ans
                }
            }
        };
        self.cache.insert(expr.clone(), ans.clone());
        ans
    }

    pub fn new(smv: &Smv) -> Self {
        let mut cudd = Cudd::new();
        let mut symbols = HashMap::new();
        let mut vars = Vec::new();
        let trans = cudd.constant(true);
        let init = cudd.constant(true);
        for i in 0..smv.vars.len() {
            assert!(symbols.insert(smv.vars[i].ident.clone(), i).is_none());
            vars.push(cudd.ith_var(i));
            cudd.ith_var(i + smv.vars.len());
        }
        let mut ret = Self {
            symbols,
            cudd,
            vars,
            trans,
            init,
            cache: HashMap::new(),
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
