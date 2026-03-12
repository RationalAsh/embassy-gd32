#[doc = "Register `CH1MADDR` reader"]
pub type R = crate::R<Ch1maddrSpec>;
#[doc = "Register `CH1MADDR` writer"]
pub type W = crate::W<Ch1maddrSpec>;
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
    pub fn maddr(&mut self) -> MaddrW<'_, Ch1maddrSpec> {
        MaddrW::new(self, 0)
    }
}
#[doc = "Channel 1 memory base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1maddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1maddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1maddrSpec;
impl crate::RegisterSpec for Ch1maddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1maddr::R`](R) reader structure"]
impl crate::Readable for Ch1maddrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch1maddr::W`](W) writer structure"]
impl crate::Writable for Ch1maddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH1MADDR to value 0"]
impl crate::Resettable for Ch1maddrSpec {}
