use bdds::{Bdd, BddManager};
use std::ops::{BitAnd, BitOr, BitXor, Not};

pub enum SmvTransBddMethod {
    Partition(usize),
    Monolithic,
}

#[derive(Clone, Debug)]
pub struct SmvTransBdd<BM: BddManager>
where
    for<'a, 'b> &'a BM::Bdd: Not<Output = BM::Bdd>
        + BitAnd<BM::Bdd, Output = BM::Bdd>
        + BitAnd<&'b BM::Bdd, Output = BM::Bdd>
        + BitOr<BM::Bdd, Output = BM::Bdd>
        + BitOr<&'b BM::Bdd, Output = BM::Bdd>
        + BitXor<BM::Bdd, Output = BM::Bdd>
        + BitXor<&'b BM::Bdd, Output = BM::Bdd>,
{
    pub manager: BM,
    pub trans: BM::Bdd,
}

impl<BM: BddManager> SmvTransBdd<BM>
where
    for<'a, 'b> &'a BM::Bdd: Not<Output = BM::Bdd>
        + BitAnd<BM::Bdd, Output = BM::Bdd>
        + BitAnd<&'b BM::Bdd, Output = BM::Bdd>
        + BitOr<BM::Bdd, Output = BM::Bdd>
        + BitOr<&'b BM::Bdd, Output = BM::Bdd>
        + BitXor<BM::Bdd, Output = BM::Bdd>
        + BitXor<&'b BM::Bdd, Output = BM::Bdd>,
{
    pub fn new(manager: BM, trans: Vec<BM::Bdd>, _method: SmvTransBddMethod) -> Self {
        let trans = {
            let mut res = vec![];
            for tran in trans {
                if !res.contains(&tran) {
                    res.push(tran);
                }
            }
            res
        };
        dbg!(trans.len());
        let mut res = manager.constant(true);
        for i in 0..trans.len() {
            dbg!(i);
            res &= &trans[i];
        }
        Self {
            manager,
            trans: res,
        }
    }

    pub fn pre_image(&self, bdd: &BM::Bdd) -> BM::Bdd {
        bdd.pre_image(&self.trans)
    }

    pub fn post_image(&self, bdd: &BM::Bdd) -> BM::Bdd {
        bdd.post_image(&self.trans)
    }

    // pub fn clone_with_new_cudd(&self) -> Self {
    //     let cudd = Cudd::new();
    //     let trans = cudd.translocate(&self.trans);
    //     Self {
    //         manager: cudd,
    //         trans,
    //     }
    // }
}
