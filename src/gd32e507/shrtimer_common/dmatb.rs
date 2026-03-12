#[doc = "Register `DMATB` writer"]
pub type W = crate::W<DmatbSpec>;
#[doc = "Field `DMATB` writer - DMA transfer buffer"]
pub type DmatbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - DMA transfer buffer"]
    #[inline(always)]
    pub fn dmatb(&mut self) -> DmatbW<'_, DmatbSpec> {
        DmatbW::new(self, 0)
    }
}
#[doc = "SHRTIMER DMA transfer buffer register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatb::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmatbSpec;
impl crate::RegisterSpec for DmatbSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dmatb::W`](W) writer structure"]
impl crate::Writable for DmatbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMATB to value 0"]
impl crate::Resettable for DmatbSpec {}
