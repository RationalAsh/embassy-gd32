#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `REFEN` reader - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
pub type RefenR = crate::BitReader;
#[doc = "Field `REFEN` writer - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
pub type RefenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BPSHAD` reader - Bypass the shadow registers"]
pub type BpshadR = crate::BitReader;
#[doc = "Field `BPSHAD` writer - Bypass the shadow registers"]
pub type BpshadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS` reader - Hour format"]
pub type CsR = crate::BitReader;
#[doc = "Field `CS` writer - Hour format"]
pub type CsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRM0EN` reader - Alarm A enable"]
pub type Alrm0enR = crate::BitReader;
#[doc = "Field `ALRM0EN` writer - Alarm A enable"]
pub type Alrm0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRM0IE` reader - Alarm A interrupt enable"]
pub type Alrm0ieR = crate::BitReader;
#[doc = "Field `ALRM0IE` writer - Alarm A interrupt enable"]
pub type Alrm0ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A1H` writer - Add 1 hour (summer time change)"]
pub type A1hW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1H` writer - Subtract 1 hour (winter time change)"]
pub type S1hW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSM` reader - Backup"]
pub type DsmR = crate::BitReader;
#[doc = "Field `DSM` writer - Backup"]
pub type DsmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
    #[inline(always)]
    pub fn refen(&self) -> RefenR {
        RefenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bypass the shadow registers"]
    #[inline(always)]
    pub fn bpshad(&self) -> BpshadR {
        BpshadR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    pub fn cs(&self) -> CsR {
        CsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alrm0en(&self) -> Alrm0enR {
        Alrm0enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alrm0ie(&self) -> Alrm0ieR {
        Alrm0ieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 18 - Backup"]
    #[inline(always)]
    pub fn dsm(&self) -> DsmR {
        DsmR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
    #[inline(always)]
    pub fn refen(&mut self) -> RefenW<'_, CtlSpec> {
        RefenW::new(self, 4)
    }
    #[doc = "Bit 5 - Bypass the shadow registers"]
    #[inline(always)]
    pub fn bpshad(&mut self) -> BpshadW<'_, CtlSpec> {
        BpshadW::new(self, 5)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    pub fn cs(&mut self) -> CsW<'_, CtlSpec> {
        CsW::new(self, 6)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alrm0en(&mut self) -> Alrm0enW<'_, CtlSpec> {
        Alrm0enW::new(self, 8)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alrm0ie(&mut self) -> Alrm0ieW<'_, CtlSpec> {
        Alrm0ieW::new(self, 12)
    }
    #[doc = "Bit 16 - Add 1 hour (summer time change)"]
    #[inline(always)]
    pub fn a1h(&mut self) -> A1hW<'_, CtlSpec> {
        A1hW::new(self, 16)
    }
    #[doc = "Bit 17 - Subtract 1 hour (winter time change)"]
    #[inline(always)]
    pub fn s1h(&mut self) -> S1hW<'_, CtlSpec> {
        S1hW::new(self, 17)
    }
    #[doc = "Bit 18 - Backup"]
    #[inline(always)]
    pub fn dsm(&mut self) -> DsmW<'_, CtlSpec> {
        DsmW::new(self, 18)
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
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {}
