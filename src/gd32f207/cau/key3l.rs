#[doc = "Register `KEY3L` writer"]
pub type W = crate::W<Key3lSpec>;
#[doc = "Field `KEY3L` writer - Key for DES,TDES,AES"]
pub type Key3lW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Key for DES,TDES,AES"]
    #[inline(always)]
    pub fn key3l(&mut self) -> Key3lW<'_, Key3lSpec> {
        Key3lW::new(self, 0)
    }
}
#[doc = "CAU key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key3l::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key3lSpec;
impl crate::RegisterSpec for Key3lSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key3l::W`](W) writer structure"]
impl crate::Writable for Key3lSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY3L to value 0"]
impl crate::Resettable for Key3lSpec {}
