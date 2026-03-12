#[doc = "Register `AHB2EN` reader"]
pub type R = crate::R<Ahb2enSpec>;
#[doc = "Register `AHB2EN` writer"]
pub type W = crate::W<Ahb2enSpec>;
#[doc = "Field `DCIEN` reader - DCI clock enable"]
pub type DcienR = crate::BitReader;
#[doc = "Field `DCIEN` writer - DCI clock enable"]
pub type DcienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAUEN` reader - CAU clock enable"]
pub type CauenR = crate::BitReader;
#[doc = "Field `CAUEN` writer - CAU clock enable"]
pub type CauenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HAUEN` reader - HAU clock enable"]
pub type HauenR = crate::BitReader;
#[doc = "Field `HAUEN` writer - HAU clock enable"]
pub type HauenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRNGEN` reader - TRNG clock enable"]
pub type TrngenR = crate::BitReader;
#[doc = "Field `TRNGEN` writer - TRNG clock enable"]
pub type TrngenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DCI clock enable"]
    #[inline(always)]
    pub fn dcien(&self) -> DcienR {
        DcienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CAU clock enable"]
    #[inline(always)]
    pub fn cauen(&self) -> CauenR {
        CauenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HAU clock enable"]
    #[inline(always)]
    pub fn hauen(&self) -> HauenR {
        HauenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TRNG clock enable"]
    #[inline(always)]
    pub fn trngen(&self) -> TrngenR {
        TrngenR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCI clock enable"]
    #[inline(always)]
    pub fn dcien(&mut self) -> DcienW<'_, Ahb2enSpec> {
        DcienW::new(self, 0)
    }
    #[doc = "Bit 4 - CAU clock enable"]
    #[inline(always)]
    pub fn cauen(&mut self) -> CauenW<'_, Ahb2enSpec> {
        CauenW::new(self, 4)
    }
    #[doc = "Bit 5 - HAU clock enable"]
    #[inline(always)]
    pub fn hauen(&mut self) -> HauenW<'_, Ahb2enSpec> {
        HauenW::new(self, 5)
    }
    #[doc = "Bit 6 - TRNG clock enable"]
    #[inline(always)]
    pub fn trngen(&mut self) -> TrngenW<'_, Ahb2enSpec> {
        TrngenW::new(self, 6)
    }
}
#[doc = "AHB2 enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb2enSpec;
impl crate::RegisterSpec for Ahb2enSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2en::R`](R) reader structure"]
impl crate::Readable for Ahb2enSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb2en::W`](W) writer structure"]
impl crate::Writable for Ahb2enSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB2EN to value 0"]
impl crate::Resettable for Ahb2enSpec {}
