#[doc = "Register `KEY1` writer"]
pub type W = crate::W<Key1Spec>;
#[doc = "Field `KEY` writer - FMC_CTL unlock register"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl W {
    #[doc = "Bits 0:31 - FMC_CTL unlock register"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, Key1Spec> {
        KeyW::new(self, 0)
    }
}
#[doc = "Unlock key register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key1Spec;
impl crate::RegisterSpec for Key1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key1::W`](W) writer structure"]
impl crate::Writable for Key1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets KEY1 to value 0"]
impl crate::Resettable for Key1Spec {}
