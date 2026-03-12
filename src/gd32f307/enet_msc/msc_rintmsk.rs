#[doc = "Register `MSC_RINTMSK` reader"]
pub type R = crate::R<MscRintmskSpec>;
#[doc = "Register `MSC_RINTMSK` writer"]
pub type W = crate::W<MscRintmskSpec>;
#[doc = "Field `RFCEIM` reader - Received frame CRC error interrupt mask"]
pub type RfceimR = crate::BitReader;
#[doc = "Field `RFCEIM` writer - Received frame CRC error interrupt mask"]
pub type RfceimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFAEIM` reader - Received frames alignment error interrupt mask"]
pub type RfaeimR = crate::BitReader;
#[doc = "Field `RFAEIM` writer - Received frames alignment error interrupt mask"]
pub type RfaeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RGUFIM` reader - Received good unicast frames interrupt mask"]
pub type RgufimR = crate::BitReader;
#[doc = "Field `RGUFIM` writer - Received good unicast frames interrupt mask"]
pub type RgufimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Received frame CRC error interrupt mask"]
    #[inline(always)]
    pub fn rfceim(&self) -> RfceimR {
        RfceimR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Received frames alignment error interrupt mask"]
    #[inline(always)]
    pub fn rfaeim(&self) -> RfaeimR {
        RfaeimR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - Received good unicast frames interrupt mask"]
    #[inline(always)]
    pub fn rgufim(&self) -> RgufimR {
        RgufimR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Received frame CRC error interrupt mask"]
    #[inline(always)]
    pub fn rfceim(&mut self) -> RfceimW<'_, MscRintmskSpec> {
        RfceimW::new(self, 5)
    }
    #[doc = "Bit 6 - Received frames alignment error interrupt mask"]
    #[inline(always)]
    pub fn rfaeim(&mut self) -> RfaeimW<'_, MscRintmskSpec> {
        RfaeimW::new(self, 6)
    }
    #[doc = "Bit 17 - Received good unicast frames interrupt mask"]
    #[inline(always)]
    pub fn rgufim(&mut self) -> RgufimW<'_, MscRintmskSpec> {
        RgufimW::new(self, 17)
    }
}
#[doc = "Ethernet MSC receive interrupt mask register (MSC_RINTMSK)\n\nYou can [`read`](crate::Reg::read) this register and get [`msc_rintmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msc_rintmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MscRintmskSpec;
impl crate::RegisterSpec for MscRintmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msc_rintmsk::R`](R) reader structure"]
impl crate::Readable for MscRintmskSpec {}
#[doc = "`write(|w| ..)` method takes [`msc_rintmsk::W`](W) writer structure"]
impl crate::Writable for MscRintmskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSC_RINTMSK to value 0"]
impl crate::Resettable for MscRintmskSpec {}
