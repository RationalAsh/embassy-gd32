#[doc = "Register `KEY1L` writer"]
pub type W = crate::W<Key1lSpec>;
#[doc = "Field `KEY1L` writer - Key for DES,TDES,AES"]
pub type Key1lW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Key for DES,TDES,AES"]
    #[inline(always)]
    pub fn key1l(&mut self) -> Key1lW<'_, Key1lSpec> {
        Key1lW::new(self, 0)
    }
}
#[doc = "CAU key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1l::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key1lSpec;
impl crate::RegisterSpec for Key1lSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key1l::W`](W) writer structure"]
impl crate::Writable for Key1lSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY1L to value 0"]
impl crate::Resettable for Key1lSpec {}
