#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `START` writer - Start message digest calculation"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAE` reader - DMA enable"]
pub type DmaeR = crate::BitReader;
#[doc = "Field `DMAE` writer - DMA enable"]
pub type DmaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAM` reader - Data type mode"]
pub type DatamR = crate::FieldReader;
#[doc = "Field `DATAM` writer - Data type mode"]
pub type DatamW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HMS` reader - HAU mode selection"]
pub type HmsR = crate::BitReader;
#[doc = "Field `HMS` writer - HAU mode selection"]
pub type HmsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALGM0` reader - Algorithm selection bit 0"]
pub type Algm0R = crate::BitReader;
#[doc = "Field `ALGM0` writer - Algorithm selection bit 0"]
pub type Algm0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NWIF` reader - Number of words in IN FIFO"]
pub type NwifR = crate::FieldReader;
#[doc = "Field `DINE` reader - DI register is not empty"]
pub type DineR = crate::BitReader;
#[doc = "Field `MDS` reader - Multiple DMA selection"]
pub type MdsR = crate::BitReader;
#[doc = "Field `MDS` writer - Multiple DMA selection"]
pub type MdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KLM` reader - Key length mode"]
pub type KlmR = crate::BitReader;
#[doc = "Field `KLM` writer - Key length mode"]
pub type KlmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALGM1` reader - Algorithm selection bit 1"]
pub type Algm1R = crate::BitReader;
#[doc = "Field `ALGM1` writer - Algorithm selection bit 1"]
pub type Algm1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - DMA enable"]
    #[inline(always)]
    pub fn dmae(&self) -> DmaeR {
        DmaeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Data type mode"]
    #[inline(always)]
    pub fn datam(&self) -> DatamR {
        DatamR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - HAU mode selection"]
    #[inline(always)]
    pub fn hms(&self) -> HmsR {
        HmsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Algorithm selection bit 0"]
    #[inline(always)]
    pub fn algm0(&self) -> Algm0R {
        Algm0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Number of words in IN FIFO"]
    #[inline(always)]
    pub fn nwif(&self) -> NwifR {
        NwifR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DI register is not empty"]
    #[inline(always)]
    pub fn dine(&self) -> DineR {
        DineR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Multiple DMA selection"]
    #[inline(always)]
    pub fn mds(&self) -> MdsR {
        MdsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Key length mode"]
    #[inline(always)]
    pub fn klm(&self) -> KlmR {
        KlmR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Algorithm selection bit 1"]
    #[inline(always)]
    pub fn algm1(&self) -> Algm1R {
        Algm1R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Start message digest calculation"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, CtlSpec> {
        StartW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA enable"]
    #[inline(always)]
    pub fn dmae(&mut self) -> DmaeW<'_, CtlSpec> {
        DmaeW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Data type mode"]
    #[inline(always)]
    pub fn datam(&mut self) -> DatamW<'_, CtlSpec> {
        DatamW::new(self, 4)
    }
    #[doc = "Bit 6 - HAU mode selection"]
    #[inline(always)]
    pub fn hms(&mut self) -> HmsW<'_, CtlSpec> {
        HmsW::new(self, 6)
    }
    #[doc = "Bit 7 - Algorithm selection bit 0"]
    #[inline(always)]
    pub fn algm0(&mut self) -> Algm0W<'_, CtlSpec> {
        Algm0W::new(self, 7)
    }
    #[doc = "Bit 13 - Multiple DMA selection"]
    #[inline(always)]
    pub fn mds(&mut self) -> MdsW<'_, CtlSpec> {
        MdsW::new(self, 13)
    }
    #[doc = "Bit 16 - Key length mode"]
    #[inline(always)]
    pub fn klm(&mut self) -> KlmW<'_, CtlSpec> {
        KlmW::new(self, 16)
    }
    #[doc = "Bit 18 - Algorithm selection bit 1"]
    #[inline(always)]
    pub fn algm1(&mut self) -> Algm1W<'_, CtlSpec> {
        Algm1W::new(self, 18)
    }
}
#[doc = "HAU control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
