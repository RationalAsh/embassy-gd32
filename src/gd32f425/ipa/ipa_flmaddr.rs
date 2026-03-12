#[doc = "Register `IPA_FLMADDR` reader"]
pub type R = crate::R<IpaFlmaddrSpec>;
#[doc = "Register `IPA_FLMADDR` writer"]
pub type W = crate::W<IpaFlmaddrSpec>;
#[doc = "Field `FLMBADDR` reader - Foreground LUT memory base address"]
pub type FlmbaddrR = crate::FieldReader<u32>;
#[doc = "Field `FLMBADDR` writer - Foreground LUT memory base address"]
pub type FlmbaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Foreground LUT memory base address"]
    #[inline(always)]
    pub fn flmbaddr(&self) -> FlmbaddrR {
        FlmbaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Foreground LUT memory base address"]
    #[inline(always)]
    pub fn flmbaddr(&mut self) -> FlmbaddrW<'_, IpaFlmaddrSpec> {
        FlmbaddrW::new(self, 0)
    }
}
#[doc = "Foreground LUT memory base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipa_flmaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipa_flmaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpaFlmaddrSpec;
impl crate::RegisterSpec for IpaFlmaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipa_flmaddr::R`](R) reader structure"]
impl crate::Readable for IpaFlmaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`ipa_flmaddr::W`](W) writer structure"]
impl crate::Writable for IpaFlmaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPA_FLMADDR to value 0"]
impl crate::Resettable for IpaFlmaddrSpec {}
