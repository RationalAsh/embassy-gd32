#[doc = "Register `IDATA0` reader"]
pub type R = crate::R<Idata0Spec>;
#[doc = "Register `IDATA0` writer"]
pub type W = crate::W<Idata0Spec>;
#[doc = "Field `IDATA0` reader - IDATA0"]
pub type Idata0R = crate::FieldReader<u32>;
#[doc = "Field `IDATA0` writer - IDATA0"]
pub type Idata0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IDATA0"]
    #[inline(always)]
    pub fn idata0(&self) -> Idata0R {
        Idata0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IDATA0"]
    #[inline(always)]
    pub fn idata0(&mut self) -> Idata0W<'_, Idata0Spec> {
        Idata0W::new(self, 0)
    }
}
#[doc = "Input data0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`idata0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idata0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idata0Spec;
impl crate::RegisterSpec for Idata0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idata0::R`](R) reader structure"]
impl crate::Readable for Idata0Spec {}
#[doc = "`write(|w| ..)` method takes [`idata0::W`](W) writer structure"]
impl crate::Writable for Idata0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDATA0 to value 0x3f80_0000"]
impl crate::Resettable for Idata0Spec {
    const RESET_VALUE: u32 = 0x3f80_0000;
}
