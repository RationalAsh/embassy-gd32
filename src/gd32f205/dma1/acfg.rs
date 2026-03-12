#[doc = "Register `ACFG` reader"]
pub type R = crate::R<AcfgSpec>;
#[doc = "Register `ACFG` writer"]
pub type W = crate::W<AcfgSpec>;
#[doc = "Field `FD_CH5EN` reader - Enable bit for channel 5 Full_Data transfer mode"]
pub type FdCh5enR = crate::BitReader;
#[doc = "Field `FD_CH5EN` writer - Enable bit for channel 5 Full_Data transfer mode"]
pub type FdCh5enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Enable bit for channel 5 Full_Data transfer mode"]
    #[inline(always)]
    pub fn fd_ch5en(&self) -> FdCh5enR {
        FdCh5enR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Enable bit for channel 5 Full_Data transfer mode"]
    #[inline(always)]
    pub fn fd_ch5en(&mut self) -> FdCh5enW<'_, AcfgSpec> {
        FdCh5enW::new(self, 5)
    }
}
#[doc = "DMA additional configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`acfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcfgSpec;
impl crate::RegisterSpec for AcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acfg::R`](R) reader structure"]
impl crate::Readable for AcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`acfg::W`](W) writer structure"]
impl crate::Writable for AcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACFG to value 0"]
impl crate::Resettable for AcfgSpec {}
