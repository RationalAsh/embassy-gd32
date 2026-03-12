#[doc = "Register `ST4CREP` reader"]
pub type R = crate::R<St4crepSpec>;
#[doc = "Register `ST4CREP` writer"]
pub type W = crate::W<St4crepSpec>;
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
    pub fn crep(&mut self) -> CrepW<'_, St4crepSpec> {
        CrepW::new(self, 0)
    }
}
#[doc = "SHRTIMER Slave_TIMER4 counter repetition register\n\nYou can [`read`](crate::Reg::read) this register and get [`st4crep::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st4crep::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St4crepSpec;
impl crate::RegisterSpec for St4crepSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st4crep::R`](R) reader structure"]
impl crate::Readable for St4crepSpec {}
#[doc = "`write(|w| ..)` method takes [`st4crep::W`](W) writer structure"]
impl crate::Writable for St4crepSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ST4CREP to value 0"]
impl crate::Resettable for St4crepSpec {}
