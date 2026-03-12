#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Field `I2CEN` reader - I2C peripheral enable"]
pub type I2cenR = crate::BitReader;
#[doc = "Field `I2CEN` writer - I2C peripheral enable"]
pub type I2cenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE` reader - Receive interrupt enable"]
pub type TieR = crate::BitReader;
#[doc = "Field `TIE` writer - Receive interrupt enable"]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBNEIE` reader - Receive interrupt enable"]
pub type RbneieR = crate::BitReader;
#[doc = "Field `RBNEIE` writer - Receive interrupt enable"]
pub type RbneieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDMIE` reader - Address match interrupt enable in slave mode"]
pub type AddmieR = crate::BitReader;
#[doc = "Field `ADDMIE` writer - Address match interrupt enable in slave mode"]
pub type AddmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACKIE` reader - Not acknowledge received interrupt enable"]
pub type NackieR = crate::BitReader;
#[doc = "Field `NACKIE` writer - Not acknowledge received interrupt enable"]
pub type NackieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STPDETIE` reader - Stop detection interrupt enable"]
pub type StpdetieR = crate::BitReader;
#[doc = "Field `STPDETIE` writer - Stop detection interrupt enable"]
pub type StpdetieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable"]
pub type TcieR = crate::BitReader;
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable"]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DNF` reader - Digital noise filter"]
pub type DnfR = crate::FieldReader;
#[doc = "Field `DNF` writer - Digital noise filter"]
pub type DnfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ANOFF` reader - Analog noise filter disable"]
pub type AnoffR = crate::BitReader;
#[doc = "Field `ANOFF` writer - Analog noise filter disable"]
pub type AnoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DENT` reader - DMA enable for transmission"]
pub type DentR = crate::BitReader;
#[doc = "Field `DENT` writer - DMA enable for transmission"]
pub type DentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DENR` reader - DMA enable for reception"]
pub type DenrR = crate::BitReader;
#[doc = "Field `DENR` writer - DMA enable for reception"]
pub type DenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBCTL` reader - Slave byte control"]
pub type SbctlR = crate::BitReader;
#[doc = "Field `SBCTL` writer - Slave byte control"]
pub type SbctlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SS` reader - Whether to stretch SCL low when data is not ready in slave mode"]
pub type SsR = crate::BitReader;
#[doc = "Field `SS` writer - Whether to stretch SCL low when data is not ready in slave mode"]
pub type SsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUEN` reader - Wakeup from Deep-sleep mode enable"]
pub type WuenR = crate::BitReader;
#[doc = "Field `WUEN` writer - Wakeup from Deep-sleep mode enable"]
pub type WuenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCEN` reader - Whether or not to response to a General Call"]
pub type GcenR = crate::BitReader;
#[doc = "Field `GCEN` writer - Whether or not to response to a General Call"]
pub type GcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBHAEN` reader - SMBus Host address enable"]
pub type SmbhaenR = crate::BitReader;
#[doc = "Field `SMBHAEN` writer - SMBus Host address enable"]
pub type SmbhaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBDAEN` reader - SMBus device default address enable"]
pub type SmbdaenR = crate::BitReader;
#[doc = "Field `SMBDAEN` writer - SMBus device default address enable"]
pub type SmbdaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBALTEN` reader - SMBus Alert enable"]
pub type SmbaltenR = crate::BitReader;
#[doc = "Field `SMBALTEN` writer - SMBus Alert enable"]
pub type SmbaltenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECEN` reader - PEC Calculation Switch"]
pub type PecenR = crate::BitReader;
#[doc = "Field `PECEN` writer - PEC Calculation Switch"]
pub type PecenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C peripheral enable"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2cenR {
        I2cenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rbneie(&self) -> RbneieR {
        RbneieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address match interrupt enable in slave mode"]
    #[inline(always)]
    pub fn addmie(&self) -> AddmieR {
        AddmieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Not acknowledge received interrupt enable"]
    #[inline(always)]
    pub fn nackie(&self) -> NackieR {
        NackieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop detection interrupt enable"]
    #[inline(always)]
    pub fn stpdetie(&self) -> StpdetieR {
        StpdetieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Digital noise filter"]
    #[inline(always)]
    pub fn dnf(&self) -> DnfR {
        DnfR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Analog noise filter disable"]
    #[inline(always)]
    pub fn anoff(&self) -> AnoffR {
        AnoffR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - DMA enable for transmission"]
    #[inline(always)]
    pub fn dent(&self) -> DentR {
        DentR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA enable for reception"]
    #[inline(always)]
    pub fn denr(&self) -> DenrR {
        DenrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave byte control"]
    #[inline(always)]
    pub fn sbctl(&self) -> SbctlR {
        SbctlR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Whether to stretch SCL low when data is not ready in slave mode"]
    #[inline(always)]
    pub fn ss(&self) -> SsR {
        SsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Wakeup from Deep-sleep mode enable"]
    #[inline(always)]
    pub fn wuen(&self) -> WuenR {
        WuenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Whether or not to response to a General Call"]
    #[inline(always)]
    pub fn gcen(&self) -> GcenR {
        GcenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SMBus Host address enable"]
    #[inline(always)]
    pub fn smbhaen(&self) -> SmbhaenR {
        SmbhaenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SMBus device default address enable"]
    #[inline(always)]
    pub fn smbdaen(&self) -> SmbdaenR {
        SmbdaenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SMBus Alert enable"]
    #[inline(always)]
    pub fn smbalten(&self) -> SmbaltenR {
        SmbaltenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PEC Calculation Switch"]
    #[inline(always)]
    pub fn pecen(&self) -> PecenR {
        PecenR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C peripheral enable"]
    #[inline(always)]
    pub fn i2cen(&mut self) -> I2cenW<'_, Ctl0Spec> {
        I2cenW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive interrupt enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<'_, Ctl0Spec> {
        TieW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rbneie(&mut self) -> RbneieW<'_, Ctl0Spec> {
        RbneieW::new(self, 2)
    }
    #[doc = "Bit 3 - Address match interrupt enable in slave mode"]
    #[inline(always)]
    pub fn addmie(&mut self) -> AddmieW<'_, Ctl0Spec> {
        AddmieW::new(self, 3)
    }
    #[doc = "Bit 4 - Not acknowledge received interrupt enable"]
    #[inline(always)]
    pub fn nackie(&mut self) -> NackieW<'_, Ctl0Spec> {
        NackieW::new(self, 4)
    }
    #[doc = "Bit 5 - Stop detection interrupt enable"]
    #[inline(always)]
    pub fn stpdetie(&mut self) -> StpdetieW<'_, Ctl0Spec> {
        StpdetieW::new(self, 5)
    }
    #[doc = "Bit 6 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<'_, Ctl0Spec> {
        TcieW::new(self, 6)
    }
    #[doc = "Bit 7 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ErrieW<'_, Ctl0Spec> {
        ErrieW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Digital noise filter"]
    #[inline(always)]
    pub fn dnf(&mut self) -> DnfW<'_, Ctl0Spec> {
        DnfW::new(self, 8)
    }
    #[doc = "Bit 12 - Analog noise filter disable"]
    #[inline(always)]
    pub fn anoff(&mut self) -> AnoffW<'_, Ctl0Spec> {
        AnoffW::new(self, 12)
    }
    #[doc = "Bit 14 - DMA enable for transmission"]
    #[inline(always)]
    pub fn dent(&mut self) -> DentW<'_, Ctl0Spec> {
        DentW::new(self, 14)
    }
    #[doc = "Bit 15 - DMA enable for reception"]
    #[inline(always)]
    pub fn denr(&mut self) -> DenrW<'_, Ctl0Spec> {
        DenrW::new(self, 15)
    }
    #[doc = "Bit 16 - Slave byte control"]
    #[inline(always)]
    pub fn sbctl(&mut self) -> SbctlW<'_, Ctl0Spec> {
        SbctlW::new(self, 16)
    }
    #[doc = "Bit 17 - Whether to stretch SCL low when data is not ready in slave mode"]
    #[inline(always)]
    pub fn ss(&mut self) -> SsW<'_, Ctl0Spec> {
        SsW::new(self, 17)
    }
    #[doc = "Bit 18 - Wakeup from Deep-sleep mode enable"]
    #[inline(always)]
    pub fn wuen(&mut self) -> WuenW<'_, Ctl0Spec> {
        WuenW::new(self, 18)
    }
    #[doc = "Bit 19 - Whether or not to response to a General Call"]
    #[inline(always)]
    pub fn gcen(&mut self) -> GcenW<'_, Ctl0Spec> {
        GcenW::new(self, 19)
    }
    #[doc = "Bit 20 - SMBus Host address enable"]
    #[inline(always)]
    pub fn smbhaen(&mut self) -> SmbhaenW<'_, Ctl0Spec> {
        SmbhaenW::new(self, 20)
    }
    #[doc = "Bit 21 - SMBus device default address enable"]
    #[inline(always)]
    pub fn smbdaen(&mut self) -> SmbdaenW<'_, Ctl0Spec> {
        SmbdaenW::new(self, 21)
    }
    #[doc = "Bit 22 - SMBus Alert enable"]
    #[inline(always)]
    pub fn smbalten(&mut self) -> SmbaltenW<'_, Ctl0Spec> {
        SmbaltenW::new(self, 22)
    }
    #[doc = "Bit 23 - PEC Calculation Switch"]
    #[inline(always)]
    pub fn pecen(&mut self) -> PecenW<'_, Ctl0Spec> {
        PecenW::new(self, 23)
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
