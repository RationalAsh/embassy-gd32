#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `OUTSEL` reader - The output value selection"]
pub type OutselR = crate::BitReader;
#[doc = "Field `OUTSEL` writer - The output value selection"]
pub type OutselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHVSEL` reader - Write CHxVAL register selection"]
pub type ChvselR = crate::BitReader;
#[doc = "Field `CHVSEL` writer - Write CHxVAL register selection"]
pub type ChvselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The output value selection"]
    #[inline(always)]
    pub fn outsel(&self) -> OutselR {
        OutselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write CHxVAL register selection"]
    #[inline(always)]
    pub fn chvsel(&self) -> ChvselR {
        ChvselR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The output value selection"]
    #[inline(always)]
    pub fn outsel(&mut self) -> OutselW<'_, CfgSpec> {
        OutselW::new(self, 0)
    }
    #[doc = "Bit 1 - Write CHxVAL register selection"]
    #[inline(always)]
    pub fn chvsel(&mut self) -> ChvselW<'_, CfgSpec> {
        ChvselW::new(self, 1)
    }
}
#[doc = "configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
