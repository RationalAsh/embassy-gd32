#[doc = "Register `ST0CAP1V` reader"]
pub type R = crate::R<St0cap1vSpec>;
#[doc = "Register `ST0CAP1V` writer"]
pub type W = crate::W<St0cap1vSpec>;
#[doc = "Field `CAP1VAL` reader - Capture 1 value"]
pub type Cap1valR = crate::FieldReader<u16>;
#[doc = "Field `CAP1VAL` writer - Capture 1 value"]
pub type Cap1valW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture 1 value"]
    #[inline(always)]
    pub fn cap1val(&self) -> Cap1valR {
        Cap1valR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture 1 value"]
    #[inline(always)]
    pub fn cap1val(&mut self) -> Cap1valW<'_, St0cap1vSpec> {
        Cap1valW::new(self, 0)
    }
}
#[doc = "SHRTIMER Slave_TIMER0 capture 1 value register\n\nYou can [`read`](crate::Reg::read) this register and get [`st0cap1v::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st0cap1v::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St0cap1vSpec;
impl crate::RegisterSpec for St0cap1vSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st0cap1v::R`](R) reader structure"]
impl crate::Readable for St0cap1vSpec {}
#[doc = "`write(|w| ..)` method takes [`st0cap1v::W`](W) writer structure"]
impl crate::Writable for St0cap1vSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ST0CAP1V to value 0"]
impl crate::Resettable for St0cap1vSpec {}
