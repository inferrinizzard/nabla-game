use std::collections::HashMap;

use crate::basis::structs::*;
use crate::cards::*;

#[derive(Debug)]
pub struct FieldBasis {
    pub basis: Option<Basis>,
    pub index: i32,
    pub history: HashMap<String, Basis>, // numerical keys for derivative/integral, INV for current inverse
}

impl FieldBasis {
    pub fn new(basis: &Basis) -> FieldBasis {
        let history = HashMap::from([(String::from("0"), basis.clone())]);
        FieldBasis {
            basis: Some(history["0"].clone()),
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
            return self.history.contains_key(&(self.index - 1).to_string());
        } else if matches!(card, Card::DerivativeCard(DerivativeCard::Laplacian)) {
            return self.history.contains_key(&(self.index - 2).to_string());
        } else if matches!(card, Card::DerivativeCard(DerivativeCard::Integral)) {
            return self.history.contains_key(&(self.index + 1).to_string());
        }
        false
    }

    pub fn derivative(&mut self, basis: Option<&Basis>)
    // -> Option<Basis>
    {
        self.index -= 1;
        if basis.is_some() {
            self.history
                .insert(self.index.to_string(), basis.unwrap().clone());
        }
        self.basis = Some(self.history[&self.index.to_string()].clone());
        // return self.basis.as_ref();
    }

    pub fn integral(&mut self, basis: Option<&Basis>)
    // -> Option<Basis>
    {
        self.index += 1;
        if basis.is_some() {
            self.history
                .insert(self.index.to_string(), basis.unwrap().clone());
        }
        self.basis = Some(self.history[&self.index.to_string()].clone());
        // return self.basis.as_ref();
    }

    // pub fn inverse(mut self, basis: &Basis) -> Option<Basis> {
    //     return self.basis;
    // }
}
