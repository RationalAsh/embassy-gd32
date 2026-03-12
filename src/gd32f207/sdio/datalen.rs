#[doc = "Register `DATALEN` reader"]
pub type R = crate::R<DatalenSpec>;
#[doc = "Register `DATALEN` writer"]
pub type W = crate::W<DatalenSpec>;
#[doc = "Field `DATALEN` reader - Data transfer length"]
pub type DatalenR = crate::FieldReader<u32>;
#[doc = "Field `DATALEN` writer - Data transfer length"]
pub type DatalenW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - Data transfer length"]
    #[inline(always)]
    pub fn datalen(&self) -> DatalenR {
        DatalenR::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - Data transfer length"]
    #[inline(always)]
    pub fn datalen(&mut self) -> DatalenW<'_, DatalenSpec> {
        DatalenW::new(self, 0)
    }
}
#[doc = "Data length register\n\nYou can [`read`](crate::Reg::read) this register and get [`datalen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datalen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatalenSpec;
impl crate::RegisterSpec for DatalenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datalen::R`](R) reader structure"]
impl crate::Readable for DatalenSpec {}
#[doc = "`write(|w| ..)` method takes [`datalen::W`](W) writer structure"]
impl crate::Writable for DatalenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATALEN to value 0"]
impl crate::Resettable for DatalenSpec {}
