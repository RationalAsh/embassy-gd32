#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `DMAREN` reader - Rx buffer DMA enable"]
pub type DmarenR = crate::BitReader;
#[doc = "Field `DMAREN` writer - Rx buffer DMA enable"]
pub type DmarenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMATEN` reader - Tx buffer DMA enable"]
pub type DmatenR = crate::BitReader;
#[doc = "Field `DMATEN` writer - Tx buffer DMA enable"]
pub type DmatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSSDRV` reader - NSS output enable"]
pub type NssdrvR = crate::BitReader;
#[doc = "Field `NSSDRV` writer - NSS output enable"]
pub type NssdrvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSSP` reader - SPI NSS Pulse Mode Enable"]
pub type NsspR = crate::BitReader;
#[doc = "Field `NSSP` writer - SPI NSS Pulse Mode Enable"]
pub type NsspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMOD` reader - SPI TI Mode Enable"]
pub type TmodR = crate::BitReader;
#[doc = "Field `TMOD` writer - SPI TI Mode Enable"]
pub type TmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBNEIE` reader - Receive Buffer Not Empty Interrupt Enable"]
pub type RbneieR = crate::BitReader;
#[doc = "Field `RBNEIE` writer - Receive Buffer Not Empty Interrupt Enable"]
pub type RbneieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBEIE` reader - Transmit Buffer Empty Interrupt Enable"]
pub type TbeieR = crate::BitReader;
#[doc = "Field `TBEIE` writer - Transmit Buffer Empty Interrupt Enable"]
pub type TbeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DZ` reader - Date size"]
pub type DzR = crate::FieldReader;
#[doc = "Field `DZ` writer - Date size"]
pub type DzW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BYTEN` reader - Byte access enable"]
pub type BytenR = crate::BitReader;
#[doc = "Field `BYTEN` writer - Byte access enable"]
pub type BytenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMA_ODD` reader - Odd bytes in RX DMA channel"]
pub type RxdmaOddR = crate::BitReader;
#[doc = "Field `RXDMA_ODD` writer - Odd bytes in RX DMA channel"]
pub type RxdmaOddW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMA_ODD` reader - Odd bytes in TX DMA channel"]
pub type TxdmaOddR = crate::BitReader;
#[doc = "Field `TXDMA_ODD` writer - Odd bytes in TX DMA channel"]
pub type TxdmaOddW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn dmaren(&self) -> DmarenR {
        DmarenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    pub fn dmaten(&self) -> DmatenR {
        DmatenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NSS output enable"]
    #[inline(always)]
    pub fn nssdrv(&self) -> NssdrvR {
        NssdrvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SPI NSS Pulse Mode Enable"]
    #[inline(always)]
    pub fn nssp(&self) -> NsspR {
        NsspR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI TI Mode Enable"]
    #[inline(always)]
    pub fn tmod(&self) -> TmodR {
        TmodR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Buffer Not Empty Interrupt Enable"]
    #[inline(always)]
    pub fn rbneie(&self) -> RbneieR {
        RbneieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tbeie(&self) -> TbeieR {
        TbeieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Date size"]
    #[inline(always)]
    pub fn dz(&self) -> DzR {
        DzR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Byte access enable"]
    #[inline(always)]
    pub fn byten(&self) -> BytenR {
        BytenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Odd bytes in RX DMA channel"]
    #[inline(always)]
    pub fn rxdma_odd(&self) -> RxdmaOddR {
        RxdmaOddR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Odd bytes in TX DMA channel"]
    #[inline(always)]
    pub fn txdma_odd(&self) -> TxdmaOddR {
        TxdmaOddR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn dmaren(&mut self) -> DmarenW<'_, Ctl1Spec> {
        DmarenW::new(self, 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    pub fn dmaten(&mut self) -> DmatenW<'_, Ctl1Spec> {
        DmatenW::new(self, 1)
    }
    #[doc = "Bit 2 - NSS output enable"]
    #[inline(always)]
    pub fn nssdrv(&mut self) -> NssdrvW<'_, Ctl1Spec> {
        NssdrvW::new(self, 2)
    }
    #[doc = "Bit 3 - SPI NSS Pulse Mode Enable"]
    #[inline(always)]
    pub fn nssp(&mut self) -> NsspW<'_, Ctl1Spec> {
        NsspW::new(self, 3)
    }
    #[doc = "Bit 4 - SPI TI Mode Enable"]
    #[inline(always)]
    pub fn tmod(&mut self) -> TmodW<'_, Ctl1Spec> {
        TmodW::new(self, 4)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ErrieW<'_, Ctl1Spec> {
        ErrieW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive Buffer Not Empty Interrupt Enable"]
    #[inline(always)]
    pub fn rbneie(&mut self) -> RbneieW<'_, Ctl1Spec> {
        RbneieW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tbeie(&mut self) -> TbeieW<'_, Ctl1Spec> {
        TbeieW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Date size"]
    #[inline(always)]
    pub fn dz(&mut self) -> DzW<'_, Ctl1Spec> {
        DzW::new(self, 8)
    }
    #[doc = "Bit 12 - Byte access enable"]
    #[inline(always)]
    pub fn byten(&mut self) -> BytenW<'_, Ctl1Spec> {
        BytenW::new(self, 12)
    }
    #[doc = "Bit 13 - Odd bytes in RX DMA channel"]
    #[inline(always)]
    pub fn rxdma_odd(&mut self) -> RxdmaOddW<'_, Ctl1Spec> {
        RxdmaOddW::new(self, 13)
    }
    #[doc = "Bit 14 - Odd bytes in TX DMA channel"]
    #[inline(always)]
    pub fn txdma_odd(&mut self) -> TxdmaOddW<'_, Ctl1Spec> {
        TxdmaOddW::new(self, 14)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {}
