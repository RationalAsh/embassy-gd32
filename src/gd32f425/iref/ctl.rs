#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `CSDT` reader - Current step data"]
pub type CsdtR = crate::FieldReader;
#[doc = "Field `CSDT` writer - Current step data"]
pub type CsdtW<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
#[doc = "Sink current mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scmod {
    #[doc = "0: Source current mode"]
    Source = 0,
    #[doc = "1: Sink current mode"]
    Sink = 1,
}
impl From<Scmod> for bool {
    #[inline(always)]
    fn from(variant: Scmod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCMOD` reader - Sink current mode"]
pub type ScmodR = crate::BitReader<Scmod>;
impl ScmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scmod {
        match self.bits {
            false => Scmod::Source,
            true => Scmod::Sink,
        }
    }
    #[doc = "Source current mode"]
    #[inline(always)]
    pub fn is_source(&self) -> bool {
        *self == Scmod::Source
    }
    #[doc = "Sink current mode"]
    #[inline(always)]
    pub fn is_sink(&self) -> bool {
        *self == Scmod::Sink
    }
}
#[doc = "Field `SCMOD` writer - Sink current mode"]
pub type ScmodW<'a, REG> = crate::BitWriter<'a, REG, Scmod>;
impl<'a, REG> ScmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Source current mode"]
    #[inline(always)]
    pub fn source(self) -> &'a mut crate::W<REG> {
        self.variant(Scmod::Source)
    }
    #[doc = "Sink current mode"]
    #[inline(always)]
    pub fn sink(self) -> &'a mut crate::W<REG> {
        self.variant(Scmod::Sink)
    }
}
#[doc = "Field `CPT` reader - Current precision trim"]
pub type CptR = crate::FieldReader;
#[doc = "Field `CPT` writer - Current precision trim"]
pub type CptW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
#[doc = "Step selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssel {
    #[doc = "0: Low power, 1uA step."]
    Ua1 = 0,
    #[doc = "1: High current, 8uA step."]
    Ua8 = 1,
}
impl From<Ssel> for bool {
    #[inline(always)]
    fn from(variant: Ssel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSEL` reader - Step selection"]
pub type SselR = crate::BitReader<Ssel>;
impl SselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssel {
        match self.bits {
            false => Ssel::Ua1,
            true => Ssel::Ua8,
        }
    }
    #[doc = "Low power, 1uA step."]
    #[inline(always)]
    pub fn is_ua1(&self) -> bool {
        *self == Ssel::Ua1
    }
    #[doc = "High current, 8uA step."]
    #[inline(always)]
    pub fn is_ua8(&self) -> bool {
        *self == Ssel::Ua8
    }
}
#[doc = "Field `SSEL` writer - Step selection"]
pub type SselW<'a, REG> = crate::BitWriter<'a, REG, Ssel>;
impl<'a, REG> SselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low power, 1uA step."]
    #[inline(always)]
    pub fn ua1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssel::Ua1)
    }
    #[doc = "High current, 8uA step."]
    #[inline(always)]
    pub fn ua8(self) -> &'a mut crate::W<REG> {
        self.variant(Ssel::Ua8)
    }
}
#[doc = "Current reference enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cren {
    #[doc = "0: Current reference disabled"]
    Disabled = 0,
    #[doc = "1: Current reference enabled"]
    Enabled = 1,
}
impl From<Cren> for bool {
    #[inline(always)]
    fn from(variant: Cren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CREN` reader - Current reference enable"]
pub type CrenR = crate::BitReader<Cren>;
impl CrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cren {
        match self.bits {
            false => Cren::Disabled,
            true => Cren::Enabled,
        }
    }
    #[doc = "Current reference disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cren::Disabled
    }
    #[doc = "Current reference enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cren::Enabled
    }
}
#[doc = "Field `CREN` writer - Current reference enable"]
pub type CrenW<'a, REG> = crate::BitWriter<'a, REG, Cren>;
impl<'a, REG> CrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Current reference disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cren::Disabled)
    }
    #[doc = "Current reference enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cren::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:5 - Current step data"]
    #[inline(always)]
    pub fn csdt(&self) -> CsdtR {
        CsdtR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Sink current mode"]
    #[inline(always)]
    pub fn scmod(&self) -> ScmodR {
        ScmodR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Current precision trim"]
    #[inline(always)]
    pub fn cpt(&self) -> CptR {
        CptR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - Step selection"]
    #[inline(always)]
    pub fn ssel(&self) -> SselR {
        SselR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Current reference enable"]
    #[inline(always)]
    pub fn cren(&self) -> CrenR {
        CrenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Current step data"]
    #[inline(always)]
    pub fn csdt(&mut self) -> CsdtW<'_, CtlSpec> {
        CsdtW::new(self, 0)
    }
    #[doc = "Bit 7 - Sink current mode"]
    #[inline(always)]
    pub fn scmod(&mut self) -> ScmodW<'_, CtlSpec> {
        ScmodW::new(self, 7)
    }
    #[doc = "Bits 8:12 - Current precision trim"]
    #[inline(always)]
    pub fn cpt(&mut self) -> CptW<'_, CtlSpec> {
        CptW::new(self, 8)
    }
    #[doc = "Bit 14 - Step selection"]
    #[inline(always)]
    pub fn ssel(&mut self) -> SselW<'_, CtlSpec> {
        SselW::new(self, 14)
    }
    #[doc = "Bit 15 - Current reference enable"]
    #[inline(always)]
    pub fn cren(&mut self) -> CrenW<'_, CtlSpec> {
        CrenW::new(self, 15)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTL to value 0x0f00"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0x0f00;
}
