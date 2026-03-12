#[doc = "Register `SWT` writer"]
pub type W = crate::W<SwtSpec>;
#[doc = "Field `SWTR0` writer - DAC0 software trigger"]
pub type Swtr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTR1` writer - DAC1 software trigger"]
pub type Swtr1W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DAC0 software trigger"]
    #[inline(always)]
    pub fn swtr0(&mut self) -> Swtr0W<'_, SwtSpec> {
        Swtr0W::new(self, 0)
    }
    #[doc = "Bit 1 - DAC1 software trigger"]
    #[inline(always)]
    pub fn swtr1(&mut self) -> Swtr1W<'_, SwtSpec> {
        Swtr1W::new(self, 1)
    }
}
#[doc = "software trigger register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwtSpec;
impl crate::RegisterSpec for SwtSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swt::W`](W) writer structure"]
impl crate::Writable for SwtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWT to value 0"]
impl crate::Resettable for SwtSpec {}
