#[doc = "Register `IPA_ITCTL` reader"]
pub type R = crate::R<IpaItctlSpec>;
#[doc = "Register `IPA_ITCTL` writer"]
pub type W = crate::W<IpaItctlSpec>;
#[doc = "Field `ITEN` reader - Inter-timer enable"]
pub type ItenR = crate::BitReader;
#[doc = "Field `ITEN` writer - Inter-timer enable"]
pub type ItenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NCCI` reader - Number of clock cycles interval"]
pub type NcciR = crate::FieldReader;
#[doc = "Field `NCCI` writer - Number of clock cycles interval"]
pub type NcciW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Inter-timer enable"]
    #[inline(always)]
    pub fn iten(&self) -> ItenR {
        ItenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - Number of clock cycles interval"]
    #[inline(always)]
    pub fn ncci(&self) -> NcciR {
        NcciR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Inter-timer enable"]
    #[inline(always)]
    pub fn iten(&mut self) -> ItenW<'_, IpaItctlSpec> {
        ItenW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Number of clock cycles interval"]
    #[inline(always)]
    pub fn ncci(&mut self) -> NcciW<'_, IpaItctlSpec> {
        NcciW::new(self, 8)
    }
}
#[doc = "Inter-timer control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipa_itctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipa_itctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpaItctlSpec;
impl crate::RegisterSpec for IpaItctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipa_itctl::R`](R) reader structure"]
impl crate::Readable for IpaItctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ipa_itctl::W`](W) writer structure"]
impl crate::Writable for IpaItctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPA_ITCTL to value 0"]
impl crate::Resettable for IpaItctlSpec {}
