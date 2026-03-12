#[doc = "Register `IPA_DMADDR` reader"]
pub type R = crate::R<IpaDmaddrSpec>;
#[doc = "Register `IPA_DMADDR` writer"]
pub type W = crate::W<IpaDmaddrSpec>;
#[doc = "Field `DMADDR` reader - Destination memory base address"]
pub type DmaddrR = crate::FieldReader<u32>;
#[doc = "Field `DMADDR` writer - Destination memory base address"]
pub type DmaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination memory base address"]
    #[inline(always)]
    pub fn dmaddr(&self) -> DmaddrR {
        DmaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination memory base address"]
    #[inline(always)]
    pub fn dmaddr(&mut self) -> DmaddrW<'_, IpaDmaddrSpec> {
        DmaddrW::new(self, 0)
    }
}
#[doc = "Destination memory base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipa_dmaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipa_dmaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpaDmaddrSpec;
impl crate::RegisterSpec for IpaDmaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipa_dmaddr::R`](R) reader structure"]
impl crate::Readable for IpaDmaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`ipa_dmaddr::W`](W) writer structure"]
impl crate::Writable for IpaDmaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPA_DMADDR to value 0"]
impl crate::Resettable for IpaDmaddrSpec {}
