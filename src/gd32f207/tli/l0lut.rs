#[doc = "Register `L0LUT` writer"]
pub type W = crate::W<L0lutSpec>;
#[doc = "Field `TB` writer - Blue channel of a LUT entry"]
pub type TbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TG` writer - Green channel of a LUT entry"]
pub type TgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TR` writer - Red Channel of a LUT entry"]
pub type TrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TADD` writer - Look Up Table Write Address"]
pub type TaddW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Blue channel of a LUT entry"]
    #[inline(always)]
    pub fn tb(&mut self) -> TbW<'_, L0lutSpec> {
        TbW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Green channel of a LUT entry"]
    #[inline(always)]
    pub fn tg(&mut self) -> TgW<'_, L0lutSpec> {
        TgW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Red Channel of a LUT entry"]
    #[inline(always)]
    pub fn tr(&mut self) -> TrW<'_, L0lutSpec> {
        TrW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Look Up Table Write Address"]
    #[inline(always)]
    pub fn tadd(&mut self) -> TaddW<'_, L0lutSpec> {
        TaddW::new(self, 24)
    }
}
#[doc = "Layer 0 look up table register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l0lut::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L0lutSpec;
impl crate::RegisterSpec for L0lutSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`l0lut::W`](W) writer structure"]
impl crate::Writable for L0lutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L0LUT to value 0"]
impl crate::Resettable for L0lutSpec {}
