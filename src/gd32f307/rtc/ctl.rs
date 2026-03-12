#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `SCIF` reader - Sencond interrupt flag"]
pub type ScifR = crate::BitReader;
#[doc = "Field `SCIF` writer - Sencond interrupt flag"]
pub type ScifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRMIF` reader - Alarm interrupt flag"]
pub type AlrmifR = crate::BitReader;
#[doc = "Field `ALRMIF` writer - Alarm interrupt flag"]
pub type AlrmifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVIF` reader - Overflow interrupt flag"]
pub type OvifR = crate::BitReader;
#[doc = "Field `OVIF` writer - Overflow interrupt flag"]
pub type OvifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSYNF` reader - Registers synchronized flag"]
pub type RsynfR = crate::BitReader;
#[doc = "Field `RSYNF` writer - Registers synchronized flag"]
pub type RsynfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMF` reader - Configuration mode flag"]
pub type CmfR = crate::BitReader;
#[doc = "Field `CMF` writer - Configuration mode flag"]
pub type CmfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LWOFF` reader - Last write operation finished flag"]
pub type LwoffR = crate::BitReader;
#[doc = "Field `LWOFF` writer - Last write operation finished flag"]
pub type LwoffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sencond interrupt flag"]
    #[inline(always)]
    pub fn scif(&self) -> ScifR {
        ScifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm interrupt flag"]
    #[inline(always)]
    pub fn alrmif(&self) -> AlrmifR {
        AlrmifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow interrupt flag"]
    #[inline(always)]
    pub fn ovif(&self) -> OvifR {
        OvifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Registers synchronized flag"]
    #[inline(always)]
    pub fn rsynf(&self) -> RsynfR {
        RsynfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configuration mode flag"]
    #[inline(always)]
    pub fn cmf(&self) -> CmfR {
        CmfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Last write operation finished flag"]
    #[inline(always)]
    pub fn lwoff(&self) -> LwoffR {
        LwoffR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sencond interrupt flag"]
    #[inline(always)]
    pub fn scif(&mut self) -> ScifW<'_, CtlSpec> {
        ScifW::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm interrupt flag"]
    #[inline(always)]
    pub fn alrmif(&mut self) -> AlrmifW<'_, CtlSpec> {
        AlrmifW::new(self, 1)
    }
    #[doc = "Bit 2 - Overflow interrupt flag"]
    #[inline(always)]
    pub fn ovif(&mut self) -> OvifW<'_, CtlSpec> {
        OvifW::new(self, 2)
    }
    #[doc = "Bit 3 - Registers synchronized flag"]
    #[inline(always)]
    pub fn rsynf(&mut self) -> RsynfW<'_, CtlSpec> {
        RsynfW::new(self, 3)
    }
    #[doc = "Bit 4 - Configuration mode flag"]
    #[inline(always)]
    pub fn cmf(&mut self) -> CmfW<'_, CtlSpec> {
        CmfW::new(self, 4)
    }
    #[doc = "Bit 5 - Last write operation finished flag"]
    #[inline(always)]
    pub fn lwoff(&mut self) -> LwoffW<'_, CtlSpec> {
        LwoffW::new(self, 5)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTL to value 0x20"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0x20;
}
