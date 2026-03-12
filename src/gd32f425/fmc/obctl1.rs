#[doc = "Register `OBCTL1` reader"]
pub type R = crate::R<Obctl1Spec>;
#[doc = "Register `OBCTL1` writer"]
pub type W = crate::W<Obctl1Spec>;
#[doc = "Field `WP1` reader - Erase/program protection of each sector when DRP is 0"]
pub type Wp1R = crate::FieldReader<u16>;
#[doc = "Field `WP1` writer - Erase/program protection of each sector when DRP is 0"]
pub type Wp1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 16:27 - Erase/program protection of each sector when DRP is 0"]
    #[inline(always)]
    pub fn wp1(&self) -> Wp1R {
        Wp1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - Erase/program protection of each sector when DRP is 0"]
    #[inline(always)]
    pub fn wp1(&mut self) -> Wp1W<'_, Obctl1Spec> {
        Wp1W::new(self, 16)
    }
}
#[doc = "Option byte control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`obctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Obctl1Spec;
impl crate::RegisterSpec for Obctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`obctl1::R`](R) reader structure"]
impl crate::Readable for Obctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`obctl1::W`](W) writer structure"]
impl crate::Writable for Obctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OBCTL1 to value 0x0fff_0000"]
impl crate::Resettable for Obctl1Spec {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
