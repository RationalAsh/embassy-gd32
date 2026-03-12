#[doc = "Register `FDATA` reader"]
pub type R = crate::R<FdataSpec>;
#[doc = "Register `FDATA` writer"]
pub type W = crate::W<FdataSpec>;
#[doc = "Field `FDATA` reader - Free Data Register bits"]
pub type FdataR = crate::FieldReader;
#[doc = "Field `FDATA` writer - Free Data Register bits"]
pub type FdataW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - Free Data Register bits"]
    #[inline(always)]
    pub fn fdata(&self) -> FdataR {
        FdataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Free Data Register bits"]
    #[inline(always)]
    pub fn fdata(&mut self) -> FdataW<'_, FdataSpec> {
        FdataW::new(self, 0)
    }
}
#[doc = "Free data register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdataSpec;
impl crate::RegisterSpec for FdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdata::R`](R) reader structure"]
impl crate::Readable for FdataSpec {}
#[doc = "`write(|w| ..)` method takes [`fdata::W`](W) writer structure"]
impl crate::Writable for FdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDATA to value 0"]
impl crate::Resettable for FdataSpec {}
