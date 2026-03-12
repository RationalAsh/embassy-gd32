#[doc = "Register `INTF` reader"]
pub type R = crate::R<IntfSpec>;
#[doc = "Register `INTF` writer"]
pub type W = crate::W<IntfSpec>;
#[doc = "Field `RBR` reader - Rx-Byte data received"]
pub type RbrR = crate::BitReader;
#[doc = "Field `RBR` writer - Rx-Byte data received"]
pub type RbrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REND` reader - End of Reception"]
pub type RendR = crate::BitReader;
#[doc = "Field `REND` writer - End of Reception"]
pub type RendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RO` reader - RX Overrun"]
pub type RoR = crate::BitReader;
#[doc = "Field `RO` writer - RX Overrun"]
pub type RoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBRE` reader - Bit Rising Error"]
pub type RbreR = crate::BitReader;
#[doc = "Field `RBRE` writer - Bit Rising Error"]
pub type RbreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSBPE` reader - Short Bit Period Error"]
pub type RsbpeR = crate::BitReader;
#[doc = "Field `RSBPE` writer - Short Bit Period Error"]
pub type RsbpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RLBPE` reader - Long Bit Period Error"]
pub type RlbpeR = crate::BitReader;
#[doc = "Field `RLBPE` writer - Long Bit Period Error"]
pub type RlbpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAE` reader - Rx ACK Error"]
pub type RaeR = crate::BitReader;
#[doc = "Field `RAE` writer - Rx ACK Error"]
pub type RaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSTARB` reader - Arbitration lost"]
pub type LstarbR = crate::BitReader;
#[doc = "Field `LSTARB` writer - Arbitration lost"]
pub type LstarbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBR` reader - Tx-Byte data request"]
pub type TbrR = crate::BitReader;
#[doc = "Field `TBR` writer - Tx-Byte data request"]
pub type TbrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEND` reader - Transmission successfully end"]
pub type TendR = crate::BitReader;
#[doc = "Field `TEND` writer - Transmission successfully end"]
pub type TendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TU` reader - Tx data buffer underrun"]
pub type TuR = crate::BitReader;
#[doc = "Field `TU` writer - Tx data buffer underrun"]
pub type TuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERR` reader - Tx-Error"]
pub type TerrR = crate::BitReader;
#[doc = "Field `TERR` writer - Tx-Error"]
pub type TerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAERR` reader - Tx ACK Error flag"]
pub type TaerrR = crate::BitReader;
#[doc = "Field `TAERR` writer - Tx ACK Error flag"]
pub type TaerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx-Byte data received"]
    #[inline(always)]
    pub fn rbr(&self) -> RbrR {
        RbrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of Reception"]
    #[inline(always)]
    pub fn rend(&self) -> RendR {
        RendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Overrun"]
    #[inline(always)]
    pub fn ro(&self) -> RoR {
        RoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit Rising Error"]
    #[inline(always)]
    pub fn rbre(&self) -> RbreR {
        RbreR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Short Bit Period Error"]
    #[inline(always)]
    pub fn rsbpe(&self) -> RsbpeR {
        RsbpeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Long Bit Period Error"]
    #[inline(always)]
    pub fn rlbpe(&self) -> RlbpeR {
        RlbpeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx ACK Error"]
    #[inline(always)]
    pub fn rae(&self) -> RaeR {
        RaeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Arbitration lost"]
    #[inline(always)]
    pub fn lstarb(&self) -> LstarbR {
        LstarbR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Tx-Byte data request"]
    #[inline(always)]
    pub fn tbr(&self) -> TbrR {
        TbrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission successfully end"]
    #[inline(always)]
    pub fn tend(&self) -> TendR {
        TendR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx data buffer underrun"]
    #[inline(always)]
    pub fn tu(&self) -> TuR {
        TuR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx-Error"]
    #[inline(always)]
    pub fn terr(&self) -> TerrR {
        TerrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx ACK Error flag"]
    #[inline(always)]
    pub fn taerr(&self) -> TaerrR {
        TaerrR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx-Byte data received"]
    #[inline(always)]
    pub fn rbr(&mut self) -> RbrW<'_, IntfSpec> {
        RbrW::new(self, 0)
    }
    #[doc = "Bit 1 - End of Reception"]
    #[inline(always)]
    pub fn rend(&mut self) -> RendW<'_, IntfSpec> {
        RendW::new(self, 1)
    }
    #[doc = "Bit 2 - RX Overrun"]
    #[inline(always)]
    pub fn ro(&mut self) -> RoW<'_, IntfSpec> {
        RoW::new(self, 2)
    }
    #[doc = "Bit 3 - Bit Rising Error"]
    #[inline(always)]
    pub fn rbre(&mut self) -> RbreW<'_, IntfSpec> {
        RbreW::new(self, 3)
    }
    #[doc = "Bit 4 - Short Bit Period Error"]
    #[inline(always)]
    pub fn rsbpe(&mut self) -> RsbpeW<'_, IntfSpec> {
        RsbpeW::new(self, 4)
    }
    #[doc = "Bit 5 - Long Bit Period Error"]
    #[inline(always)]
    pub fn rlbpe(&mut self) -> RlbpeW<'_, IntfSpec> {
        RlbpeW::new(self, 5)
    }
    #[doc = "Bit 6 - Rx ACK Error"]
    #[inline(always)]
    pub fn rae(&mut self) -> RaeW<'_, IntfSpec> {
        RaeW::new(self, 6)
    }
    #[doc = "Bit 7 - Arbitration lost"]
    #[inline(always)]
    pub fn lstarb(&mut self) -> LstarbW<'_, IntfSpec> {
        LstarbW::new(self, 7)
    }
    #[doc = "Bit 8 - Tx-Byte data request"]
    #[inline(always)]
    pub fn tbr(&mut self) -> TbrW<'_, IntfSpec> {
        TbrW::new(self, 8)
    }
    #[doc = "Bit 9 - Transmission successfully end"]
    #[inline(always)]
    pub fn tend(&mut self) -> TendW<'_, IntfSpec> {
        TendW::new(self, 9)
    }
    #[doc = "Bit 10 - Tx data buffer underrun"]
    #[inline(always)]
    pub fn tu(&mut self) -> TuW<'_, IntfSpec> {
        TuW::new(self, 10)
    }
    #[doc = "Bit 11 - Tx-Error"]
    #[inline(always)]
    pub fn terr(&mut self) -> TerrW<'_, IntfSpec> {
        TerrW::new(self, 11)
    }
    #[doc = "Bit 12 - Tx ACK Error flag"]
    #[inline(always)]
    pub fn taerr(&mut self) -> TaerrW<'_, IntfSpec> {
        TaerrW::new(self, 12)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfSpec;
impl crate::RegisterSpec for IntfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf::R`](R) reader structure"]
impl crate::Readable for IntfSpec {}
#[doc = "`write(|w| ..)` method takes [`intf::W`](W) writer structure"]
impl crate::Writable for IntfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for IntfSpec {}
