#[doc = "Register `KEY3H` writer"]
pub type W = crate::W<Key3hSpec>;
#[doc = "Field `KEY3H` writer - Key for DES,TDES,AES"]
pub type Key3hW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Key for DES,TDES,AES"]
    #[inline(always)]
    pub fn key3h(&mut self) -> Key3hW<'_, Key3hSpec> {
        Key3hW::new(self, 0)
    }
}
#[doc = "CAU key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key3h::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key3hSpec;
impl crate::RegisterSpec for Key3hSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key3h::W`](W) writer structure"]
impl crate::Writable for Key3hSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY3H to value 0"]
impl crate::Resettable for Key3hSpec {}
