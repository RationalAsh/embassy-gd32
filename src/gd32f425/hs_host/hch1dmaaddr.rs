#[doc = "Register `HCH1DMAADDR` reader"]
pub type R = crate::R<Hch1dmaaddrSpec>;
#[doc = "Register `HCH1DMAADDR` writer"]
pub type W = crate::W<Hch1dmaaddrSpec>;
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
    pub fn dmaaddr(&mut self) -> DmaaddrW<'_, Hch1dmaaddrSpec> {
        DmaaddrW::new(self, 0)
    }
}
#[doc = "host channel-1 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`hch1dmaaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hch1dmaaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hch1dmaaddrSpec;
impl crate::RegisterSpec for Hch1dmaaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hch1dmaaddr::R`](R) reader structure"]
impl crate::Readable for Hch1dmaaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`hch1dmaaddr::W`](W) writer structure"]
impl crate::Writable for Hch1dmaaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCH1DMAADDR to value 0"]
impl crate::Resettable for Hch1dmaaddrSpec {}
