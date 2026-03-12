#[doc = "Register `LPMINTF` reader"]
pub type R = crate::R<LpmintfSpec>;
#[doc = "Register `LPMINTF` writer"]
pub type W = crate::W<LpmintfSpec>;
#[doc = "Field `LPMSTIF` reader - LPM token Correct transfer interrupt flag"]
pub type LpmstifR = crate::BitReader;
#[doc = "Field `LPMSTIF` writer - LPM token Correct transfer interrupt flag"]
pub type LpmstifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 15 - LPM token Correct transfer interrupt flag"]
    #[inline(always)]
    pub fn lpmstif(&self) -> LpmstifR {
        LpmstifR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - LPM token Correct transfer interrupt flag"]
    #[inline(always)]
    pub fn lpmstif(&mut self) -> LpmstifW<'_, LpmintfSpec> {
        LpmstifW::new(self, 15)
    }
}
#[doc = "USB LPM interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpmintf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmintf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpmintfSpec;
impl crate::RegisterSpec for LpmintfSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lpmintf::R`](R) reader structure"]
impl crate::Readable for LpmintfSpec {}
#[doc = "`write(|w| ..)` method takes [`lpmintf::W`](W) writer structure"]
impl crate::Writable for LpmintfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPMINTF to value 0"]
impl crate::Resettable for LpmintfSpec {}
