#[doc = "Register `MTCNT` reader"]
pub type R = crate::R<MtcntSpec>;
#[doc = "Register `MTCNT` writer"]
pub type W = crate::W<MtcntSpec>;
#[doc = "Field `CNT` reader - The current counter value"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - The current counter value"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The current counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The current counter value"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, MtcntSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "SHRTIMER Master_TIMER counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtcntSpec;
impl crate::RegisterSpec for MtcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtcnt::R`](R) reader structure"]
impl crate::Readable for MtcntSpec {}
#[doc = "`write(|w| ..)` method takes [`mtcnt::W`](W) writer structure"]
impl crate::Writable for MtcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTCNT to value 0"]
impl crate::Resettable for MtcntSpec {}
