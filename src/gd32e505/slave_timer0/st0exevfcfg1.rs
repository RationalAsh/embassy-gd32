#[doc = "Register `ST0EXEVFCFG1` reader"]
pub type R = crate::R<St0exevfcfg1Spec>;
#[doc = "Register `ST0EXEVFCFG1` writer"]
pub type W = crate::W<St0exevfcfg1Spec>;
#[doc = "Field `EXEV5MEEN` reader - External event 5 memorized enable"]
pub type Exev5meenR = crate::BitReader;
#[doc = "Field `EXEV5MEEN` writer - External event 5 memorized enable"]
pub type Exev5meenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXEV5FM` reader - External event 5 filter mode"]
pub type Exev5fmR = crate::FieldReader;
#[doc = "Field `EXEV5FM` writer - External event 5 filter mode"]
pub type Exev5fmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXEV6MEEN` reader - External event 6 memorized enable"]
pub type Exev6meenR = crate::BitReader;
#[doc = "Field `EXEV6MEEN` writer - External event 6 memorized enable"]
pub type Exev6meenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXEV6FM` reader - External event 6 filter mode"]
pub type Exev6fmR = crate::FieldReader;
#[doc = "Field `EXEV6FM` writer - External event 6 filter mode"]
pub type Exev6fmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXEV7MEEN` reader - External event 7 memorized enable"]
pub type Exev7meenR = crate::BitReader;
#[doc = "Field `EXEV7MEEN` writer - External event 7 memorized enable"]
pub type Exev7meenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXEV7FM` reader - External event 7 filter mode"]
pub type Exev7fmR = crate::FieldReader;
#[doc = "Field `EXEV7FM` writer - External event 7 filter mode"]
pub type Exev7fmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXEV8MEEN` reader - External event 8 memorized enable"]
pub type Exev8meenR = crate::BitReader;
#[doc = "Field `EXEV8MEEN` writer - External event 8 memorized enable"]
pub type Exev8meenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXEV8FM` reader - External event 8 filter mode"]
pub type Exev8fmR = crate::FieldReader;
#[doc = "Field `EXEV8FM` writer - External event 8 filter mode"]
pub type Exev8fmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXEV9MEEN` reader - External event 9 memorized enable"]
pub type Exev9meenR = crate::BitReader;
#[doc = "Field `EXEV9MEEN` writer - External event 9 memorized enable"]
pub type Exev9meenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXEV9FM` reader - External event 9 filter mode"]
pub type Exev9fmR = crate::FieldReader;
#[doc = "Field `EXEV9FM` writer - External event 9 filter mode"]
pub type Exev9fmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - External event 5 memorized enable"]
    #[inline(always)]
    pub fn exev5meen(&self) -> Exev5meenR {
        Exev5meenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - External event 5 filter mode"]
    #[inline(always)]
    pub fn exev5fm(&self) -> Exev5fmR {
        Exev5fmR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - External event 6 memorized enable"]
    #[inline(always)]
    pub fn exev6meen(&self) -> Exev6meenR {
        Exev6meenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:10 - External event 6 filter mode"]
    #[inline(always)]
    pub fn exev6fm(&self) -> Exev6fmR {
        Exev6fmR::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - External event 7 memorized enable"]
    #[inline(always)]
    pub fn exev7meen(&self) -> Exev7meenR {
        Exev7meenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - External event 7 filter mode"]
    #[inline(always)]
    pub fn exev7fm(&self) -> Exev7fmR {
        Exev7fmR::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - External event 8 memorized enable"]
    #[inline(always)]
    pub fn exev8meen(&self) -> Exev8meenR {
        Exev8meenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - External event 8 filter mode"]
    #[inline(always)]
    pub fn exev8fm(&self) -> Exev8fmR {
        Exev8fmR::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - External event 9 memorized enable"]
    #[inline(always)]
    pub fn exev9meen(&self) -> Exev9meenR {
        Exev9meenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - External event 9 filter mode"]
    #[inline(always)]
    pub fn exev9fm(&self) -> Exev9fmR {
        Exev9fmR::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External event 5 memorized enable"]
    #[inline(always)]
    pub fn exev5meen(&mut self) -> Exev5meenW<'_, St0exevfcfg1Spec> {
        Exev5meenW::new(self, 0)
    }
    #[doc = "Bits 1:4 - External event 5 filter mode"]
    #[inline(always)]
    pub fn exev5fm(&mut self) -> Exev5fmW<'_, St0exevfcfg1Spec> {
        Exev5fmW::new(self, 1)
    }
    #[doc = "Bit 6 - External event 6 memorized enable"]
    #[inline(always)]
    pub fn exev6meen(&mut self) -> Exev6meenW<'_, St0exevfcfg1Spec> {
        Exev6meenW::new(self, 6)
    }
    #[doc = "Bits 7:10 - External event 6 filter mode"]
    #[inline(always)]
    pub fn exev6fm(&mut self) -> Exev6fmW<'_, St0exevfcfg1Spec> {
        Exev6fmW::new(self, 7)
    }
    #[doc = "Bit 12 - External event 7 memorized enable"]
    #[inline(always)]
    pub fn exev7meen(&mut self) -> Exev7meenW<'_, St0exevfcfg1Spec> {
        Exev7meenW::new(self, 12)
    }
    #[doc = "Bits 13:16 - External event 7 filter mode"]
    #[inline(always)]
    pub fn exev7fm(&mut self) -> Exev7fmW<'_, St0exevfcfg1Spec> {
        Exev7fmW::new(self, 13)
    }
    #[doc = "Bit 18 - External event 8 memorized enable"]
    #[inline(always)]
    pub fn exev8meen(&mut self) -> Exev8meenW<'_, St0exevfcfg1Spec> {
        Exev8meenW::new(self, 18)
    }
    #[doc = "Bits 19:22 - External event 8 filter mode"]
    #[inline(always)]
    pub fn exev8fm(&mut self) -> Exev8fmW<'_, St0exevfcfg1Spec> {
        Exev8fmW::new(self, 19)
    }
    #[doc = "Bit 24 - External event 9 memorized enable"]
    #[inline(always)]
    pub fn exev9meen(&mut self) -> Exev9meenW<'_, St0exevfcfg1Spec> {
        Exev9meenW::new(self, 24)
    }
    #[doc = "Bits 25:28 - External event 9 filter mode"]
    #[inline(always)]
    pub fn exev9fm(&mut self) -> Exev9fmW<'_, St0exevfcfg1Spec> {
        Exev9fmW::new(self, 25)
    }
}
#[doc = "SHRTIMER Slave_TIMERx external event filter configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`st0exevfcfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st0exevfcfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St0exevfcfg1Spec;
impl crate::RegisterSpec for St0exevfcfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st0exevfcfg1::R`](R) reader structure"]
impl crate::Readable for St0exevfcfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`st0exevfcfg1::W`](W) writer structure"]
impl crate::Writable for St0exevfcfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ST0EXEVFCFG1 to value 0"]
impl crate::Resettable for St0exevfcfg1Spec {}
