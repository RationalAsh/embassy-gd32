#[doc = "Register `SWEVG` writer"]
pub type W = crate::W<SwevgSpec>;
#[doc = "Update generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Upg {
    #[doc = "1: Re-initializes the timer counter and generates an update of the registers."]
    Update = 1,
}
impl From<Upg> for bool {
    #[inline(always)]
    fn from(variant: Upg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPG` writer - Update generation"]
pub type UpgW<'a, REG> = crate::BitWriter<'a, REG, Upg>;
impl<'a, REG> UpgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Re-initializes the timer counter and generates an update of the registers."]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(Upg::Update)
    }
}
#[doc = "Channel 0 capture or compare event generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0g {
    #[doc = "1: Generate a capture or compare event"]
    CaptureCompare = 1,
}
impl From<Ch0g> for bool {
    #[inline(always)]
    fn from(variant: Ch0g) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0G` writer - Channel 0 capture or compare event generation"]
pub type Ch0gW<'a, REG> = crate::BitWriter<'a, REG, Ch0g>;
impl<'a, REG> Ch0gW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate a capture or compare event"]
    #[inline(always)]
    pub fn capture_compare(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0g::CaptureCompare)
    }
}
#[doc = "Field `CH1G` writer - Channel 1 capture or compare event generation"]
pub use Ch0gW as Ch1gW;
#[doc = "Field `CH2G` writer - Channel 2 capture or compare event generation"]
pub use Ch0gW as Ch2gW;
#[doc = "Field `CH3G` writer - Channel 3 capture or compare event generation"]
pub use Ch0gW as Ch3gW;
#[doc = "Channel commutation event generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmtg {
    #[doc = "1: Generate a channel commutation event, updating capture/compare control registers based on the value of CCSE"]
    Update = 1,
}
impl From<Cmtg> for bool {
    #[inline(always)]
    fn from(variant: Cmtg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMTG` writer - Channel commutation event generation"]
pub type CmtgW<'a, REG> = crate::BitWriter<'a, REG, Cmtg>;
impl<'a, REG> CmtgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate a channel commutation event, updating capture/compare control registers based on the value of CCSE"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(Cmtg::Update)
    }
}
#[doc = "Trigger event generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trgg {
    #[doc = "1: Generate a trigger event"]
    Trigger = 1,
}
impl From<Trgg> for bool {
    #[inline(always)]
    fn from(variant: Trgg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGG` writer - Trigger event generation"]
pub type TrggW<'a, REG> = crate::BitWriter<'a, REG, Trgg>;
impl<'a, REG> TrggW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate a trigger event"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgg::Trigger)
    }
}
#[doc = "Break event generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brkg {
    #[doc = "1: Generate a break event"]
    Break = 1,
}
impl From<Brkg> for bool {
    #[inline(always)]
    fn from(variant: Brkg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKG` writer - Break event generation"]
pub type BrkgW<'a, REG> = crate::BitWriter<'a, REG, Brkg>;
impl<'a, REG> BrkgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate a break event"]
    #[inline(always)]
    pub fn break_(self) -> &'a mut crate::W<REG> {
        self.variant(Brkg::Break)
    }
}
impl W {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    pub fn upg(&mut self) -> UpgW<'_, SwevgSpec> {
        UpgW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 0 capture or compare event generation"]
    #[inline(always)]
    pub fn ch0g(&mut self) -> Ch0gW<'_, SwevgSpec> {
        Ch0gW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 1 capture or compare event generation"]
    #[inline(always)]
    pub fn ch1g(&mut self) -> Ch1gW<'_, SwevgSpec> {
        Ch1gW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 2 capture or compare event generation"]
    #[inline(always)]
    pub fn ch2g(&mut self) -> Ch2gW<'_, SwevgSpec> {
        Ch2gW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 3 capture or compare event generation"]
    #[inline(always)]
    pub fn ch3g(&mut self) -> Ch3gW<'_, SwevgSpec> {
        Ch3gW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel commutation event generation"]
    #[inline(always)]
    pub fn cmtg(&mut self) -> CmtgW<'_, SwevgSpec> {
        CmtgW::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger event generation"]
    #[inline(always)]
    pub fn trgg(&mut self) -> TrggW<'_, SwevgSpec> {
        TrggW::new(self, 6)
    }
    #[doc = "Bit 7 - Break event generation"]
    #[inline(always)]
    pub fn brkg(&mut self) -> BrkgW<'_, SwevgSpec> {
        BrkgW::new(self, 7)
    }
}
#[doc = "Software event generation register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swevg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwevgSpec;
impl crate::RegisterSpec for SwevgSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`swevg::W`](W) writer structure"]
impl crate::Writable for SwevgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWEVG to value 0"]
impl crate::Resettable for SwevgSpec {}
