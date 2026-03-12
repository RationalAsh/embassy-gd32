#[doc = "Register `BKP3` reader"]
pub type R = crate::R<Bkp3Spec>;
#[doc = "Register `BKP3` writer"]
pub type W = crate::W<Bkp3Spec>;
#[doc = "Field `DATA` reader - BKP data"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - BKP data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - BKP data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BKP data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Bkp3Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bkp3Spec;
impl crate::RegisterSpec for Bkp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp3::R`](R) reader structure"]
impl crate::Readable for Bkp3Spec {}
#[doc = "`write(|w| ..)` method takes [`bkp3::W`](W) writer structure"]
impl crate::Writable for Bkp3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BKP3 to value 0"]
impl crate::Resettable for Bkp3Spec {}
