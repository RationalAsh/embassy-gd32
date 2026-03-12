#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `ADCON` reader - ADC on"]
pub type AdconR = crate::BitReader;
#[doc = "Field `ADCON` writer - ADC on"]
pub type AdconW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTN` reader - Continuous mode"]
pub type CtnR = crate::BitReader;
#[doc = "Field `CTN` writer - Continuous mode"]
pub type CtnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLB` reader - ADC calibration"]
pub type ClbR = crate::BitReader;
#[doc = "Field `CLB` writer - ADC calibration"]
pub type ClbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTCLB` reader - Reset calibration"]
pub type RstclbR = crate::BitReader;
#[doc = "Field `RSTCLB` writer - Reset calibration"]
pub type RstclbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALNUM` reader - Calibration Times"]
pub type CalnumR = crate::FieldReader;
#[doc = "Field `CALNUM` writer - Calibration Times"]
pub type CalnumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DMA` reader - DMA request enable"]
pub type DmaR = crate::BitReader;
#[doc = "Field `DMA` writer - DMA request enable"]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAL` reader - Data alignment"]
pub type DalR = crate::BitReader;
#[doc = "Field `DAL` writer - Data alignment"]
pub type DalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETSIC` reader - External trigger select for inserted channel"]
pub type EtsicR = crate::FieldReader;
#[doc = "Field `ETSIC` writer - External trigger select for inserted channel"]
pub type EtsicW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETEIC` reader - External trigger enable for inserted channel"]
pub type EteicR = crate::BitReader;
#[doc = "Field `ETEIC` writer - External trigger enable for inserted channel"]
pub type EteicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETSRC` reader - External trigger select for regular channel"]
pub type EtsrcR = crate::FieldReader;
#[doc = "Field `ETSRC` writer - External trigger select for regular channel"]
pub type EtsrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETERC` reader - External trigger enable for regular channel"]
pub type EtercR = crate::BitReader;
#[doc = "Field `ETERC` writer - External trigger enable for regular channel"]
pub type EtercW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWICST` reader - Start on inserted channel"]
pub type SwicstR = crate::BitReader;
#[doc = "Field `SWICST` writer - Start on inserted channel"]
pub type SwicstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRCST` reader - Start on regular channel"]
pub type SwrcstR = crate::BitReader;
#[doc = "Field `SWRCST` writer - Start on regular channel"]
pub type SwrcstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSVREN` reader - Channel 16 and 17 enable of ADC0"]
pub type TsvrenR = crate::BitReader;
#[doc = "Field `TSVREN` writer - Channel 16 and 17 enable of ADC0"]
pub type TsvrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETSIC_3` reader - The third bit of ETSIC"]
pub type Etsic3R = crate::BitReader;
#[doc = "Field `ETSIC_3` writer - The third bit of ETSIC"]
pub type Etsic3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETSRC_3` reader - The third bit of ETSRC"]
pub type Etsrc3R = crate::BitReader;
#[doc = "Field `ETSRC_3` writer - The third bit of ETSRC"]
pub type Etsrc3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC on"]
    #[inline(always)]
    pub fn adcon(&self) -> AdconR {
        AdconR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Continuous mode"]
    #[inline(always)]
    pub fn ctn(&self) -> CtnR {
        CtnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC calibration"]
    #[inline(always)]
    pub fn clb(&self) -> ClbR {
        ClbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    pub fn rstclb(&self) -> RstclbR {
        RstclbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Calibration Times"]
    #[inline(always)]
    pub fn calnum(&self) -> CalnumR {
        CalnumR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - DMA request enable"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn dal(&self) -> DalR {
        DalR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - External trigger select for inserted channel"]
    #[inline(always)]
    pub fn etsic(&self) -> EtsicR {
        EtsicR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - External trigger enable for inserted channel"]
    #[inline(always)]
    pub fn eteic(&self) -> EteicR {
        EteicR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:19 - External trigger select for regular channel"]
    #[inline(always)]
    pub fn etsrc(&self) -> EtsrcR {
        EtsrcR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - External trigger enable for regular channel"]
    #[inline(always)]
    pub fn eterc(&self) -> EtercR {
        EtercR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Start on inserted channel"]
    #[inline(always)]
    pub fn swicst(&self) -> SwicstR {
        SwicstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Start on regular channel"]
    #[inline(always)]
    pub fn swrcst(&self) -> SwrcstR {
        SwrcstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel 16 and 17 enable of ADC0"]
    #[inline(always)]
    pub fn tsvren(&self) -> TsvrenR {
        TsvrenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - The third bit of ETSIC"]
    #[inline(always)]
    pub fn etsic_3(&self) -> Etsic3R {
        Etsic3R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - The third bit of ETSRC"]
    #[inline(always)]
    pub fn etsrc_3(&self) -> Etsrc3R {
        Etsrc3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC on"]
    #[inline(always)]
    pub fn adcon(&mut self) -> AdconW<'_, Ctl1Spec> {
        AdconW::new(self, 0)
    }
    #[doc = "Bit 1 - Continuous mode"]
    #[inline(always)]
    pub fn ctn(&mut self) -> CtnW<'_, Ctl1Spec> {
        CtnW::new(self, 1)
    }
    #[doc = "Bit 2 - ADC calibration"]
    #[inline(always)]
    pub fn clb(&mut self) -> ClbW<'_, Ctl1Spec> {
        ClbW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    pub fn rstclb(&mut self) -> RstclbW<'_, Ctl1Spec> {
        RstclbW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Calibration Times"]
    #[inline(always)]
    pub fn calnum(&mut self) -> CalnumW<'_, Ctl1Spec> {
        CalnumW::new(self, 4)
    }
    #[doc = "Bit 8 - DMA request enable"]
    #[inline(always)]
    pub fn dma(&mut self) -> DmaW<'_, Ctl1Spec> {
        DmaW::new(self, 8)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn dal(&mut self) -> DalW<'_, Ctl1Spec> {
        DalW::new(self, 11)
    }
    #[doc = "Bits 12:14 - External trigger select for inserted channel"]
    #[inline(always)]
    pub fn etsic(&mut self) -> EtsicW<'_, Ctl1Spec> {
        EtsicW::new(self, 12)
    }
    #[doc = "Bit 15 - External trigger enable for inserted channel"]
    #[inline(always)]
    pub fn eteic(&mut self) -> EteicW<'_, Ctl1Spec> {
        EteicW::new(self, 15)
    }
    #[doc = "Bits 17:19 - External trigger select for regular channel"]
    #[inline(always)]
    pub fn etsrc(&mut self) -> EtsrcW<'_, Ctl1Spec> {
        EtsrcW::new(self, 17)
    }
    #[doc = "Bit 20 - External trigger enable for regular channel"]
    #[inline(always)]
    pub fn eterc(&mut self) -> EtercW<'_, Ctl1Spec> {
        EtercW::new(self, 20)
    }
    #[doc = "Bit 21 - Start on inserted channel"]
    #[inline(always)]
    pub fn swicst(&mut self) -> SwicstW<'_, Ctl1Spec> {
        SwicstW::new(self, 21)
    }
    #[doc = "Bit 22 - Start on regular channel"]
    #[inline(always)]
    pub fn swrcst(&mut self) -> SwrcstW<'_, Ctl1Spec> {
        SwrcstW::new(self, 22)
    }
    #[doc = "Bit 23 - Channel 16 and 17 enable of ADC0"]
    #[inline(always)]
    pub fn tsvren(&mut self) -> TsvrenW<'_, Ctl1Spec> {
        TsvrenW::new(self, 23)
    }
    #[doc = "Bit 30 - The third bit of ETSIC"]
    #[inline(always)]
    pub fn etsic_3(&mut self) -> Etsic3W<'_, Ctl1Spec> {
        Etsic3W::new(self, 30)
    }
    #[doc = "Bit 31 - The third bit of ETSRC"]
    #[inline(always)]
    pub fn etsrc_3(&mut self) -> Etsrc3W<'_, Ctl1Spec> {
        Etsrc3W::new(self, 31)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {}
