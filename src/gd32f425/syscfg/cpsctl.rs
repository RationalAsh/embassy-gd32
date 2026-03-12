#[doc = "Register `CPSCTL` reader"]
pub type R = crate::R<CpsctlSpec>;
#[doc = "Register `CPSCTL` writer"]
pub type W = crate::W<CpsctlSpec>;
#[doc = "Field `CPS_EN` reader - I/O compensation cell enable"]
pub type CpsEnR = crate::BitReader;
#[doc = "Field `CPS_EN` writer - I/O compensation cell enable"]
pub type CpsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPS_RDY` reader - I/O compensation cell is ready or not"]
pub type CpsRdyR = crate::BitReader;
#[doc = "Field `CPS_RDY` writer - I/O compensation cell is ready or not"]
pub type CpsRdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I/O compensation cell enable"]
    #[inline(always)]
    pub fn cps_en(&self) -> CpsEnR {
        CpsEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - I/O compensation cell is ready or not"]
    #[inline(always)]
    pub fn cps_rdy(&self) -> CpsRdyR {
        CpsRdyR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O compensation cell enable"]
    #[inline(always)]
    pub fn cps_en(&mut self) -> CpsEnW<'_, CpsctlSpec> {
        CpsEnW::new(self, 0)
    }
    #[doc = "Bit 8 - I/O compensation cell is ready or not"]
    #[inline(always)]
    pub fn cps_rdy(&mut self) -> CpsRdyW<'_, CpsctlSpec> {
        CpsRdyW::new(self, 8)
    }
}
#[doc = "I/O compensation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpsctlSpec;
impl crate::RegisterSpec for CpsctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsctl::R`](R) reader structure"]
impl crate::Readable for CpsctlSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsctl::W`](W) writer structure"]
impl crate::Writable for CpsctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPSCTL to value 0"]
impl crate::Resettable for CpsctlSpec {}
