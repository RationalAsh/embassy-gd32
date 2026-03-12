#[doc = "Register `PWRCLKCTL` reader"]
pub type R = crate::R<PwrclkctlSpec>;
#[doc = "Register `PWRCLKCTL` writer"]
pub type W = crate::W<PwrclkctlSpec>;
#[doc = "Field `SUCLK` reader - Stop the USB clock"]
pub type SuclkR = crate::BitReader;
#[doc = "Field `SUCLK` writer - Stop the USB clock"]
pub type SuclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHCLK` reader - Stop HCLK"]
pub type ShclkR = crate::BitReader;
#[doc = "Field `SHCLK` writer - Stop HCLK"]
pub type ShclkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stop the USB clock"]
    #[inline(always)]
    pub fn suclk(&self) -> SuclkR {
        SuclkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stop HCLK"]
    #[inline(always)]
    pub fn shclk(&self) -> ShclkR {
        ShclkR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop the USB clock"]
    #[inline(always)]
    pub fn suclk(&mut self) -> SuclkW<'_, PwrclkctlSpec> {
        SuclkW::new(self, 0)
    }
    #[doc = "Bit 1 - Stop HCLK"]
    #[inline(always)]
    pub fn shclk(&mut self) -> ShclkW<'_, PwrclkctlSpec> {
        ShclkW::new(self, 1)
    }
}
#[doc = "power and clock gating control register (PWRCLKCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrclkctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrclkctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrclkctlSpec;
impl crate::RegisterSpec for PwrclkctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrclkctl::R`](R) reader structure"]
impl crate::Readable for PwrclkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrclkctl::W`](W) writer structure"]
impl crate::Writable for PwrclkctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRCLKCTL to value 0"]
impl crate::Resettable for PwrclkctlSpec {}
