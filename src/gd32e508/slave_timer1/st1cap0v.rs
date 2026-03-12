#[doc = "Register `ST1CAP0V` reader"]
pub type R = crate::R<St1cap0vSpec>;
#[doc = "Register `ST1CAP0V` writer"]
pub type W = crate::W<St1cap0vSpec>;
#[doc = "Field `CAP0VAL` reader - Capture 0 value"]
pub type Cap0valR = crate::FieldReader<u16>;
#[doc = "Field `CAP0VAL` writer - Capture 0 value"]
pub type Cap0valW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture 0 value"]
    #[inline(always)]
    pub fn cap0val(&self) -> Cap0valR {
        Cap0valR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture 0 value"]
    #[inline(always)]
    pub fn cap0val(&mut self) -> Cap0valW<'_, St1cap0vSpec> {
        Cap0valW::new(self, 0)
    }
}
#[doc = "SHRTIMER Slave_TIMER1 capture 0 value register\n\nYou can [`read`](crate::Reg::read) this register and get [`st1cap0v::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st1cap0v::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St1cap0vSpec;
impl crate::RegisterSpec for St1cap0vSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st1cap0v::R`](R) reader structure"]
impl crate::Readable for St1cap0vSpec {}
#[doc = "`write(|w| ..)` method takes [`st1cap0v::W`](W) writer structure"]
impl crate::Writable for St1cap0vSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ST1CAP0V to value 0"]
impl crate::Resettable for St1cap0vSpec {}
