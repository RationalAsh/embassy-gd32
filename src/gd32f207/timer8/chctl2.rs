#[doc = "Register `CHCTL2` reader"]
pub type R = crate::R<Chctl2Spec>;
#[doc = "Register `CHCTL2` writer"]
pub type W = crate::W<Chctl2Spec>;
#[doc = "Field `CH0EN` reader - Channel 0 capture/compare function enable"]
pub type Ch0enR = crate::BitReader;
#[doc = "Field `CH0EN` writer - Channel 0 capture/compare function enable"]
pub type Ch0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0P` reader - Channel 0 capture/compare function polarity"]
pub type Ch0pR = crate::BitReader;
#[doc = "Field `CH0P` writer - Channel 0 capture/compare function polarity"]
pub type Ch0pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0NP` reader - Channel 0 complementary output polarity"]
pub type Ch0npR = crate::BitReader;
#[doc = "Field `CH0NP` writer - Channel 0 complementary output polarity"]
pub type Ch0npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1EN` reader - Channel 1 capture/compare function enable"]
pub type Ch1enR = crate::BitReader;
#[doc = "Field `CH1EN` writer - Channel 1 capture/compare function enable"]
pub type Ch1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1P` reader - Channel 1 capture/compare function polarity"]
pub type Ch1pR = crate::BitReader;
#[doc = "Field `CH1P` writer - Channel 1 capture/compare function polarity"]
pub type Ch1pW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 3 - Channel 0 complementary output polarity"]
    #[inline(always)]
    pub fn ch0np(&self) -> Ch0npR {
        Ch0npR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 3 - Channel 0 complementary output polarity"]
    #[inline(always)]
    pub fn ch0np(&mut self) -> Ch0npW<'_, Chctl2Spec> {
        Ch0npW::new(self, 3)
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
