use num::PrimInt;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum Radices {
    Binary,
    Hex,
    Decimal,
}
#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct RadixedValue<T: PrimInt> {
    radix: Radices,
    value: T,
}

impl<T: PrimInt> std::fmt::Display for RadixedValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.radix)
    }
}

pub mod memory_view;
pub mod register_view;
