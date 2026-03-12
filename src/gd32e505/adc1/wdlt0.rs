#[doc = "Register `WDLT0` reader"]
pub type R = crate::R<Wdlt0Spec>;
#[doc = "Register `WDLT0` writer"]
pub type W = crate::W<Wdlt0Spec>;
#[doc = "Field `WDLT0` reader - Analog watchdog 0 lower threshold"]
pub type Wdlt0R = crate::FieldReader<u16>;
#[doc = "Field `WDLT0` writer - Analog watchdog 0 lower threshold"]
pub type Wdlt0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Analog watchdog 0 lower threshold"]
    #[inline(always)]
    pub fn wdlt0(&self) -> Wdlt0R {
        Wdlt0R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog 0 lower threshold"]
    #[inline(always)]
    pub fn wdlt0(&mut self) -> Wdlt0W<'_, Wdlt0Spec> {
        Wdlt0W::new(self, 0)
    }
}
#[doc = "watchdog lower threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdlt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdlt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdlt0Spec;
impl crate::RegisterSpec for Wdlt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdlt0::R`](R) reader structure"]
impl crate::Readable for Wdlt0Spec {}
#[doc = "`write(|w| ..)` method takes [`wdlt0::W`](W) writer structure"]
impl crate::Writable for Wdlt0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDLT0 to value 0"]
impl crate::Resettable for Wdlt0Spec {}
