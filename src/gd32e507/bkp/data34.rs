#[doc = "Register `DATA34` reader"]
pub type R = crate::R<Data34Spec>;
#[doc = "Register `DATA34` writer"]
pub type W = crate::W<Data34Spec>;
#[doc = "Field `DATA` reader - Backup data"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - Backup data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Data34Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Backup data register 34\n\nYou can [`read`](crate::Reg::read) this register and get [`data34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data34Spec;
impl crate::RegisterSpec for Data34Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`data34::R`](R) reader structure"]
impl crate::Readable for Data34Spec {}
#[doc = "`write(|w| ..)` method takes [`data34::W`](W) writer structure"]
impl crate::Writable for Data34Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA34 to value 0"]
impl crate::Resettable for Data34Spec {}
