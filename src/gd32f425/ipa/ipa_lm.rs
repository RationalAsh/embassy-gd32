#[doc = "Register `IPA_LM` reader"]
pub type R = crate::R<IpaLmSpec>;
#[doc = "Register `IPA_LM` writer"]
pub type W = crate::W<IpaLmSpec>;
#[doc = "Field `LM` reader - line mark"]
pub type LmR = crate::FieldReader<u16>;
#[doc = "Field `LM` writer - line mark"]
pub type LmW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - line mark"]
    #[inline(always)]
    pub fn lm(&self) -> LmR {
        LmR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - line mark"]
    #[inline(always)]
    pub fn lm(&mut self) -> LmW<'_, IpaLmSpec> {
        LmW::new(self, 0)
    }
}
#[doc = "Line mark register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipa_lm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipa_lm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpaLmSpec;
impl crate::RegisterSpec for IpaLmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipa_lm::R`](R) reader structure"]
impl crate::Readable for IpaLmSpec {}
#[doc = "`write(|w| ..)` method takes [`ipa_lm::W`](W) writer structure"]
impl crate::Writable for IpaLmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPA_LM to value 0"]
impl crate::Resettable for IpaLmSpec {}
