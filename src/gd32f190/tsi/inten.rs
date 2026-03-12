#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `CTCFIE` reader - Charge-transfer complete flag Interrupt Enable"]
pub type CtcfieR = crate::BitReader;
#[doc = "Field `CTCFIE` writer - Charge-transfer complete flag Interrupt Enable"]
pub type CtcfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MNERRIE` reader - Max Cycle Number Error Interrupt Enable"]
pub type MnerrieR = crate::BitReader;
#[doc = "Field `MNERRIE` writer - Max Cycle Number Error Interrupt Enable"]
pub type MnerrieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Charge-transfer complete flag Interrupt Enable"]
    #[inline(always)]
    pub fn ctcfie(&self) -> CtcfieR {
        CtcfieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Max Cycle Number Error Interrupt Enable"]
    #[inline(always)]
    pub fn mnerrie(&self) -> MnerrieR {
        MnerrieR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Charge-transfer complete flag Interrupt Enable"]
    #[inline(always)]
    pub fn ctcfie(&mut self) -> CtcfieW<'_, IntenSpec> {
        CtcfieW::new(self, 0)
    }
    #[doc = "Bit 1 - Max Cycle Number Error Interrupt Enable"]
    #[inline(always)]
    pub fn mnerrie(&mut self) -> MnerrieW<'_, IntenSpec> {
        MnerrieW::new(self, 1)
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {}
