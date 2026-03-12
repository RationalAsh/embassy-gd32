#[doc = "Register `CH4MADDR` reader"]
pub type R = crate::R<Ch4maddrSpec>;
#[doc = "Register `CH4MADDR` writer"]
pub type W = crate::W<Ch4maddrSpec>;
#[doc = "Field `MADDR` reader - Memory base address"]
pub type MaddrR = crate::FieldReader<u32>;
#[doc = "Field `MADDR` writer - Memory base address"]
pub type MaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory base address"]
    #[inline(always)]
    pub fn maddr(&self) -> MaddrR {
        MaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory base address"]
    #[inline(always)]
    pub fn maddr(&mut self) -> MaddrW<'_, Ch4maddrSpec> {
        MaddrW::new(self, 0)
    }
}
#[doc = "DMA channel 4 memory base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4maddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4maddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch4maddrSpec;
impl crate::RegisterSpec for Ch4maddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4maddr::R`](R) reader structure"]
impl crate::Readable for Ch4maddrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch4maddr::W`](W) writer structure"]
impl crate::Writable for Ch4maddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH4MADDR to value 0"]
impl crate::Resettable for Ch4maddrSpec {}
