macro_rules! enum_cast {
    // tries to cast enum to variant
    ($enum: expr, $variant: path) => {{
        // if variant is a valid member of enum
        if let $variant(output) = $enum {
            *output
        } else {
            panic!(
                "mismatch variant when casting {} to {}",
                stringify!($enum),
                stringify!($variant)
            );
        }
    }};
}
pub(crate) use enum_cast;
