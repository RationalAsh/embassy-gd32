#[doc = "Register `PWRCTL` reader"]
pub type R = crate::R<PwrctlSpec>;
#[doc = "Register `PWRCTL` writer"]
pub type W = crate::W<PwrctlSpec>;
#[doc = "Field `PWRCTL` reader - SDIO power control bits"]
pub type PwrctlR = crate::FieldReader;
#[doc = "Field `PWRCTL` writer - SDIO power control bits"]
pub type PwrctlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - SDIO power control bits"]
    #[inline(always)]
    pub fn pwrctl(&self) -> PwrctlR {
        PwrctlR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SDIO power control bits"]
    #[inline(always)]
    pub fn pwrctl(&mut self) -> PwrctlW<'_, PwrctlSpec> {
        PwrctlW::new(self, 0)
    }
}
#[doc = "Power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrctlSpec;
impl crate::RegisterSpec for PwrctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrctl::R`](R) reader structure"]
impl crate::Readable for PwrctlSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrctl::W`](W) writer structure"]
impl crate::Writable for PwrctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRCTL to value 0"]
impl crate::Resettable for PwrctlSpec {}
