#[doc = "Register `WDHT` reader"]
pub type R = crate::R<WdhtSpec>;
#[doc = "Register `WDHT` writer"]
pub type W = crate::W<WdhtSpec>;
#[doc = "Field `WDHT0` reader - Analog watchdog 0 higher threshold"]
pub type Wdht0R = crate::FieldReader<u16>;
#[doc = "Field `WDHT0` writer - Analog watchdog 0 higher threshold"]
pub type Wdht0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Analog watchdog 0 higher threshold"]
    #[inline(always)]
    pub fn wdht0(&self) -> Wdht0R {
        Wdht0R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog 0 higher threshold"]
    #[inline(always)]
    pub fn wdht0(&mut self) -> Wdht0W<'_, WdhtSpec> {
        Wdht0W::new(self, 0)
    }
}
#[doc = "watchdog higher threshold register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`wdht::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdht::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdhtSpec;
impl crate::RegisterSpec for WdhtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdht::R`](R) reader structure"]
impl crate::Readable for WdhtSpec {}
#[doc = "`write(|w| ..)` method takes [`wdht::W`](W) writer structure"]
impl crate::Writable for WdhtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDHT to value 0x0fff"]
impl crate::Resettable for WdhtSpec {
    const RESET_VALUE: u32 = 0x0fff;
}
