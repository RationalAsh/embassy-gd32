#[doc = "Register `RL` reader"]
pub type R = crate::R<RlSpec>;
#[doc = "Register `RL` writer"]
pub type W = crate::W<RlSpec>;
#[doc = "Field `RQR` reader - Request Reload"]
pub type RqrR = crate::BitReader;
#[doc = "Field `RQR` writer - Request Reload"]
pub type RqrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBR` reader - Frame Blank Reload"]
pub type FbrR = crate::BitReader;
#[doc = "Field `FBR` writer - Frame Blank Reload"]
pub type FbrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Request Reload"]
    #[inline(always)]
    pub fn rqr(&self) -> RqrR {
        RqrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame Blank Reload"]
    #[inline(always)]
    pub fn fbr(&self) -> FbrR {
        FbrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Request Reload"]
    #[inline(always)]
    pub fn rqr(&mut self) -> RqrW<'_, RlSpec> {
        RqrW::new(self, 0)
    }
    #[doc = "Bit 1 - Frame Blank Reload"]
    #[inline(always)]
    pub fn fbr(&mut self) -> FbrW<'_, RlSpec> {
        FbrW::new(self, 1)
    }
}
#[doc = "Reload layer register\n\nYou can [`read`](crate::Reg::read) this register and get [`rl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RlSpec;
impl crate::RegisterSpec for RlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rl::R`](R) reader structure"]
impl crate::Readable for RlSpec {}
#[doc = "`write(|w| ..)` method takes [`rl::W`](W) writer structure"]
impl crate::Writable for RlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RL to value 0"]
impl crate::Resettable for RlSpec {}
