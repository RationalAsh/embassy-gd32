#[doc = "Register `L1CKEY` reader"]
pub type R = crate::R<L1ckeySpec>;
#[doc = "Register `L1CKEY` writer"]
pub type W = crate::W<L1ckeySpec>;
#[doc = "Field `CKEYB` reader - Color Key Blue"]
pub type CkeybR = crate::FieldReader;
#[doc = "Field `CKEYB` writer - Color Key Blue"]
pub type CkeybW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CKEYG` reader - Color Key Green"]
pub type CkeygR = crate::FieldReader;
#[doc = "Field `CKEYG` writer - Color Key Green"]
pub type CkeygW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CKEYR` reader - Color Key Red"]
pub type CkeyrR = crate::FieldReader;
#[doc = "Field `CKEYR` writer - Color Key Red"]
pub type CkeyrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Color Key Blue"]
    #[inline(always)]
    pub fn ckeyb(&self) -> CkeybR {
        CkeybR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Color Key Green"]
    #[inline(always)]
    pub fn ckeyg(&self) -> CkeygR {
        CkeygR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Color Key Red"]
    #[inline(always)]
    pub fn ckeyr(&self) -> CkeyrR {
        CkeyrR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Color Key Blue"]
    #[inline(always)]
    pub fn ckeyb(&mut self) -> CkeybW<'_, L1ckeySpec> {
        CkeybW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Color Key Green"]
    #[inline(always)]
    pub fn ckeyg(&mut self) -> CkeygW<'_, L1ckeySpec> {
        CkeygW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Color Key Red"]
    #[inline(always)]
    pub fn ckeyr(&mut self) -> CkeyrW<'_, L1ckeySpec> {
        CkeyrW::new(self, 16)
    }
}
#[doc = "Layer 1 color key register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1ckey::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1ckey::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1ckeySpec;
impl crate::RegisterSpec for L1ckeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1ckey::R`](R) reader structure"]
impl crate::Readable for L1ckeySpec {}
#[doc = "`write(|w| ..)` method takes [`l1ckey::W`](W) writer structure"]
impl crate::Writable for L1ckeySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1CKEY to value 0"]
impl crate::Resettable for L1ckeySpec {}
