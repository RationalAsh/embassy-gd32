#[doc = "Register `AHB1RST` reader"]
pub type R = crate::R<Ahb1rstSpec>;
#[doc = "Register `AHB1RST` writer"]
pub type W = crate::W<Ahb1rstSpec>;
#[doc = "GPIO port A reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Parst {
    #[doc = "1: Reset the selected module."]
    Reset = 1,
}
impl From<Parst> for bool {
    #[inline(always)]
    fn from(variant: Parst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARST` reader - GPIO port A reset"]
pub type ParstR = crate::BitReader<Parst>;
impl ParstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Parst> {
        match self.bits {
            true => Some(Parst::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Parst::Reset
    }
}
#[doc = "Field `PARST` writer - GPIO port A reset"]
pub type ParstW<'a, REG> = crate::BitWriter<'a, REG, Parst>;
impl<'a, REG> ParstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Parst::Reset)
    }
}
#[doc = "Field `PBRST` reader - GPIO port B reset"]
pub use ParstR as PbrstR;
#[doc = "Field `PCRST` reader - GPIO port C reset"]
pub use ParstR as PcrstR;
#[doc = "Field `PDRST` reader - GPIO port D reset"]
pub use ParstR as PdrstR;
#[doc = "Field `PERST` reader - GPIO port E reset"]
pub use ParstR as PerstR;
#[doc = "Field `PFRST` reader - GPIO port F reset"]
pub use ParstR as PfrstR;
#[doc = "Field `PGRST` reader - GPIO port G reset"]
pub use ParstR as PgrstR;
#[doc = "Field `PHRST` reader - GPIO port H reset"]
pub use ParstR as PhrstR;
#[doc = "Field `PIRST` reader - GPIO port I reset"]
pub use ParstR as PirstR;
#[doc = "Field `CRCRST` reader - CRC reset"]
pub use ParstR as CrcrstR;
#[doc = "Field `DMA0RST` reader - DMA0 reset"]
pub use ParstR as Dma0rstR;
#[doc = "Field `DMA1RST` reader - DMA1 reset"]
pub use ParstR as Dma1rstR;
#[doc = "Field `IPARST` reader - IPA reset"]
pub use ParstR as IparstR;
#[doc = "Field `ENETRST` reader - Ethernet reset"]
pub use ParstR as EnetrstR;
#[doc = "Field `USBHSRST` reader - USBHS reset"]
pub use ParstR as UsbhsrstR;
#[doc = "Field `PBRST` writer - GPIO port B reset"]
pub use ParstW as PbrstW;
#[doc = "Field `PCRST` writer - GPIO port C reset"]
pub use ParstW as PcrstW;
#[doc = "Field `PDRST` writer - GPIO port D reset"]
pub use ParstW as PdrstW;
#[doc = "Field `PERST` writer - GPIO port E reset"]
pub use ParstW as PerstW;
#[doc = "Field `PFRST` writer - GPIO port F reset"]
pub use ParstW as PfrstW;
#[doc = "Field `PGRST` writer - GPIO port G reset"]
pub use ParstW as PgrstW;
#[doc = "Field `PHRST` writer - GPIO port H reset"]
pub use ParstW as PhrstW;
#[doc = "Field `PIRST` writer - GPIO port I reset"]
pub use ParstW as PirstW;
#[doc = "Field `CRCRST` writer - CRC reset"]
pub use ParstW as CrcrstW;
#[doc = "Field `DMA0RST` writer - DMA0 reset"]
pub use ParstW as Dma0rstW;
#[doc = "Field `DMA1RST` writer - DMA1 reset"]
pub use ParstW as Dma1rstW;
#[doc = "Field `IPARST` writer - IPA reset"]
pub use ParstW as IparstW;
#[doc = "Field `ENETRST` writer - Ethernet reset"]
pub use ParstW as EnetrstW;
#[doc = "Field `USBHSRST` writer - USBHS reset"]
pub use ParstW as UsbhsrstW;
impl R {
    #[doc = "Bit 0 - GPIO port A reset"]
    #[inline(always)]
    pub fn parst(&self) -> ParstR {
        ParstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO port B reset"]
    #[inline(always)]
    pub fn pbrst(&self) -> PbrstR {
        PbrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO port C reset"]
    #[inline(always)]
    pub fn pcrst(&self) -> PcrstR {
        PcrstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO port D reset"]
    #[inline(always)]
    pub fn pdrst(&self) -> PdrstR {
        PdrstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO port E reset"]
    #[inline(always)]
    pub fn perst(&self) -> PerstR {
        PerstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO port F reset"]
    #[inline(always)]
    pub fn pfrst(&self) -> PfrstR {
        PfrstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO port G reset"]
    #[inline(always)]
    pub fn pgrst(&self) -> PgrstR {
        PgrstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO port H reset"]
    #[inline(always)]
    pub fn phrst(&self) -> PhrstR {
        PhrstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO port I reset"]
    #[inline(always)]
    pub fn pirst(&self) -> PirstR {
        PirstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn crcrst(&self) -> CrcrstR {
        CrcrstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA0 reset"]
    #[inline(always)]
    pub fn dma0rst(&self) -> Dma0rstR {
        Dma0rstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA1 reset"]
    #[inline(always)]
    pub fn dma1rst(&self) -> Dma1rstR {
        Dma1rstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - IPA reset"]
    #[inline(always)]
    pub fn iparst(&self) -> IparstR {
        IparstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Ethernet reset"]
    #[inline(always)]
    pub fn enetrst(&self) -> EnetrstR {
        EnetrstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - USBHS reset"]
    #[inline(always)]
    pub fn usbhsrst(&self) -> UsbhsrstR {
        UsbhsrstR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO port A reset"]
    #[inline(always)]
    pub fn parst(&mut self) -> ParstW<'_, Ahb1rstSpec> {
        ParstW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO port B reset"]
    #[inline(always)]
    pub fn pbrst(&mut self) -> PbrstW<'_, Ahb1rstSpec> {
        PbrstW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO port C reset"]
    #[inline(always)]
    pub fn pcrst(&mut self) -> PcrstW<'_, Ahb1rstSpec> {
        PcrstW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO port D reset"]
    #[inline(always)]
    pub fn pdrst(&mut self) -> PdrstW<'_, Ahb1rstSpec> {
        PdrstW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO port E reset"]
    #[inline(always)]
    pub fn perst(&mut self) -> PerstW<'_, Ahb1rstSpec> {
        PerstW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO port F reset"]
    #[inline(always)]
    pub fn pfrst(&mut self) -> PfrstW<'_, Ahb1rstSpec> {
        PfrstW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO port G reset"]
    #[inline(always)]
    pub fn pgrst(&mut self) -> PgrstW<'_, Ahb1rstSpec> {
        PgrstW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO port H reset"]
    #[inline(always)]
    pub fn phrst(&mut self) -> PhrstW<'_, Ahb1rstSpec> {
        PhrstW::new(self, 7)
    }
    #[doc = "Bit 8 - GPIO port I reset"]
    #[inline(always)]
    pub fn pirst(&mut self) -> PirstW<'_, Ahb1rstSpec> {
        PirstW::new(self, 8)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn crcrst(&mut self) -> CrcrstW<'_, Ahb1rstSpec> {
        CrcrstW::new(self, 12)
    }
    #[doc = "Bit 21 - DMA0 reset"]
    #[inline(always)]
    pub fn dma0rst(&mut self) -> Dma0rstW<'_, Ahb1rstSpec> {
        Dma0rstW::new(self, 21)
    }
    #[doc = "Bit 22 - DMA1 reset"]
    #[inline(always)]
    pub fn dma1rst(&mut self) -> Dma1rstW<'_, Ahb1rstSpec> {
        Dma1rstW::new(self, 22)
    }
    #[doc = "Bit 23 - IPA reset"]
    #[inline(always)]
    pub fn iparst(&mut self) -> IparstW<'_, Ahb1rstSpec> {
        IparstW::new(self, 23)
    }
    #[doc = "Bit 25 - Ethernet reset"]
    #[inline(always)]
    pub fn enetrst(&mut self) -> EnetrstW<'_, Ahb1rstSpec> {
        EnetrstW::new(self, 25)
    }
    #[doc = "Bit 29 - USBHS reset"]
    #[inline(always)]
    pub fn usbhsrst(&mut self) -> UsbhsrstW<'_, Ahb1rstSpec> {
        UsbhsrstW::new(self, 29)
    }
}
#[doc = "AHB1 reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb1rstSpec;
impl crate::RegisterSpec for Ahb1rstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1rst::R`](R) reader structure"]
impl crate::Readable for Ahb1rstSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb1rst::W`](W) writer structure"]
impl crate::Writable for Ahb1rstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB1RST to value 0"]
impl crate::Resettable for Ahb1rstSpec {}
