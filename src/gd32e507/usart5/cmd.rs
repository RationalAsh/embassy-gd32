#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `ABDCMD` writer - Auto baudrate detection command"]
pub type AbdcmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBKCMD` writer - Send break command"]
pub type SbkcmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMCMD` writer - Mute mode command"]
pub type MmcmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFCMD` writer - Receive data flush command"]
pub type RxfcmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFCMD` writer - Transmit data flush request"]
pub type TxfcmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Auto baudrate detection command"]
    #[inline(always)]
    pub fn abdcmd(&mut self) -> AbdcmdW<'_, CmdSpec> {
        AbdcmdW::new(self, 0)
    }
    #[doc = "Bit 1 - Send break command"]
    #[inline(always)]
    pub fn sbkcmd(&mut self) -> SbkcmdW<'_, CmdSpec> {
        SbkcmdW::new(self, 1)
    }
    #[doc = "Bit 2 - Mute mode command"]
    #[inline(always)]
    pub fn mmcmd(&mut self) -> MmcmdW<'_, CmdSpec> {
        MmcmdW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive data flush command"]
    #[inline(always)]
    pub fn rxfcmd(&mut self) -> RxfcmdW<'_, CmdSpec> {
        RxfcmdW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit data flush request"]
    #[inline(always)]
    pub fn txfcmd(&mut self) -> TxfcmdW<'_, CmdSpec> {
        TxfcmdW::new(self, 4)
    }
}
#[doc = "Command register (USART_CMD)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {}
