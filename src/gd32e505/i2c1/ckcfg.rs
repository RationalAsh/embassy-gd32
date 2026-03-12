#[doc = "Register `CKCFG` reader"]
pub type R = crate::R<CkcfgSpec>;
#[doc = "Register `CKCFG` writer"]
pub type W = crate::W<CkcfgSpec>;
#[doc = "Field `CLKC` reader - I2C Clock control in master mode"]
pub type ClkcR = crate::FieldReader<u16>;
#[doc = "Field `CLKC` writer - I2C Clock control in master mode"]
pub type ClkcW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DTCY` reader - Duty cycle in fast mode"]
pub type DtcyR = crate::BitReader;
#[doc = "Field `DTCY` writer - Duty cycle in fast mode"]
pub type DtcyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAST` reader - I2C speed selection in master mode"]
pub type FastR = crate::BitReader;
#[doc = "Field `FAST` writer - I2C speed selection in master mode"]
pub type FastW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - I2C Clock control in master mode"]
    #[inline(always)]
    pub fn clkc(&self) -> ClkcR {
        ClkcR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - Duty cycle in fast mode"]
    #[inline(always)]
    pub fn dtcy(&self) -> DtcyR {
        DtcyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C speed selection in master mode"]
    #[inline(always)]
    pub fn fast(&self) -> FastR {
        FastR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - I2C Clock control in master mode"]
    #[inline(always)]
    pub fn clkc(&mut self) -> ClkcW<'_, CkcfgSpec> {
        ClkcW::new(self, 0)
    }
    #[doc = "Bit 14 - Duty cycle in fast mode"]
    #[inline(always)]
    pub fn dtcy(&mut self) -> DtcyW<'_, CkcfgSpec> {
        DtcyW::new(self, 14)
    }
    #[doc = "Bit 15 - I2C speed selection in master mode"]
    #[inline(always)]
    pub fn fast(&mut self) -> FastW<'_, CkcfgSpec> {
        FastW::new(self, 15)
    }
}
#[doc = "Clock configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CkcfgSpec;
impl crate::RegisterSpec for CkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckcfg::R`](R) reader structure"]
impl crate::Readable for CkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ckcfg::W`](W) writer structure"]
impl crate::Writable for CkcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CKCFG to value 0"]
impl crate::Resettable for CkcfgSpec {}
