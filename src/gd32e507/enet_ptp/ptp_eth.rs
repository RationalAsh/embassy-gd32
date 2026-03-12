#[doc = "Register `PTP_ETH` reader"]
pub type R = crate::R<PtpEthSpec>;
#[doc = "Register `PTP_ETH` writer"]
pub type W = crate::W<PtpEthSpec>;
#[doc = "Field `ETSH` reader - Expected time stamp high"]
pub type EtshR = crate::FieldReader<u32>;
#[doc = "Field `ETSH` writer - Expected time stamp high"]
pub type EtshW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Expected time stamp high"]
    #[inline(always)]
    pub fn etsh(&self) -> EtshR {
        EtshR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Expected time stamp high"]
    #[inline(always)]
    pub fn etsh(&mut self) -> EtshW<'_, PtpEthSpec> {
        EtshW::new(self, 0)
    }
}
#[doc = "Ethernet PTP expected time high register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptp_eth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptp_eth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtpEthSpec;
impl crate::RegisterSpec for PtpEthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptp_eth::R`](R) reader structure"]
impl crate::Readable for PtpEthSpec {}
#[doc = "`write(|w| ..)` method takes [`ptp_eth::W`](W) writer structure"]
impl crate::Writable for PtpEthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PTP_ETH to value 0"]
impl crate::Resettable for PtpEthSpec {}
