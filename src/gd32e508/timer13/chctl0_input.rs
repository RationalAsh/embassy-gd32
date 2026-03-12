#[doc = "Register `CHCTL0_Input` reader"]
pub type R = crate::R<Chctl0InputSpec>;
#[doc = "Register `CHCTL0_Input` writer"]
pub type W = crate::W<Chctl0InputSpec>;
#[doc = "Channel 0 input capture filter control"]
pub use crate::gd32e508::timer0::chctl0_input::Ch0capflt;
#[doc = "Field `CH0CAPFLT` reader - Channel 0 input capture filter control"]
pub use crate::gd32e508::timer0::chctl0_input::Ch0capfltR;
#[doc = "Field `CH0CAPFLT` writer - Channel 0 input capture filter control"]
pub use crate::gd32e508::timer0::chctl0_input::Ch0capfltW;
#[doc = "Channel 0 input capture prescaler"]
pub use crate::gd32e508::timer0::chctl0_input::Ch0cappsc;
#[doc = "Field `CH0CAPPSC` reader - Channel 0 input capture prescaler"]
pub use crate::gd32e508::timer0::chctl0_input::Ch0cappscR;
#[doc = "Field `CH0CAPPSC` writer - Channel 0 input capture prescaler"]
pub use crate::gd32e508::timer0::chctl0_input::Ch0cappscW;
#[doc = "Channel 0 mode selection"]
pub use crate::gd32e508::timer0::chctl0_input::Ch0ms;
#[doc = "Field `CH0MS` reader - Channel 0 mode selection"]
pub use crate::gd32e508::timer0::chctl0_input::Ch0msR;
#[doc = "Field `CH0MS` writer - Channel 0 mode selection"]
pub use crate::gd32e508::timer0::chctl0_input::Ch0msW;
impl R {
    #[doc = "Bits 0:1 - Channel 0 mode selection"]
    #[inline(always)]
    pub fn ch0ms(&self) -> Ch0msR {
        Ch0msR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel 0 input capture prescaler"]
    #[inline(always)]
    pub fn ch0cappsc(&self) -> Ch0cappscR {
        Ch0cappscR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Channel 0 input capture filter control"]
    #[inline(always)]
    pub fn ch0capflt(&self) -> Ch0capfltR {
        Ch0capfltR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 0 mode selection"]
    #[inline(always)]
    pub fn ch0ms(&mut self) -> Ch0msW<'_, Chctl0InputSpec> {
        Ch0msW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Channel 0 input capture prescaler"]
    #[inline(always)]
    pub fn ch0cappsc(&mut self) -> Ch0cappscW<'_, Chctl0InputSpec> {
        Ch0cappscW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Channel 0 input capture filter control"]
    #[inline(always)]
    pub fn ch0capflt(&mut self) -> Ch0capfltW<'_, Chctl0InputSpec> {
        Ch0capfltW::new(self, 4)
    }
}
#[doc = "Channel control register 0 ( (input mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`chctl0_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctl0_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
