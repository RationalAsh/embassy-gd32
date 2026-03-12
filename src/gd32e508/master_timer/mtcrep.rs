#[doc = "Register `MTCREP` reader"]
pub type R = crate::R<MtcrepSpec>;
#[doc = "Register `MTCREP` writer"]
pub type W = crate::W<MtcrepSpec>;
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
    pub fn crep(&mut self) -> CrepW<'_, MtcrepSpec> {
        CrepW::new(self, 0)
    }
}
#[doc = "SHRTIMER Master_TIMER counter repetition register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtcrep::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtcrep::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtcrepSpec;
impl crate::RegisterSpec for MtcrepSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtcrep::R`](R) reader structure"]
impl crate::Readable for MtcrepSpec {}
#[doc = "`write(|w| ..)` method takes [`mtcrep::W`](W) writer structure"]
impl crate::Writable for MtcrepSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTCREP to value 0"]
impl crate::Resettable for MtcrepSpec {}
