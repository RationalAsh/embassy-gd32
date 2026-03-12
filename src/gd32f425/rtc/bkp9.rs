#[doc = "Register `BKP9` reader"]
pub type R = crate::R<Bkp9Spec>;
#[doc = "Register `BKP9` writer"]
pub type W = crate::W<Bkp9Spec>;
#[doc = "Field `DATA` reader - Data"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Bkp9Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bkp9Spec;
impl crate::RegisterSpec for Bkp9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp9::R`](R) reader structure"]
impl crate::Readable for Bkp9Spec {}
#[doc = "`write(|w| ..)` method takes [`bkp9::W`](W) writer structure"]
impl crate::Writable for Bkp9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BKP9 to value 0"]
impl crate::Resettable for Bkp9Spec {}
