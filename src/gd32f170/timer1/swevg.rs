#[doc = "Register `SWEVG` writer"]
pub type W = crate::W<SwevgSpec>;
#[doc = "Channel 0 capture or compare event generation"]
pub use crate::gd32f170::timer0::swevg::Ch0g;
#[doc = "Field `CH0G` writer - Channel 0 capture or compare event generation"]
pub use crate::gd32f170::timer0::swevg::Ch0gW;
#[doc = "Field `CH1G` writer - Channel 1 capture or compare event generation"]
pub use crate::gd32f170::timer0::swevg::Ch0gW as Ch1gW;
#[doc = "Field `CH2G` writer - Channel 2 capture or compare event generation"]
pub use crate::gd32f170::timer0::swevg::Ch0gW as Ch2gW;
#[doc = "Field `CH3G` writer - Channel 3 capture or compare event generation"]
pub use crate::gd32f170::timer0::swevg::Ch0gW as Ch3gW;
#[doc = "Update generation"]
pub use crate::gd32f170::timer0::swevg::Upg;
#[doc = "Field `UPG` writer - Update generation"]
pub use crate::gd32f170::timer0::swevg::UpgW;
#[doc = "Trigger generation\n\nValue on reset: 0"]
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
#[doc = "Field `TRGG` writer - Trigger generation"]
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
    #[doc = "Bit 6 - Trigger generation"]
    #[inline(always)]
    pub fn trgg(&mut self) -> TrggW<'_, SwevgSpec> {
        TrggW::new(self, 6)
    }
}
#[doc = "event generation register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swevg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
