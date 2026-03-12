#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `TMUEN` reader - start TMU module calculation"]
pub type TmuenR = crate::BitReader;
#[doc = "Field `TMUEN` writer - start TMU module calculation"]
pub type TmuenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - Set the mode of TMU"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - Set the mode of TMU"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFIE` reader - CFIE"]
pub type CfieR = crate::BitReader;
#[doc = "Field `CFIE` writer - CFIE"]
pub type CfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFIF` reader - CFIF"]
pub type CfifR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - start TMU module calculation"]
    #[inline(always)]
    pub fn tmuen(&self) -> TmuenR {
        TmuenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Set the mode of TMU"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - CFIE"]
    #[inline(always)]
    pub fn cfie(&self) -> CfieR {
        CfieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CFIF"]
    #[inline(always)]
    pub fn cfif(&self) -> CfifR {
        CfifR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - start TMU module calculation"]
    #[inline(always)]
    pub fn tmuen(&mut self) -> TmuenW<'_, CtlSpec> {
        TmuenW::new(self, 0)
    }
    #[doc = "Bits 1:4 - Set the mode of TMU"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CtlSpec> {
        ModeW::new(self, 1)
    }
    #[doc = "Bit 5 - CFIE"]
    #[inline(always)]
    pub fn cfie(&mut self) -> CfieW<'_, CtlSpec> {
        CfieW::new(self, 5)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {}
