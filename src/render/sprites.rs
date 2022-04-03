// std imports
use std::collections::HashMap;
// external crate imports
use crate::cards::*;
use crate::game::flags::DISPLAY_LN_FOR_LOG;

/// hash key enum to store player card corners
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum CornerSpriteKey {
    ElementLeft,
    ElementRight,
    FunctionLeft,
    FunctionRight,
}

/// struct to hold sprite data for player cards
pub struct SpriteLookup {
    map: HashMap<Card, (i32, i32)>,
    other: HashMap<CornerSpriteKey, (i32, i32)>,
    pub card_height: f64,
    pub card_width: f64,
}

impl SpriteLookup {
    pub fn new() -> Self {
        Self {
            map: HashMap::from([
                (Card::BasisCard(BasisCard::Zero), (2, 0)),
                (Card::BasisCard(BasisCard::One), (3, 0)),
                (Card::BasisCard(BasisCard::X), (4, 0)),
                (Card::BasisCard(BasisCard::X2), (5, 0)),
                (Card::BasisCard(BasisCard::E), (0, 1)),
                (Card::AlgebraicCard(AlgebraicCard::Mult), (1, 1)),
                (Card::AlgebraicCard(AlgebraicCard::Div), (2, 1)),
                (Card::AlgebraicCard(AlgebraicCard::Sqrt), (3, 1)),
                (Card::DerivativeCard(DerivativeCard::Nabla), (4, 1)),
                (Card::DerivativeCard(DerivativeCard::Laplacian), (5, 1)),
                (Card::BasisCard(BasisCard::Cos), (0, 2)),
                (Card::BasisCard(BasisCard::Sin), (1, 2)),
                (Card::AlgebraicCard(AlgebraicCard::Inverse), (2, 2)),
                (Card::AlgebraicCard(AlgebraicCard::Log), (4, 2)),
                (Card::DerivativeCard(DerivativeCard::Derivative), (5, 2)),
                (Card::DerivativeCard(DerivativeCard::Integral), (0, 3)),
                (Card::LimitCard(LimitCard::LimPosInf), (1, 3)),
                (Card::LimitCard(LimitCard::LimNegInf), (2, 3)),
                (Card::LimitCard(LimitCard::Lim0), (3, 3)),
                (Card::LimitCard(LimitCard::Liminf), (4, 3)),
                (Card::LimitCard(LimitCard::Limsup), (5, 3)),
            ]),
            other: HashMap::from([
                // ("LN".to_string(), (3, 2)),
                (CornerSpriteKey::ElementLeft, (0, 0)),
                (CornerSpriteKey::ElementRight, (0, 0)),
                (CornerSpriteKey::FunctionLeft, (1, 0)),
                (CornerSpriteKey::FunctionRight, (1, 0)),
            ]),
            card_height: 288.0,
            card_width: 216.0,
        }
    }

    /// fetches sprite dimensions for corresponding card
    pub fn get_card(&self, card: &Card) -> (f64, f64, f64, f64) {
        let flag = unsafe { DISPLAY_LN_FOR_LOG };

        let (x, y) = match card {
            Card::AlgebraicCard(AlgebraicCard::Log) if flag => (3, 2), // separate sprite
            _ => self.map[card],
        };

        (
            x as f64 * self.card_width,
            y as f64 * self.card_height,
            self.card_width,
            self.card_height,
        )
    }

    /// fetches sprite dimensions for corresponding corner item
    pub fn get_corner(&self, corner: CornerSpriteKey) -> (f64, f64, f64, f64) {
        let (x, y) = *self.other.get(&corner).unwrap_or(&(0, 0));
        let gutter = self.card_width / 4.0;
        (
            x as f64 * self.card_width,
            y as f64 * self.card_height,
            gutter * 1.5,
            gutter * 1.5,
        )
    }
}
