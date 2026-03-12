#[doc = "Register `ST3CREP` reader"]
pub type R = crate::R<St3crepSpec>;
#[doc = "Register `ST3CREP` writer"]
pub type W = crate::W<St3crepSpec>;
#[doc = "Field `CREP` reader - Counter repetition value"]
pub type CrepR = crate::FieldReader;
#[doc = "Field `CREP` writer - Counter repetition value"]
pub type CrepW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Counter repetition value"]
    #[inline(always)]
    pub fn crep(&self) -> CrepR {
        CrepR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter repetition value"]
    #[inline(always)]
    pub fn crep(&mut self) -> CrepW<'_, St3crepSpec> {
        CrepW::new(self, 0)
    }
}
#[doc = "SHRTIMER Slave_TIMER3 counter repetition register\n\nYou can [`read`](crate::Reg::read) this register and get [`st3crep::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st3crep::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St3crepSpec;
impl crate::RegisterSpec for St3crepSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st3crep::R`](R) reader structure"]
impl crate::Readable for St3crepSpec {}
#[doc = "`write(|w| ..)` method takes [`st3crep::W`](W) writer structure"]
impl crate::Writable for St3crepSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ST3CREP to value 0"]
impl crate::Resettable for St3crepSpec {}
