use cudd::{Cudd, Bdd};

pub enum SmvTransBddMethod {
    Partition(usize),
    Monolithic,
}

#[derive(Clone, Debug)]
pub struct SmvTransBdd {
    pub cudd: Cudd,
    pub trans: Bdd,
}

impl SmvTransBdd {
    pub fn new(cudd: Cudd, trans: Vec<Bdd>, _method: SmvTransBddMethod) -> Self {
        let trans = trans.iter().fold(cudd.constant(true), |tran, x| tran & x);
        Self { cudd, trans }
    }

    pub fn pre_image(&self, bdd: &Bdd) -> Bdd {
        let vars = (0..self.cudd.num_var()).filter(|x| x % 2 == 0);
        let next_vars = (0..self.cudd.num_var()).filter(|x| x % 2 == 1);
        let mut bdd = bdd.swap_vars(vars, next_vars.clone());
        bdd = bdd.and_abstract(&self.trans, next_vars);
        bdd
    }

    pub fn post_image(&self, bdd: &Bdd) -> Bdd {
        let vars = (0..self.cudd.num_var()).filter(|x| x % 2 == 0);
        let next_vars = (0..self.cudd.num_var()).filter(|x| x % 2 == 1);
        let bdd = bdd.and_abstract(&self.trans, vars.clone());
        bdd.swap_vars(next_vars, vars)
    }

    pub fn clone_with_new_cudd(&self) -> Self {
        let cudd = Cudd::new();
        let trans = cudd.translocate(&self.trans);
        Self { cudd, trans }
    }
}
