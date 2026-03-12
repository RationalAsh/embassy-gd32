#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Field `SBKCMD` reader - Send break command"]
pub type SbkcmdR = crate::BitReader;
#[doc = "Field `SBKCMD` writer - Send break command"]
pub type SbkcmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWU` reader - Receiver wakeup from mute mode"]
pub type RwuR = crate::BitReader;
#[doc = "Field `RWU` writer - Receiver wakeup from mute mode"]
pub type RwuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN` reader - Receiver enable"]
pub type RenR = crate::BitReader;
#[doc = "Field `REN` writer - Receiver enable"]
pub type RenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEN` reader - Transmitter enable"]
pub type TenR = crate::BitReader;
#[doc = "Field `TEN` writer - Transmitter enable"]
pub type TenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEIE` reader - IDLE line detected interrupt enable"]
pub type IdleieR = crate::BitReader;
#[doc = "Field `IDLEIE` writer - IDLE line detected interrupt enable"]
pub type IdleieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBNEIE` reader - Read data buffer not empty interrupt and overrun error interrupt enable"]
pub type RbneieR = crate::BitReader;
#[doc = "Field `RBNEIE` writer - Read data buffer not empty interrupt and overrun error interrupt enable"]
pub type RbneieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - Transmission complete interrupt enable"]
pub type TcieR = crate::BitReader;
#[doc = "Field `TCIE` writer - Transmission complete interrupt enable"]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBEIE` reader - Transmitter buffer empty interrupt enable"]
pub type TbeieR = crate::BitReader;
#[doc = "Field `TBEIE` writer - Transmitter buffer empty interrupt enable"]
pub type TbeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERRIE` reader - Parity error interrupt enable"]
pub type PerrieR = crate::BitReader;
#[doc = "Field `PERRIE` writer - Parity error interrupt enable"]
pub type PerrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PM` reader - Parity mode"]
pub type PmR = crate::BitReader;
#[doc = "Field `PM` writer - Parity mode"]
pub type PmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCEN` reader - Parity check function enable"]
pub type PcenR = crate::BitReader;
#[doc = "Field `PCEN` writer - Parity check function enable"]
pub type PcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WM` reader - Wakeup method in mute mode"]
pub type WmR = crate::BitReader;
#[doc = "Field `WM` writer - Wakeup method in mute mode"]
pub type WmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WL` reader - Word length"]
pub type WlR = crate::BitReader;
#[doc = "Field `WL` writer - Word length"]
pub type WlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UEN` reader - USART enable"]
pub type UenR = crate::BitReader;
#[doc = "Field `UEN` writer - USART enable"]
pub type UenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVSMOD` reader - Oversampling mode"]
pub type OvsmodR = crate::BitReader;
#[doc = "Field `OVSMOD` writer - Oversampling mode"]
pub type OvsmodW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Send break command"]
    #[inline(always)]
    pub fn sbkcmd(&self) -> SbkcmdR {
        SbkcmdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver wakeup from mute mode"]
    #[inline(always)]
    pub fn rwu(&self) -> RwuR {
        RwuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn ren(&self) -> RenR {
        RenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn ten(&self) -> TenR {
        TenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE line detected interrupt enable"]
    #[inline(always)]
    pub fn idleie(&self) -> IdleieR {
        IdleieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read data buffer not empty interrupt and overrun error interrupt enable"]
    #[inline(always)]
    pub fn rbneie(&self) -> RbneieR {
        RbneieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmitter buffer empty interrupt enable"]
    #[inline(always)]
    pub fn tbeie(&self) -> TbeieR {
        TbeieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity error interrupt enable"]
    #[inline(always)]
    pub fn perrie(&self) -> PerrieR {
        PerrieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity mode"]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity check function enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PcenR {
        PcenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wakeup method in mute mode"]
    #[inline(always)]
    pub fn wm(&self) -> WmR {
        WmR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    pub fn wl(&self) -> WlR {
        WlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    pub fn uen(&self) -> UenR {
        UenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Oversampling mode"]
    #[inline(always)]
    pub fn ovsmod(&self) -> OvsmodR {
        OvsmodR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Send break command"]
    #[inline(always)]
    pub fn sbkcmd(&mut self) -> SbkcmdW<'_, Ctl0Spec> {
        SbkcmdW::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver wakeup from mute mode"]
    #[inline(always)]
    pub fn rwu(&mut self) -> RwuW<'_, Ctl0Spec> {
        RwuW::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn ren(&mut self) -> RenW<'_, Ctl0Spec> {
        RenW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn ten(&mut self) -> TenW<'_, Ctl0Spec> {
        TenW::new(self, 3)
    }
    #[doc = "Bit 4 - IDLE line detected interrupt enable"]
    #[inline(always)]
    pub fn idleie(&mut self) -> IdleieW<'_, Ctl0Spec> {
        IdleieW::new(self, 4)
    }
    #[doc = "Bit 5 - Read data buffer not empty interrupt and overrun error interrupt enable"]
    #[inline(always)]
    pub fn rbneie(&mut self) -> RbneieW<'_, Ctl0Spec> {
        RbneieW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<'_, Ctl0Spec> {
        TcieW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmitter buffer empty interrupt enable"]
    #[inline(always)]
    pub fn tbeie(&mut self) -> TbeieW<'_, Ctl0Spec> {
        TbeieW::new(self, 7)
    }
    #[doc = "Bit 8 - Parity error interrupt enable"]
    #[inline(always)]
    pub fn perrie(&mut self) -> PerrieW<'_, Ctl0Spec> {
        PerrieW::new(self, 8)
    }
    #[doc = "Bit 9 - Parity mode"]
    #[inline(always)]
    pub fn pm(&mut self) -> PmW<'_, Ctl0Spec> {
        PmW::new(self, 9)
    }
    #[doc = "Bit 10 - Parity check function enable"]
    #[inline(always)]
    pub fn pcen(&mut self) -> PcenW<'_, Ctl0Spec> {
        PcenW::new(self, 10)
    }
    #[doc = "Bit 11 - Wakeup method in mute mode"]
    #[inline(always)]
    pub fn wm(&mut self) -> WmW<'_, Ctl0Spec> {
        WmW::new(self, 11)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    pub fn wl(&mut self) -> WlW<'_, Ctl0Spec> {
        WlW::new(self, 12)
    }
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    pub fn uen(&mut self) -> UenW<'_, Ctl0Spec> {
        UenW::new(self, 13)
    }
    #[doc = "Bit 15 - Oversampling mode"]
    #[inline(always)]
    pub fn ovsmod(&mut self) -> OvsmodW<'_, Ctl0Spec> {
        OvsmodW::new(self, 15)
    }
}
#[doc = "Control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for Ctl0Spec {}
