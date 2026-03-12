#[doc = "Register `INTC` reader"]
pub type R = crate::R<IntcSpec>;
#[doc = "Register `INTC` writer"]
pub type W = crate::W<IntcSpec>;
#[doc = "CKOKIF interrupt clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckokicw {
    #[doc = "1: Clear CKOKIF bit in CTC_STAT register"]
    Clear = 1,
}
impl From<Ckokicw> for bool {
    #[inline(always)]
    fn from(variant: Ckokicw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKOKIC` reader - CKOKIF interrupt clear bit"]
pub type CkokicR = crate::BitReader<Ckokicw>;
impl CkokicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ckokicw> {
        match self.bits {
            true => Some(Ckokicw::Clear),
            _ => None,
        }
    }
    #[doc = "Clear CKOKIF bit in CTC_STAT register"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Ckokicw::Clear
    }
}
#[doc = "Field `CKOKIC` writer - CKOKIF interrupt clear bit"]
pub type CkokicW<'a, REG> = crate::BitWriter<'a, REG, Ckokicw>;
impl<'a, REG> CkokicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear CKOKIF bit in CTC_STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ckokicw::Clear)
    }
}
#[doc = "CKWARNIF interrupt clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckwarnicw {
    #[doc = "1: Clear CKWARNIF bit in CTC_STAT register"]
    Clear = 1,
}
impl From<Ckwarnicw> for bool {
    #[inline(always)]
    fn from(variant: Ckwarnicw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKWARNIC` reader - CKWARNIF interrupt clear bit"]
pub type CkwarnicR = crate::BitReader<Ckwarnicw>;
impl CkwarnicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ckwarnicw> {
        match self.bits {
            true => Some(Ckwarnicw::Clear),
            _ => None,
        }
    }
    #[doc = "Clear CKWARNIF bit in CTC_STAT register"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Ckwarnicw::Clear
    }
}
#[doc = "Field `CKWARNIC` writer - CKWARNIF interrupt clear bit"]
pub type CkwarnicW<'a, REG> = crate::BitWriter<'a, REG, Ckwarnicw>;
impl<'a, REG> CkwarnicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear CKWARNIF bit in CTC_STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ckwarnicw::Clear)
    }
}
#[doc = "ERRIF interrupt clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erricw {
    #[doc = "1: Clear ERRIF, TRIMERR, REFMISS, and CKERR bits in CTC_STAT register"]
    Clear = 1,
}
impl From<Erricw> for bool {
    #[inline(always)]
    fn from(variant: Erricw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIC` reader - ERRIF interrupt clear bit"]
pub type ErricR = crate::BitReader<Erricw>;
impl ErricR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Erricw> {
        match self.bits {
            true => Some(Erricw::Clear),
            _ => None,
        }
    }
    #[doc = "Clear ERRIF, TRIMERR, REFMISS, and CKERR bits in CTC_STAT register"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Erricw::Clear
    }
}
#[doc = "Field `ERRIC` writer - ERRIF interrupt clear bit"]
pub type ErricW<'a, REG> = crate::BitWriter<'a, REG, Erricw>;
impl<'a, REG> ErricW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear ERRIF, TRIMERR, REFMISS, and CKERR bits in CTC_STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Erricw::Clear)
    }
}
#[doc = "EREFIF interrupt clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ereficw {
    #[doc = "1: Clear the EREFIF bit in CTC_STAT register"]
    Clear = 1,
}
impl From<Ereficw> for bool {
    #[inline(always)]
    fn from(variant: Ereficw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EREFIC` reader - EREFIF interrupt clear bit"]
pub type EreficR = crate::BitReader<Ereficw>;
impl EreficR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ereficw> {
        match self.bits {
            true => Some(Ereficw::Clear),
            _ => None,
        }
    }
    #[doc = "Clear the EREFIF bit in CTC_STAT register"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Ereficw::Clear
    }
}
#[doc = "Field `EREFIC` writer - EREFIF interrupt clear bit"]
pub type EreficW<'a, REG> = crate::BitWriter<'a, REG, Ereficw>;
impl<'a, REG> EreficW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the EREFIF bit in CTC_STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ereficw::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - CKOKIF interrupt clear bit"]
    #[inline(always)]
    pub fn ckokic(&self) -> CkokicR {
        CkokicR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CKWARNIF interrupt clear bit"]
    #[inline(always)]
    pub fn ckwarnic(&self) -> CkwarnicR {
        CkwarnicR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ERRIF interrupt clear bit"]
    #[inline(always)]
    pub fn erric(&self) -> ErricR {
        ErricR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EREFIF interrupt clear bit"]
    #[inline(always)]
    pub fn erefic(&self) -> EreficR {
        EreficR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CKOKIF interrupt clear bit"]
    #[inline(always)]
    pub fn ckokic(&mut self) -> CkokicW<'_, IntcSpec> {
        CkokicW::new(self, 0)
    }
    #[doc = "Bit 1 - CKWARNIF interrupt clear bit"]
    #[inline(always)]
    pub fn ckwarnic(&mut self) -> CkwarnicW<'_, IntcSpec> {
        CkwarnicW::new(self, 1)
    }
    #[doc = "Bit 2 - ERRIF interrupt clear bit"]
    #[inline(always)]
    pub fn erric(&mut self) -> ErricW<'_, IntcSpec> {
        ErricW::new(self, 2)
    }
    #[doc = "Bit 3 - EREFIF interrupt clear bit"]
    #[inline(always)]
    pub fn erefic(&mut self) -> EreficW<'_, IntcSpec> {
        EreficW::new(self, 3)
    }
}
#[doc = "Interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`intc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcSpec;
impl crate::RegisterSpec for IntcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intc::R`](R) reader structure"]
impl crate::Readable for IntcSpec {}
#[doc = "`write(|w| ..)` method takes [`intc::W`](W) writer structure"]
impl crate::Writable for IntcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTC to value 0"]
impl crate::Resettable for IntcSpec {}
