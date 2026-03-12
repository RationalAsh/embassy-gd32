#[doc = "Register `DMA_CTL` reader"]
pub type R = crate::R<DmaCtlSpec>;
#[doc = "Register `DMA_CTL` writer"]
pub type W = crate::W<DmaCtlSpec>;
#[doc = "Field `SRE` reader - Start/stop receive enable"]
pub type SreR = crate::BitReader;
#[doc = "Field `SRE` writer - Start/stop receive enable"]
pub type SreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSF` reader - Operate on second frame"]
pub type OsfR = crate::BitReader;
#[doc = "Field `OSF` writer - Operate on second frame"]
pub type OsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTHC` reader - Receive threshold control"]
pub type RthcR = crate::FieldReader;
#[doc = "Field `RTHC` writer - Receive threshold control"]
pub type RthcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FUF` reader - Forward undersized good frames"]
pub type FufR = crate::BitReader;
#[doc = "Field `FUF` writer - Forward undersized good frames"]
pub type FufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERF` reader - Forward error frames"]
pub type FerfR = crate::BitReader;
#[doc = "Field `FERF` writer - Forward error frames"]
pub type FerfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STE` reader - Start/stop transmission enable"]
pub type SteR = crate::BitReader;
#[doc = "Field `STE` writer - Start/stop transmission enable"]
pub type SteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTHC` reader - Transmit threshold control"]
pub type TthcR = crate::FieldReader;
#[doc = "Field `TTHC` writer - Transmit threshold control"]
pub type TthcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FTF` reader - Flush transmit FIFO"]
pub type FtfR = crate::BitReader;
#[doc = "Field `FTF` writer - Flush transmit FIFO"]
pub type FtfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSFD` reader - Transmit Store-and-Forward"]
pub type TsfdR = crate::BitReader;
#[doc = "Field `TSFD` writer - Transmit Store-and-Forward"]
pub type TsfdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAFRF` reader - Disable flushing of received frames"]
pub type DafrfR = crate::BitReader;
#[doc = "Field `DAFRF` writer - Disable flushing of received frames"]
pub type DafrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSFD` reader - Receive Store-and-Forward"]
pub type RsfdR = crate::BitReader;
#[doc = "Field `RSFD` writer - Receive Store-and-Forward"]
pub type RsfdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTCERFD` reader - Dropping of TCP/IP checksum error frames disable"]
pub type DtcerfdR = crate::BitReader;
#[doc = "Field `DTCERFD` writer - Dropping of TCP/IP checksum error frames disable"]
pub type DtcerfdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Start/stop receive enable"]
    #[inline(always)]
    pub fn sre(&self) -> SreR {
        SreR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Operate on second frame"]
    #[inline(always)]
    pub fn osf(&self) -> OsfR {
        OsfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Receive threshold control"]
    #[inline(always)]
    pub fn rthc(&self) -> RthcR {
        RthcR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 6 - Forward undersized good frames"]
    #[inline(always)]
    pub fn fuf(&self) -> FufR {
        FufR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Forward error frames"]
    #[inline(always)]
    pub fn ferf(&self) -> FerfR {
        FerfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - Start/stop transmission enable"]
    #[inline(always)]
    pub fn ste(&self) -> SteR {
        SteR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16 - Transmit threshold control"]
    #[inline(always)]
    pub fn tthc(&self) -> TthcR {
        TthcR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 20 - Flush transmit FIFO"]
    #[inline(always)]
    pub fn ftf(&self) -> FtfR {
        FtfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit Store-and-Forward"]
    #[inline(always)]
    pub fn tsfd(&self) -> TsfdR {
        TsfdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Disable flushing of received frames"]
    #[inline(always)]
    pub fn dafrf(&self) -> DafrfR {
        DafrfR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Receive Store-and-Forward"]
    #[inline(always)]
    pub fn rsfd(&self) -> RsfdR {
        RsfdR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Dropping of TCP/IP checksum error frames disable"]
    #[inline(always)]
    pub fn dtcerfd(&self) -> DtcerfdR {
        DtcerfdR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Start/stop receive enable"]
    #[inline(always)]
    pub fn sre(&mut self) -> SreW<'_, DmaCtlSpec> {
        SreW::new(self, 1)
    }
    #[doc = "Bit 2 - Operate on second frame"]
    #[inline(always)]
    pub fn osf(&mut self) -> OsfW<'_, DmaCtlSpec> {
        OsfW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Receive threshold control"]
    #[inline(always)]
    pub fn rthc(&mut self) -> RthcW<'_, DmaCtlSpec> {
        RthcW::new(self, 3)
    }
    #[doc = "Bit 6 - Forward undersized good frames"]
    #[inline(always)]
    pub fn fuf(&mut self) -> FufW<'_, DmaCtlSpec> {
        FufW::new(self, 6)
    }
    #[doc = "Bit 7 - Forward error frames"]
    #[inline(always)]
    pub fn ferf(&mut self) -> FerfW<'_, DmaCtlSpec> {
        FerfW::new(self, 7)
    }
    #[doc = "Bit 13 - Start/stop transmission enable"]
    #[inline(always)]
    pub fn ste(&mut self) -> SteW<'_, DmaCtlSpec> {
        SteW::new(self, 13)
    }
    #[doc = "Bits 14:16 - Transmit threshold control"]
    #[inline(always)]
    pub fn tthc(&mut self) -> TthcW<'_, DmaCtlSpec> {
        TthcW::new(self, 14)
    }
    #[doc = "Bit 20 - Flush transmit FIFO"]
    #[inline(always)]
    pub fn ftf(&mut self) -> FtfW<'_, DmaCtlSpec> {
        FtfW::new(self, 20)
    }
    #[doc = "Bit 21 - Transmit Store-and-Forward"]
    #[inline(always)]
    pub fn tsfd(&mut self) -> TsfdW<'_, DmaCtlSpec> {
        TsfdW::new(self, 21)
    }
    #[doc = "Bit 24 - Disable flushing of received frames"]
    #[inline(always)]
    pub fn dafrf(&mut self) -> DafrfW<'_, DmaCtlSpec> {
        DafrfW::new(self, 24)
    }
    #[doc = "Bit 25 - Receive Store-and-Forward"]
    #[inline(always)]
    pub fn rsfd(&mut self) -> RsfdW<'_, DmaCtlSpec> {
        RsfdW::new(self, 25)
    }
    #[doc = "Bit 26 - Dropping of TCP/IP checksum error frames disable"]
    #[inline(always)]
    pub fn dtcerfd(&mut self) -> DtcerfdW<'_, DmaCtlSpec> {
        DtcerfdW::new(self, 26)
    }
}
#[doc = "Ethernet DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCtlSpec;
impl crate::RegisterSpec for DmaCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_ctl::R`](R) reader structure"]
impl crate::Readable for DmaCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_ctl::W`](W) writer structure"]
impl crate::Writable for DmaCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CTL to value 0"]
impl crate::Resettable for DmaCtlSpec {}
