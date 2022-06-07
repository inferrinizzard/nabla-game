/// used in LaTeX formatting to use LN instead of LOG for the natural logarithm
pub static mut DISPLAY_LN_FOR_LOG: bool = false;
/// allow multiple field Basis with same base but different coefficients
pub static mut ALLOW_LINEAR_DEPENDENCE: bool = true;
/// allow limits of inverse trigonometric functions beyond the range of the function (ie. lim→INF)
pub static mut ALLOW_LIMITS_BEYOND_BOUNDS: bool = true;
/// fully expand all functions (ie. integrals, inverse)
pub static mut FULL_COMPUTE: bool = false;
/// display fractional exponents as a rational number or with nth root notation
pub static mut USE_FRACTIONAL_EXPONENTS: bool = true;
/// restrict field to maximum 3 Basis
pub static mut LIMIT_FIELD_BASIS: bool = true;
