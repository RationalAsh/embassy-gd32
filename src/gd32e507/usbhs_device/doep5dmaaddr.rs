#[doc = "Register `DOEP5DMAADDR` reader"]
pub type R = crate::R<Doep5dmaaddrSpec>;
#[doc = "Register `DOEP5DMAADDR` writer"]
pub type W = crate::W<Doep5dmaaddrSpec>;
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
    pub fn dmaaddr(&mut self) -> DmaaddrW<'_, Doep5dmaaddrSpec> {
        DmaaddrW::new(self, 0)
    }
}
#[doc = "Device OUT endpoint 5 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep5dmaaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep5dmaaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doep5dmaaddrSpec;
impl crate::RegisterSpec for Doep5dmaaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doep5dmaaddr::R`](R) reader structure"]
impl crate::Readable for Doep5dmaaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`doep5dmaaddr::W`](W) writer structure"]
impl crate::Writable for Doep5dmaaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEP5DMAADDR to value 0"]
impl crate::Resettable for Doep5dmaaddrSpec {}
