#[doc = "Register `APB1SPEN` reader"]
pub type R = crate::R<Apb1spenSpec>;
#[doc = "Register `APB1SPEN` writer"]
pub type W = crate::W<Apb1spenSpec>;
#[doc = "TIMER1 timer clock enable when sleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer1spen {
    #[doc = "0: Disable the selected module in sleep mode."]
    Disable = 0,
    #[doc = "1: Enable the selected module in sleep mode."]
    Enable = 1,
}
impl From<Timer1spen> for bool {
    #[inline(always)]
    fn from(variant: Timer1spen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER1SPEN` reader - TIMER1 timer clock enable when sleep mode"]
pub type Timer1spenR = crate::BitReader<Timer1spen>;
impl Timer1spenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer1spen {
        match self.bits {
            false => Timer1spen::Disable,
            true => Timer1spen::Enable,
        }
    }
    #[doc = "Disable the selected module in sleep mode."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer1spen::Disable
    }
    #[doc = "Enable the selected module in sleep mode."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer1spen::Enable
    }
}
#[doc = "Field `TIMER1SPEN` writer - TIMER1 timer clock enable when sleep mode"]
pub type Timer1spenW<'a, REG> = crate::BitWriter<'a, REG, Timer1spen>;
impl<'a, REG> Timer1spenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the selected module in sleep mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer1spen::Disable)
    }
    #[doc = "Enable the selected module in sleep mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer1spen::Enable)
    }
}
#[doc = "Field `TIMER2SPEN` reader - TIMER2 timer clock enable when sleep mode"]
pub use Timer1spenR as Timer2spenR;
#[doc = "Field `TIMER3SPEN` reader - TIMER3 timer clock enable when sleep mode"]
pub use Timer1spenR as Timer3spenR;
#[doc = "Field `TIMER4SPEN` reader - TIMER4 timer clock enable when sleep mode"]
pub use Timer1spenR as Timer4spenR;
#[doc = "Field `TIMER5SPEN` reader - TIMER5 timer clock enable when sleep mode"]
pub use Timer1spenR as Timer5spenR;
#[doc = "Field `TIMER6SPEN` reader - TIMER6 timer clock enable when sleep mode"]
pub use Timer1spenR as Timer6spenR;
#[doc = "Field `TIMER11SPEN` reader - TIMER11 timer clock enable when sleep mode"]
pub use Timer1spenR as Timer11spenR;
#[doc = "Field `TIMER12SPEN` reader - TIMER12 timer clock enable when sleep mode"]
pub use Timer1spenR as Timer12spenR;
#[doc = "Field `TIMER13SPEN` reader - TIMER13 timer clock enable when sleep mode"]
pub use Timer1spenR as Timer13spenR;
#[doc = "Field `WWDGTSPEN` reader - Window watchdog timer clock enable when sleep mode"]
pub use Timer1spenR as WwdgtspenR;
#[doc = "Field `SPI1SPEN` reader - SPI1 clock enable when sleep mode"]
pub use Timer1spenR as Spi1spenR;
#[doc = "Field `SPI2SPEN` reader - SPI2 clock enable when sleep mode"]
pub use Timer1spenR as Spi2spenR;
#[doc = "Field `USART1SPEN` reader - USART1 clock enable when sleep mode"]
pub use Timer1spenR as Usart1spenR;
#[doc = "Field `USART2SPEN` reader - USART2 clock enable when sleep mode"]
pub use Timer1spenR as Usart2spenR;
#[doc = "Field `UART3SPEN` reader - UART3 clock enable when sleep mode"]
pub use Timer1spenR as Uart3spenR;
#[doc = "Field `UART4SPEN` reader - UART4 clock enable when sleep mode"]
pub use Timer1spenR as Uart4spenR;
#[doc = "Field `I2C0SPEN` reader - I2C0 clock enable when sleep mode"]
pub use Timer1spenR as I2c0spenR;
#[doc = "Field `I2C1SPEN` reader - I2C1 clock enable when sleep mode"]
pub use Timer1spenR as I2c1spenR;
#[doc = "Field `I2C2SPEN` reader - I2C2 clock enable when sleep mode"]
pub use Timer1spenR as I2c2spenR;
#[doc = "Field `CAN0SPEN` reader - CAN0 clock enable when sleep mode"]
pub use Timer1spenR as Can0spenR;
#[doc = "Field `CAN1SPEN` reader - CAN1 clock enable when sleep mode"]
pub use Timer1spenR as Can1spenR;
#[doc = "Field `PMUSPEN` reader - Power control clock enable when sleep mode"]
pub use Timer1spenR as PmuspenR;
#[doc = "Field `DACSPEN` reader - DAC clock enable when sleep mode"]
pub use Timer1spenR as DacspenR;
#[doc = "Field `UART6SPEN` reader - UART6 clock enable when sleep mode"]
pub use Timer1spenR as Uart6spenR;
#[doc = "Field `UART7SPEN` reader - UART7 clock enable when sleep mode"]
pub use Timer1spenR as Uart7spenR;
#[doc = "Field `TIMER2SPEN` writer - TIMER2 timer clock enable when sleep mode"]
pub use Timer1spenW as Timer2spenW;
#[doc = "Field `TIMER3SPEN` writer - TIMER3 timer clock enable when sleep mode"]
pub use Timer1spenW as Timer3spenW;
#[doc = "Field `TIMER4SPEN` writer - TIMER4 timer clock enable when sleep mode"]
pub use Timer1spenW as Timer4spenW;
#[doc = "Field `TIMER5SPEN` writer - TIMER5 timer clock enable when sleep mode"]
pub use Timer1spenW as Timer5spenW;
#[doc = "Field `TIMER6SPEN` writer - TIMER6 timer clock enable when sleep mode"]
pub use Timer1spenW as Timer6spenW;
#[doc = "Field `TIMER11SPEN` writer - TIMER11 timer clock enable when sleep mode"]
pub use Timer1spenW as Timer11spenW;
#[doc = "Field `TIMER12SPEN` writer - TIMER12 timer clock enable when sleep mode"]
pub use Timer1spenW as Timer12spenW;
#[doc = "Field `TIMER13SPEN` writer - TIMER13 timer clock enable when sleep mode"]
pub use Timer1spenW as Timer13spenW;
#[doc = "Field `WWDGTSPEN` writer - Window watchdog timer clock enable when sleep mode"]
pub use Timer1spenW as WwdgtspenW;
#[doc = "Field `SPI1SPEN` writer - SPI1 clock enable when sleep mode"]
pub use Timer1spenW as Spi1spenW;
#[doc = "Field `SPI2SPEN` writer - SPI2 clock enable when sleep mode"]
pub use Timer1spenW as Spi2spenW;
#[doc = "Field `USART1SPEN` writer - USART1 clock enable when sleep mode"]
pub use Timer1spenW as Usart1spenW;
#[doc = "Field `USART2SPEN` writer - USART2 clock enable when sleep mode"]
pub use Timer1spenW as Usart2spenW;
#[doc = "Field `UART3SPEN` writer - UART3 clock enable when sleep mode"]
pub use Timer1spenW as Uart3spenW;
#[doc = "Field `UART4SPEN` writer - UART4 clock enable when sleep mode"]
pub use Timer1spenW as Uart4spenW;
#[doc = "Field `I2C0SPEN` writer - I2C0 clock enable when sleep mode"]
pub use Timer1spenW as I2c0spenW;
#[doc = "Field `I2C1SPEN` writer - I2C1 clock enable when sleep mode"]
pub use Timer1spenW as I2c1spenW;
#[doc = "Field `I2C2SPEN` writer - I2C2 clock enable when sleep mode"]
pub use Timer1spenW as I2c2spenW;
#[doc = "Field `CAN0SPEN` writer - CAN0 clock enable when sleep mode"]
pub use Timer1spenW as Can0spenW;
#[doc = "Field `CAN1SPEN` writer - CAN1 clock enable when sleep mode"]
pub use Timer1spenW as Can1spenW;
#[doc = "Field `PMUSPEN` writer - Power control clock enable when sleep mode"]
pub use Timer1spenW as PmuspenW;
#[doc = "Field `DACSPEN` writer - DAC clock enable when sleep mode"]
pub use Timer1spenW as DacspenW;
#[doc = "Field `UART6SPEN` writer - UART6 clock enable when sleep mode"]
pub use Timer1spenW as Uart6spenW;
#[doc = "Field `UART7SPEN` writer - UART7 clock enable when sleep mode"]
pub use Timer1spenW as Uart7spenW;
impl R {
    #[doc = "Bit 0 - TIMER1 timer clock enable when sleep mode"]
    #[inline(always)]
    pub fn timer1spen(&self) -> Timer1spenR {
        Timer1spenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIMER2 timer clock enable when sleep mode"]
    #[inline(always)]
    pub fn timer2spen(&self) -> Timer2spenR {
        Timer2spenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIMER3 timer clock enable when sleep mode"]
    #[inline(always)]
    pub fn timer3spen(&self) -> Timer3spenR {
        Timer3spenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIMER4 timer clock enable when sleep mode"]
    #[inline(always)]
    pub fn timer4spen(&self) -> Timer4spenR {
        Timer4spenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIMER5 timer clock enable when sleep mode"]
    #[inline(always)]
    pub fn timer5spen(&self) -> Timer5spenR {
        Timer5spenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIMER6 timer clock enable when sleep mode"]
    #[inline(always)]
    pub fn timer6spen(&self) -> Timer6spenR {
        Timer6spenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIMER11 timer clock enable when sleep mode"]
    #[inline(always)]
    pub fn timer11spen(&self) -> Timer11spenR {
        Timer11spenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIMER12 timer clock enable when sleep mode"]
    #[inline(always)]
    pub fn timer12spen(&self) -> Timer12spenR {
        Timer12spenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIMER13 timer clock enable when sleep mode"]
    #[inline(always)]
    pub fn timer13spen(&self) -> Timer13spenR {
        Timer13spenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog timer clock enable when sleep mode"]
    #[inline(always)]
    pub fn wwdgtspen(&self) -> WwdgtspenR {
        WwdgtspenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI1 clock enable when sleep mode"]
    #[inline(always)]
    pub fn spi1spen(&self) -> Spi1spenR {
        Spi1spenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI2 clock enable when sleep mode"]
    #[inline(always)]
    pub fn spi2spen(&self) -> Spi2spenR {
        Spi2spenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART1 clock enable when sleep mode"]
    #[inline(always)]
    pub fn usart1spen(&self) -> Usart1spenR {
        Usart1spenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART2 clock enable when sleep mode"]
    #[inline(always)]
    pub fn usart2spen(&self) -> Usart2spenR {
        Usart2spenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART3 clock enable when sleep mode"]
    #[inline(always)]
    pub fn uart3spen(&self) -> Uart3spenR {
        Uart3spenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART4 clock enable when sleep mode"]
    #[inline(always)]
    pub fn uart4spen(&self) -> Uart4spenR {
        Uart4spenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C0 clock enable when sleep mode"]
    #[inline(always)]
    pub fn i2c0spen(&self) -> I2c0spenR {
        I2c0spenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C1 clock enable when sleep mode"]
    #[inline(always)]
    pub fn i2c1spen(&self) -> I2c1spenR {
        I2c1spenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C2 clock enable when sleep mode"]
    #[inline(always)]
    pub fn i2c2spen(&self) -> I2c2spenR {
        I2c2spenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN0 clock enable when sleep mode"]
    #[inline(always)]
    pub fn can0spen(&self) -> Can0spenR {
        Can0spenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN1 clock enable when sleep mode"]
    #[inline(always)]
    pub fn can1spen(&self) -> Can1spenR {
        Can1spenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Power control clock enable when sleep mode"]
    #[inline(always)]
    pub fn pmuspen(&self) -> PmuspenR {
        PmuspenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC clock enable when sleep mode"]
    #[inline(always)]
    pub fn dacspen(&self) -> DacspenR {
        DacspenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - UART6 clock enable when sleep mode"]
    #[inline(always)]
    pub fn uart6spen(&self) -> Uart6spenR {
        Uart6spenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UART7 clock enable when sleep mode"]
    #[inline(always)]
    pub fn uart7spen(&self) -> Uart7spenR {
        Uart7spenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIMER1 timer clock enable when sleep mode"]
    #[inline(always)]
    pub fn timer1spen(&mut self) -> Timer1spenW<'_, Apb1spenSpec> {
        Timer1spenW::new(self, 0)
    }
    #[doc = "Bit 1 - TIMER2 timer clock enable when sleep mode"]
    #[inline(always)]
    pub fn timer2spen(&mut self) -> Timer2spenW<'_, Apb1spenSpec> {
        Timer2spenW::new(self, 1)
    }
    #[doc = "Bit 2 - TIMER3 timer clock enable when sleep mode"]
    #[inline(always)]
    pub fn timer3spen(&mut self) -> Timer3spenW<'_, Apb1spenSpec> {
        Timer3spenW::new(self, 2)
    }
    #[doc = "Bit 3 - TIMER4 timer clock enable when sleep mode"]
    #[inline(always)]
    pub fn timer4spen(&mut self) -> Timer4spenW<'_, Apb1spenSpec> {
        Timer4spenW::new(self, 3)
    }
    #[doc = "Bit 4 - TIMER5 timer clock enable when sleep mode"]
    #[inline(always)]
    pub fn timer5spen(&mut self) -> Timer5spenW<'_, Apb1spenSpec> {
        Timer5spenW::new(self, 4)
    }
    #[doc = "Bit 5 - TIMER6 timer clock enable when sleep mode"]
    #[inline(always)]
    pub fn timer6spen(&mut self) -> Timer6spenW<'_, Apb1spenSpec> {
        Timer6spenW::new(self, 5)
    }
    #[doc = "Bit 6 - TIMER11 timer clock enable when sleep mode"]
    #[inline(always)]
    pub fn timer11spen(&mut self) -> Timer11spenW<'_, Apb1spenSpec> {
        Timer11spenW::new(self, 6)
    }
    #[doc = "Bit 7 - TIMER12 timer clock enable when sleep mode"]
    #[inline(always)]
    pub fn timer12spen(&mut self) -> Timer12spenW<'_, Apb1spenSpec> {
        Timer12spenW::new(self, 7)
    }
    #[doc = "Bit 8 - TIMER13 timer clock enable when sleep mode"]
    #[inline(always)]
    pub fn timer13spen(&mut self) -> Timer13spenW<'_, Apb1spenSpec> {
        Timer13spenW::new(self, 8)
    }
    #[doc = "Bit 11 - Window watchdog timer clock enable when sleep mode"]
    #[inline(always)]
    pub fn wwdgtspen(&mut self) -> WwdgtspenW<'_, Apb1spenSpec> {
        WwdgtspenW::new(self, 11)
    }
    #[doc = "Bit 14 - SPI1 clock enable when sleep mode"]
    #[inline(always)]
    pub fn spi1spen(&mut self) -> Spi1spenW<'_, Apb1spenSpec> {
        Spi1spenW::new(self, 14)
    }
    #[doc = "Bit 15 - SPI2 clock enable when sleep mode"]
    #[inline(always)]
    pub fn spi2spen(&mut self) -> Spi2spenW<'_, Apb1spenSpec> {
        Spi2spenW::new(self, 15)
    }
    #[doc = "Bit 17 - USART1 clock enable when sleep mode"]
    #[inline(always)]
    pub fn usart1spen(&mut self) -> Usart1spenW<'_, Apb1spenSpec> {
        Usart1spenW::new(self, 17)
    }
    #[doc = "Bit 18 - USART2 clock enable when sleep mode"]
    #[inline(always)]
    pub fn usart2spen(&mut self) -> Usart2spenW<'_, Apb1spenSpec> {
        Usart2spenW::new(self, 18)
    }
    #[doc = "Bit 19 - UART3 clock enable when sleep mode"]
    #[inline(always)]
    pub fn uart3spen(&mut self) -> Uart3spenW<'_, Apb1spenSpec> {
        Uart3spenW::new(self, 19)
    }
    #[doc = "Bit 20 - UART4 clock enable when sleep mode"]
    #[inline(always)]
    pub fn uart4spen(&mut self) -> Uart4spenW<'_, Apb1spenSpec> {
        Uart4spenW::new(self, 20)
    }
    #[doc = "Bit 21 - I2C0 clock enable when sleep mode"]
    #[inline(always)]
    pub fn i2c0spen(&mut self) -> I2c0spenW<'_, Apb1spenSpec> {
        I2c0spenW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C1 clock enable when sleep mode"]
    #[inline(always)]
    pub fn i2c1spen(&mut self) -> I2c1spenW<'_, Apb1spenSpec> {
        I2c1spenW::new(self, 22)
    }
    #[doc = "Bit 23 - I2C2 clock enable when sleep mode"]
    #[inline(always)]
    pub fn i2c2spen(&mut self) -> I2c2spenW<'_, Apb1spenSpec> {
        I2c2spenW::new(self, 23)
    }
    #[doc = "Bit 25 - CAN0 clock enable when sleep mode"]
    #[inline(always)]
    pub fn can0spen(&mut self) -> Can0spenW<'_, Apb1spenSpec> {
        Can0spenW::new(self, 25)
    }
    #[doc = "Bit 26 - CAN1 clock enable when sleep mode"]
    #[inline(always)]
    pub fn can1spen(&mut self) -> Can1spenW<'_, Apb1spenSpec> {
        Can1spenW::new(self, 26)
    }
    #[doc = "Bit 28 - Power control clock enable when sleep mode"]
    #[inline(always)]
    pub fn pmuspen(&mut self) -> PmuspenW<'_, Apb1spenSpec> {
        PmuspenW::new(self, 28)
    }
    #[doc = "Bit 29 - DAC clock enable when sleep mode"]
    #[inline(always)]
    pub fn dacspen(&mut self) -> DacspenW<'_, Apb1spenSpec> {
        DacspenW::new(self, 29)
    }
    #[doc = "Bit 30 - UART6 clock enable when sleep mode"]
    #[inline(always)]
    pub fn uart6spen(&mut self) -> Uart6spenW<'_, Apb1spenSpec> {
        Uart6spenW::new(self, 30)
    }
    #[doc = "Bit 31 - UART7 clock enable when sleep mode"]
    #[inline(always)]
    pub fn uart7spen(&mut self) -> Uart7spenW<'_, Apb1spenSpec> {
        Uart7spenW::new(self, 31)
    }
}
#[doc = "APB1 sleep mode clock enable register (RCU_APB1EN)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1spen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1spen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1spenSpec;
impl crate::RegisterSpec for Apb1spenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1spen::R`](R) reader structure"]
impl crate::Readable for Apb1spenSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1spen::W`](W) writer structure"]
impl crate::Writable for Apb1spenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1SPEN to value 0xf6fe_c9ff"]
impl crate::Resettable for Apb1spenSpec {
    const RESET_VALUE: u32 = 0xf6fe_c9ff;
}
