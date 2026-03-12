#[doc = "Register `ST3CNT` reader"]
pub type R = crate::R<St3cntSpec>;
#[doc = "Register `ST3CNT` writer"]
pub type W = crate::W<St3cntSpec>;
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
    pub fn cnt(&mut self) -> CntW<'_, St3cntSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "SHRTIMER Slave_TIMERx counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`st3cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st3cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St3cntSpec;
impl crate::RegisterSpec for St3cntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st3cnt::R`](R) reader structure"]
impl crate::Readable for St3cntSpec {}
#[doc = "`write(|w| ..)` method takes [`st3cnt::W`](W) writer structure"]
impl crate::Writable for St3cntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ST3CNT to value 0"]
impl crate::Resettable for St3cntSpec {}
