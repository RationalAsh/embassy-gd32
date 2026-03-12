#[doc = "Register `CHCTL0_Input` reader"]
pub type R = crate::R<Chctl0InputSpec>;
#[doc = "Register `CHCTL0_Input` writer"]
pub type W = crate::W<Chctl0InputSpec>;
#[doc = "Input capture 0 filter"]
pub use crate::gd32e230::timer0::chctl0_input::Ch0capflt;
#[doc = "Field `CH0CAPFLT` reader - Input capture 0 filter"]
pub use crate::gd32e230::timer0::chctl0_input::Ch0capfltR;
#[doc = "Field `CH1CAPFLT` reader - Input capture 1 filter"]
pub use crate::gd32e230::timer0::chctl0_input::Ch0capfltR as Ch1capfltR;
#[doc = "Field `CH0CAPFLT` writer - Input capture 0 filter"]
pub use crate::gd32e230::timer0::chctl0_input::Ch0capfltW;
#[doc = "Field `CH1CAPFLT` writer - Input capture 1 filter"]
pub use crate::gd32e230::timer0::chctl0_input::Ch0capfltW as Ch1capfltW;
#[doc = "Input capture 0 prescaler"]
pub use crate::gd32e230::timer0::chctl0_input::Ch0cappsc;
#[doc = "Field `CH0CAPPSC` reader - Input capture 0 prescaler"]
pub use crate::gd32e230::timer0::chctl0_input::Ch0cappscR;
#[doc = "Field `CH1CAPPSC` reader - Input capture 1 prescaler"]
pub use crate::gd32e230::timer0::chctl0_input::Ch0cappscR as Ch1cappscR;
#[doc = "Field `CH0CAPPSC` writer - Input capture 0 prescaler"]
pub use crate::gd32e230::timer0::chctl0_input::Ch0cappscW;
#[doc = "Field `CH1CAPPSC` writer - Input capture 1 prescaler"]
pub use crate::gd32e230::timer0::chctl0_input::Ch0cappscW as Ch1cappscW;
#[doc = "Capture/Compare 0 selection"]
pub use crate::gd32e230::timer0::chctl0_input::Ch0ms;
#[doc = "Field `CH0MS` reader - Capture/Compare 0 selection"]
pub use crate::gd32e230::timer0::chctl0_input::Ch0msR;
#[doc = "Field `CH1MS` reader - Capture/Compare 1 selection"]
pub use crate::gd32e230::timer0::chctl0_input::Ch0msR as Ch1msR;
#[doc = "Field `CH0MS` writer - Capture/Compare 0 selection"]
pub use crate::gd32e230::timer0::chctl0_input::Ch0msW;
#[doc = "Field `CH1MS` writer - Capture/Compare 1 selection"]
pub use crate::gd32e230::timer0::chctl0_input::Ch0msW as Ch1msW;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 0 selection"]
    #[inline(always)]
    pub fn ch0ms(&self) -> Ch0msR {
        Ch0msR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Input capture 0 prescaler"]
    #[inline(always)]
    pub fn ch0cappsc(&self) -> Ch0cappscR {
        Ch0cappscR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 0 filter"]
    #[inline(always)]
    pub fn ch0capflt(&self) -> Ch0capfltR {
        Ch0capfltR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn ch1ms(&self) -> Ch1msR {
        Ch1msR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Input capture 1 prescaler"]
    #[inline(always)]
    pub fn ch1cappsc(&self) -> Ch1cappscR {
        Ch1cappscR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Input capture 1 filter"]
    #[inline(always)]
    pub fn ch1capflt(&self) -> Ch1capfltR {
        Ch1capfltR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 0 selection"]
    #[inline(always)]
    pub fn ch0ms(&mut self) -> Ch0msW<'_, Chctl0InputSpec> {
        Ch0msW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Input capture 0 prescaler"]
    #[inline(always)]
    pub fn ch0cappsc(&mut self) -> Ch0cappscW<'_, Chctl0InputSpec> {
        Ch0cappscW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Input capture 0 filter"]
    #[inline(always)]
    pub fn ch0capflt(&mut self) -> Ch0capfltW<'_, Chctl0InputSpec> {
        Ch0capfltW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn ch1ms(&mut self) -> Ch1msW<'_, Chctl0InputSpec> {
        Ch1msW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Input capture 1 prescaler"]
    #[inline(always)]
    pub fn ch1cappsc(&mut self) -> Ch1cappscW<'_, Chctl0InputSpec> {
        Ch1cappscW::new(self, 10)
    }
    #[doc = "Bits 12:15 - Input capture 1 filter"]
    #[inline(always)]
    pub fn ch1capflt(&mut self) -> Ch1capfltW<'_, Chctl0InputSpec> {
        Ch1capfltW::new(self, 12)
    }
}
#[doc = "capture/compare mode register 0 (input mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`chctl0_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctl0_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chctl0InputSpec;
impl crate::RegisterSpec for Chctl0InputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctl0_input::R`](R) reader structure"]
impl crate::Readable for Chctl0InputSpec {}
#[doc = "`write(|w| ..)` method takes [`chctl0_input::W`](W) writer structure"]
impl crate::Writable for Chctl0InputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHCTL0_Input to value 0"]
impl crate::Resettable for Chctl0InputSpec {}
