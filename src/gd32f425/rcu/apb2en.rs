#[doc = "Register `APB2EN` reader"]
pub type R = crate::R<Apb2enSpec>;
#[doc = "Register `APB2EN` writer"]
pub type W = crate::W<Apb2enSpec>;
#[doc = "TIMER0 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0en {
    #[doc = "0: Disable the selected module clock."]
    Disable = 0,
    #[doc = "1: Enable the selected module clock."]
    Enable = 1,
}
impl From<Timer0en> for bool {
    #[inline(always)]
    fn from(variant: Timer0en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER0EN` reader - TIMER0 clock enable"]
pub type Timer0enR = crate::BitReader<Timer0en>;
impl Timer0enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0en {
        match self.bits {
            false => Timer0en::Disable,
            true => Timer0en::Enable,
        }
    }
    #[doc = "Disable the selected module clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer0en::Disable
    }
    #[doc = "Enable the selected module clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer0en::Enable
    }
}
#[doc = "Field `TIMER0EN` writer - TIMER0 clock enable"]
pub type Timer0enW<'a, REG> = crate::BitWriter<'a, REG, Timer0en>;
impl<'a, REG> Timer0enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the selected module clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0en::Disable)
    }
    #[doc = "Enable the selected module clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0en::Enable)
    }
}
#[doc = "Field `TIMER7EN` reader - TIMER7 clock enable"]
pub use Timer0enR as Timer7enR;
#[doc = "Field `USART0EN` reader - USART0 clock enable"]
pub use Timer0enR as Usart0enR;
#[doc = "Field `USART5EN` reader - USART5 clock enable"]
pub use Timer0enR as Usart5enR;
#[doc = "Field `ADC0EN` reader - ADC0 clock enable"]
pub use Timer0enR as Adc0enR;
#[doc = "Field `ADC1EN` reader - ADC1 clock enable"]
pub use Timer0enR as Adc1enR;
#[doc = "Field `ADC2EN` reader - ADC2 clock enable"]
pub use Timer0enR as Adc2enR;
#[doc = "Field `SDIOEN` reader - SDIO clock enable"]
pub use Timer0enR as SdioenR;
#[doc = "Field `SPI0EN` reader - SPI0 clock enable"]
pub use Timer0enR as Spi0enR;
#[doc = "Field `SPI3EN` reader - SPI3 clock enable"]
pub use Timer0enR as Spi3enR;
#[doc = "Field `SYSCFGEN` reader - SYSCFG clock enable"]
pub use Timer0enR as SyscfgenR;
#[doc = "Field `TIMER8EN` reader - TIMER8 clock enable"]
pub use Timer0enR as Timer8enR;
#[doc = "Field `TIMER9EN` reader - TIMER9 clock enable"]
pub use Timer0enR as Timer9enR;
#[doc = "Field `TIMER10EN` reader - TIMER10 clock enable"]
pub use Timer0enR as Timer10enR;
#[doc = "Field `SPI4EN` reader - SPI4 clock enable"]
pub use Timer0enR as Spi4enR;
#[doc = "Field `SPI5EN` reader - SPI5 clock enable"]
pub use Timer0enR as Spi5enR;
#[doc = "Field `TLIEN` reader - TLI clock enable"]
pub use Timer0enR as TlienR;
#[doc = "Field `TIMER7EN` writer - TIMER7 clock enable"]
pub use Timer0enW as Timer7enW;
#[doc = "Field `USART0EN` writer - USART0 clock enable"]
pub use Timer0enW as Usart0enW;
#[doc = "Field `USART5EN` writer - USART5 clock enable"]
pub use Timer0enW as Usart5enW;
#[doc = "Field `ADC0EN` writer - ADC0 clock enable"]
pub use Timer0enW as Adc0enW;
#[doc = "Field `ADC1EN` writer - ADC1 clock enable"]
pub use Timer0enW as Adc1enW;
#[doc = "Field `ADC2EN` writer - ADC2 clock enable"]
pub use Timer0enW as Adc2enW;
#[doc = "Field `SDIOEN` writer - SDIO clock enable"]
pub use Timer0enW as SdioenW;
#[doc = "Field `SPI0EN` writer - SPI0 clock enable"]
pub use Timer0enW as Spi0enW;
#[doc = "Field `SPI3EN` writer - SPI3 clock enable"]
pub use Timer0enW as Spi3enW;
#[doc = "Field `SYSCFGEN` writer - SYSCFG clock enable"]
pub use Timer0enW as SyscfgenW;
#[doc = "Field `TIMER8EN` writer - TIMER8 clock enable"]
pub use Timer0enW as Timer8enW;
#[doc = "Field `TIMER9EN` writer - TIMER9 clock enable"]
pub use Timer0enW as Timer9enW;
#[doc = "Field `TIMER10EN` writer - TIMER10 clock enable"]
pub use Timer0enW as Timer10enW;
#[doc = "Field `SPI4EN` writer - SPI4 clock enable"]
pub use Timer0enW as Spi4enW;
#[doc = "Field `SPI5EN` writer - SPI5 clock enable"]
pub use Timer0enW as Spi5enW;
#[doc = "Field `TLIEN` writer - TLI clock enable"]
pub use Timer0enW as TlienW;
impl R {
    #[doc = "Bit 0 - TIMER0 clock enable"]
    #[inline(always)]
    pub fn timer0en(&self) -> Timer0enR {
        Timer0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIMER7 clock enable"]
    #[inline(always)]
    pub fn timer7en(&self) -> Timer7enR {
        Timer7enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART0 clock enable"]
    #[inline(always)]
    pub fn usart0en(&self) -> Usart0enR {
        Usart0enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART5 clock enable"]
    #[inline(always)]
    pub fn usart5en(&self) -> Usart5enR {
        Usart5enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC0 clock enable"]
    #[inline(always)]
    pub fn adc0en(&self) -> Adc0enR {
        Adc0enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC1 clock enable"]
    #[inline(always)]
    pub fn adc1en(&self) -> Adc1enR {
        Adc1enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC2 clock enable"]
    #[inline(always)]
    pub fn adc2en(&self) -> Adc2enR {
        Adc2enR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SDIO clock enable"]
    #[inline(always)]
    pub fn sdioen(&self) -> SdioenR {
        SdioenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI0 clock enable"]
    #[inline(always)]
    pub fn spi0en(&self) -> Spi0enR {
        Spi0enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI3 clock enable"]
    #[inline(always)]
    pub fn spi3en(&self) -> Spi3enR {
        Spi3enR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SYSCFG clock enable"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SyscfgenR {
        SyscfgenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIMER8 clock enable"]
    #[inline(always)]
    pub fn timer8en(&self) -> Timer8enR {
        Timer8enR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIMER9 clock enable"]
    #[inline(always)]
    pub fn timer9en(&self) -> Timer9enR {
        Timer9enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIMER10 clock enable"]
    #[inline(always)]
    pub fn timer10en(&self) -> Timer10enR {
        Timer10enR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI4 clock enable"]
    #[inline(always)]
    pub fn spi4en(&self) -> Spi4enR {
        Spi4enR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SPI5 clock enable"]
    #[inline(always)]
    pub fn spi5en(&self) -> Spi5enR {
        Spi5enR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - TLI clock enable"]
    #[inline(always)]
    pub fn tlien(&self) -> TlienR {
        TlienR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIMER0 clock enable"]
    #[inline(always)]
    pub fn timer0en(&mut self) -> Timer0enW<'_, Apb2enSpec> {
        Timer0enW::new(self, 0)
    }
    #[doc = "Bit 1 - TIMER7 clock enable"]
    #[inline(always)]
    pub fn timer7en(&mut self) -> Timer7enW<'_, Apb2enSpec> {
        Timer7enW::new(self, 1)
    }
    #[doc = "Bit 4 - USART0 clock enable"]
    #[inline(always)]
    pub fn usart0en(&mut self) -> Usart0enW<'_, Apb2enSpec> {
        Usart0enW::new(self, 4)
    }
    #[doc = "Bit 5 - USART5 clock enable"]
    #[inline(always)]
    pub fn usart5en(&mut self) -> Usart5enW<'_, Apb2enSpec> {
        Usart5enW::new(self, 5)
    }
    #[doc = "Bit 8 - ADC0 clock enable"]
    #[inline(always)]
    pub fn adc0en(&mut self) -> Adc0enW<'_, Apb2enSpec> {
        Adc0enW::new(self, 8)
    }
    #[doc = "Bit 9 - ADC1 clock enable"]
    #[inline(always)]
    pub fn adc1en(&mut self) -> Adc1enW<'_, Apb2enSpec> {
        Adc1enW::new(self, 9)
    }
    #[doc = "Bit 10 - ADC2 clock enable"]
    #[inline(always)]
    pub fn adc2en(&mut self) -> Adc2enW<'_, Apb2enSpec> {
        Adc2enW::new(self, 10)
    }
    #[doc = "Bit 11 - SDIO clock enable"]
    #[inline(always)]
    pub fn sdioen(&mut self) -> SdioenW<'_, Apb2enSpec> {
        SdioenW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI0 clock enable"]
    #[inline(always)]
    pub fn spi0en(&mut self) -> Spi0enW<'_, Apb2enSpec> {
        Spi0enW::new(self, 12)
    }
    #[doc = "Bit 13 - SPI3 clock enable"]
    #[inline(always)]
    pub fn spi3en(&mut self) -> Spi3enW<'_, Apb2enSpec> {
        Spi3enW::new(self, 13)
    }
    #[doc = "Bit 14 - SYSCFG clock enable"]
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SyscfgenW<'_, Apb2enSpec> {
        SyscfgenW::new(self, 14)
    }
    #[doc = "Bit 16 - TIMER8 clock enable"]
    #[inline(always)]
    pub fn timer8en(&mut self) -> Timer8enW<'_, Apb2enSpec> {
        Timer8enW::new(self, 16)
    }
    #[doc = "Bit 17 - TIMER9 clock enable"]
    #[inline(always)]
    pub fn timer9en(&mut self) -> Timer9enW<'_, Apb2enSpec> {
        Timer9enW::new(self, 17)
    }
    #[doc = "Bit 18 - TIMER10 clock enable"]
    #[inline(always)]
    pub fn timer10en(&mut self) -> Timer10enW<'_, Apb2enSpec> {
        Timer10enW::new(self, 18)
    }
    #[doc = "Bit 20 - SPI4 clock enable"]
    #[inline(always)]
    pub fn spi4en(&mut self) -> Spi4enW<'_, Apb2enSpec> {
        Spi4enW::new(self, 20)
    }
    #[doc = "Bit 21 - SPI5 clock enable"]
    #[inline(always)]
    pub fn spi5en(&mut self) -> Spi5enW<'_, Apb2enSpec> {
        Spi5enW::new(self, 21)
    }
    #[doc = "Bit 26 - TLI clock enable"]
    #[inline(always)]
    pub fn tlien(&mut self) -> TlienW<'_, Apb2enSpec> {
        TlienW::new(self, 26)
    }
}
#[doc = "APB2 clock enable register (RCU_APB2EN)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2enSpec;
impl crate::RegisterSpec for Apb2enSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2en::R`](R) reader structure"]
impl crate::Readable for Apb2enSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2en::W`](W) writer structure"]
impl crate::Writable for Apb2enSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2EN to value 0"]
impl crate::Resettable for Apb2enSpec {}
