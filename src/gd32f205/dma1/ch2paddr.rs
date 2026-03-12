#[doc = "Register `CH2PADDR` reader"]
pub type R = crate::R<Ch2paddrSpec>;
#[doc = "Register `CH2PADDR` writer"]
pub type W = crate::W<Ch2paddrSpec>;
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
    pub fn paddr(&mut self) -> PaddrW<'_, Ch2paddrSpec> {
        PaddrW::new(self, 0)
    }
}
#[doc = "Channel 2 peripheral base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2paddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2paddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2paddrSpec;
impl crate::RegisterSpec for Ch2paddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2paddr::R`](R) reader structure"]
impl crate::Readable for Ch2paddrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2paddr::W`](W) writer structure"]
impl crate::Writable for Ch2paddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH2PADDR to value 0"]
impl crate::Resettable for Ch2paddrSpec {}
