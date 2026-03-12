#[doc = "Register `MTACTL` reader"]
pub type R = crate::R<MtactlSpec>;
#[doc = "Register `MTACTL` writer"]
pub type W = crate::W<MtactlSpec>;
#[doc = "Field `CNTCKDIV_3` reader - Counter clock division"]
pub type Cntckdiv3R = crate::BitReader;
#[doc = "Field `CNTCKDIV_3` writer - Counter clock division"]
pub type Cntckdiv3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Counter clock division"]
    #[inline(always)]
    pub fn cntckdiv_3(&self) -> Cntckdiv3R {
        Cntckdiv3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Counter clock division"]
    #[inline(always)]
    pub fn cntckdiv_3(&mut self) -> Cntckdiv3W<'_, MtactlSpec> {
        Cntckdiv3W::new(self, 3)
    }
}
#[doc = "SHRTIMER Master_TIMER additional control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtactl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtactl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtactlSpec;
impl crate::RegisterSpec for MtactlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtactl::R`](R) reader structure"]
impl crate::Readable for MtactlSpec {}
#[doc = "`write(|w| ..)` method takes [`mtactl::W`](W) writer structure"]
impl crate::Writable for MtactlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTACTL to value 0"]
impl crate::Resettable for MtactlSpec {}
