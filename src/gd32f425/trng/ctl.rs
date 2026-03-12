#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "TRNG enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trngen {
    #[doc = "0: Disable TRNG"]
    Disable = 0,
    #[doc = "1: Enable TRNG"]
    Enable = 1,
}
impl From<Trngen> for bool {
    #[inline(always)]
    fn from(variant: Trngen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRNGEN` reader - TRNG enable bit"]
pub type TrngenR = crate::BitReader<Trngen>;
impl TrngenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trngen {
        match self.bits {
            false => Trngen::Disable,
            true => Trngen::Enable,
        }
    }
    #[doc = "Disable TRNG"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Trngen::Disable
    }
    #[doc = "Enable TRNG"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Trngen::Enable
    }
}
#[doc = "Field `TRNGEN` writer - TRNG enable bit"]
pub type TrngenW<'a, REG> = crate::BitWriter<'a, REG, Trngen>;
impl<'a, REG> TrngenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable TRNG"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Trngen::Disable)
    }
    #[doc = "Enable TRNG"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Trngen::Enable)
    }
}
#[doc = "Interrupt bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ie {
    #[doc = "0: Disable TRNG interrupt"]
    Disable = 0,
    #[doc = "1: Enable TRNG interrupt"]
    Enable = 1,
}
impl From<Ie> for bool {
    #[inline(always)]
    fn from(variant: Ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IE` reader - Interrupt bit"]
pub type IeR = crate::BitReader<Ie>;
impl IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ie {
        match self.bits {
            false => Ie::Disable,
            true => Ie::Enable,
        }
    }
    #[doc = "Disable TRNG interrupt"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ie::Disable
    }
    #[doc = "Enable TRNG interrupt"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ie::Enable
    }
}
#[doc = "Field `IE` writer - Interrupt bit"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG, Ie>;
impl<'a, REG> IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable TRNG interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ie::Disable)
    }
    #[doc = "Enable TRNG interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ie::Enable)
    }
}
impl R {
    #[doc = "Bit 2 - TRNG enable bit"]
    #[inline(always)]
    pub fn trngen(&self) -> TrngenR {
        TrngenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt bit"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - TRNG enable bit"]
    #[inline(always)]
    pub fn trngen(&mut self) -> TrngenW<'_, CtlSpec> {
        TrngenW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt bit"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<'_, CtlSpec> {
        IeW::new(self, 3)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {}
