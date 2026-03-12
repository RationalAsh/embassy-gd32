#[doc = "Register `HACHINTEN` reader"]
pub type R = crate::R<HachintenSpec>;
#[doc = "Register `HACHINTEN` writer"]
pub type W = crate::W<HachintenSpec>;
#[doc = "Field `CINTEN` reader - Channel interrupt enable"]
pub type CintenR = crate::FieldReader;
#[doc = "Field `CINTEN` writer - Channel interrupt enable"]
pub type CintenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Channel interrupt enable"]
    #[inline(always)]
    pub fn cinten(&self) -> CintenR {
        CintenR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel interrupt enable"]
    #[inline(always)]
    pub fn cinten(&mut self) -> CintenW<'_, HachintenSpec> {
        CintenW::new(self, 0)
    }
}
#[doc = "host all channels interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`hachinten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hachinten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HachintenSpec;
impl crate::RegisterSpec for HachintenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hachinten::R`](R) reader structure"]
impl crate::Readable for HachintenSpec {}
#[doc = "`write(|w| ..)` method takes [`hachinten::W`](W) writer structure"]
impl crate::Writable for HachintenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HACHINTEN to value 0"]
impl crate::Resettable for HachintenSpec {}
