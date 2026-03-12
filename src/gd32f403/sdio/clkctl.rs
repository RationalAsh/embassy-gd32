#[doc = "Register `CLKCTL` reader"]
pub type R = crate::R<ClkctlSpec>;
#[doc = "Register `CLKCTL` writer"]
pub type W = crate::W<ClkctlSpec>;
#[doc = "Field `DIV_0_7` reader - Clock division"]
pub type Div0_7R = crate::FieldReader;
#[doc = "Field `DIV_0_7` writer - Clock division"]
pub type Div0_7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLKEN` reader - SDIO_CLK clock output enable bit"]
pub type ClkenR = crate::BitReader;
#[doc = "Field `CLKEN` writer - SDIO_CLK clock output enable bit"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKPWRSAV` reader - SDIO_CLK clock dynamic switch on/off for power saving"]
pub type ClkpwrsavR = crate::BitReader;
#[doc = "Field `CLKPWRSAV` writer - SDIO_CLK clock dynamic switch on/off for power saving"]
pub type ClkpwrsavW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKBYP` reader - Clock bypass enable bit"]
pub type ClkbypR = crate::BitReader;
#[doc = "Field `CLKBYP` writer - Clock bypass enable bit"]
pub type ClkbypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSMODE` reader - SDIO card bus mode control bit"]
pub type BusmodeR = crate::FieldReader;
#[doc = "Field `BUSMODE` writer - SDIO card bus mode control bit"]
pub type BusmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLKEDGE` reader - SDIO_CLK clock edge selection bit"]
pub type ClkedgeR = crate::BitReader;
#[doc = "Field `CLKEDGE` writer - SDIO_CLK clock edge selection bit"]
pub type ClkedgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWCLKEN` reader - Hardware Clock Control enable bit"]
pub type HwclkenR = crate::BitReader;
#[doc = "Field `HWCLKEN` writer - Hardware Clock Control enable bit"]
pub type HwclkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIV_8` reader - MSB of Clock division"]
pub type Div8R = crate::BitReader;
#[doc = "Field `DIV_8` writer - MSB of Clock division"]
pub type Div8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Clock division"]
    #[inline(always)]
    pub fn div_0_7(&self) -> Div0_7R {
        Div0_7R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - SDIO_CLK clock output enable bit"]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SDIO_CLK clock dynamic switch on/off for power saving"]
    #[inline(always)]
    pub fn clkpwrsav(&self) -> ClkpwrsavR {
        ClkpwrsavR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock bypass enable bit"]
    #[inline(always)]
    pub fn clkbyp(&self) -> ClkbypR {
        ClkbypR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - SDIO card bus mode control bit"]
    #[inline(always)]
    pub fn busmode(&self) -> BusmodeR {
        BusmodeR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - SDIO_CLK clock edge selection bit"]
    #[inline(always)]
    pub fn clkedge(&self) -> ClkedgeR {
        ClkedgeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Hardware Clock Control enable bit"]
    #[inline(always)]
    pub fn hwclken(&self) -> HwclkenR {
        HwclkenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 31 - MSB of Clock division"]
    #[inline(always)]
    pub fn div_8(&self) -> Div8R {
        Div8R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock division"]
    #[inline(always)]
    pub fn div_0_7(&mut self) -> Div0_7W<'_, ClkctlSpec> {
        Div0_7W::new(self, 0)
    }
    #[doc = "Bit 8 - SDIO_CLK clock output enable bit"]
    #[inline(always)]
    pub fn clken(&mut self) -> ClkenW<'_, ClkctlSpec> {
        ClkenW::new(self, 8)
    }
    #[doc = "Bit 9 - SDIO_CLK clock dynamic switch on/off for power saving"]
    #[inline(always)]
    pub fn clkpwrsav(&mut self) -> ClkpwrsavW<'_, ClkctlSpec> {
        ClkpwrsavW::new(self, 9)
    }
    #[doc = "Bit 10 - Clock bypass enable bit"]
    #[inline(always)]
    pub fn clkbyp(&mut self) -> ClkbypW<'_, ClkctlSpec> {
        ClkbypW::new(self, 10)
    }
    #[doc = "Bits 11:12 - SDIO card bus mode control bit"]
    #[inline(always)]
    pub fn busmode(&mut self) -> BusmodeW<'_, ClkctlSpec> {
        BusmodeW::new(self, 11)
    }
    #[doc = "Bit 13 - SDIO_CLK clock edge selection bit"]
    #[inline(always)]
    pub fn clkedge(&mut self) -> ClkedgeW<'_, ClkctlSpec> {
        ClkedgeW::new(self, 13)
    }
    #[doc = "Bit 14 - Hardware Clock Control enable bit"]
    #[inline(always)]
    pub fn hwclken(&mut self) -> HwclkenW<'_, ClkctlSpec> {
        HwclkenW::new(self, 14)
    }
    #[doc = "Bit 31 - MSB of Clock division"]
    #[inline(always)]
    pub fn div_8(&mut self) -> Div8W<'_, ClkctlSpec> {
        Div8W::new(self, 31)
    }
}
#[doc = "Clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkctlSpec;
impl crate::RegisterSpec for ClkctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkctl::R`](R) reader structure"]
impl crate::Readable for ClkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`clkctl::W`](W) writer structure"]
impl crate::Writable for ClkctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKCTL to value 0"]
impl crate::Resettable for ClkctlSpec {}
