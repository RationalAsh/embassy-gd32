#[doc = "Register `PSCH` writer"]
pub type W = crate::W<PschSpec>;
#[doc = "Field `PSC` writer - RTC prescaler value high"]
pub type PscW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl W {
    #[doc = "Bits 0:3 - RTC prescaler value high"]
    #[inline(always)]
    pub fn psc(&mut self) -> PscW<'_, PschSpec> {
        PscW::new(self, 0)
    }
}
#[doc = "RTC prescaler high register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psch::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PschSpec;
impl crate::RegisterSpec for PschSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`psch::W`](W) writer structure"]
impl crate::Writable for PschSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSCH to value 0"]
impl crate::Resettable for PschSpec {}
