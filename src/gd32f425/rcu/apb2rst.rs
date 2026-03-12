#[doc = "Register `APB2RST` reader"]
pub type R = crate::R<Apb2rstSpec>;
#[doc = "Register `APB2RST` writer"]
pub type W = crate::W<Apb2rstSpec>;
#[doc = "TIMER0 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0rst {
    #[doc = "1: Reset the selected module."]
    Reset = 1,
}
impl From<Timer0rst> for bool {
    #[inline(always)]
    fn from(variant: Timer0rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER0RST` reader - TIMER0 reset"]
pub type Timer0rstR = crate::BitReader<Timer0rst>;
impl Timer0rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Timer0rst> {
        match self.bits {
            true => Some(Timer0rst::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Timer0rst::Reset
    }
}
#[doc = "Field `TIMER0RST` writer - TIMER0 reset"]
pub type Timer0rstW<'a, REG> = crate::BitWriter<'a, REG, Timer0rst>;
impl<'a, REG> Timer0rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0rst::Reset)
    }
}
#[doc = "Field `TIMER7RST` reader - TIMER7 reset"]
pub use Timer0rstR as Timer7rstR;
#[doc = "Field `USART0RST` reader - USART0 reset"]
pub use Timer0rstR as Usart0rstR;
#[doc = "Field `USART5RST` reader - USART5 reset"]
pub use Timer0rstR as Usart5rstR;
#[doc = "Field `ADCRST` reader - ADC reset"]
pub use Timer0rstR as AdcrstR;
#[doc = "Field `SDIORST` reader - SDIO reset"]
pub use Timer0rstR as SdiorstR;
#[doc = "Field `SPI0RST` reader - SPI0 Reset"]
pub use Timer0rstR as Spi0rstR;
#[doc = "Field `SPI3RST` reader - SPI3 Reset"]
pub use Timer0rstR as Spi3rstR;
#[doc = "Field `SYSCFGRST` reader - SYSCFG Reset"]
pub use Timer0rstR as SyscfgrstR;
#[doc = "Field `TIMER8RST` reader - TIMER8 reset"]
pub use Timer0rstR as Timer8rstR;
#[doc = "Field `TIMER9RST` reader - TIMER9 reset"]
pub use Timer0rstR as Timer9rstR;
#[doc = "Field `TIMER10RST` reader - TIMER10 reset"]
pub use Timer0rstR as Timer10rstR;
#[doc = "Field `SPI4RST` reader - SPI4 Reset"]
pub use Timer0rstR as Spi4rstR;
#[doc = "Field `SPI5RST` reader - SPI5 Reset"]
pub use Timer0rstR as Spi5rstR;
#[doc = "Field `TLIRST` reader - TLI Reset"]
pub use Timer0rstR as TlirstR;
#[doc = "Field `TIMER7RST` writer - TIMER7 reset"]
pub use Timer0rstW as Timer7rstW;
#[doc = "Field `USART0RST` writer - USART0 reset"]
pub use Timer0rstW as Usart0rstW;
#[doc = "Field `USART5RST` writer - USART5 reset"]
pub use Timer0rstW as Usart5rstW;
#[doc = "Field `ADCRST` writer - ADC reset"]
pub use Timer0rstW as AdcrstW;
#[doc = "Field `SDIORST` writer - SDIO reset"]
pub use Timer0rstW as SdiorstW;
#[doc = "Field `SPI0RST` writer - SPI0 Reset"]
pub use Timer0rstW as Spi0rstW;
#[doc = "Field `SPI3RST` writer - SPI3 Reset"]
pub use Timer0rstW as Spi3rstW;
#[doc = "Field `SYSCFGRST` writer - SYSCFG Reset"]
pub use Timer0rstW as SyscfgrstW;
#[doc = "Field `TIMER8RST` writer - TIMER8 reset"]
pub use Timer0rstW as Timer8rstW;
#[doc = "Field `TIMER9RST` writer - TIMER9 reset"]
pub use Timer0rstW as Timer9rstW;
#[doc = "Field `TIMER10RST` writer - TIMER10 reset"]
pub use Timer0rstW as Timer10rstW;
#[doc = "Field `SPI4RST` writer - SPI4 Reset"]
pub use Timer0rstW as Spi4rstW;
#[doc = "Field `SPI5RST` writer - SPI5 Reset"]
pub use Timer0rstW as Spi5rstW;
#[doc = "Field `TLIRST` writer - TLI Reset"]
pub use Timer0rstW as TlirstW;
impl R {
    #[doc = "Bit 0 - TIMER0 reset"]
    #[inline(always)]
    pub fn timer0rst(&self) -> Timer0rstR {
        Timer0rstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIMER7 reset"]
    #[inline(always)]
    pub fn timer7rst(&self) -> Timer7rstR {
        Timer7rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART0 reset"]
    #[inline(always)]
    pub fn usart0rst(&self) -> Usart0rstR {
        Usart0rstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART5 reset"]
    #[inline(always)]
    pub fn usart5rst(&self) -> Usart5rstR {
        Usart5rstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC reset"]
    #[inline(always)]
    pub fn adcrst(&self) -> AdcrstR {
        AdcrstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - SDIO reset"]
    #[inline(always)]
    pub fn sdiorst(&self) -> SdiorstR {
        SdiorstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI0 Reset"]
    #[inline(always)]
    pub fn spi0rst(&self) -> Spi0rstR {
        Spi0rstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI3 Reset"]
    #[inline(always)]
    pub fn spi3rst(&self) -> Spi3rstR {
        Spi3rstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SYSCFG Reset"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SyscfgrstR {
        SyscfgrstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIMER8 reset"]
    #[inline(always)]
    pub fn timer8rst(&self) -> Timer8rstR {
        Timer8rstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIMER9 reset"]
    #[inline(always)]
    pub fn timer9rst(&self) -> Timer9rstR {
        Timer9rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIMER10 reset"]
    #[inline(always)]
    pub fn timer10rst(&self) -> Timer10rstR {
        Timer10rstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI4 Reset"]
    #[inline(always)]
    pub fn spi4rst(&self) -> Spi4rstR {
        Spi4rstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SPI5 Reset"]
    #[inline(always)]
    pub fn spi5rst(&self) -> Spi5rstR {
        Spi5rstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - TLI Reset"]
    #[inline(always)]
    pub fn tlirst(&self) -> TlirstR {
        TlirstR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIMER0 reset"]
    #[inline(always)]
    pub fn timer0rst(&mut self) -> Timer0rstW<'_, Apb2rstSpec> {
        Timer0rstW::new(self, 0)
    }
    #[doc = "Bit 1 - TIMER7 reset"]
    #[inline(always)]
    pub fn timer7rst(&mut self) -> Timer7rstW<'_, Apb2rstSpec> {
        Timer7rstW::new(self, 1)
    }
    #[doc = "Bit 4 - USART0 reset"]
    #[inline(always)]
    pub fn usart0rst(&mut self) -> Usart0rstW<'_, Apb2rstSpec> {
        Usart0rstW::new(self, 4)
    }
    #[doc = "Bit 5 - USART5 reset"]
    #[inline(always)]
    pub fn usart5rst(&mut self) -> Usart5rstW<'_, Apb2rstSpec> {
        Usart5rstW::new(self, 5)
    }
    #[doc = "Bit 8 - ADC reset"]
    #[inline(always)]
    pub fn adcrst(&mut self) -> AdcrstW<'_, Apb2rstSpec> {
        AdcrstW::new(self, 8)
    }
    #[doc = "Bit 11 - SDIO reset"]
    #[inline(always)]
    pub fn sdiorst(&mut self) -> SdiorstW<'_, Apb2rstSpec> {
        SdiorstW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI0 Reset"]
    #[inline(always)]
    pub fn spi0rst(&mut self) -> Spi0rstW<'_, Apb2rstSpec> {
        Spi0rstW::new(self, 12)
    }
    #[doc = "Bit 13 - SPI3 Reset"]
    #[inline(always)]
    pub fn spi3rst(&mut self) -> Spi3rstW<'_, Apb2rstSpec> {
        Spi3rstW::new(self, 13)
    }
    #[doc = "Bit 14 - SYSCFG Reset"]
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SyscfgrstW<'_, Apb2rstSpec> {
        SyscfgrstW::new(self, 14)
    }
    #[doc = "Bit 16 - TIMER8 reset"]
    #[inline(always)]
    pub fn timer8rst(&mut self) -> Timer8rstW<'_, Apb2rstSpec> {
        Timer8rstW::new(self, 16)
    }
    #[doc = "Bit 17 - TIMER9 reset"]
    #[inline(always)]
    pub fn timer9rst(&mut self) -> Timer9rstW<'_, Apb2rstSpec> {
        Timer9rstW::new(self, 17)
    }
    #[doc = "Bit 18 - TIMER10 reset"]
    #[inline(always)]
    pub fn timer10rst(&mut self) -> Timer10rstW<'_, Apb2rstSpec> {
        Timer10rstW::new(self, 18)
    }
    #[doc = "Bit 20 - SPI4 Reset"]
    #[inline(always)]
    pub fn spi4rst(&mut self) -> Spi4rstW<'_, Apb2rstSpec> {
        Spi4rstW::new(self, 20)
    }
    #[doc = "Bit 21 - SPI5 Reset"]
    #[inline(always)]
    pub fn spi5rst(&mut self) -> Spi5rstW<'_, Apb2rstSpec> {
        Spi5rstW::new(self, 21)
    }
    #[doc = "Bit 26 - TLI Reset"]
    #[inline(always)]
    pub fn tlirst(&mut self) -> TlirstW<'_, Apb2rstSpec> {
        TlirstW::new(self, 26)
    }
}
#[doc = "APB2 reset register (RCU_APB2RST)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2rstSpec;
impl crate::RegisterSpec for Apb2rstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rst::R`](R) reader structure"]
impl crate::Readable for Apb2rstSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2rst::W`](W) writer structure"]
impl crate::Writable for Apb2rstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2RST to value 0"]
impl crate::Resettable for Apb2rstSpec {}
