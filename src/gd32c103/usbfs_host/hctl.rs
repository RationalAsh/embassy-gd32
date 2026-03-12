#[doc = "Register `HCTL` reader"]
pub type R = crate::R<HctlSpec>;
#[doc = "Register `HCTL` writer"]
pub type W = crate::W<HctlSpec>;
#[doc = "Field `CLKSEL` reader - clock select for USB clock"]
pub type ClkselR = crate::FieldReader;
#[doc = "Field `CLKSEL` writer - clock select for USB clock"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - clock select for USB clock"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - clock select for USB clock"]
    #[inline(always)]
    pub fn clksel(&mut self) -> ClkselW<'_, HctlSpec> {
        ClkselW::new(self, 0)
    }
}
#[doc = "host configuration register (HCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`hctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HctlSpec;
impl crate::RegisterSpec for HctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hctl::R`](R) reader structure"]
impl crate::Readable for HctlSpec {}
#[doc = "`write(|w| ..)` method takes [`hctl::W`](W) writer structure"]
impl crate::Writable for HctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCTL to value 0"]
impl crate::Resettable for HctlSpec {}
