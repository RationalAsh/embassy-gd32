#![allow(non_camel_case_types)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "gd32c103")]
pub mod gd32c103;
#[cfg(feature = "gd32c103")]
pub use self::gd32c103 as pac;

#[cfg(feature = "gd32c113")]
pub mod gd32c113;
#[cfg(feature = "gd32c113")]
pub use self::gd32c113 as pac;

#[cfg(feature = "gd32e103")]
pub mod gd32e103;
#[cfg(feature = "gd32e103")]
pub use self::gd32e103 as pac;

#[cfg(feature = "gd32e230")]
pub mod gd32e230;
#[cfg(feature = "gd32e230")]
pub use self::gd32e230 as pac;

#[cfg(feature = "gd32e231")]
pub mod gd32e231;
#[cfg(feature = "gd32e231")]
pub use self::gd32e231 as pac;

#[cfg(feature = "gd32e503")]
pub mod gd32e503;
#[cfg(feature = "gd32e503")]
pub use self::gd32e503 as pac;

#[cfg(feature = "gd32e505")]
pub mod gd32e505;
#[cfg(feature = "gd32e505")]
pub use self::gd32e505 as pac;

#[cfg(feature = "gd32e507")]
pub mod gd32e507;
#[cfg(feature = "gd32e507")]
pub use self::gd32e507 as pac;

#[cfg(feature = "gd32e508")]
pub mod gd32e508;
#[cfg(feature = "gd32e508")]
pub use self::gd32e508 as pac;

#[cfg(feature = "gd32f130")]
pub mod gd32f130;
#[cfg(feature = "gd32f130")]
pub use self::gd32f130 as pac;

#[cfg(feature = "gd32f150")]
pub mod gd32f150;
#[cfg(feature = "gd32f150")]
pub use self::gd32f150 as pac;

#[cfg(feature = "gd32f170")]
pub mod gd32f170;
#[cfg(feature = "gd32f170")]
pub use self::gd32f170 as pac;

#[cfg(feature = "gd32f190")]
pub mod gd32f190;
#[cfg(feature = "gd32f190")]
pub use self::gd32f190 as pac;

#[cfg(feature = "gd32f205")]
pub mod gd32f205;
#[cfg(feature = "gd32f205")]
pub use self::gd32f205 as pac;

#[cfg(feature = "gd32f207")]
pub mod gd32f207;
#[cfg(feature = "gd32f207")]
pub use self::gd32f207 as pac;

#[cfg(feature = "gd32f303")]
pub mod gd32f303;
#[cfg(feature = "gd32f303")]
pub use self::gd32f303 as pac;

#[cfg(feature = "gd32f305")]
pub mod gd32f305;
#[cfg(feature = "gd32f305")]
pub use self::gd32f305 as pac;

#[cfg(feature = "gd32f307")]
pub mod gd32f307;
#[cfg(feature = "gd32f307")]
pub use self::gd32f307 as pac;

#[cfg(feature = "gd32f403")]
pub mod gd32f403;
#[cfg(feature = "gd32f403")]
pub use self::gd32f403 as pac;

#[cfg(feature = "gd32f425")]
pub mod gd32f425;
#[cfg(feature = "gd32f425")]
pub use self::gd32f425 as pac;

#[cfg(feature = "gd32f425")]
pub mod time_driver;
