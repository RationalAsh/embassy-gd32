#[doc = "Register `DMA_RDTADDR` reader"]
pub type R = crate::R<DmaRdtaddrSpec>;
#[doc = "Register `DMA_RDTADDR` writer"]
pub type W = crate::W<DmaRdtaddrSpec>;
#[doc = "Field `SRT` reader - Start address of receive table"]
pub type SrtR = crate::FieldReader<u32>;
#[doc = "Field `SRT` writer - Start address of receive table"]
pub type SrtW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start address of receive table"]
    #[inline(always)]
    pub fn srt(&self) -> SrtR {
        SrtR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address of receive table"]
    #[inline(always)]
    pub fn srt(&mut self) -> SrtW<'_, DmaRdtaddrSpec> {
        SrtW::new(self, 0)
    }
}
#[doc = "Ethernet DMA receive descriptor table address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_rdtaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_rdtaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaRdtaddrSpec;
impl crate::RegisterSpec for DmaRdtaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_rdtaddr::R`](R) reader structure"]
impl crate::Readable for DmaRdtaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_rdtaddr::W`](W) writer structure"]
impl crate::Writable for DmaRdtaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_RDTADDR to value 0"]
impl crate::Resettable for DmaRdtaddrSpec {}
