#[doc = "Register `KEY0` writer"]
pub type W = crate::W<Key0Spec>;
#[doc = "Field `KEY` writer - FMC_CTL0 unlock key"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl W {
    #[doc = "Bits 0:31 - FMC_CTL0 unlock key"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, Key0Spec> {
        KeyW::new(self, 0)
    }
}
#[doc = "Flash unlock key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key0Spec;
impl crate::RegisterSpec for Key0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key0::W`](W) writer structure"]
impl crate::Writable for Key0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets KEY0 to value 0"]
impl crate::Resettable for Key0Spec {}
