#[doc = "Register `PLLTINT` reader"]
pub type R = crate::R<PlltintSpec>;
#[doc = "Register `PLLTINT` writer"]
pub type W = crate::W<PlltintSpec>;
#[doc = "Field `PLLTSTBIF` reader - PLLT stabilization interrupt flag"]
pub type PlltstbifR = crate::BitReader;
#[doc = "Field `PLLTSTBIE` reader - PLLT stabilization Interrupt Enable"]
pub type PlltstbieR = crate::BitReader;
#[doc = "Field `PLLTSTBIE` writer - PLLT stabilization Interrupt Enable"]
pub type PlltstbieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLTSTBIC` reader - PLLT stabilization Interrupt clear"]
pub type PlltstbicR = crate::BitReader;
#[doc = "Field `PLLTSTBIC` writer - PLLT stabilization Interrupt clear"]
pub type PlltstbicW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - PLLT stabilization interrupt flag"]
    #[inline(always)]
    pub fn plltstbif(&self) -> PlltstbifR {
        PlltstbifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 14 - PLLT stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn plltstbie(&self) -> PlltstbieR {
        PlltstbieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 22 - PLLT stabilization Interrupt clear"]
    #[inline(always)]
    pub fn plltstbic(&self) -> PlltstbicR {
        PlltstbicR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - PLLT stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn plltstbie(&mut self) -> PlltstbieW<'_, PlltintSpec> {
        PlltstbieW::new(self, 14)
    }
    #[doc = "Bit 22 - PLLT stabilization Interrupt clear"]
    #[inline(always)]
    pub fn plltstbic(&mut self) -> PlltstbicW<'_, PlltintSpec> {
        PlltstbicW::new(self, 22)
    }
}
#[doc = "PLLT interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`plltint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plltint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlltintSpec;
impl crate::RegisterSpec for PlltintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plltint::R`](R) reader structure"]
impl crate::Readable for PlltintSpec {}
#[doc = "`write(|w| ..)` method takes [`plltint::W`](W) writer structure"]
impl crate::Writable for PlltintSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLLTINT to value 0"]
impl crate::Resettable for PlltintSpec {}
