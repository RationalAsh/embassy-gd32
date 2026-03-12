#[doc = "Register `ADDAPB1RST` reader"]
pub type R = crate::R<Addapb1rstSpec>;
#[doc = "Register `ADDAPB1RST` writer"]
pub type W = crate::W<Addapb1rstSpec>;
#[doc = "Field `CTCRST` reader - CTC reset"]
pub type CtcrstR = crate::BitReader;
#[doc = "Field `CTCRST` writer - CTC reset"]
pub type CtcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN2RST` reader - CAN2 reset"]
pub type Can2rstR = crate::BitReader;
#[doc = "Field `CAN2RST` writer - CAN2 reset"]
pub type Can2rstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 27 - CTC reset"]
    #[inline(always)]
    pub fn ctcrst(&self) -> CtcrstR {
        CtcrstR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - CAN2 reset"]
    #[inline(always)]
    pub fn can2rst(&self) -> Can2rstR {
        Can2rstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - CTC reset"]
    #[inline(always)]
    pub fn ctcrst(&mut self) -> CtcrstW<'_, Addapb1rstSpec> {
        CtcrstW::new(self, 27)
    }
    #[doc = "Bit 31 - CAN2 reset"]
    #[inline(always)]
    pub fn can2rst(&mut self) -> Can2rstW<'_, Addapb1rstSpec> {
        Can2rstW::new(self, 31)
    }
}
#[doc = "APB1 additional reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`addapb1rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addapb1rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addapb1rstSpec;
impl crate::RegisterSpec for Addapb1rstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addapb1rst::R`](R) reader structure"]
impl crate::Readable for Addapb1rstSpec {}
#[doc = "`write(|w| ..)` method takes [`addapb1rst::W`](W) writer structure"]
impl crate::Writable for Addapb1rstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDAPB1RST to value 0"]
impl crate::Resettable for Addapb1rstSpec {}
