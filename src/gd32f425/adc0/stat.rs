#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Analog watchdog event flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wder {
    #[doc = "0: No analog watchdog event occurred"]
    NoEvent = 0,
    #[doc = "1: Analog watchdog event occurred"]
    Event = 1,
}
impl From<Wder> for bool {
    #[inline(always)]
    fn from(variant: Wder) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDE` reader - Analog watchdog event flag"]
pub type WdeR = crate::BitReader<Wder>;
impl WdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wder {
        match self.bits {
            false => Wder::NoEvent,
            true => Wder::Event,
        }
    }
    #[doc = "No analog watchdog event occurred"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == Wder::NoEvent
    }
    #[doc = "Analog watchdog event occurred"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == Wder::Event
    }
}
#[doc = "Analog watchdog event flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WdewWO {
    #[doc = "0: Clear the analog watchdog event flag"]
    Clear = 0,
}
impl From<WdewWO> for bool {
    #[inline(always)]
    fn from(variant: WdewWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDE` writer - Analog watchdog event flag"]
pub type WdeW<'a, REG> = crate::BitWriter<'a, REG, WdewWO>;
impl<'a, REG> WdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the analog watchdog event flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(WdewWO::Clear)
    }
}
#[doc = "End of group conversion flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eocr {
    #[doc = "0: Conversion is not complete"]
    NotComplete = 0,
    #[doc = "1: Conversion complete"]
    Complete = 1,
}
impl From<Eocr> for bool {
    #[inline(always)]
    fn from(variant: Eocr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC` reader - End of group conversion flag"]
pub type EocR = crate::BitReader<Eocr>;
impl EocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eocr {
        match self.bits {
            false => Eocr::NotComplete,
            true => Eocr::Complete,
        }
    }
    #[doc = "Conversion is not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == Eocr::NotComplete
    }
    #[doc = "Conversion complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == Eocr::Complete
    }
}
#[doc = "End of group conversion flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EocwWO {
    #[doc = "0: Clear end of group conversion flag"]
    Clear = 0,
}
impl From<EocwWO> for bool {
    #[inline(always)]
    fn from(variant: EocwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC` writer - End of group conversion flag"]
pub type EocW<'a, REG> = crate::BitWriter<'a, REG, EocwWO>;
impl<'a, REG> EocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear end of group conversion flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EocwWO::Clear)
    }
}
#[doc = "Field `EOIC` reader - End of inserted group conversion flag"]
pub type EoicR = crate::BitReader;
#[doc = "Field `EOIC` writer - End of inserted group conversion flag"]
pub type EoicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STIC` reader - Start flag of inserted channel group"]
pub type SticR = crate::BitReader;
#[doc = "Field `STIC` writer - Start flag of inserted channel group"]
pub type SticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Start flag of regular channel group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Strcr {
    #[doc = "0: No regular channel conversion started"]
    NotStarted = 0,
    #[doc = "1: Regular channel conversion has started"]
    Started = 1,
}
impl From<Strcr> for bool {
    #[inline(always)]
    fn from(variant: Strcr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRC` reader - Start flag of regular channel group"]
pub type StrcR = crate::BitReader<Strcr>;
impl StrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Strcr {
        match self.bits {
            false => Strcr::NotStarted,
            true => Strcr::Started,
        }
    }
    #[doc = "No regular channel conversion started"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == Strcr::NotStarted
    }
    #[doc = "Regular channel conversion has started"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == Strcr::Started
    }
}
#[doc = "Start flag of regular channel group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrcwWO {
    #[doc = "0: Clear the regular channel start flag"]
    Clear = 0,
}
impl From<StrcwWO> for bool {
    #[inline(always)]
    fn from(variant: StrcwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRC` writer - Start flag of regular channel group"]
pub type StrcW<'a, REG> = crate::BitWriter<'a, REG, StrcwWO>;
impl<'a, REG> StrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the regular channel start flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(StrcwWO::Clear)
    }
}
#[doc = "Regular data register overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rovfr {
    #[doc = "0: No overrun occurred in routine data register"]
    NoOverrun = 0,
    #[doc = "1: Overrun overrun occurred in routine data register"]
    Overrun = 1,
}
impl From<Rovfr> for bool {
    #[inline(always)]
    fn from(variant: Rovfr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROVF` reader - Regular data register overflow"]
pub type RovfR = crate::BitReader<Rovfr>;
impl RovfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rovfr {
        match self.bits {
            false => Rovfr::NoOverrun,
            true => Rovfr::Overrun,
        }
    }
    #[doc = "No overrun occurred in routine data register"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == Rovfr::NoOverrun
    }
    #[doc = "Overrun overrun occurred in routine data register"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == Rovfr::Overrun
    }
}
#[doc = "Regular data register overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RovfwWO {
    #[doc = "0: Clear the routine data register overrun flag"]
    Clear = 0,
}
impl From<RovfwWO> for bool {
    #[inline(always)]
    fn from(variant: RovfwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROVF` writer - Regular data register overflow"]
pub type RovfW<'a, REG> = crate::BitWriter<'a, REG, RovfwWO>;
impl<'a, REG> RovfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the routine data register overrun flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RovfwWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Analog watchdog event flag"]
    #[inline(always)]
    pub fn wde(&self) -> WdeR {
        WdeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of group conversion flag"]
    #[inline(always)]
    pub fn eoc(&self) -> EocR {
        EocR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of inserted group conversion flag"]
    #[inline(always)]
    pub fn eoic(&self) -> EoicR {
        EoicR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start flag of inserted channel group"]
    #[inline(always)]
    pub fn stic(&self) -> SticR {
        SticR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Start flag of regular channel group"]
    #[inline(always)]
    pub fn strc(&self) -> StrcR {
        StrcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Regular data register overflow"]
    #[inline(always)]
    pub fn rovf(&self) -> RovfR {
        RovfR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog watchdog event flag"]
    #[inline(always)]
    pub fn wde(&mut self) -> WdeW<'_, StatSpec> {
        WdeW::new(self, 0)
    }
    #[doc = "Bit 1 - End of group conversion flag"]
    #[inline(always)]
    pub fn eoc(&mut self) -> EocW<'_, StatSpec> {
        EocW::new(self, 1)
    }
    #[doc = "Bit 2 - End of inserted group conversion flag"]
    #[inline(always)]
    pub fn eoic(&mut self) -> EoicW<'_, StatSpec> {
        EoicW::new(self, 2)
    }
    #[doc = "Bit 3 - Start flag of inserted channel group"]
    #[inline(always)]
    pub fn stic(&mut self) -> SticW<'_, StatSpec> {
        SticW::new(self, 3)
    }
    #[doc = "Bit 4 - Start flag of regular channel group"]
    #[inline(always)]
    pub fn strc(&mut self) -> StrcW<'_, StatSpec> {
        StrcW::new(self, 4)
    }
    #[doc = "Bit 5 - Regular data register overflow"]
    #[inline(always)]
    pub fn rovf(&mut self) -> RovfW<'_, StatSpec> {
        RovfW::new(self, 5)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {}
