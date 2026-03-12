#[doc = "Register `ST0ACTL` reader"]
pub type R = crate::R<St0actlSpec>;
#[doc = "Register `ST0ACTL` writer"]
pub type W = crate::W<St0actlSpec>;
#[doc = "Field `CNTCKDIV_3` reader - Counter clock division"]
pub type Cntckdiv3R = crate::BitReader;
#[doc = "Field `CNTCKDIV_3` writer - Counter clock division"]
pub type Cntckdiv3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTRCFG_15_9` reader - Rising edge dead-time value configure"]
pub type Dtrcfg15_9R = crate::FieldReader;
#[doc = "Field `DTRCFG_15_9` writer - Rising edge dead-time value configure"]
pub type Dtrcfg15_9W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DTFCFG_15_9` reader - Falling edge dead-time value configure"]
pub type Dtfcfg15_9R = crate::FieldReader;
#[doc = "Field `DTFCFG_15_9` writer - Falling edge dead-time value configure"]
pub type Dtfcfg15_9W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 3 - Counter clock division"]
    #[inline(always)]
    pub fn cntckdiv_3(&self) -> Cntckdiv3R {
        Cntckdiv3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Rising edge dead-time value configure"]
    #[inline(always)]
    pub fn dtrcfg_15_9(&self) -> Dtrcfg15_9R {
        Dtrcfg15_9R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 25:31 - Falling edge dead-time value configure"]
    #[inline(always)]
    pub fn dtfcfg_15_9(&self) -> Dtfcfg15_9R {
        Dtfcfg15_9R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - Counter clock division"]
    #[inline(always)]
    pub fn cntckdiv_3(&mut self) -> Cntckdiv3W<'_, St0actlSpec> {
        Cntckdiv3W::new(self, 3)
    }
    #[doc = "Bits 9:15 - Rising edge dead-time value configure"]
    #[inline(always)]
    pub fn dtrcfg_15_9(&mut self) -> Dtrcfg15_9W<'_, St0actlSpec> {
        Dtrcfg15_9W::new(self, 9)
    }
    #[doc = "Bits 25:31 - Falling edge dead-time value configure"]
    #[inline(always)]
    pub fn dtfcfg_15_9(&mut self) -> Dtfcfg15_9W<'_, St0actlSpec> {
        Dtfcfg15_9W::new(self, 25)
    }
}
#[doc = "SHRTIMER Slave_TIMERx additional control register\n\nYou can [`read`](crate::Reg::read) this register and get [`st0actl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st0actl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St0actlSpec;
impl crate::RegisterSpec for St0actlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st0actl::R`](R) reader structure"]
impl crate::Readable for St0actlSpec {}
#[doc = "`write(|w| ..)` method takes [`st0actl::W`](W) writer structure"]
impl crate::Writable for St0actlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ST0ACTL to value 0"]
impl crate::Resettable for St0actlSpec {}
