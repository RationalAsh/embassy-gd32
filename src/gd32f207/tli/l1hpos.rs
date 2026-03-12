#[doc = "Register `L1HPOS` reader"]
pub type R = crate::R<L1hposSpec>;
#[doc = "Register `L1HPOS` writer"]
pub type W = crate::W<L1hposSpec>;
#[doc = "Field `WLP` reader - Window left position"]
pub type WlpR = crate::FieldReader<u16>;
#[doc = "Field `WLP` writer - Window left position"]
pub type WlpW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WRP` reader - Window right position"]
pub type WrpR = crate::FieldReader<u16>;
#[doc = "Field `WRP` writer - Window right position"]
pub type WrpW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Window left position"]
    #[inline(always)]
    pub fn wlp(&self) -> WlpR {
        WlpR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Window right position"]
    #[inline(always)]
    pub fn wrp(&self) -> WrpR {
        WrpR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Window left position"]
    #[inline(always)]
    pub fn wlp(&mut self) -> WlpW<'_, L1hposSpec> {
        WlpW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Window right position"]
    #[inline(always)]
    pub fn wrp(&mut self) -> WrpW<'_, L1hposSpec> {
        WrpW::new(self, 16)
    }
}
#[doc = "Layer 1 horizontal position parameters register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1hpos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1hpos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1hposSpec;
impl crate::RegisterSpec for L1hposSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1hpos::R`](R) reader structure"]
impl crate::Readable for L1hposSpec {}
#[doc = "`write(|w| ..)` method takes [`l1hpos::W`](W) writer structure"]
impl crate::Writable for L1hposSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1HPOS to value 0"]
impl crate::Resettable for L1hposSpec {}
