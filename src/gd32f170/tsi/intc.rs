#[doc = "Register `INTC` reader"]
pub type R = crate::R<IntcSpec>;
#[doc = "Register `INTC` writer"]
pub type W = crate::W<IntcSpec>;
#[doc = "Field `CCTCF` reader - Clear charge-transfer complete flag"]
pub type CctcfR = crate::BitReader;
#[doc = "Field `CCTCF` writer - Clear charge-transfer complete flag"]
pub type CctcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMNERR` reader - Clear max cycle number error"]
pub type CmnerrR = crate::BitReader;
#[doc = "Field `CMNERR` writer - Clear max cycle number error"]
pub type CmnerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear charge-transfer complete flag"]
    #[inline(always)]
    pub fn cctcf(&self) -> CctcfR {
        CctcfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear max cycle number error"]
    #[inline(always)]
    pub fn cmnerr(&self) -> CmnerrR {
        CmnerrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear charge-transfer complete flag"]
    #[inline(always)]
    pub fn cctcf(&mut self) -> CctcfW<'_, IntcSpec> {
        CctcfW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear max cycle number error"]
    #[inline(always)]
    pub fn cmnerr(&mut self) -> CmnerrW<'_, IntcSpec> {
        CmnerrW::new(self, 1)
    }
}
#[doc = "interrupt flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`intc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcSpec;
impl crate::RegisterSpec for IntcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intc::R`](R) reader structure"]
impl crate::Readable for IntcSpec {}
#[doc = "`write(|w| ..)` method takes [`intc::W`](W) writer structure"]
impl crate::Writable for IntcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTC to value 0"]
impl crate::Resettable for IntcSpec {}
