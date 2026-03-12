#[doc = "Register `SPSZ` reader"]
pub type R = crate::R<SpszSpec>;
#[doc = "Register `SPSZ` writer"]
pub type W = crate::W<SpszSpec>;
#[doc = "Field `VPSZ` reader - size of vertical synchronous pluse"]
pub type VpszR = crate::FieldReader<u16>;
#[doc = "Field `VPSZ` writer - size of vertical synchronous pluse"]
pub type VpszW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HPSZ` reader - size of horizontal synchronous pluse"]
pub type HpszR = crate::FieldReader<u16>;
#[doc = "Field `HPSZ` writer - size of horizontal synchronous pluse"]
pub type HpszW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - size of vertical synchronous pluse"]
    #[inline(always)]
    pub fn vpsz(&self) -> VpszR {
        VpszR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - size of horizontal synchronous pluse"]
    #[inline(always)]
    pub fn hpsz(&self) -> HpszR {
        HpszR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - size of vertical synchronous pluse"]
    #[inline(always)]
    pub fn vpsz(&mut self) -> VpszW<'_, SpszSpec> {
        VpszW::new(self, 0)
    }
    #[doc = "Bits 16:27 - size of horizontal synchronous pluse"]
    #[inline(always)]
    pub fn hpsz(&mut self) -> HpszW<'_, SpszSpec> {
        HpszW::new(self, 16)
    }
}
#[doc = "Synchronous pulse size register\n\nYou can [`read`](crate::Reg::read) this register and get [`spsz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spsz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpszSpec;
impl crate::RegisterSpec for SpszSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spsz::R`](R) reader structure"]
impl crate::Readable for SpszSpec {}
#[doc = "`write(|w| ..)` method takes [`spsz::W`](W) writer structure"]
impl crate::Writable for SpszSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPSZ to value 0"]
impl crate::Resettable for SpszSpec {}
