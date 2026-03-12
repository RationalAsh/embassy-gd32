#[doc = "Register `L0VPOS` reader"]
pub type R = crate::R<L0vposSpec>;
#[doc = "Register `L0VPOS` writer"]
pub type W = crate::W<L0vposSpec>;
#[doc = "Field `WTP` reader - Window top position"]
pub type WtpR = crate::FieldReader<u16>;
#[doc = "Field `WTP` writer - Window top position"]
pub type WtpW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WBP` reader - Window bottom position"]
pub type WbpR = crate::FieldReader<u16>;
#[doc = "Field `WBP` writer - Window bottom position"]
pub type WbpW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Window top position"]
    #[inline(always)]
    pub fn wtp(&self) -> WtpR {
        WtpR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Window bottom position"]
    #[inline(always)]
    pub fn wbp(&self) -> WbpR {
        WbpR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Window top position"]
    #[inline(always)]
    pub fn wtp(&mut self) -> WtpW<'_, L0vposSpec> {
        WtpW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Window bottom position"]
    #[inline(always)]
    pub fn wbp(&mut self) -> WbpW<'_, L0vposSpec> {
        WbpW::new(self, 16)
    }
}
#[doc = "Layer 0 vertical position parameters register\n\nYou can [`read`](crate::Reg::read) this register and get [`l0vpos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l0vpos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L0vposSpec;
impl crate::RegisterSpec for L0vposSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l0vpos::R`](R) reader structure"]
impl crate::Readable for L0vposSpec {}
#[doc = "`write(|w| ..)` method takes [`l0vpos::W`](W) writer structure"]
impl crate::Writable for L0vposSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L0VPOS to value 0"]
impl crate::Resettable for L0vposSpec {}
