#[doc = "Register `BKP11` reader"]
pub type R = crate::R<Bkp11Spec>;
#[doc = "Register `BKP11` writer"]
pub type W = crate::W<Bkp11Spec>;
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
    pub fn data(&mut self) -> DataW<'_, Bkp11Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bkp11Spec;
impl crate::RegisterSpec for Bkp11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp11::R`](R) reader structure"]
impl crate::Readable for Bkp11Spec {}
#[doc = "`write(|w| ..)` method takes [`bkp11::W`](W) writer structure"]
impl crate::Writable for Bkp11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BKP11 to value 0"]
impl crate::Resettable for Bkp11Spec {}
