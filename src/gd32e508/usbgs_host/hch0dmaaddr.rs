#[doc = "Register `HCH0DMAADDR` reader"]
pub type R = crate::R<Hch0dmaaddrSpec>;
#[doc = "Register `HCH0DMAADDR` writer"]
pub type W = crate::W<Hch0dmaaddrSpec>;
#[doc = "Field `DMAADDR` reader - DMA address"]
pub type DmaaddrR = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR` writer - DMA address"]
pub type DmaaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DmaaddrR {
        DmaaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA address"]
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DmaaddrW<'_, Hch0dmaaddrSpec> {
        DmaaddrW::new(self, 0)
    }
}
#[doc = "Host channel 0 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`hch0dmaaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hch0dmaaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hch0dmaaddrSpec;
impl crate::RegisterSpec for Hch0dmaaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hch0dmaaddr::R`](R) reader structure"]
impl crate::Readable for Hch0dmaaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`hch0dmaaddr::W`](W) writer structure"]
impl crate::Writable for Hch0dmaaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCH0DMAADDR to value 0"]
impl crate::Resettable for Hch0dmaaddrSpec {}
