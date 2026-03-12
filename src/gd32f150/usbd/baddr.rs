#[doc = "Register `BADDR` reader"]
pub type R = crate::R<BaddrSpec>;
#[doc = "Register `BADDR` writer"]
pub type W = crate::W<BaddrSpec>;
#[doc = "Field `BAR` reader - Buffer address"]
pub type BarR = crate::FieldReader<u16>;
#[doc = "Field `BAR` writer - Buffer address"]
pub type BarW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 3:15 - Buffer address"]
    #[inline(always)]
    pub fn bar(&self) -> BarR {
        BarR::new((self.bits >> 3) & 0x1fff)
    }
}
impl W {
    #[doc = "Bits 3:15 - Buffer address"]
    #[inline(always)]
    pub fn bar(&mut self) -> BarW<'_, BaddrSpec> {
        BarW::new(self, 3)
    }
}
#[doc = "Buffer address register\n\nYou can [`read`](crate::Reg::read) this register and get [`baddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaddrSpec;
impl crate::RegisterSpec for BaddrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`baddr::R`](R) reader structure"]
impl crate::Readable for BaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`baddr::W`](W) writer structure"]
impl crate::Writable for BaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BADDR to value 0"]
impl crate::Resettable for BaddrSpec {}
