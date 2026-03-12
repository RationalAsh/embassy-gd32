#[doc = "Register `L1LUT` writer"]
pub type W = crate::W<L1lutSpec>;
#[doc = "Field `TB` writer - Blue channel of a LUT entry"]
pub type TbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TG` writer - Green channel of a LUT entry"]
pub type TgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TR` writer - Red channel of a LUT entry"]
pub type TrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TADD` writer - Look Up Table Write Address"]
pub type TaddW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Blue channel of a LUT entry"]
    #[inline(always)]
    pub fn tb(&mut self) -> TbW<'_, L1lutSpec> {
        TbW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Green channel of a LUT entry"]
    #[inline(always)]
    pub fn tg(&mut self) -> TgW<'_, L1lutSpec> {
        TgW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Red channel of a LUT entry"]
    #[inline(always)]
    pub fn tr(&mut self) -> TrW<'_, L1lutSpec> {
        TrW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Look Up Table Write Address"]
    #[inline(always)]
    pub fn tadd(&mut self) -> TaddW<'_, L1lutSpec> {
        TaddW::new(self, 24)
    }
}
#[doc = "Layer 1 look up table register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1lut::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1lutSpec;
impl crate::RegisterSpec for L1lutSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`l1lut::W`](W) writer structure"]
impl crate::Writable for L1lutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1LUT to value 0"]
impl crate::Resettable for L1lutSpec {}
