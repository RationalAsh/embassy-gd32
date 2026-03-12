#[doc = "Register `TDATA` reader"]
pub type R = crate::R<TdataSpec>;
#[doc = "Register `TDATA` writer"]
pub type W = crate::W<TdataSpec>;
#[doc = "Field `TDATA` reader - Transmit Data value"]
pub type TdataR = crate::FieldReader<u16>;
#[doc = "Field `TDATA` writer - Transmit Data value"]
pub type TdataW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Transmit Data value"]
    #[inline(always)]
    pub fn tdata(&self) -> TdataR {
        TdataR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Transmit Data value"]
    #[inline(always)]
    pub fn tdata(&mut self) -> TdataW<'_, TdataSpec> {
        TdataW::new(self, 0)
    }
}
#[doc = "Transmit data register\n\nYou can [`read`](crate::Reg::read) this register and get [`tdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdataSpec;
impl crate::RegisterSpec for TdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdata::R`](R) reader structure"]
impl crate::Readable for TdataSpec {}
#[doc = "`write(|w| ..)` method takes [`tdata::W`](W) writer structure"]
impl crate::Writable for TdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TDATA to value 0"]
impl crate::Resettable for TdataSpec {}
