#[doc = "Register `CH1CNT` reader"]
pub type R = crate::R<Ch1cntSpec>;
#[doc = "Register `CH1CNT` writer"]
pub type W = crate::W<Ch1cntSpec>;
#[doc = "Field `CNT` reader - Transfer counter"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - Transfer counter"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Transfer counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transfer counter"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, Ch1cntSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "Channel 1 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1cntSpec;
impl crate::RegisterSpec for Ch1cntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1cnt::R`](R) reader structure"]
impl crate::Readable for Ch1cntSpec {}
#[doc = "`write(|w| ..)` method takes [`ch1cnt::W`](W) writer structure"]
impl crate::Writable for Ch1cntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH1CNT to value 0"]
impl crate::Resettable for Ch1cntSpec {}
