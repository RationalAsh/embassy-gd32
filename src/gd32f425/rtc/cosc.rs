#[doc = "Register `COSC` reader"]
pub type R = crate::R<CoscSpec>;
#[doc = "Register `COSC` writer"]
pub type W = crate::W<CoscSpec>;
#[doc = "Field `COSS` reader - Coarse Calibration step"]
pub type CossR = crate::FieldReader;
#[doc = "Field `COSS` writer - Coarse Calibration step"]
pub type CossW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `COSD` reader - Coarse Calibration direction"]
pub type CosdR = crate::BitReader;
#[doc = "Field `COSD` writer - Coarse Calibration direction"]
pub type CosdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Coarse Calibration step"]
    #[inline(always)]
    pub fn coss(&self) -> CossR {
        CossR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Coarse Calibration direction"]
    #[inline(always)]
    pub fn cosd(&self) -> CosdR {
        CosdR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Coarse Calibration step"]
    #[inline(always)]
    pub fn coss(&mut self) -> CossW<'_, CoscSpec> {
        CossW::new(self, 0)
    }
    #[doc = "Bit 7 - Coarse Calibration direction"]
    #[inline(always)]
    pub fn cosd(&mut self) -> CosdW<'_, CoscSpec> {
        CosdW::new(self, 7)
    }
}
#[doc = "Coarse calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cosc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cosc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoscSpec;
impl crate::RegisterSpec for CoscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cosc::R`](R) reader structure"]
impl crate::Readable for CoscSpec {}
#[doc = "`write(|w| ..)` method takes [`cosc::W`](W) writer structure"]
impl crate::Writable for CoscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COSC to value 0"]
impl crate::Resettable for CoscSpec {}
