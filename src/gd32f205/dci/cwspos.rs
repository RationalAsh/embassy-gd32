#[doc = "Register `CWSPOS` reader"]
pub type R = crate::R<CwsposSpec>;
#[doc = "Register `CWSPOS` writer"]
pub type W = crate::W<CwsposSpec>;
#[doc = "Field `WHSP` reader - Window horizontal start position"]
pub type WhspR = crate::FieldReader<u16>;
#[doc = "Field `WHSP` writer - Window horizontal start position"]
pub type WhspW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `WVSP` reader - Window Vertical start position"]
pub type WvspR = crate::FieldReader<u16>;
#[doc = "Field `WVSP` writer - Window Vertical start position"]
pub type WvspW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:13 - Window horizontal start position"]
    #[inline(always)]
    pub fn whsp(&self) -> WhspR {
        WhspR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:28 - Window Vertical start position"]
    #[inline(always)]
    pub fn wvsp(&self) -> WvspR {
        WvspR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Window horizontal start position"]
    #[inline(always)]
    pub fn whsp(&mut self) -> WhspW<'_, CwsposSpec> {
        WhspW::new(self, 0)
    }
    #[doc = "Bits 16:28 - Window Vertical start position"]
    #[inline(always)]
    pub fn wvsp(&mut self) -> WvspW<'_, CwsposSpec> {
        WvspW::new(self, 16)
    }
}
#[doc = "DCI Cropping window start position register\n\nYou can [`read`](crate::Reg::read) this register and get [`cwspos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwspos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CwsposSpec;
impl crate::RegisterSpec for CwsposSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwspos::R`](R) reader structure"]
impl crate::Readable for CwsposSpec {}
#[doc = "`write(|w| ..)` method takes [`cwspos::W`](W) writer structure"]
impl crate::Writable for CwsposSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CWSPOS to value 0"]
impl crate::Resettable for CwsposSpec {}
