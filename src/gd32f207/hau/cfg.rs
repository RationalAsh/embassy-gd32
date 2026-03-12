#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `VBL` reader - Valid bits length in the last word"]
pub type VblR = crate::FieldReader;
#[doc = "Field `VBL` writer - Valid bits length in the last word"]
pub type VblW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CALEN` writer - Digest calculation enable"]
pub type CalenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Valid bits length in the last word"]
    #[inline(always)]
    pub fn vbl(&self) -> VblR {
        VblR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Valid bits length in the last word"]
    #[inline(always)]
    pub fn vbl(&mut self) -> VblW<'_, CfgSpec> {
        VblW::new(self, 0)
    }
    #[doc = "Bit 8 - Digest calculation enable"]
    #[inline(always)]
    pub fn calen(&mut self) -> CalenW<'_, CfgSpec> {
        CalenW::new(self, 8)
    }
}
#[doc = "HAU configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {}
