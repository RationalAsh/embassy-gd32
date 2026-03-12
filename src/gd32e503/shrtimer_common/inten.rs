#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `SYSFLTIE` reader - System fault interrupt enable"]
pub type SysfltieR = crate::BitReader;
#[doc = "Field `SYSFLTIE` writer - System fault interrupt enable"]
pub type SysfltieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLLCALIE` reader - DLL calibration completed interrupt enable"]
pub type DllcalieR = crate::BitReader;
#[doc = "Field `DLLCALIE` writer - DLL calibration completed interrupt enable"]
pub type DllcalieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMPERIE` reader - Bunch mode period interrupt enable"]
pub type BmperieR = crate::BitReader;
#[doc = "Field `BMPERIE` writer - Bunch mode period interrupt enable"]
pub type BmperieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - System fault interrupt enable"]
    #[inline(always)]
    pub fn sysfltie(&self) -> SysfltieR {
        SysfltieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - DLL calibration completed interrupt enable"]
    #[inline(always)]
    pub fn dllcalie(&self) -> DllcalieR {
        DllcalieR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bunch mode period interrupt enable"]
    #[inline(always)]
    pub fn bmperie(&self) -> BmperieR {
        BmperieR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - System fault interrupt enable"]
    #[inline(always)]
    pub fn sysfltie(&mut self) -> SysfltieW<'_, IntenSpec> {
        SysfltieW::new(self, 5)
    }
    #[doc = "Bit 16 - DLL calibration completed interrupt enable"]
    #[inline(always)]
    pub fn dllcalie(&mut self) -> DllcalieW<'_, IntenSpec> {
        DllcalieW::new(self, 16)
    }
    #[doc = "Bit 17 - Bunch mode period interrupt enable"]
    #[inline(always)]
    pub fn bmperie(&mut self) -> BmperieW<'_, IntenSpec> {
        BmperieW::new(self, 17)
    }
}
#[doc = "SHRTIMER interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {}
