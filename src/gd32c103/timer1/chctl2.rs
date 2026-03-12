#[doc = "Register `CHCTL2` reader"]
pub type R = crate::R<Chctl2Spec>;
#[doc = "Register `CHCTL2` writer"]
pub type W = crate::W<Chctl2Spec>;
#[doc = "Channel 0 capture/compare function enable"]
pub use crate::gd32c103::timer0::chctl2::Ch0en;
#[doc = "Field `CH0EN` reader - Channel 0 capture/compare function enable"]
pub use crate::gd32c103::timer0::chctl2::Ch0enR;
#[doc = "Field `CH1EN` reader - Channel 1 capture/compare function enable"]
pub use crate::gd32c103::timer0::chctl2::Ch0enR as Ch1enR;
#[doc = "Field `CH2EN` reader - Channel 2 capture/compare function enable"]
pub use crate::gd32c103::timer0::chctl2::Ch0enR as Ch2enR;
#[doc = "Field `CH3EN` reader - Channel 3 capture/compare function enable"]
pub use crate::gd32c103::timer0::chctl2::Ch0enR as Ch3enR;
#[doc = "Field `CH0EN` writer - Channel 0 capture/compare function enable"]
pub use crate::gd32c103::timer0::chctl2::Ch0enW;
#[doc = "Field `CH1EN` writer - Channel 1 capture/compare function enable"]
pub use crate::gd32c103::timer0::chctl2::Ch0enW as Ch1enW;
#[doc = "Field `CH2EN` writer - Channel 2 capture/compare function enable"]
pub use crate::gd32c103::timer0::chctl2::Ch0enW as Ch2enW;
#[doc = "Field `CH3EN` writer - Channel 3 capture/compare function enable"]
pub use crate::gd32c103::timer0::chctl2::Ch0enW as Ch3enW;
#[doc = "Channel 0 capture/compare function polarity"]
pub use crate::gd32c103::timer0::chctl2::Ch0p;
#[doc = "Field `CH0P` reader - Channel 0 capture/compare function polarity"]
pub use crate::gd32c103::timer0::chctl2::Ch0pR;
#[doc = "Field `CH1P` reader - Channel 1 capture/compare function polarity"]
pub use crate::gd32c103::timer0::chctl2::Ch0pR as Ch1pR;
#[doc = "Field `CH2P` reader - Channel 2 capture/compare function polarity"]
pub use crate::gd32c103::timer0::chctl2::Ch0pR as Ch2pR;
#[doc = "Field `CH3P` reader - Channel 3 capture/compare function polarity"]
pub use crate::gd32c103::timer0::chctl2::Ch0pR as Ch3pR;
#[doc = "Field `CH0P` writer - Channel 0 capture/compare function polarity"]
pub use crate::gd32c103::timer0::chctl2::Ch0pW;
#[doc = "Field `CH1P` writer - Channel 1 capture/compare function polarity"]
pub use crate::gd32c103::timer0::chctl2::Ch0pW as Ch1pW;
#[doc = "Field `CH2P` writer - Channel 2 capture/compare function polarity"]
pub use crate::gd32c103::timer0::chctl2::Ch0pW as Ch2pW;
#[doc = "Field `CH3P` writer - Channel 3 capture/compare function polarity"]
pub use crate::gd32c103::timer0::chctl2::Ch0pW as Ch3pW;
impl R {
    #[doc = "Bit 0 - Channel 0 capture/compare function enable"]
    #[inline(always)]
    pub fn ch0en(&self) -> Ch0enR {
        Ch0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch0p(&self) -> Ch0pR {
        Ch0pR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 1 capture/compare function enable"]
    #[inline(always)]
    pub fn ch1en(&self) -> Ch1enR {
        Ch1enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 1 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch1p(&self) -> Ch1pR {
        Ch1pR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 2 capture/compare function enable"]
    #[inline(always)]
    pub fn ch2en(&self) -> Ch2enR {
        Ch2enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 2 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch2p(&self) -> Ch2pR {
        Ch2pR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 3 capture/compare function enable"]
    #[inline(always)]
    pub fn ch3en(&self) -> Ch3enR {
        Ch3enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 3 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch3p(&self) -> Ch3pR {
        Ch3pR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 capture/compare function enable"]
    #[inline(always)]
    pub fn ch0en(&mut self) -> Ch0enW<'_, Chctl2Spec> {
        Ch0enW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch0p(&mut self) -> Ch0pW<'_, Chctl2Spec> {
        Ch0pW::new(self, 1)
    }
    #[doc = "Bit 4 - Channel 1 capture/compare function enable"]
    #[inline(always)]
    pub fn ch1en(&mut self) -> Ch1enW<'_, Chctl2Spec> {
        Ch1enW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 1 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch1p(&mut self) -> Ch1pW<'_, Chctl2Spec> {
        Ch1pW::new(self, 5)
    }
    #[doc = "Bit 8 - Channel 2 capture/compare function enable"]
    #[inline(always)]
    pub fn ch2en(&mut self) -> Ch2enW<'_, Chctl2Spec> {
        Ch2enW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 2 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch2p(&mut self) -> Ch2pW<'_, Chctl2Spec> {
        Ch2pW::new(self, 9)
    }
    #[doc = "Bit 12 - Channel 3 capture/compare function enable"]
    #[inline(always)]
    pub fn ch3en(&mut self) -> Ch3enW<'_, Chctl2Spec> {
        Ch3enW::new(self, 12)
    }
    #[doc = "Bit 13 - Channel 3 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch3p(&mut self) -> Ch3pW<'_, Chctl2Spec> {
        Ch3pW::new(self, 13)
    }
}
#[doc = "Channel control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`chctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chctl2Spec;
impl crate::RegisterSpec for Chctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctl2::R`](R) reader structure"]
impl crate::Readable for Chctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`chctl2::W`](W) writer structure"]
impl crate::Writable for Chctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHCTL2 to value 0"]
impl crate::Resettable for Chctl2Spec {}
