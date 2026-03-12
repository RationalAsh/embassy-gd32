#[doc = "Register `PTP_TSADDEND` reader"]
pub type R = crate::R<PtpTsaddendSpec>;
#[doc = "Register `PTP_TSADDEND` writer"]
pub type W = crate::W<PtpTsaddendSpec>;
#[doc = "Field `TMSA` reader - Time stamp addend"]
pub type TmsaR = crate::FieldReader<u32>;
#[doc = "Field `TMSA` writer - Time stamp addend"]
pub type TmsaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Time stamp addend"]
    #[inline(always)]
    pub fn tmsa(&self) -> TmsaR {
        TmsaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Time stamp addend"]
    #[inline(always)]
    pub fn tmsa(&mut self) -> TmsaW<'_, PtpTsaddendSpec> {
        TmsaW::new(self, 0)
    }
}
#[doc = "Ethernet PTP time stamp addend register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptp_tsaddend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptp_tsaddend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtpTsaddendSpec;
impl crate::RegisterSpec for PtpTsaddendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptp_tsaddend::R`](R) reader structure"]
impl crate::Readable for PtpTsaddendSpec {}
#[doc = "`write(|w| ..)` method takes [`ptp_tsaddend::W`](W) writer structure"]
impl crate::Writable for PtpTsaddendSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PTP_TSADDEND to value 0"]
impl crate::Resettable for PtpTsaddendSpec {}
