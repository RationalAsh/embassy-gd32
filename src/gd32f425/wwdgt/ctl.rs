#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `CNT` reader - 7-bit counter"]
pub type CntR = crate::FieldReader;
#[doc = "Field `CNT` writer - 7-bit counter"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WDGTEN` reader - Activation bit"]
pub type WdgtenR = crate::BitReader;
#[doc = "Field `WDGTEN` writer - Activation bit"]
pub type WdgtenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 7-bit counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Activation bit"]
    #[inline(always)]
    pub fn wdgten(&self) -> WdgtenR {
        WdgtenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit counter"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, CtlSpec> {
        CntW::new(self, 0)
    }
    #[doc = "Bit 7 - Activation bit"]
    #[inline(always)]
    pub fn wdgten(&mut self) -> WdgtenW<'_, CtlSpec> {
        WdgtenW::new(self, 7)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL to value 0x7f"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0x7f;
}
