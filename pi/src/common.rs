/// The address where I/O peripherals are mapped to.
pub const IO_BASE: usize = 0x3F000000;

/// Generates `pub enums` with no variants for each `ident` passed in.
pub macro states($($name:ident),*) {
    $(
        /// A possible state.
        #[doc(hidden)]
        pub enum $name {  }
    )*
}

// Get one bit from a intergral value
pub macro is_bit_set($val:expr, $pos:expr) {
    ($val & (1 << $pos)) != 0
}