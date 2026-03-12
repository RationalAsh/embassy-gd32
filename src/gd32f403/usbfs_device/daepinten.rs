#[doc = "Register `DAEPINTEN` reader"]
pub type R = crate::R<DaepintenSpec>;
#[doc = "Register `DAEPINTEN` writer"]
pub type W = crate::W<DaepintenSpec>;
#[doc = "Field `IEPIE` reader - IN EP interrupt interrupt enable bits"]
pub type IepieR = crate::FieldReader;
#[doc = "Field `IEPIE` writer - IN EP interrupt interrupt enable bits"]
pub type IepieW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OEPIE` reader - OUT endpoint interrupt enable bits"]
pub type OepieR = crate::FieldReader;
#[doc = "Field `OEPIE` writer - OUT endpoint interrupt enable bits"]
pub type OepieW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - IN EP interrupt interrupt enable bits"]
    #[inline(always)]
    pub fn iepie(&self) -> IepieR {
        IepieR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - OUT endpoint interrupt enable bits"]
    #[inline(always)]
    pub fn oepie(&self) -> OepieR {
        OepieR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - IN EP interrupt interrupt enable bits"]
    #[inline(always)]
    pub fn iepie(&mut self) -> IepieW<'_, DaepintenSpec> {
        IepieW::new(self, 0)
    }
    #[doc = "Bits 16:19 - OUT endpoint interrupt enable bits"]
    #[inline(always)]
    pub fn oepie(&mut self) -> OepieW<'_, DaepintenSpec> {
        OepieW::new(self, 16)
    }
}
#[doc = "Device all endpoints interrupt enable register (DAEPINTEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`daepinten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daepinten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaepintenSpec;
impl crate::RegisterSpec for DaepintenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daepinten::R`](R) reader structure"]
impl crate::Readable for DaepintenSpec {}
#[doc = "`write(|w| ..)` method takes [`daepinten::W`](W) writer structure"]
impl crate::Writable for DaepintenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAEPINTEN to value 0"]
impl crate::Resettable for DaepintenSpec {}
