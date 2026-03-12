#[doc = "Register `TPCTL` reader"]
pub type R = crate::R<TpctlSpec>;
#[doc = "Register `TPCTL` writer"]
pub type W = crate::W<TpctlSpec>;
#[doc = "Field `TPEN` reader - TAMPER detection enable"]
pub type TpenR = crate::BitReader;
#[doc = "Field `TPEN` writer - TAMPER detection enable"]
pub type TpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPAL` reader - TAMPER pin active level"]
pub type TpalR = crate::BitReader;
#[doc = "Field `TPAL` writer - TAMPER pin active level"]
pub type TpalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TAMPER detection enable"]
    #[inline(always)]
    pub fn tpen(&self) -> TpenR {
        TpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMPER pin active level"]
    #[inline(always)]
    pub fn tpal(&self) -> TpalR {
        TpalR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMPER detection enable"]
    #[inline(always)]
    pub fn tpen(&mut self) -> TpenW<'_, TpctlSpec> {
        TpenW::new(self, 0)
    }
    #[doc = "Bit 1 - TAMPER pin active level"]
    #[inline(always)]
    pub fn tpal(&mut self) -> TpalW<'_, TpctlSpec> {
        TpalW::new(self, 1)
    }
}
#[doc = "Tamper pin control register\n\nYou can [`read`](crate::Reg::read) this register and get [`tpctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TpctlSpec;
impl crate::RegisterSpec for TpctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tpctl::R`](R) reader structure"]
impl crate::Readable for TpctlSpec {}
#[doc = "`write(|w| ..)` method takes [`tpctl::W`](W) writer structure"]
impl crate::Writable for TpctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TPCTL to value 0"]
impl crate::Resettable for TpctlSpec {}
