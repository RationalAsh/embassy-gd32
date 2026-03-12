#![allow(non_camel_case_types)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "gd32c103")]
pub mod gd32c103;

#[cfg(feature = "gd32c113")]
pub mod gd32c113;

#[cfg(feature = "gd32e103")]
pub mod gd32e103;

#[cfg(feature = "gd32e230")]
pub mod gd32e230;

#[cfg(feature = "gd32e231")]
pub mod gd32e231;

#[cfg(feature = "gd32e503")]
pub mod gd32e503;

#[cfg(feature = "gd32e505")]
pub mod gd32e505;

#[cfg(feature = "gd32e507")]
pub mod gd32e507;

#[cfg(feature = "gd32e508")]
pub mod gd32e508;

#[cfg(feature = "gd32f130")]
pub mod gd32f130;

#[cfg(feature = "gd32f150")]
pub mod gd32f150;

#[cfg(feature = "gd32f170")]
pub mod gd32f170;

#[cfg(feature = "gd32f190")]
pub mod gd32f190;

#[cfg(feature = "gd32f205")]
pub mod gd32f205;

#[cfg(feature = "gd32f207")]
pub mod gd32f207;

#[cfg(feature = "gd32f303")]
pub mod gd32f303;

#[cfg(feature = "gd32f305")]
pub mod gd32f305;

#[cfg(feature = "gd32f307")]
pub mod gd32f307;

#[cfg(feature = "gd32f403")]
pub mod gd32f403;

#[cfg(feature = "gd32f425")]
pub mod gd32f425;

