#[doc = "Register `CH4PADDR` reader"]
pub type R = crate::R<Ch4paddrSpec>;
#[doc = "Register `CH4PADDR` writer"]
pub type W = crate::W<Ch4paddrSpec>;
#[doc = "Field `PADDR` reader - Peripheral base address"]
pub type PaddrR = crate::FieldReader<u32>;
#[doc = "Field `PADDR` writer - Peripheral base address"]
pub type PaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral base address"]
    #[inline(always)]
    pub fn paddr(&self) -> PaddrR {
        PaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral base address"]
    #[inline(always)]
    pub fn paddr(&mut self) -> PaddrW<'_, Ch4paddrSpec> {
        PaddrW::new(self, 0)
    }
}
#[doc = "DMA channel 4 peripheral base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4paddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4paddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch4paddrSpec;
impl crate::RegisterSpec for Ch4paddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4paddr::R`](R) reader structure"]
impl crate::Readable for Ch4paddrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch4paddr::W`](W) writer structure"]
impl crate::Writable for Ch4paddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH4PADDR to value 0"]
impl crate::Resettable for Ch4paddrSpec {}
