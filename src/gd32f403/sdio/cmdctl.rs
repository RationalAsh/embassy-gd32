#[doc = "Register `CMDCTL` reader"]
pub type R = crate::R<CmdctlSpec>;
#[doc = "Register `CMDCTL` writer"]
pub type W = crate::W<CmdctlSpec>;
#[doc = "Field `CMDIDX` reader - Command index"]
pub type CmdidxR = crate::FieldReader;
#[doc = "Field `CMDIDX` writer - Command index"]
pub type CmdidxW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CMDRESP` reader - Command response type bits"]
pub type CmdrespR = crate::FieldReader;
#[doc = "Field `CMDRESP` writer - Command response type bits"]
pub type CmdrespW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INTWAIT` reader - Interrupt wait instead of timeout"]
pub type IntwaitR = crate::BitReader;
#[doc = "Field `INTWAIT` writer - Interrupt wait instead of timeout"]
pub type IntwaitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAITDEND` reader - Waits for ends of data transfer"]
pub type WaitdendR = crate::BitReader;
#[doc = "Field `WAITDEND` writer - Waits for ends of data transfer"]
pub type WaitdendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSMEN` reader - Command state machine (CSM) enable bit"]
pub type CsmenR = crate::BitReader;
#[doc = "Field `CSMEN` writer - Command state machine (CSM) enable bit"]
pub type CsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPEND` reader - SD I/O suspend command(SD I/O only)"]
pub type SuspendR = crate::BitReader;
#[doc = "Field `SUSPEND` writer - SD I/O suspend command(SD I/O only)"]
pub type SuspendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENCMDC` reader - CMD completion signal enabled (CE-ATA only)"]
pub type EncmdcR = crate::BitReader;
#[doc = "Field `ENCMDC` writer - CMD completion signal enabled (CE-ATA only)"]
pub type EncmdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NINTEN` reader - No CE-ATA Interrupt (CE-ATA only)"]
pub type NintenR = crate::BitReader;
#[doc = "Field `NINTEN` writer - No CE-ATA Interrupt (CE-ATA only)"]
pub type NintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATAEN` reader - CE-ATA command enable(CE-ATA only)"]
pub type AtaenR = crate::BitReader;
#[doc = "Field `ATAEN` writer - CE-ATA command enable(CE-ATA only)"]
pub type AtaenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Command index"]
    #[inline(always)]
    pub fn cmdidx(&self) -> CmdidxR {
        CmdidxR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Command response type bits"]
    #[inline(always)]
    pub fn cmdresp(&self) -> CmdrespR {
        CmdrespR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Interrupt wait instead of timeout"]
    #[inline(always)]
    pub fn intwait(&self) -> IntwaitR {
        IntwaitR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Waits for ends of data transfer"]
    #[inline(always)]
    pub fn waitdend(&self) -> WaitdendR {
        WaitdendR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Command state machine (CSM) enable bit"]
    #[inline(always)]
    pub fn csmen(&self) -> CsmenR {
        CsmenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SD I/O suspend command(SD I/O only)"]
    #[inline(always)]
    pub fn suspend(&self) -> SuspendR {
        SuspendR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CMD completion signal enabled (CE-ATA only)"]
    #[inline(always)]
    pub fn encmdc(&self) -> EncmdcR {
        EncmdcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - No CE-ATA Interrupt (CE-ATA only)"]
    #[inline(always)]
    pub fn ninten(&self) -> NintenR {
        NintenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CE-ATA command enable(CE-ATA only)"]
    #[inline(always)]
    pub fn ataen(&self) -> AtaenR {
        AtaenR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Command index"]
    #[inline(always)]
    pub fn cmdidx(&mut self) -> CmdidxW<'_, CmdctlSpec> {
        CmdidxW::new(self, 0)
    }
    #[doc = "Bits 6:7 - Command response type bits"]
    #[inline(always)]
    pub fn cmdresp(&mut self) -> CmdrespW<'_, CmdctlSpec> {
        CmdrespW::new(self, 6)
    }
    #[doc = "Bit 8 - Interrupt wait instead of timeout"]
    #[inline(always)]
    pub fn intwait(&mut self) -> IntwaitW<'_, CmdctlSpec> {
        IntwaitW::new(self, 8)
    }
    #[doc = "Bit 9 - Waits for ends of data transfer"]
    #[inline(always)]
    pub fn waitdend(&mut self) -> WaitdendW<'_, CmdctlSpec> {
        WaitdendW::new(self, 9)
    }
    #[doc = "Bit 10 - Command state machine (CSM) enable bit"]
    #[inline(always)]
    pub fn csmen(&mut self) -> CsmenW<'_, CmdctlSpec> {
        CsmenW::new(self, 10)
    }
    #[doc = "Bit 11 - SD I/O suspend command(SD I/O only)"]
    #[inline(always)]
    pub fn suspend(&mut self) -> SuspendW<'_, CmdctlSpec> {
        SuspendW::new(self, 11)
    }
    #[doc = "Bit 12 - CMD completion signal enabled (CE-ATA only)"]
    #[inline(always)]
    pub fn encmdc(&mut self) -> EncmdcW<'_, CmdctlSpec> {
        EncmdcW::new(self, 12)
    }
    #[doc = "Bit 13 - No CE-ATA Interrupt (CE-ATA only)"]
    #[inline(always)]
    pub fn ninten(&mut self) -> NintenW<'_, CmdctlSpec> {
        NintenW::new(self, 13)
    }
    #[doc = "Bit 14 - CE-ATA command enable(CE-ATA only)"]
    #[inline(always)]
    pub fn ataen(&mut self) -> AtaenW<'_, CmdctlSpec> {
        AtaenW::new(self, 14)
    }
}
#[doc = "Command control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdctlSpec;
impl crate::RegisterSpec for CmdctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdctl::R`](R) reader structure"]
impl crate::Readable for CmdctlSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdctl::W`](W) writer structure"]
impl crate::Writable for CmdctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMDCTL to value 0"]
impl crate::Resettable for CmdctlSpec {}
