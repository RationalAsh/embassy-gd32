#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `SCIE` reader - Second interrupt"]
pub type ScieR = crate::BitReader;
#[doc = "Field `SCIE` writer - Second interrupt"]
pub type ScieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRMIE` reader - Alarm interrupt enable"]
pub type AlrmieR = crate::BitReader;
#[doc = "Field `ALRMIE` writer - Alarm interrupt enable"]
pub type AlrmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVIE` reader - Overflow interrupt enable"]
pub type OvieR = crate::BitReader;
#[doc = "Field `OVIE` writer - Overflow interrupt enable"]
pub type OvieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Second interrupt"]
    #[inline(always)]
    pub fn scie(&self) -> ScieR {
        ScieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm interrupt enable"]
    #[inline(always)]
    pub fn alrmie(&self) -> AlrmieR {
        AlrmieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ovie(&self) -> OvieR {
        OvieR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Second interrupt"]
    #[inline(always)]
    pub fn scie(&mut self) -> ScieW<'_, IntenSpec> {
        ScieW::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm interrupt enable"]
    #[inline(always)]
    pub fn alrmie(&mut self) -> AlrmieW<'_, IntenSpec> {
        AlrmieW::new(self, 1)
    }
    #[doc = "Bit 2 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ovie(&mut self) -> OvieW<'_, IntenSpec> {
        OvieW::new(self, 2)
    }
}
#[doc = "RTC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
