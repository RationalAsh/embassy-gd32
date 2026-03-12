#[doc = "Register `CS` reader"]
pub type R = crate::R<CsSpec>;
#[doc = "Register `CS` writer"]
pub type W = crate::W<CsSpec>;
#[doc = "Field `STLO` reader - Start lost flag"]
pub type StloR = crate::BitReader;
#[doc = "Field `STLO` writer - Start lost flag"]
pub type StloW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STPSEND` reader - Stop condition sent out in master mode"]
pub type StpsendR = crate::BitReader;
#[doc = "Field `STPSEND` writer - Stop condition sent out in master mode"]
pub type StpsendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STLOIE` reader - Interrupt enable for start lost"]
pub type StloieR = crate::BitReader;
#[doc = "Field `STLOIE` writer - Interrupt enable for start lost"]
pub type StloieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STPSENDIE` reader - Interrupt enable for stop condition sent"]
pub type StpsendieR = crate::BitReader;
#[doc = "Field `STPSENDIE` writer - Interrupt enable for stop condition sent"]
pub type StpsendieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start lost flag"]
    #[inline(always)]
    pub fn stlo(&self) -> StloR {
        StloR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stop condition sent out in master mode"]
    #[inline(always)]
    pub fn stpsend(&self) -> StpsendR {
        StpsendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt enable for start lost"]
    #[inline(always)]
    pub fn stloie(&self) -> StloieR {
        StloieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt enable for stop condition sent"]
    #[inline(always)]
    pub fn stpsendie(&self) -> StpsendieR {
        StpsendieR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start lost flag"]
    #[inline(always)]
    pub fn stlo(&mut self) -> StloW<'_, CsSpec> {
        StloW::new(self, 0)
    }
    #[doc = "Bit 1 - Stop condition sent out in master mode"]
    #[inline(always)]
    pub fn stpsend(&mut self) -> StpsendW<'_, CsSpec> {
        StpsendW::new(self, 1)
    }
    #[doc = "Bit 8 - Interrupt enable for start lost"]
    #[inline(always)]
    pub fn stloie(&mut self) -> StloieW<'_, CsSpec> {
        StloieW::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt enable for stop condition sent"]
    #[inline(always)]
    pub fn stpsendie(&mut self) -> StpsendieW<'_, CsSpec> {
        StpsendieW::new(self, 9)
    }
}
#[doc = "Control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsSpec;
impl crate::RegisterSpec for CsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs::R`](R) reader structure"]
impl crate::Readable for CsSpec {}
#[doc = "`write(|w| ..)` method takes [`cs::W`](W) writer structure"]
impl crate::Writable for CsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CS to value 0"]
impl crate::Resettable for CsSpec {}
