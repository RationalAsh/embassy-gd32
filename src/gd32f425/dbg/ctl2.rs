#[doc = "Register `CTL2` reader"]
pub type R = crate::R<Ctl2Spec>;
#[doc = "Register `CTL2` writer"]
pub type W = crate::W<Ctl2Spec>;
#[doc = "TIMER 0 hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0Hold {
    #[doc = "0: Continue running the timer as usual"]
    Continue = 0,
    #[doc = "1: Hold the timer counter for debug when the core is halted"]
    Stop = 1,
}
impl From<Timer0Hold> for bool {
    #[inline(always)]
    fn from(variant: Timer0Hold) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER0_HOLD` reader - TIMER 0 hold register"]
pub type Timer0HoldR = crate::BitReader<Timer0Hold>;
impl Timer0HoldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0Hold {
        match self.bits {
            false => Timer0Hold::Continue,
            true => Timer0Hold::Stop,
        }
    }
    #[doc = "Continue running the timer as usual"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == Timer0Hold::Continue
    }
    #[doc = "Hold the timer counter for debug when the core is halted"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Timer0Hold::Stop
    }
}
#[doc = "Field `TIMER0_HOLD` writer - TIMER 0 hold register"]
pub type Timer0HoldW<'a, REG> = crate::BitWriter<'a, REG, Timer0Hold>;
impl<'a, REG> Timer0HoldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continue running the timer as usual"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0Hold::Continue)
    }
    #[doc = "Hold the timer counter for debug when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0Hold::Stop)
    }
}
#[doc = "Field `TIMER7_HOLD` reader - TIMER 7 hold register"]
pub use Timer0HoldR as Timer7HoldR;
#[doc = "Field `TIMER8_HOLD` reader - TIMER 8 hold register"]
pub use Timer0HoldR as Timer8HoldR;
#[doc = "Field `TIMER9_HOLD` reader - TIMER 9 hold register"]
pub use Timer0HoldR as Timer9HoldR;
#[doc = "Field `TIMER10_HOLD` reader - TIMER 10 hold register"]
pub use Timer0HoldR as Timer10HoldR;
#[doc = "Field `TIMER7_HOLD` writer - TIMER 7 hold register"]
pub use Timer0HoldW as Timer7HoldW;
#[doc = "Field `TIMER8_HOLD` writer - TIMER 8 hold register"]
pub use Timer0HoldW as Timer8HoldW;
#[doc = "Field `TIMER9_HOLD` writer - TIMER 9 hold register"]
pub use Timer0HoldW as Timer9HoldW;
#[doc = "Field `TIMER10_HOLD` writer - TIMER 10 hold register"]
pub use Timer0HoldW as Timer10HoldW;
impl R {
    #[doc = "Bit 0 - TIMER 0 hold register"]
    #[inline(always)]
    pub fn timer0_hold(&self) -> Timer0HoldR {
        Timer0HoldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIMER 7 hold register"]
    #[inline(always)]
    pub fn timer7_hold(&self) -> Timer7HoldR {
        Timer7HoldR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - TIMER 8 hold register"]
    #[inline(always)]
    pub fn timer8_hold(&self) -> Timer8HoldR {
        Timer8HoldR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIMER 9 hold register"]
    #[inline(always)]
    pub fn timer9_hold(&self) -> Timer9HoldR {
        Timer9HoldR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIMER 10 hold register"]
    #[inline(always)]
    pub fn timer10_hold(&self) -> Timer10HoldR {
        Timer10HoldR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIMER 0 hold register"]
    #[inline(always)]
    pub fn timer0_hold(&mut self) -> Timer0HoldW<'_, Ctl2Spec> {
        Timer0HoldW::new(self, 0)
    }
    #[doc = "Bit 1 - TIMER 7 hold register"]
    #[inline(always)]
    pub fn timer7_hold(&mut self) -> Timer7HoldW<'_, Ctl2Spec> {
        Timer7HoldW::new(self, 1)
    }
    #[doc = "Bit 16 - TIMER 8 hold register"]
    #[inline(always)]
    pub fn timer8_hold(&mut self) -> Timer8HoldW<'_, Ctl2Spec> {
        Timer8HoldW::new(self, 16)
    }
    #[doc = "Bit 17 - TIMER 9 hold register"]
    #[inline(always)]
    pub fn timer9_hold(&mut self) -> Timer9HoldW<'_, Ctl2Spec> {
        Timer9HoldW::new(self, 17)
    }
    #[doc = "Bit 18 - TIMER 10 hold register"]
    #[inline(always)]
    pub fn timer10_hold(&mut self) -> Timer10HoldW<'_, Ctl2Spec> {
        Timer10HoldW::new(self, 18)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl2Spec;
impl crate::RegisterSpec for Ctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl2::R`](R) reader structure"]
impl crate::Readable for Ctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl2::W`](W) writer structure"]
impl crate::Writable for Ctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL2 to value 0"]
impl crate::Resettable for Ctl2Spec {}
