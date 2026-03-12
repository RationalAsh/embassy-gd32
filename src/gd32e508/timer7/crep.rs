#[doc = "Register `CREP` reader"]
pub type R = crate::R<CrepSpec>;
#[doc = "Register `CREP` writer"]
pub type W = crate::W<CrepSpec>;
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
    pub fn crep(&mut self) -> CrepW<'_, CrepSpec> {
        CrepW::new(self, 0)
    }
}
#[doc = "Counter repetition register\n\nYou can [`read`](crate::Reg::read) this register and get [`crep::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crep::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrepSpec;
impl crate::RegisterSpec for CrepSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crep::R`](R) reader structure"]
impl crate::Readable for CrepSpec {}
#[doc = "`write(|w| ..)` method takes [`crep::W`](W) writer structure"]
impl crate::Writable for CrepSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CREP to value 0"]
impl crate::Resettable for CrepSpec {}
