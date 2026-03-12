#[doc = "Register `ADDCFG` reader"]
pub type R = crate::R<AddcfgSpec>;
#[doc = "Register `ADDCFG` writer"]
pub type W = crate::W<AddcfgSpec>;
#[doc = "Field `PLLUSBPREDV` reader - PLLUSBPREDV division factor"]
pub type PllusbpredvR = crate::FieldReader;
#[doc = "Field `PLLUSBPREDV` writer - PLLUSBPREDV division factor"]
pub type PllusbpredvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PLLUSBPRESEL` reader - PLLUSB clock source preselection"]
pub type PllusbpreselR = crate::BitReader;
#[doc = "Field `PLLUSBPRESEL` writer - PLLUSB clock source preselection"]
pub type PllusbpreselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLUSBPREDVSEL` reader - PLLUSBPREDV input Clock Source Selection"]
pub type PllusbpredvselR = crate::BitReader;
#[doc = "Field `PLLUSBPREDVSEL` writer - PLLUSBPREDV input Clock Source Selection"]
pub type PllusbpredvselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLUSBMF` reader - The PLLUSB clock multiplication factor"]
pub type PllusbmfR = crate::FieldReader;
#[doc = "Field `PLLUSBMF` writer - The PLLUSB clock multiplication factor"]
pub type PllusbmfW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:3 - PLLUSBPREDV division factor"]
    #[inline(always)]
    pub fn pllusbpredv(&self) -> PllusbpredvR {
        PllusbpredvR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 16 - PLLUSB clock source preselection"]
    #[inline(always)]
    pub fn pllusbpresel(&self) -> PllusbpreselR {
        PllusbpreselR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PLLUSBPREDV input Clock Source Selection"]
    #[inline(always)]
    pub fn pllusbpredvsel(&self) -> PllusbpredvselR {
        PllusbpredvselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:24 - The PLLUSB clock multiplication factor"]
    #[inline(always)]
    pub fn pllusbmf(&self) -> PllusbmfR {
        PllusbmfR::new(((self.bits >> 18) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PLLUSBPREDV division factor"]
    #[inline(always)]
    pub fn pllusbpredv(&mut self) -> PllusbpredvW<'_, AddcfgSpec> {
        PllusbpredvW::new(self, 0)
    }
    #[doc = "Bit 16 - PLLUSB clock source preselection"]
    #[inline(always)]
    pub fn pllusbpresel(&mut self) -> PllusbpreselW<'_, AddcfgSpec> {
        PllusbpreselW::new(self, 16)
    }
    #[doc = "Bit 17 - PLLUSBPREDV input Clock Source Selection"]
    #[inline(always)]
    pub fn pllusbpredvsel(&mut self) -> PllusbpredvselW<'_, AddcfgSpec> {
        PllusbpredvselW::new(self, 17)
    }
    #[doc = "Bits 18:24 - The PLLUSB clock multiplication factor"]
    #[inline(always)]
    pub fn pllusbmf(&mut self) -> PllusbmfW<'_, AddcfgSpec> {
        PllusbmfW::new(self, 18)
    }
}
#[doc = "Additional clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`addcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddcfgSpec;
impl crate::RegisterSpec for AddcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addcfg::R`](R) reader structure"]
impl crate::Readable for AddcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`addcfg::W`](W) writer structure"]
impl crate::Writable for AddcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDCFG to value 0"]
impl crate::Resettable for AddcfgSpec {}
