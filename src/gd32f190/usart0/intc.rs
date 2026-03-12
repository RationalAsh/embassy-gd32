#[doc = "Register `INTC` writer"]
pub type W = crate::W<IntcSpec>;
#[doc = "Parity error clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pec {
    #[doc = "1: Clears the PERR bit in the STAT register"]
    Clear = 1,
}
impl From<Pec> for bool {
    #[inline(always)]
    fn from(variant: Pec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEC` writer - Parity error clear flag"]
pub type PecW<'a, REG> = crate::BitWriter<'a, REG, Pec>;
impl<'a, REG> PecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the PERR bit in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pec::Clear)
    }
}
#[doc = "Framing error clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fec {
    #[doc = "1: Clears the FERR bit in the STAT register"]
    Clear = 1,
}
impl From<Fec> for bool {
    #[inline(always)]
    fn from(variant: Fec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEC` writer - Framing error clear flag"]
pub type FecW<'a, REG> = crate::BitWriter<'a, REG, Fec>;
impl<'a, REG> FecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the FERR bit in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Fec::Clear)
    }
}
#[doc = "Noise detected clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nec {
    #[doc = "1: Clears the NERR bit in the STAT register"]
    Clear = 1,
}
impl From<Nec> for bool {
    #[inline(always)]
    fn from(variant: Nec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NEC` writer - Noise detected clear"]
pub type NecW<'a, REG> = crate::BitWriter<'a, REG, Nec>;
impl<'a, REG> NecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the NERR bit in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Nec::Clear)
    }
}
#[doc = "Overrun error clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orec {
    #[doc = "1: Clears the ORERR bit in the STAT register"]
    Clear = 1,
}
impl From<Orec> for bool {
    #[inline(always)]
    fn from(variant: Orec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OREC` writer - Overrun error clear flag"]
pub type OrecW<'a, REG> = crate::BitWriter<'a, REG, Orec>;
impl<'a, REG> OrecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ORERR bit in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Orec::Clear)
    }
}
#[doc = "Idle line detected clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idlec {
    #[doc = "1: Clears the IDLEF flag in the STAT register"]
    Clear = 1,
}
impl From<Idlec> for bool {
    #[inline(always)]
    fn from(variant: Idlec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLEC` writer - Idle line detected clear"]
pub type IdlecW<'a, REG> = crate::BitWriter<'a, REG, Idlec>;
impl<'a, REG> IdlecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the IDLEF flag in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Idlec::Clear)
    }
}
#[doc = "Transmission complete clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcc {
    #[doc = "1: Clears the TC bit in the STAT register"]
    Clear = 1,
}
impl From<Tcc> for bool {
    #[inline(always)]
    fn from(variant: Tcc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCC` writer - Transmission complete clear flag"]
pub type TccW<'a, REG> = crate::BitWriter<'a, REG, Tcc>;
impl<'a, REG> TccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the TC bit in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Tcc::Clear)
    }
}
#[doc = "LIN break detected clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbdc {
    #[doc = "1: Clears the LBDF flag in the STAT register"]
    Clear = 1,
}
impl From<Lbdc> for bool {
    #[inline(always)]
    fn from(variant: Lbdc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBDC` writer - LIN break detected clear"]
pub type LbdcW<'a, REG> = crate::BitWriter<'a, REG, Lbdc>;
impl<'a, REG> LbdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the LBDF flag in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Lbdc::Clear)
    }
}
#[doc = "CTS change clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsc {
    #[doc = "1: Clears the CTSF flag in the STAT register"]
    Clear = 1,
}
impl From<Ctsc> for bool {
    #[inline(always)]
    fn from(variant: Ctsc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSC` writer - CTS change clear"]
pub type CtscW<'a, REG> = crate::BitWriter<'a, REG, Ctsc>;
impl<'a, REG> CtscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the CTSF flag in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsc::Clear)
    }
}
#[doc = "Receiver timeout clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtc {
    #[doc = "1: Clears the RCF flag in the STAT register"]
    Clear = 1,
}
impl From<Rtc> for bool {
    #[inline(always)]
    fn from(variant: Rtc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC` writer - Receiver timeout clear flag"]
pub type RtcW<'a, REG> = crate::BitWriter<'a, REG, Rtc>;
impl<'a, REG> RtcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the RCF flag in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Clear)
    }
}
#[doc = "End of timeout clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ebc {
    #[doc = "1: Clears the EBF flag in the STAT register"]
    Clear = 1,
}
impl From<Ebc> for bool {
    #[inline(always)]
    fn from(variant: Ebc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBC` writer - End of timeout clear"]
pub type EbcW<'a, REG> = crate::BitWriter<'a, REG, Ebc>;
impl<'a, REG> EbcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the EBF flag in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ebc::Clear)
    }
}
#[doc = "ADDR match clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Amc {
    #[doc = "1: Clears the AMF flag in the STAT register"]
    Clear = 1,
}
impl From<Amc> for bool {
    #[inline(always)]
    fn from(variant: Amc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMC` writer - ADDR match clear"]
pub type AmcW<'a, REG> = crate::BitWriter<'a, REG, Amc>;
impl<'a, REG> AmcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the AMF flag in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Amc::Clear)
    }
}
#[doc = "Wakeup from Deep-sleep mode clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuc {
    #[doc = "1: Clears the WUF flag in the STAT register"]
    Clear = 1,
}
impl From<Wuc> for bool {
    #[inline(always)]
    fn from(variant: Wuc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUC` writer - Wakeup from Deep-sleep mode clear"]
pub type WucW<'a, REG> = crate::BitWriter<'a, REG, Wuc>;
impl<'a, REG> WucW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the WUF flag in the STAT register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Wuc::Clear)
    }
}
impl W {
    #[doc = "Bit 0 - Parity error clear flag"]
    #[inline(always)]
    pub fn pec(&mut self) -> PecW<'_, IntcSpec> {
        PecW::new(self, 0)
    }
    #[doc = "Bit 1 - Framing error clear flag"]
    #[inline(always)]
    pub fn fec(&mut self) -> FecW<'_, IntcSpec> {
        FecW::new(self, 1)
    }
    #[doc = "Bit 2 - Noise detected clear"]
    #[inline(always)]
    pub fn nec(&mut self) -> NecW<'_, IntcSpec> {
        NecW::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun error clear flag"]
    #[inline(always)]
    pub fn orec(&mut self) -> OrecW<'_, IntcSpec> {
        OrecW::new(self, 3)
    }
    #[doc = "Bit 4 - Idle line detected clear"]
    #[inline(always)]
    pub fn idlec(&mut self) -> IdlecW<'_, IntcSpec> {
        IdlecW::new(self, 4)
    }
    #[doc = "Bit 6 - Transmission complete clear flag"]
    #[inline(always)]
    pub fn tcc(&mut self) -> TccW<'_, IntcSpec> {
        TccW::new(self, 6)
    }
    #[doc = "Bit 8 - LIN break detected clear"]
    #[inline(always)]
    pub fn lbdc(&mut self) -> LbdcW<'_, IntcSpec> {
        LbdcW::new(self, 8)
    }
    #[doc = "Bit 9 - CTS change clear"]
    #[inline(always)]
    pub fn ctsc(&mut self) -> CtscW<'_, IntcSpec> {
        CtscW::new(self, 9)
    }
    #[doc = "Bit 11 - Receiver timeout clear flag"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RtcW<'_, IntcSpec> {
        RtcW::new(self, 11)
    }
    #[doc = "Bit 12 - End of timeout clear"]
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
#[doc = "Interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
