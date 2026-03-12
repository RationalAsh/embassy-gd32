#[doc = "Register `CH6CNT` reader"]
pub type R = crate::R<Ch6cntSpec>;
#[doc = "Register `CH6CNT` writer"]
pub type W = crate::W<Ch6cntSpec>;
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
    pub fn cnt(&mut self) -> CntW<'_, Ch6cntSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "DMA channel 6 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch6cntSpec;
impl crate::RegisterSpec for Ch6cntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch6cnt::R`](R) reader structure"]
impl crate::Readable for Ch6cntSpec {}
#[doc = "`write(|w| ..)` method takes [`ch6cnt::W`](W) writer structure"]
impl crate::Writable for Ch6cntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH6CNT to value 0"]
impl crate::Resettable for Ch6cntSpec {}
