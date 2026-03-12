#[doc = "Register `DIEP0DMAADDR` reader"]
pub type R = crate::R<Diep0dmaaddrSpec>;
#[doc = "Register `DIEP0DMAADDR` writer"]
pub type W = crate::W<Diep0dmaaddrSpec>;
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
    pub fn dmaaddr(&mut self) -> DmaaddrW<'_, Diep0dmaaddrSpec> {
        DmaaddrW::new(self, 0)
    }
}
#[doc = "Device IN endpoint 0 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0dmaaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0dmaaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diep0dmaaddrSpec;
impl crate::RegisterSpec for Diep0dmaaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep0dmaaddr::R`](R) reader structure"]
impl crate::Readable for Diep0dmaaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`diep0dmaaddr::W`](W) writer structure"]
impl crate::Writable for Diep0dmaaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEP0DMAADDR to value 0"]
impl crate::Resettable for Diep0dmaaddrSpec {}
