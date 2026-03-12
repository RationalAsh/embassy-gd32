#[doc = "Register `ST2CNT` reader"]
pub type R = crate::R<St2cntSpec>;
#[doc = "Register `ST2CNT` writer"]
pub type W = crate::W<St2cntSpec>;
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
    pub fn cnt(&mut self) -> CntW<'_, St2cntSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "SHRTIMER Slave_TIMERx counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`st2cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st2cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St2cntSpec;
impl crate::RegisterSpec for St2cntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st2cnt::R`](R) reader structure"]
impl crate::Readable for St2cntSpec {}
#[doc = "`write(|w| ..)` method takes [`st2cnt::W`](W) writer structure"]
impl crate::Writable for St2cntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ST2CNT to value 0"]
impl crate::Resettable for St2cntSpec {}
