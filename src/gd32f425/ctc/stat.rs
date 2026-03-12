#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Clock trim OK interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckokif {
    #[doc = "0: No clock trim OK occurs"]
    NotOk = 0,
    #[doc = "1: Clock trim OK occurs"]
    Ok = 1,
}
impl From<Ckokif> for bool {
    #[inline(always)]
    fn from(variant: Ckokif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKOKIF` reader - Clock trim OK interrupt flag"]
pub type CkokifR = crate::BitReader<Ckokif>;
impl CkokifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckokif {
        match self.bits {
            false => Ckokif::NotOk,
            true => Ckokif::Ok,
        }
    }
    #[doc = "No clock trim OK occurs"]
    #[inline(always)]
    pub fn is_not_ok(&self) -> bool {
        *self == Ckokif::NotOk
    }
    #[doc = "Clock trim OK occurs"]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == Ckokif::Ok
    }
}
#[doc = "Clock trim warning interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckwarnif {
    #[doc = "0: No clock trim warning occurs"]
    NoWarning = 0,
    #[doc = "1: Clock trim warning occurs"]
    WarningOccurred = 1,
}
impl From<Ckwarnif> for bool {
    #[inline(always)]
    fn from(variant: Ckwarnif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKWARNIF` reader - Clock trim warning interrupt flag"]
pub type CkwarnifR = crate::BitReader<Ckwarnif>;
impl CkwarnifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckwarnif {
        match self.bits {
            false => Ckwarnif::NoWarning,
            true => Ckwarnif::WarningOccurred,
        }
    }
    #[doc = "No clock trim warning occurs"]
    #[inline(always)]
    pub fn is_no_warning(&self) -> bool {
        *self == Ckwarnif::NoWarning
    }
    #[doc = "Clock trim warning occurs"]
    #[inline(always)]
    pub fn is_warning_occurred(&self) -> bool {
        *self == Ckwarnif::WarningOccurred
    }
}
#[doc = "Error interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errif {
    #[doc = "0: No error occurs"]
    NoError = 0,
    #[doc = "1: An error occurs"]
    ErrorOccurred = 1,
}
impl From<Errif> for bool {
    #[inline(always)]
    fn from(variant: Errif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIF` reader - Error interrupt flag"]
pub type ErrifR = crate::BitReader<Errif>;
impl ErrifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errif {
        match self.bits {
            false => Errif::NoError,
            true => Errif::ErrorOccurred,
        }
    }
    #[doc = "No error occurs"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Errif::NoError
    }
    #[doc = "An error occurs"]
    #[inline(always)]
    pub fn is_error_occurred(&self) -> bool {
        *self == Errif::ErrorOccurred
    }
}
#[doc = "Expect reference interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erefif {
    #[doc = "0: No expected reference occurs"]
    NoReference = 0,
    #[doc = "1: Expected reference occurs"]
    ReferenceOccurred = 1,
}
impl From<Erefif> for bool {
    #[inline(always)]
    fn from(variant: Erefif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EREFIF` reader - Expect reference interrupt flag"]
pub type ErefifR = crate::BitReader<Erefif>;
impl ErefifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erefif {
        match self.bits {
            false => Erefif::NoReference,
            true => Erefif::ReferenceOccurred,
        }
    }
    #[doc = "No expected reference occurs"]
    #[inline(always)]
    pub fn is_no_reference(&self) -> bool {
        *self == Erefif::NoReference
    }
    #[doc = "Expected reference occurs"]
    #[inline(always)]
    pub fn is_reference_occurred(&self) -> bool {
        *self == Erefif::ReferenceOccurred
    }
}
#[doc = "Clock trim error bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckerr {
    #[doc = "0: No clock trim error occurs"]
    NoError = 0,
    #[doc = "1: Clock trim error occurs"]
    ErrorOccurred = 1,
}
impl From<Ckerr> for bool {
    #[inline(always)]
    fn from(variant: Ckerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKERR` reader - Clock trim error bit"]
pub type CkerrR = crate::BitReader<Ckerr>;
impl CkerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckerr {
        match self.bits {
            false => Ckerr::NoError,
            true => Ckerr::ErrorOccurred,
        }
    }
    #[doc = "No clock trim error occurs"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Ckerr::NoError
    }
    #[doc = "Clock trim error occurs"]
    #[inline(always)]
    pub fn is_error_occurred(&self) -> bool {
        *self == Ckerr::ErrorOccurred
    }
}
#[doc = "Reference sync pulse miss\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refmiss {
    #[doc = "0: No reference sync pulse miss occurs"]
    NoMiss = 0,
    #[doc = "1: Reference sync pulse miss occurs"]
    MissOccurred = 1,
}
impl From<Refmiss> for bool {
    #[inline(always)]
    fn from(variant: Refmiss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFMISS` reader - Reference sync pulse miss"]
pub type RefmissR = crate::BitReader<Refmiss>;
impl RefmissR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refmiss {
        match self.bits {
            false => Refmiss::NoMiss,
            true => Refmiss::MissOccurred,
        }
    }
    #[doc = "No reference sync pulse miss occurs"]
    #[inline(always)]
    pub fn is_no_miss(&self) -> bool {
        *self == Refmiss::NoMiss
    }
    #[doc = "Reference sync pulse miss occurs"]
    #[inline(always)]
    pub fn is_miss_occurred(&self) -> bool {
        *self == Refmiss::MissOccurred
    }
}
#[doc = "Trim value error bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trimerr {
    #[doc = "0: No trim value error occurs"]
    NoError = 0,
    #[doc = "1: Trim value error occurs"]
    ErrorOccurred = 1,
}
impl From<Trimerr> for bool {
    #[inline(always)]
    fn from(variant: Trimerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIMERR` reader - Trim value error bit"]
pub type TrimerrR = crate::BitReader<Trimerr>;
impl TrimerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trimerr {
        match self.bits {
            false => Trimerr::NoError,
            true => Trimerr::ErrorOccurred,
        }
    }
    #[doc = "No trim value error occurs"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Trimerr::NoError
    }
    #[doc = "Trim value error occurs"]
    #[inline(always)]
    pub fn is_error_occurred(&self) -> bool {
        *self == Trimerr::ErrorOccurred
    }
}
#[doc = "CTC trim counter direction when reference sync pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refdir {
    #[doc = "0: CTC trim counter up-counting"]
    Up = 0,
    #[doc = "1: CTC trim counter down-counting"]
    Down = 1,
}
impl From<Refdir> for bool {
    #[inline(always)]
    fn from(variant: Refdir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFDIR` reader - CTC trim counter direction when reference sync pulse"]
pub type RefdirR = crate::BitReader<Refdir>;
impl RefdirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refdir {
        match self.bits {
            false => Refdir::Up,
            true => Refdir::Down,
        }
    }
    #[doc = "CTC trim counter up-counting"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Refdir::Up
    }
    #[doc = "CTC trim counter down-counting"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Refdir::Down
    }
}
#[doc = "Field `REFCAP` reader - CTC counter capture when reference sync pulse"]
pub type RefcapR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Clock trim OK interrupt flag"]
    #[inline(always)]
    pub fn ckokif(&self) -> CkokifR {
        CkokifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock trim warning interrupt flag"]
    #[inline(always)]
    pub fn ckwarnif(&self) -> CkwarnifR {
        CkwarnifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error interrupt flag"]
    #[inline(always)]
    pub fn errif(&self) -> ErrifR {
        ErrifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Expect reference interrupt flag"]
    #[inline(always)]
    pub fn erefif(&self) -> ErefifR {
        ErefifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Clock trim error bit"]
    #[inline(always)]
    pub fn ckerr(&self) -> CkerrR {
        CkerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reference sync pulse miss"]
    #[inline(always)]
    pub fn refmiss(&self) -> RefmissR {
        RefmissR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Trim value error bit"]
    #[inline(always)]
    pub fn trimerr(&self) -> TrimerrR {
        TrimerrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - CTC trim counter direction when reference sync pulse"]
    #[inline(always)]
    pub fn refdir(&self) -> RefdirR {
        RefdirR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - CTC counter capture when reference sync pulse"]
    #[inline(always)]
    pub fn refcap(&self) -> RefcapR {
        RefcapR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {}
