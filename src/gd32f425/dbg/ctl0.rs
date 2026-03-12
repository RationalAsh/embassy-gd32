#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Sleep mode hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlpHold {
    #[doc = "0: No effect"]
    Disabled = 0,
    #[doc = "1: In sleep mode the AHB clock is on"]
    Enabled = 1,
}
impl From<SlpHold> for bool {
    #[inline(always)]
    fn from(variant: SlpHold) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLP_HOLD` reader - Sleep mode hold register"]
pub type SlpHoldR = crate::BitReader<SlpHold>;
impl SlpHoldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SlpHold {
        match self.bits {
            false => SlpHold::Disabled,
            true => SlpHold::Enabled,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SlpHold::Disabled
    }
    #[doc = "In sleep mode the AHB clock is on"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SlpHold::Enabled
    }
}
#[doc = "Field `SLP_HOLD` writer - Sleep mode hold register"]
pub type SlpHoldW<'a, REG> = crate::BitWriter<'a, REG, SlpHold>;
impl<'a, REG> SlpHoldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SlpHold::Disabled)
    }
    #[doc = "In sleep mode the AHB clock is on"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SlpHold::Enabled)
    }
}
#[doc = "Deep-sleep mode hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DslpHold {
    #[doc = "0: No effect"]
    Disabled = 0,
    #[doc = "1: In deep-sleep mode the AHB clock and system clock are provided by IRC16M, a system reset generated when exiting deep-sleep mode."]
    Enabled = 1,
}
impl From<DslpHold> for bool {
    #[inline(always)]
    fn from(variant: DslpHold) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSLP_HOLD` reader - Deep-sleep mode hold register"]
pub type DslpHoldR = crate::BitReader<DslpHold>;
impl DslpHoldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DslpHold {
        match self.bits {
            false => DslpHold::Disabled,
            true => DslpHold::Enabled,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DslpHold::Disabled
    }
    #[doc = "In deep-sleep mode the AHB clock and system clock are provided by IRC16M, a system reset generated when exiting deep-sleep mode."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DslpHold::Enabled
    }
}
#[doc = "Field `DSLP_HOLD` writer - Deep-sleep mode hold register"]
pub type DslpHoldW<'a, REG> = crate::BitWriter<'a, REG, DslpHold>;
impl<'a, REG> DslpHoldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DslpHold::Disabled)
    }
    #[doc = "In deep-sleep mode the AHB clock and system clock are provided by IRC16M, a system reset generated when exiting deep-sleep mode."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DslpHold::Enabled)
    }
}
#[doc = "Standby mode hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StbHold {
    #[doc = "0: No effect"]
    Disabled = 0,
    #[doc = "1: In standby mode the AHB clock and system clock are provided by IRC16M, a system reset generated when exiting standby mode."]
    Enabled = 1,
}
impl From<StbHold> for bool {
    #[inline(always)]
    fn from(variant: StbHold) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STB_HOLD` reader - Standby mode hold register"]
pub type StbHoldR = crate::BitReader<StbHold>;
impl StbHoldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StbHold {
        match self.bits {
            false => StbHold::Disabled,
            true => StbHold::Enabled,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == StbHold::Disabled
    }
    #[doc = "In standby mode the AHB clock and system clock are provided by IRC16M, a system reset generated when exiting standby mode."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == StbHold::Enabled
    }
}
#[doc = "Field `STB_HOLD` writer - Standby mode hold register"]
pub type StbHoldW<'a, REG> = crate::BitWriter<'a, REG, StbHold>;
impl<'a, REG> StbHoldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(StbHold::Disabled)
    }
    #[doc = "In standby mode the AHB clock and system clock are provided by IRC16M, a system reset generated when exiting standby mode."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(StbHold::Enabled)
    }
}
#[doc = "Trace pin allocation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraceIoen {
    #[doc = "0: Trace pin allocation disabled"]
    Disabled = 0,
    #[doc = "1: Trace pin allocation enabled"]
    Enabled = 1,
}
impl From<TraceIoen> for bool {
    #[inline(always)]
    fn from(variant: TraceIoen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRACE_IOEN` reader - Trace pin allocation enable"]
pub type TraceIoenR = crate::BitReader<TraceIoen>;
impl TraceIoenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TraceIoen {
        match self.bits {
            false => TraceIoen::Disabled,
            true => TraceIoen::Enabled,
        }
    }
    #[doc = "Trace pin allocation disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TraceIoen::Disabled
    }
    #[doc = "Trace pin allocation enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TraceIoen::Enabled
    }
}
#[doc = "Field `TRACE_IOEN` writer - Trace pin allocation enable"]
pub type TraceIoenW<'a, REG> = crate::BitWriter<'a, REG, TraceIoen>;
impl<'a, REG> TraceIoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trace pin allocation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TraceIoen::Disabled)
    }
    #[doc = "Trace pin allocation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TraceIoen::Enabled)
    }
}
#[doc = "Field `TRACE_MODE` reader - Trace pin allocation mode"]
pub type TraceModeR = crate::FieldReader;
#[doc = "Field `TRACE_MODE` writer - Trace pin allocation mode"]
pub type TraceModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Sleep mode hold register"]
    #[inline(always)]
    pub fn slp_hold(&self) -> SlpHoldR {
        SlpHoldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Deep-sleep mode hold register"]
    #[inline(always)]
    pub fn dslp_hold(&self) -> DslpHoldR {
        DslpHoldR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Standby mode hold register"]
    #[inline(always)]
    pub fn stb_hold(&self) -> StbHoldR {
        StbHoldR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Trace pin allocation enable"]
    #[inline(always)]
    pub fn trace_ioen(&self) -> TraceIoenR {
        TraceIoenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Trace pin allocation mode"]
    #[inline(always)]
    pub fn trace_mode(&self) -> TraceModeR {
        TraceModeR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Sleep mode hold register"]
    #[inline(always)]
    pub fn slp_hold(&mut self) -> SlpHoldW<'_, Ctl0Spec> {
        SlpHoldW::new(self, 0)
    }
    #[doc = "Bit 1 - Deep-sleep mode hold register"]
    #[inline(always)]
    pub fn dslp_hold(&mut self) -> DslpHoldW<'_, Ctl0Spec> {
        DslpHoldW::new(self, 1)
    }
    #[doc = "Bit 2 - Standby mode hold register"]
    #[inline(always)]
    pub fn stb_hold(&mut self) -> StbHoldW<'_, Ctl0Spec> {
        StbHoldW::new(self, 2)
    }
    #[doc = "Bit 5 - Trace pin allocation enable"]
    #[inline(always)]
    pub fn trace_ioen(&mut self) -> TraceIoenW<'_, Ctl0Spec> {
        TraceIoenW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Trace pin allocation mode"]
    #[inline(always)]
    pub fn trace_mode(&mut self) -> TraceModeW<'_, Ctl0Spec> {
        TraceModeW::new(self, 6)
    }
}
#[doc = "Control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for Ctl0Spec {}
