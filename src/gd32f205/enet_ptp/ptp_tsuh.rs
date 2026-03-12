#[doc = "Register `PTP_TSUH` reader"]
pub type R = crate::R<PtpTsuhSpec>;
#[doc = "Register `PTP_TSUH` writer"]
pub type W = crate::W<PtpTsuhSpec>;
#[doc = "Field `TMSUS` reader - Time stamp update second"]
pub type TmsusR = crate::FieldReader<u32>;
#[doc = "Field `TMSUS` writer - Time stamp update second"]
pub type TmsusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Time stamp update second"]
    #[inline(always)]
    pub fn tmsus(&self) -> TmsusR {
        TmsusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Time stamp update second"]
    #[inline(always)]
    pub fn tmsus(&mut self) -> TmsusW<'_, PtpTsuhSpec> {
        TmsusW::new(self, 0)
    }
}
#[doc = "Ethernet PTP time stamp high update register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptp_tsuh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptp_tsuh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtpTsuhSpec;
impl crate::RegisterSpec for PtpTsuhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptp_tsuh::R`](R) reader structure"]
impl crate::Readable for PtpTsuhSpec {}
#[doc = "`write(|w| ..)` method takes [`ptp_tsuh::W`](W) writer structure"]
impl crate::Writable for PtpTsuhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PTP_TSUH to value 0"]
impl crate::Resettable for PtpTsuhSpec {}
