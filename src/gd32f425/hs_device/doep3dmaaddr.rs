#[doc = "Register `DOEP3DMAADDR` reader"]
pub type R = crate::R<Doep3dmaaddrSpec>;
#[doc = "Register `DOEP3DMAADDR` writer"]
pub type W = crate::W<Doep3dmaaddrSpec>;
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
    pub fn dmaaddr(&mut self) -> DmaaddrW<'_, Doep3dmaaddrSpec> {
        DmaaddrW::new(self, 0)
    }
}
#[doc = "device OUT endpoint 3 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep3dmaaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep3dmaaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doep3dmaaddrSpec;
impl crate::RegisterSpec for Doep3dmaaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doep3dmaaddr::R`](R) reader structure"]
impl crate::Readable for Doep3dmaaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`doep3dmaaddr::W`](W) writer structure"]
impl crate::Writable for Doep3dmaaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEP3DMAADDR to value 0"]
impl crate::Resettable for Doep3dmaaddrSpec {}
