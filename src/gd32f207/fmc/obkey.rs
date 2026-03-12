#[doc = "Register `OBKEY` writer"]
pub type W = crate::W<ObkeySpec>;
#[doc = "Field `OBKEY` writer - FMC_CTL0 option bytes operation unlock key"]
pub type ObkeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl W {
    #[doc = "Bits 0:31 - FMC_CTL0 option bytes operation unlock key"]
    #[inline(always)]
    pub fn obkey(&mut self) -> ObkeyW<'_, ObkeySpec> {
        ObkeyW::new(self, 0)
    }
}
#[doc = "Option byte unlock key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obkey::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ObkeySpec;
impl crate::RegisterSpec for ObkeySpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`obkey::W`](W) writer structure"]
impl crate::Writable for ObkeySpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OBKEY to value 0"]
impl crate::Resettable for ObkeySpec {}
