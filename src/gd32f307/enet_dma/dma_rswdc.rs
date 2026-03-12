#[doc = "Register `DMA_RSWDC` reader"]
pub type R = crate::R<DmaRswdcSpec>;
#[doc = "Register `DMA_RSWDC` writer"]
pub type W = crate::W<DmaRswdcSpec>;
#[doc = "Field `WDCFRS` reader - Watchdog counter for receive status (RS)"]
pub type WdcfrsR = crate::FieldReader;
#[doc = "Field `WDCFRS` writer - Watchdog counter for receive status (RS)"]
pub type WdcfrsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Watchdog counter for receive status (RS)"]
    #[inline(always)]
    pub fn wdcfrs(&self) -> WdcfrsR {
        WdcfrsR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Watchdog counter for receive status (RS)"]
    #[inline(always)]
    pub fn wdcfrs(&mut self) -> WdcfrsW<'_, DmaRswdcSpec> {
        WdcfrsW::new(self, 0)
    }
}
#[doc = "Ethernet DMA receive state watchdog counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_rswdc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_rswdc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaRswdcSpec;
impl crate::RegisterSpec for DmaRswdcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_rswdc::R`](R) reader structure"]
impl crate::Readable for DmaRswdcSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_rswdc::W`](W) writer structure"]
impl crate::Writable for DmaRswdcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_RSWDC to value 0"]
impl crate::Resettable for DmaRswdcSpec {}
