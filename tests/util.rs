use nabla_game::basis::builders::*;
use nabla_game::basis::structs::*;

pub fn sin_x() -> Basis {
    sin(&Basis::x())
}
pub fn sin(operand: &Basis) -> Basis {
    SinBasisNode(operand)
}

pub fn cos_x() -> Basis {
    CosBasisNode(&Basis::x())
}
pub fn cos(operand: &Basis) -> Basis {
    CosBasisNode(operand)
}

pub fn e_x() -> Basis {
    EBasisNode(&Basis::x())
}
pub fn e(operand: &Basis) -> Basis {
    EBasisNode(operand)
}

pub fn log_x() -> Basis {
    LogBasisNode(&Basis::x())
}
pub fn log(operand: &Basis) -> Basis {
    LogBasisNode(operand)
}
