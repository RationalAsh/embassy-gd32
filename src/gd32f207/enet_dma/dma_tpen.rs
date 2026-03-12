#[doc = "Register `DMA_TPEN` reader"]
pub type R = crate::R<DmaTpenSpec>;
#[doc = "Register `DMA_TPEN` writer"]
pub type W = crate::W<DmaTpenSpec>;
#[doc = "Field `TPE` reader - Transmit poll enable"]
pub type TpeR = crate::FieldReader<u32>;
#[doc = "Field `TPE` writer - Transmit poll enable"]
pub type TpeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit poll enable"]
    #[inline(always)]
    pub fn tpe(&self) -> TpeR {
        TpeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit poll enable"]
    #[inline(always)]
    pub fn tpe(&mut self) -> TpeW<'_, DmaTpenSpec> {
        TpeW::new(self, 0)
    }
}
#[doc = "Ethernet DMA transmit poll enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_tpen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_tpen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaTpenSpec;
impl crate::RegisterSpec for DmaTpenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_tpen::R`](R) reader structure"]
impl crate::Readable for DmaTpenSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_tpen::W`](W) writer structure"]
impl crate::Writable for DmaTpenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_TPEN to value 0"]
impl crate::Resettable for DmaTpenSpec {}
