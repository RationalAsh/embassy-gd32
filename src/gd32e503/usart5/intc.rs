#[doc = "Register `INTC` writer"]
pub type W = crate::W<IntcSpec>;
#[doc = "Field `PEC` writer - Parity error clear"]
pub type PecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEC` writer - Frame error flag clear"]
pub type FecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEC` writer - Noise detected clear"]
pub type NecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OREC` writer - Overrun error clear"]
pub type OrecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEC` writer - Idle line detected clear"]
pub type IdlecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC` writer - Transmission complete clear"]
pub type TccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDC` writer - LIN break detected clear"]
pub type LbdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC` writer - Receiver timeout clear"]
pub type RtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EBC` writer - End of block clear"]
pub type EbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMC` writer - ADDR match clear"]
pub type AmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUC` writer - Wakeup from Deep-sleep mode clear"]
pub type WucW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Parity error clear"]
    #[inline(always)]
    pub fn pec(&mut self) -> PecW<'_, IntcSpec> {
        PecW::new(self, 0)
    }
    #[doc = "Bit 1 - Frame error flag clear"]
    #[inline(always)]
    pub fn fec(&mut self) -> FecW<'_, IntcSpec> {
        FecW::new(self, 1)
    }
    #[doc = "Bit 2 - Noise detected clear"]
    #[inline(always)]
    pub fn nec(&mut self) -> NecW<'_, IntcSpec> {
        NecW::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun error clear"]
    #[inline(always)]
    pub fn orec(&mut self) -> OrecW<'_, IntcSpec> {
        OrecW::new(self, 3)
    }
    #[doc = "Bit 4 - Idle line detected clear"]
    #[inline(always)]
    pub fn idlec(&mut self) -> IdlecW<'_, IntcSpec> {
        IdlecW::new(self, 4)
    }
    #[doc = "Bit 6 - Transmission complete clear"]
    #[inline(always)]
    pub fn tcc(&mut self) -> TccW<'_, IntcSpec> {
        TccW::new(self, 6)
    }
    #[doc = "Bit 8 - LIN break detected clear"]
    #[inline(always)]
    pub fn lbdc(&mut self) -> LbdcW<'_, IntcSpec> {
        LbdcW::new(self, 8)
    }
    #[doc = "Bit 11 - Receiver timeout clear"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RtcW<'_, IntcSpec> {
        RtcW::new(self, 11)
    }
    #[doc = "Bit 12 - End of block clear"]
    #[inline(always)]
    pub fn ebc(&mut self) -> EbcW<'_, IntcSpec> {
        EbcW::new(self, 12)
    }
    #[doc = "Bit 17 - ADDR match clear"]
    #[inline(always)]
    pub fn amc(&mut self) -> AmcW<'_, IntcSpec> {
        AmcW::new(self, 17)
    }
    #[doc = "Bit 20 - Wakeup from Deep-sleep mode clear"]
    #[inline(always)]
    pub fn wuc(&mut self) -> WucW<'_, IntcSpec> {
        WucW::new(self, 20)
    }
}
#[doc = "Interrupt status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcSpec;
impl crate::RegisterSpec for IntcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intc::W`](W) writer structure"]
impl crate::Writable for IntcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTC to value 0"]
impl crate::Resettable for IntcSpec {}
