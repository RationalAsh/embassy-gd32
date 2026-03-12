#[doc = "Register `STATC` reader"]
pub type R = crate::R<StatcSpec>;
#[doc = "Register `STATC` writer"]
pub type W = crate::W<StatcSpec>;
#[doc = "Field `SOFC` reader - Start of frame flag clear"]
pub type SofcR = crate::BitReader;
#[doc = "Field `SOFC` writer - Start of frame flag clear"]
pub type SofcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDC` reader - SLCD data update done clear bit"]
pub type UpdcR = crate::BitReader;
#[doc = "Field `UPDC` writer - SLCD data update done clear bit"]
pub type UpdcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Start of frame flag clear"]
    #[inline(always)]
    pub fn sofc(&self) -> SofcR {
        SofcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SLCD data update done clear bit"]
    #[inline(always)]
    pub fn updc(&self) -> UpdcR {
        UpdcR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Start of frame flag clear"]
    #[inline(always)]
    pub fn sofc(&mut self) -> SofcW<'_, StatcSpec> {
        SofcW::new(self, 1)
    }
    #[doc = "Bit 3 - SLCD data update done clear bit"]
    #[inline(always)]
    pub fn updc(&mut self) -> UpdcW<'_, StatcSpec> {
        UpdcW::new(self, 3)
    }
}
#[doc = "SLCD status flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`statc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatcSpec;
impl crate::RegisterSpec for StatcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statc::R`](R) reader structure"]
impl crate::Readable for StatcSpec {}
#[doc = "`write(|w| ..)` method takes [`statc::W`](W) writer structure"]
impl crate::Writable for StatcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATC to value 0"]
impl crate::Resettable for StatcSpec {}
