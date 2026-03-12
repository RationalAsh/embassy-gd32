#[doc = "Register `DOEP1INTEN` reader"]
pub type R = crate::R<Doep1intenSpec>;
#[doc = "Register `DOEP1INTEN` writer"]
pub type W = crate::W<Doep1intenSpec>;
#[doc = "Field `TFEN` reader - Transfer finished interrupt enable bit"]
pub type TfenR = crate::BitReader;
#[doc = "Field `TFEN` writer - Transfer finished interrupt enable bit"]
pub type TfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISEN` reader - Endpoint disabled interrupt enable bit"]
pub type EpdisenR = crate::BitReader;
#[doc = "Field `EPDISEN` writer - Endpoint disabled interrupt enable bit"]
pub type EpdisenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STPFEN` reader - SETUP phase finished interrupt enable bit"]
pub type StpfenR = crate::BitReader;
#[doc = "Field `STPFEN` writer - SETUP phase finished interrupt enable bit"]
pub type StpfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPRXFOVREN` reader - Endpoint Rx FIFO over run interrupt enable bit"]
pub type EprxfovrenR = crate::BitReader;
#[doc = "Field `EPRXFOVREN` writer - Endpoint Rx FIFO over run interrupt enable bit"]
pub type EprxfovrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTBSTPEN` reader - Back-to-back SETUP packets interrupt enable bit"]
pub type BtbstpenR = crate::BitReader;
#[doc = "Field `BTBSTPEN` writer - Back-to-back SETUP packets interrupt enable bit"]
pub type BtbstpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYETEN` reader - Send NYET handshake interrupt enable bit"]
pub type NyetenR = crate::BitReader;
#[doc = "Field `NYETEN` writer - Send NYET handshake interrupt enable bit"]
pub type NyetenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer finished interrupt enable bit"]
    #[inline(always)]
    pub fn tfen(&self) -> TfenR {
        TfenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt enable bit"]
    #[inline(always)]
    pub fn epdisen(&self) -> EpdisenR {
        EpdisenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SETUP phase finished interrupt enable bit"]
    #[inline(always)]
    pub fn stpfen(&self) -> StpfenR {
        StpfenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint Rx FIFO over run interrupt enable bit"]
    #[inline(always)]
    pub fn eprxfovren(&self) -> EprxfovrenR {
        EprxfovrenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets interrupt enable bit"]
    #[inline(always)]
    pub fn btbstpen(&self) -> BtbstpenR {
        BtbstpenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 13 - Send NYET handshake interrupt enable bit"]
    #[inline(always)]
    pub fn nyeten(&self) -> NyetenR {
        NyetenR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer finished interrupt enable bit"]
    #[inline(always)]
    pub fn tfen(&mut self) -> TfenW<'_, Doep1intenSpec> {
        TfenW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt enable bit"]
    #[inline(always)]
    pub fn epdisen(&mut self) -> EpdisenW<'_, Doep1intenSpec> {
        EpdisenW::new(self, 1)
    }
    #[doc = "Bit 3 - SETUP phase finished interrupt enable bit"]
    #[inline(always)]
    pub fn stpfen(&mut self) -> StpfenW<'_, Doep1intenSpec> {
        StpfenW::new(self, 3)
    }
    #[doc = "Bit 4 - Endpoint Rx FIFO over run interrupt enable bit"]
    #[inline(always)]
    pub fn eprxfovren(&mut self) -> EprxfovrenW<'_, Doep1intenSpec> {
        EprxfovrenW::new(self, 4)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets interrupt enable bit"]
    #[inline(always)]
    pub fn btbstpen(&mut self) -> BtbstpenW<'_, Doep1intenSpec> {
        BtbstpenW::new(self, 6)
    }
    #[doc = "Bit 13 - Send NYET handshake interrupt enable bit"]
    #[inline(always)]
    pub fn nyeten(&mut self) -> NyetenW<'_, Doep1intenSpec> {
        NyetenW::new(self, 13)
    }
}
#[doc = "Device OUT endpoint 1 interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep1inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep1inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doep1intenSpec;
impl crate::RegisterSpec for Doep1intenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doep1inten::R`](R) reader structure"]
impl crate::Readable for Doep1intenSpec {}
#[doc = "`write(|w| ..)` method takes [`doep1inten::W`](W) writer structure"]
impl crate::Writable for Doep1intenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEP1INTEN to value 0"]
impl crate::Resettable for Doep1intenSpec {}
