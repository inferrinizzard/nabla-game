use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;

use crate::basis::structs::*;
use crate::cards::*;

#[derive(Debug)]
pub struct Field {
    pub basis: [FieldBasis; 6], // [0-2] for player_1, [3-5] for player_2
    pub inverses: HashMap<Basis, Basis>,
}

impl Field {
    pub fn new() -> Field {
        let inverses = HashMap::default();
        Field {
            basis: [
                FieldBasis::new(&Basis::from(BasisCard::One)),
                FieldBasis::new(&Basis::from(BasisCard::X)),
                FieldBasis::new(&Basis::from(BasisCard::X2)),
                FieldBasis::new(&Basis::from(BasisCard::One)),
                FieldBasis::new(&Basis::from(BasisCard::X)),
                FieldBasis::new(&Basis::from(BasisCard::X2)),
            ],
            inverses,
        }
    }

    pub fn derivative(&mut self, i: usize, basis: Option<Basis>) {
        let mut self_basis = &mut self[i];
        self_basis.index -= 1;
        if basis.is_some() {
            self_basis.history.insert(self_basis.index, basis.unwrap());
        }
        self_basis.basis = Some(self_basis.history[&self_basis.index].clone());
    }

    pub fn integral(&mut self, i: usize, basis: Option<Basis>) {
        let mut self_basis = &mut self[i];
        self_basis.index += 1;
        if basis.is_some() {
            self_basis.history.insert(self_basis.index, basis.unwrap());
        }
        self_basis.basis = Some(self_basis.history[&self_basis.index].clone());
    }

    pub fn inverse(&mut self, i: usize, inverse_basis: Option<Basis>) {
        let basis = self.basis[i].basis.as_ref().unwrap();
        // assumes that basis is not None
        if self.inverses.contains_key(&basis) {
            self.basis[i].basis = Some(self.inverses[&basis].clone());
        } else {
            // assumes that inverse_bases is not None
            self.inverses
                .insert(basis.clone(), inverse_basis.unwrap().clone());
            self.inverses
                .insert(self.inverses[basis].clone(), basis.clone());
            self.basis[i].basis = Some(self.inverses[basis].clone());
        }
    }
}

impl<Idx> Index<Idx> for Field
where
    Idx: SliceIndex<[FieldBasis]>,
{
    type Output = Idx::Output;

    fn index(&self, i: Idx) -> &Self::Output {
        &self.basis[i]
    }
}
impl IndexMut<usize> for Field {
    fn index_mut(&mut self, i: usize) -> &mut FieldBasis {
        &mut self.basis[i]
    }
}

#[derive(Debug)]
pub struct FieldBasis {
    pub basis: Option<Basis>,
    pub index: i32,
    pub history: HashMap<i32, Basis>, // lookup for derivative/integral
}

impl FieldBasis {
    pub fn new(basis: &Basis) -> FieldBasis {
        let history = HashMap::from([(0, basis.clone())]);
        FieldBasis {
            basis: Some(history[&0].clone()),
            index: 0,
            history,
        }
    }
    pub fn none() -> FieldBasis {
        FieldBasis {
            basis: None,
            index: 0,
            history: HashMap::default(),
        }
    }

    pub fn has_value(&self, card: &Card) -> bool {
        if matches!(
            card,
            Card::DerivativeCard(DerivativeCard::Derivative | DerivativeCard::Nabla)
        ) {
            return self.history.contains_key(&(self.index - 1));
        } else if matches!(card, Card::DerivativeCard(DerivativeCard::Laplacian)) {
            return self.history.contains_key(&(self.index - 2));
        } else if matches!(card, Card::DerivativeCard(DerivativeCard::Integral)) {
            return self.history.contains_key(&(self.index + 1));
        }
        false
    }
}
