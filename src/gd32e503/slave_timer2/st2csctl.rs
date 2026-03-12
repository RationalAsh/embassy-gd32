#[doc = "Register `ST2CSCTL` reader"]
pub type R = crate::R<St2csctlSpec>;
#[doc = "Register `ST2CSCTL` writer"]
pub type W = crate::W<St2csctlSpec>;
#[doc = "Field `CSPRD` reader - Carrier signal period"]
pub type CsprdR = crate::FieldReader;
#[doc = "Field `CSPRD` writer - Carrier signal period"]
pub type CsprdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CSDTY` reader - Carrier signal duty cycle"]
pub type CsdtyR = crate::FieldReader;
#[doc = "Field `CSDTY` writer - Carrier signal duty cycle"]
pub type CsdtyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CSFSTPW` reader - First carrier-signal pulse width"]
pub type CsfstpwR = crate::FieldReader;
#[doc = "Field `CSFSTPW` writer - First carrier-signal pulse width"]
pub type CsfstpwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Carrier signal period"]
    #[inline(always)]
    pub fn csprd(&self) -> CsprdR {
        CsprdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Carrier signal duty cycle"]
    #[inline(always)]
    pub fn csdty(&self) -> CsdtyR {
        CsdtyR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:10 - First carrier-signal pulse width"]
    #[inline(always)]
    pub fn csfstpw(&self) -> CsfstpwR {
        CsfstpwR::new(((self.bits >> 7) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Carrier signal period"]
    #[inline(always)]
    pub fn csprd(&mut self) -> CsprdW<'_, St2csctlSpec> {
        CsprdW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Carrier signal duty cycle"]
    #[inline(always)]
    pub fn csdty(&mut self) -> CsdtyW<'_, St2csctlSpec> {
        CsdtyW::new(self, 4)
    }
    #[doc = "Bits 7:10 - First carrier-signal pulse width"]
    #[inline(always)]
    pub fn csfstpw(&mut self) -> CsfstpwW<'_, St2csctlSpec> {
        CsfstpwW::new(self, 7)
    }
}
#[doc = "SHRTIMER Slave_TIMERx carrier-signal control register\n\nYou can [`read`](crate::Reg::read) this register and get [`st2csctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st2csctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St2csctlSpec;
impl crate::RegisterSpec for St2csctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st2csctl::R`](R) reader structure"]
impl crate::Readable for St2csctlSpec {}
#[doc = "`write(|w| ..)` method takes [`st2csctl::W`](W) writer structure"]
impl crate::Writable for St2csctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ST2CSCTL to value 0"]
impl crate::Resettable for St2csctlSpec {}
