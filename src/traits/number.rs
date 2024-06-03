pub trait OmnixNumber {
    // Define a method that takes &self to work on the instance of the implementing type
    fn to_hex_string(&self) -> String;
}

impl<T> OmnixNumber for T
where
    T: std::fmt::LowerHex + Sized,
{
    fn to_hex_string(&self) -> String {
        format!("0x{:x}", self)
    }
}
