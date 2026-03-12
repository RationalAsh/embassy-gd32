#[doc = "Register `DATA6` reader"]
pub type R = crate::R<Data6Spec>;
#[doc = "Register `DATA6` writer"]
pub type W = crate::W<Data6Spec>;
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
    pub fn data(&mut self) -> DataW<'_, Data6Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Backup data register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`data6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data6Spec;
impl crate::RegisterSpec for Data6Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`data6::R`](R) reader structure"]
impl crate::Readable for Data6Spec {}
#[doc = "`write(|w| ..)` method takes [`data6::W`](W) writer structure"]
impl crate::Writable for Data6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA6 to value 0"]
impl crate::Resettable for Data6Spec {}
