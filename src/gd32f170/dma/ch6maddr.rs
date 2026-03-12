#[doc = "Register `CH6MADDR` reader"]
pub type R = crate::R<Ch6maddrSpec>;
#[doc = "Register `CH6MADDR` writer"]
pub type W = crate::W<Ch6maddrSpec>;
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
    pub fn maddr(&mut self) -> MaddrW<'_, Ch6maddrSpec> {
        MaddrW::new(self, 0)
    }
}
#[doc = "DMA channel 6 memory base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6maddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6maddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch6maddrSpec;
impl crate::RegisterSpec for Ch6maddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch6maddr::R`](R) reader structure"]
impl crate::Readable for Ch6maddrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch6maddr::W`](W) writer structure"]
impl crate::Writable for Ch6maddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH6MADDR to value 0"]
impl crate::Resettable for Ch6maddrSpec {}
