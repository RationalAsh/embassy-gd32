#[doc = "Register `OVSAMPCTL` reader"]
pub type R = crate::R<OvsampctlSpec>;
#[doc = "Oversampling Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovsen {
    #[doc = "0: Oversampling disabled"]
    Disabled = 0,
    #[doc = "1: Oversampling enabled"]
    Enabled = 1,
}
impl From<Ovsen> for bool {
    #[inline(always)]
    fn from(variant: Ovsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVSEN` reader - Oversampling Enable"]
pub type OvsenR = crate::BitReader<Ovsen>;
impl OvsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovsen {
        match self.bits {
            false => Ovsen::Disabled,
            true => Ovsen::Enabled,
        }
    }
    #[doc = "Oversampling disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ovsen::Disabled
    }
    #[doc = "Oversampling enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ovsen::Enabled
    }
}
#[doc = "Oversampling ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ovsr {
    #[doc = "0: Oversampling ratio is 2"]
    Times2 = 0,
    #[doc = "1: Oversampling ratio is 4"]
    Times4 = 1,
    #[doc = "2: Oversampling ratio is 8"]
    Times8 = 2,
    #[doc = "3: Oversampling ratio is 16"]
    Times16 = 3,
    #[doc = "4: Oversampling ratio is 32"]
    Times32 = 4,
    #[doc = "5: Oversampling ratio is 64"]
    Times64 = 5,
    #[doc = "6: Oversampling ratio is 128"]
    Times128 = 6,
    #[doc = "7: Oversampling ratio is 256"]
    Times256 = 7,
}
impl From<Ovsr> for u8 {
    #[inline(always)]
    fn from(variant: Ovsr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ovsr {
    type Ux = u8;
}
impl crate::IsEnum for Ovsr {}
#[doc = "Field `OVSR` reader - Oversampling ratio"]
pub type OvsrR = crate::FieldReader<Ovsr>;
impl OvsrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovsr {
        match self.bits {
            0 => Ovsr::Times2,
            1 => Ovsr::Times4,
            2 => Ovsr::Times8,
            3 => Ovsr::Times16,
            4 => Ovsr::Times32,
            5 => Ovsr::Times64,
            6 => Ovsr::Times128,
            7 => Ovsr::Times256,
            _ => unreachable!(),
        }
    }
    #[doc = "Oversampling ratio is 2"]
    #[inline(always)]
    pub fn is_times2(&self) -> bool {
        *self == Ovsr::Times2
    }
    #[doc = "Oversampling ratio is 4"]
    #[inline(always)]
    pub fn is_times4(&self) -> bool {
        *self == Ovsr::Times4
    }
    #[doc = "Oversampling ratio is 8"]
    #[inline(always)]
    pub fn is_times8(&self) -> bool {
        *self == Ovsr::Times8
    }
    #[doc = "Oversampling ratio is 16"]
    #[inline(always)]
    pub fn is_times16(&self) -> bool {
        *self == Ovsr::Times16
    }
    #[doc = "Oversampling ratio is 32"]
    #[inline(always)]
    pub fn is_times32(&self) -> bool {
        *self == Ovsr::Times32
    }
    #[doc = "Oversampling ratio is 64"]
    #[inline(always)]
    pub fn is_times64(&self) -> bool {
        *self == Ovsr::Times64
    }
    #[doc = "Oversampling ratio is 128"]
    #[inline(always)]
    pub fn is_times128(&self) -> bool {
        *self == Ovsr::Times128
    }
    #[doc = "Oversampling ratio is 256"]
    #[inline(always)]
    pub fn is_times256(&self) -> bool {
        *self == Ovsr::Times256
    }
}
#[doc = "Field `OVSS` reader - Oversampling shift"]
pub type OvssR = crate::FieldReader;
#[doc = "Triggered Oversampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tovs {
    #[doc = "0: All oversampled conversaions for a channel are done consecutively"]
    Consecutive = 0,
    #[doc = "1: Oversampled conversions for a channel done wit a separate trigger."]
    Separate = 1,
}
impl From<Tovs> for bool {
    #[inline(always)]
    fn from(variant: Tovs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOVS` reader - Triggered Oversampling"]
pub type TovsR = crate::BitReader<Tovs>;
impl TovsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tovs {
        match self.bits {
            false => Tovs::Consecutive,
            true => Tovs::Separate,
        }
    }
    #[doc = "All oversampled conversaions for a channel are done consecutively"]
    #[inline(always)]
    pub fn is_consecutive(&self) -> bool {
        *self == Tovs::Consecutive
    }
    #[doc = "Oversampled conversions for a channel done wit a separate trigger."]
    #[inline(always)]
    pub fn is_separate(&self) -> bool {
        *self == Tovs::Separate
    }
}
impl R {
    #[doc = "Bit 0 - Oversampling Enable"]
    #[inline(always)]
    pub fn ovsen(&self) -> OvsenR {
        OvsenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:4 - Oversampling ratio"]
    #[inline(always)]
    pub fn ovsr(&self) -> OvsrR {
        OvsrR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:8 - Oversampling shift"]
    #[inline(always)]
    pub fn ovss(&self) -> OvssR {
        OvssR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Triggered Oversampling"]
    #[inline(always)]
    pub fn tovs(&self) -> TovsR {
        TovsR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Oversample control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ovsampctl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OvsampctlSpec;
impl crate::RegisterSpec for OvsampctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ovsampctl::R`](R) reader structure"]
impl crate::Readable for OvsampctlSpec {}
#[doc = "`reset()` method sets OVSAMPCTL to value 0"]
impl crate::Resettable for OvsampctlSpec {}
