#[doc = "Register `BAUD` reader"]
pub type R = crate::R<BaudSpec>;
#[doc = "Register `BAUD` writer"]
pub type W = crate::W<BaudSpec>;
#[doc = "Field `BRR_0_3` reader - Fraction of baud-rate divider"]
pub type Brr0_3R = crate::FieldReader;
#[doc = "Field `BRR_0_3` writer - Fraction of baud-rate divider"]
pub type Brr0_3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BRR_4_15` reader - Integer of baud-rate divider"]
pub type Brr4_15R = crate::FieldReader<u16>;
#[doc = "Field `BRR_4_15` writer - Integer of baud-rate divider"]
pub type Brr4_15W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:3 - Fraction of baud-rate divider"]
    #[inline(always)]
    pub fn brr_0_3(&self) -> Brr0_3R {
        Brr0_3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Integer of baud-rate divider"]
    #[inline(always)]
    pub fn brr_4_15(&self) -> Brr4_15R {
        Brr4_15R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Fraction of baud-rate divider"]
    #[inline(always)]
    pub fn brr_0_3(&mut self) -> Brr0_3W<'_, BaudSpec> {
        Brr0_3W::new(self, 0)
    }
    #[doc = "Bits 4:15 - Integer of baud-rate divider"]
    #[inline(always)]
    pub fn brr_4_15(&mut self) -> Brr4_15W<'_, BaudSpec> {
        Brr4_15W::new(self, 4)
    }
}
#[doc = "Baud rate generator register\n\nYou can [`read`](crate::Reg::read) this register and get [`baud::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baud::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaudSpec;
impl crate::RegisterSpec for BaudSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`baud::R`](R) reader structure"]
impl crate::Readable for BaudSpec {}
#[doc = "`write(|w| ..)` method takes [`baud::W`](W) writer structure"]
impl crate::Writable for BaudSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BAUD to value 0"]
impl crate::Resettable for BaudSpec {}
