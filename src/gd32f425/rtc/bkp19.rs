#[doc = "Register `BKP19` reader"]
pub type R = crate::R<Bkp19Spec>;
#[doc = "Register `BKP19` writer"]
pub type W = crate::W<Bkp19Spec>;
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
    pub fn data(&mut self) -> DataW<'_, Bkp19Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bkp19Spec;
impl crate::RegisterSpec for Bkp19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp19::R`](R) reader structure"]
impl crate::Readable for Bkp19Spec {}
#[doc = "`write(|w| ..)` method takes [`bkp19::W`](W) writer structure"]
impl crate::Writable for Bkp19Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BKP19 to value 0"]
impl crate::Resettable for Bkp19Spec {}
