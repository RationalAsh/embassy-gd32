#[doc = "Register `KEY2L` writer"]
pub type W = crate::W<Key2lSpec>;
#[doc = "Field `KEY2L` writer - Key for DES,TDES,AES"]
pub type Key2lW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Key for DES,TDES,AES"]
    #[inline(always)]
    pub fn key2l(&mut self) -> Key2lW<'_, Key2lSpec> {
        Key2lW::new(self, 0)
    }
}
#[doc = "CAU key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key2l::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key2lSpec;
impl crate::RegisterSpec for Key2lSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key2l::W`](W) writer structure"]
impl crate::Writable for Key2lSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY2L to value 0"]
impl crate::Resettable for Key2lSpec {}
