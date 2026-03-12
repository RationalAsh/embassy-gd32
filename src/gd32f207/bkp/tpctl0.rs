#[doc = "Register `TPCTL0` reader"]
pub type R = crate::R<Tpctl0Spec>;
#[doc = "Register `TPCTL0` writer"]
pub type W = crate::W<Tpctl0Spec>;
#[doc = "Field `TPEN0` reader - TAMPER0 detection enable"]
pub type Tpen0R = crate::BitReader;
#[doc = "Field `TPEN0` writer - TAMPER0 detection enable"]
pub type Tpen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPAL0` reader - TAMPER0 pin active level"]
pub type Tpal0R = crate::BitReader;
#[doc = "Field `TPAL0` writer - TAMPER0 pin active level"]
pub type Tpal0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TAMPER0 detection enable"]
    #[inline(always)]
    pub fn tpen0(&self) -> Tpen0R {
        Tpen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMPER0 pin active level"]
    #[inline(always)]
    pub fn tpal0(&self) -> Tpal0R {
        Tpal0R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMPER0 detection enable"]
    #[inline(always)]
    pub fn tpen0(&mut self) -> Tpen0W<'_, Tpctl0Spec> {
        Tpen0W::new(self, 0)
    }
    #[doc = "Bit 1 - TAMPER0 pin active level"]
    #[inline(always)]
    pub fn tpal0(&mut self) -> Tpal0W<'_, Tpctl0Spec> {
        Tpal0W::new(self, 1)
    }
}
#[doc = "Tamper pin control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tpctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tpctl0Spec;
impl crate::RegisterSpec for Tpctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpctl0::R`](R) reader structure"]
impl crate::Readable for Tpctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`tpctl0::W`](W) writer structure"]
impl crate::Writable for Tpctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TPCTL0 to value 0"]
impl crate::Resettable for Tpctl0Spec {}
