#[doc = "Register `DIEP1INTF` reader"]
pub type R = crate::R<Diep1intfSpec>;
#[doc = "Register `DIEP1INTF` writer"]
pub type W = crate::W<Diep1intfSpec>;
#[doc = "Field `TF` reader - Transfer finished"]
pub type TfR = crate::BitReader;
#[doc = "Field `TF` writer - Transfer finished"]
pub type TfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS` reader - Endpoint finished"]
pub type EpdisR = crate::BitReader;
#[doc = "Field `EPDIS` writer - Endpoint finished"]
pub type EpdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITO` reader - Control in timeout interrupt"]
pub type CitoR = crate::BitReader;
#[doc = "Field `CITO` writer - Control in timeout interrupt"]
pub type CitoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTXFUD` reader - Endpoint Tx FIFO underrun"]
pub type EptxfudR = crate::BitReader;
#[doc = "Field `EPTXFUD` writer - Endpoint Tx FIFO underrun"]
pub type EptxfudW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPNE` reader - IN endpoint NAK effective"]
pub type IepneR = crate::BitReader;
#[doc = "Field `IEPNE` writer - IN endpoint NAK effective"]
pub type IepneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFE` reader - Transmit FIFO empty"]
pub type TxfeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transfer finished"]
    #[inline(always)]
    pub fn tf(&self) -> TfR {
        TfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint finished"]
    #[inline(always)]
    pub fn epdis(&self) -> EpdisR {
        EpdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Control in timeout interrupt"]
    #[inline(always)]
    pub fn cito(&self) -> CitoR {
        CitoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint Tx FIFO underrun"]
    #[inline(always)]
    pub fn eptxfud(&self) -> EptxfudR {
        EptxfudR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective"]
    #[inline(always)]
    pub fn iepne(&self) -> IepneR {
        IepneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO empty"]
    #[inline(always)]
    pub fn txfe(&self) -> TxfeR {
        TxfeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer finished"]
    #[inline(always)]
    pub fn tf(&mut self) -> TfW<'_, Diep1intfSpec> {
        TfW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint finished"]
    #[inline(always)]
    pub fn epdis(&mut self) -> EpdisW<'_, Diep1intfSpec> {
        EpdisW::new(self, 1)
    }
    #[doc = "Bit 3 - Control in timeout interrupt"]
    #[inline(always)]
    pub fn cito(&mut self) -> CitoW<'_, Diep1intfSpec> {
        CitoW::new(self, 3)
    }
    #[doc = "Bit 4 - Endpoint Tx FIFO underrun"]
    #[inline(always)]
    pub fn eptxfud(&mut self) -> EptxfudW<'_, Diep1intfSpec> {
        EptxfudW::new(self, 4)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective"]
    #[inline(always)]
    pub fn iepne(&mut self) -> IepneW<'_, Diep1intfSpec> {
        IepneW::new(self, 6)
    }
}
#[doc = "device endpoint-1 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep1intf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep1intf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diep1intfSpec;
impl crate::RegisterSpec for Diep1intfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep1intf::R`](R) reader structure"]
impl crate::Readable for Diep1intfSpec {}
#[doc = "`write(|w| ..)` method takes [`diep1intf::W`](W) writer structure"]
impl crate::Writable for Diep1intfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEP1INTF to value 0x80"]
impl crate::Resettable for Diep1intfSpec {
    const RESET_VALUE: u32 = 0x80;
}
