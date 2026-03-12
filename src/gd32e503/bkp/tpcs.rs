#[doc = "Register `TPCS` reader"]
pub type R = crate::R<TpcsSpec>;
#[doc = "Register `TPCS` writer"]
pub type W = crate::W<TpcsSpec>;
#[doc = "Field `TER` reader - Tamper event reset"]
pub type TerR = crate::BitReader;
#[doc = "Field `TER` writer - Tamper event reset"]
pub type TerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIR` reader - Tamper interrupt reset"]
pub type TirR = crate::BitReader;
#[doc = "Field `TIR` writer - Tamper interrupt reset"]
pub type TirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPIE` reader - Tamper interrupt enable"]
pub type TpieR = crate::BitReader;
#[doc = "Field `TPIE` writer - Tamper interrupt enable"]
pub type TpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEF` reader - Tamper event flag"]
pub type TefR = crate::BitReader;
#[doc = "Field `TEF` writer - Tamper event flag"]
pub type TefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIF` reader - Tamper interrupt flag"]
pub type TifR = crate::BitReader;
#[doc = "Field `TIF` writer - Tamper interrupt flag"]
pub type TifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tamper event reset"]
    #[inline(always)]
    pub fn ter(&self) -> TerR {
        TerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper interrupt reset"]
    #[inline(always)]
    pub fn tir(&self) -> TirR {
        TirR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tpie(&self) -> TpieR {
        TpieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Tamper event flag"]
    #[inline(always)]
    pub fn tef(&self) -> TefR {
        TefR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tamper interrupt flag"]
    #[inline(always)]
    pub fn tif(&self) -> TifR {
        TifR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper event reset"]
    #[inline(always)]
    pub fn ter(&mut self) -> TerW<'_, TpcsSpec> {
        TerW::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper interrupt reset"]
    #[inline(always)]
    pub fn tir(&mut self) -> TirW<'_, TpcsSpec> {
        TirW::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tpie(&mut self) -> TpieW<'_, TpcsSpec> {
        TpieW::new(self, 2)
    }
    #[doc = "Bit 8 - Tamper event flag"]
    #[inline(always)]
    pub fn tef(&mut self) -> TefW<'_, TpcsSpec> {
        TefW::new(self, 8)
    }
    #[doc = "Bit 9 - Tamper interrupt flag"]
    #[inline(always)]
    pub fn tif(&mut self) -> TifW<'_, TpcsSpec> {
        TifW::new(self, 9)
    }
}
#[doc = "Tamper control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`tpcs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpcs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TpcsSpec;
impl crate::RegisterSpec for TpcsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tpcs::R`](R) reader structure"]
impl crate::Readable for TpcsSpec {}
#[doc = "`write(|w| ..)` method takes [`tpcs::W`](W) writer structure"]
impl crate::Writable for TpcsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TPCS to value 0"]
impl crate::Resettable for TpcsSpec {}
