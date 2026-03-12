#[doc = "Register `WPK` writer"]
pub type W = crate::W<WpkSpec>;
#[doc = "Field `WPK` writer - Write protection key"]
pub type WpkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Write protection key"]
    #[inline(always)]
    pub fn wpk(&mut self) -> WpkW<'_, WpkSpec> {
        WpkW::new(self, 0)
    }
}
#[doc = "Write protection key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpk::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpkSpec;
impl crate::RegisterSpec for WpkSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wpk::W`](W) writer structure"]
impl crate::Writable for WpkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WPK to value 0"]
impl crate::Resettable for WpkSpec {}
