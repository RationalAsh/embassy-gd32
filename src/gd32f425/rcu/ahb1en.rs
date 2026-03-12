#[doc = "Register `AHB1EN` reader"]
pub type R = crate::R<Ahb1enSpec>;
#[doc = "Register `AHB1EN` writer"]
pub type W = crate::W<Ahb1enSpec>;
#[doc = "GPIO port A clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Paen {
    #[doc = "0: Disable the selected module clock."]
    Disable = 0,
    #[doc = "1: Enable the selected module clock."]
    Enable = 1,
}
impl From<Paen> for bool {
    #[inline(always)]
    fn from(variant: Paen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAEN` reader - GPIO port A clock enable"]
pub type PaenR = crate::BitReader<Paen>;
impl PaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Paen {
        match self.bits {
            false => Paen::Disable,
            true => Paen::Enable,
        }
    }
    #[doc = "Disable the selected module clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Paen::Disable
    }
    #[doc = "Enable the selected module clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Paen::Enable
    }
}
#[doc = "Field `PAEN` writer - GPIO port A clock enable"]
pub type PaenW<'a, REG> = crate::BitWriter<'a, REG, Paen>;
impl<'a, REG> PaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the selected module clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Paen::Disable)
    }
    #[doc = "Enable the selected module clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Paen::Enable)
    }
}
#[doc = "Field `PBEN` reader - GPIO port B clock enable"]
pub use PaenR as PbenR;
#[doc = "Field `PCEN` reader - GPIO port C clock enable"]
pub use PaenR as PcenR;
#[doc = "Field `PDEN` reader - GPIO port D clock enable"]
pub use PaenR as PdenR;
#[doc = "Field `PEEN` reader - GPIO port E clock enable"]
pub use PaenR as PeenR;
#[doc = "Field `PFEN` reader - GPIO port F clock enable"]
pub use PaenR as PfenR;
#[doc = "Field `PGEN` reader - GPIO port G clock enable"]
pub use PaenR as PgenR;
#[doc = "Field `PHEN` reader - GPIO port H clock enable"]
pub use PaenR as PhenR;
#[doc = "Field `PIEN` reader - GPIO port I clock enable"]
pub use PaenR as PienR;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub use PaenR as CrcenR;
#[doc = "Field `BKPSRAMEN` reader - BKPSRAM clock enable"]
pub use PaenR as BkpsramenR;
#[doc = "Field `TCMSRAMEN` reader - TCMSRAM clock enable"]
pub use PaenR as TcmsramenR;
#[doc = "Field `DMA0EN` reader - DMA0 clock enable"]
pub use PaenR as Dma0enR;
#[doc = "Field `DMA1EN` reader - DMA1 clock enable"]
pub use PaenR as Dma1enR;
#[doc = "Field `IPAEN` reader - IPA clock enable"]
pub use PaenR as IpaenR;
#[doc = "Field `ENETEN` reader - Ethernet clock enable"]
pub use PaenR as EnetenR;
#[doc = "Field `ENETTXEN` reader - Ethernet TX clock enable"]
pub use PaenR as EnettxenR;
#[doc = "Field `ENETRXEN` reader - Ethernet RX clock enable"]
pub use PaenR as EnetrxenR;
#[doc = "Field `ENETPTPEN` reader - Ethernet PTP clock enable"]
pub use PaenR as EnetptpenR;
#[doc = "Field `USBHSEN` reader - USBHS clock enable"]
pub use PaenR as UsbhsenR;
#[doc = "Field `USBHSULPIEN` reader - USBHS ULPI clock enable"]
pub use PaenR as UsbhsulpienR;
#[doc = "Field `PBEN` writer - GPIO port B clock enable"]
pub use PaenW as PbenW;
#[doc = "Field `PCEN` writer - GPIO port C clock enable"]
pub use PaenW as PcenW;
#[doc = "Field `PDEN` writer - GPIO port D clock enable"]
pub use PaenW as PdenW;
#[doc = "Field `PEEN` writer - GPIO port E clock enable"]
pub use PaenW as PeenW;
#[doc = "Field `PFEN` writer - GPIO port F clock enable"]
pub use PaenW as PfenW;
#[doc = "Field `PGEN` writer - GPIO port G clock enable"]
pub use PaenW as PgenW;
#[doc = "Field `PHEN` writer - GPIO port H clock enable"]
pub use PaenW as PhenW;
#[doc = "Field `PIEN` writer - GPIO port I clock enable"]
pub use PaenW as PienW;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub use PaenW as CrcenW;
#[doc = "Field `BKPSRAMEN` writer - BKPSRAM clock enable"]
pub use PaenW as BkpsramenW;
#[doc = "Field `TCMSRAMEN` writer - TCMSRAM clock enable"]
pub use PaenW as TcmsramenW;
#[doc = "Field `DMA0EN` writer - DMA0 clock enable"]
pub use PaenW as Dma0enW;
#[doc = "Field `DMA1EN` writer - DMA1 clock enable"]
pub use PaenW as Dma1enW;
#[doc = "Field `IPAEN` writer - IPA clock enable"]
pub use PaenW as IpaenW;
#[doc = "Field `ENETEN` writer - Ethernet clock enable"]
pub use PaenW as EnetenW;
#[doc = "Field `ENETTXEN` writer - Ethernet TX clock enable"]
pub use PaenW as EnettxenW;
#[doc = "Field `ENETRXEN` writer - Ethernet RX clock enable"]
pub use PaenW as EnetrxenW;
#[doc = "Field `ENETPTPEN` writer - Ethernet PTP clock enable"]
pub use PaenW as EnetptpenW;
#[doc = "Field `USBHSEN` writer - USBHS clock enable"]
pub use PaenW as UsbhsenW;
#[doc = "Field `USBHSULPIEN` writer - USBHS ULPI clock enable"]
pub use PaenW as UsbhsulpienW;
impl R {
    #[doc = "Bit 0 - GPIO port A clock enable"]
    #[inline(always)]
    pub fn paen(&self) -> PaenR {
        PaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO port B clock enable"]
    #[inline(always)]
    pub fn pben(&self) -> PbenR {
        PbenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO port C clock enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PcenR {
        PcenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO port D clock enable"]
    #[inline(always)]
    pub fn pden(&self) -> PdenR {
        PdenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO port E clock enable"]
    #[inline(always)]
    pub fn peen(&self) -> PeenR {
        PeenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO port F clock enable"]
    #[inline(always)]
    pub fn pfen(&self) -> PfenR {
        PfenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO port G clock enable"]
    #[inline(always)]
    pub fn pgen(&self) -> PgenR {
        PgenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO port H clock enable"]
    #[inline(always)]
    pub fn phen(&self) -> PhenR {
        PhenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO port I clock enable"]
    #[inline(always)]
    pub fn pien(&self) -> PienR {
        PienR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 18 - BKPSRAM clock enable"]
    #[inline(always)]
    pub fn bkpsramen(&self) -> BkpsramenR {
        BkpsramenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - TCMSRAM clock enable"]
    #[inline(always)]
    pub fn tcmsramen(&self) -> TcmsramenR {
        TcmsramenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA0 clock enable"]
    #[inline(always)]
    pub fn dma0en(&self) -> Dma0enR {
        Dma0enR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> Dma1enR {
        Dma1enR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - IPA clock enable"]
    #[inline(always)]
    pub fn ipaen(&self) -> IpaenR {
        IpaenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Ethernet clock enable"]
    #[inline(always)]
    pub fn eneten(&self) -> EnetenR {
        EnetenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Ethernet TX clock enable"]
    #[inline(always)]
    pub fn enettxen(&self) -> EnettxenR {
        EnettxenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Ethernet RX clock enable"]
    #[inline(always)]
    pub fn enetrxen(&self) -> EnetrxenR {
        EnetrxenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Ethernet PTP clock enable"]
    #[inline(always)]
    pub fn enetptpen(&self) -> EnetptpenR {
        EnetptpenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - USBHS clock enable"]
    #[inline(always)]
    pub fn usbhsen(&self) -> UsbhsenR {
        UsbhsenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - USBHS ULPI clock enable"]
    #[inline(always)]
    pub fn usbhsulpien(&self) -> UsbhsulpienR {
        UsbhsulpienR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO port A clock enable"]
    #[inline(always)]
    pub fn paen(&mut self) -> PaenW<'_, Ahb1enSpec> {
        PaenW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO port B clock enable"]
    #[inline(always)]
    pub fn pben(&mut self) -> PbenW<'_, Ahb1enSpec> {
        PbenW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO port C clock enable"]
    #[inline(always)]
    pub fn pcen(&mut self) -> PcenW<'_, Ahb1enSpec> {
        PcenW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO port D clock enable"]
    #[inline(always)]
    pub fn pden(&mut self) -> PdenW<'_, Ahb1enSpec> {
        PdenW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO port E clock enable"]
    #[inline(always)]
    pub fn peen(&mut self) -> PeenW<'_, Ahb1enSpec> {
        PeenW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO port F clock enable"]
    #[inline(always)]
    pub fn pfen(&mut self) -> PfenW<'_, Ahb1enSpec> {
        PfenW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO port G clock enable"]
    #[inline(always)]
    pub fn pgen(&mut self) -> PgenW<'_, Ahb1enSpec> {
        PgenW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO port H clock enable"]
    #[inline(always)]
    pub fn phen(&mut self) -> PhenW<'_, Ahb1enSpec> {
        PhenW::new(self, 7)
    }
    #[doc = "Bit 8 - GPIO port I clock enable"]
    #[inline(always)]
    pub fn pien(&mut self) -> PienW<'_, Ahb1enSpec> {
        PienW::new(self, 8)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CrcenW<'_, Ahb1enSpec> {
        CrcenW::new(self, 12)
    }
    #[doc = "Bit 18 - BKPSRAM clock enable"]
    #[inline(always)]
    pub fn bkpsramen(&mut self) -> BkpsramenW<'_, Ahb1enSpec> {
        BkpsramenW::new(self, 18)
    }
    #[doc = "Bit 20 - TCMSRAM clock enable"]
    #[inline(always)]
    pub fn tcmsramen(&mut self) -> TcmsramenW<'_, Ahb1enSpec> {
        TcmsramenW::new(self, 20)
    }
    #[doc = "Bit 21 - DMA0 clock enable"]
    #[inline(always)]
    pub fn dma0en(&mut self) -> Dma0enW<'_, Ahb1enSpec> {
        Dma0enW::new(self, 21)
    }
    #[doc = "Bit 22 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&mut self) -> Dma1enW<'_, Ahb1enSpec> {
        Dma1enW::new(self, 22)
    }
    #[doc = "Bit 23 - IPA clock enable"]
    #[inline(always)]
    pub fn ipaen(&mut self) -> IpaenW<'_, Ahb1enSpec> {
        IpaenW::new(self, 23)
    }
    #[doc = "Bit 25 - Ethernet clock enable"]
    #[inline(always)]
    pub fn eneten(&mut self) -> EnetenW<'_, Ahb1enSpec> {
        EnetenW::new(self, 25)
    }
    #[doc = "Bit 26 - Ethernet TX clock enable"]
    #[inline(always)]
    pub fn enettxen(&mut self) -> EnettxenW<'_, Ahb1enSpec> {
        EnettxenW::new(self, 26)
    }
    #[doc = "Bit 27 - Ethernet RX clock enable"]
    #[inline(always)]
    pub fn enetrxen(&mut self) -> EnetrxenW<'_, Ahb1enSpec> {
        EnetrxenW::new(self, 27)
    }
    #[doc = "Bit 28 - Ethernet PTP clock enable"]
    #[inline(always)]
    pub fn enetptpen(&mut self) -> EnetptpenW<'_, Ahb1enSpec> {
        EnetptpenW::new(self, 28)
    }
    #[doc = "Bit 29 - USBHS clock enable"]
    #[inline(always)]
    pub fn usbhsen(&mut self) -> UsbhsenW<'_, Ahb1enSpec> {
        UsbhsenW::new(self, 29)
    }
    #[doc = "Bit 30 - USBHS ULPI clock enable"]
    #[inline(always)]
    pub fn usbhsulpien(&mut self) -> UsbhsulpienW<'_, Ahb1enSpec> {
        UsbhsulpienW::new(self, 30)
    }
}
#[doc = "AHB1 enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb1enSpec;
impl crate::RegisterSpec for Ahb1enSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1en::R`](R) reader structure"]
impl crate::Readable for Ahb1enSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb1en::W`](W) writer structure"]
impl crate::Writable for Ahb1enSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB1EN to value 0x0010_0000"]
impl crate::Resettable for Ahb1enSpec {
    const RESET_VALUE: u32 = 0x0010_0000;
}
