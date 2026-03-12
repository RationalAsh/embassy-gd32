#[doc = "Register `INTF` reader"]
pub type R = crate::R<IntfSpec>;
#[doc = "Register `INTF` writer"]
pub type W = crate::W<IntfSpec>;
#[doc = "Update interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Upif {
    #[doc = "0: No update interrupt occurred"]
    Clear = 0,
    #[doc = "1: Update interrupt occurred"]
    UpdatePending = 1,
}
impl From<Upif> for bool {
    #[inline(always)]
    fn from(variant: Upif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPIF` reader - Update interrupt flag"]
pub type UpifR = crate::BitReader<Upif>;
impl UpifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Upif {
        match self.bits {
            false => Upif::Clear,
            true => Upif::UpdatePending,
        }
    }
    #[doc = "No update interrupt occurred"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Upif::Clear
    }
    #[doc = "Update interrupt occurred"]
    #[inline(always)]
    pub fn is_update_pending(&self) -> bool {
        *self == Upif::UpdatePending
    }
}
#[doc = "Field `UPIF` writer - Update interrupt flag"]
pub type UpifW<'a, REG> = crate::BitWriter<'a, REG, Upif>;
impl<'a, REG> UpifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No update interrupt occurred"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Upif::Clear)
    }
    #[doc = "Update interrupt occurred"]
    #[inline(always)]
    pub fn update_pending(self) -> &'a mut crate::W<REG> {
        self.variant(Upif::UpdatePending)
    }
}
impl R {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn upif(&self) -> UpifR {
        UpifR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn upif(&mut self) -> UpifW<'_, IntfSpec> {
        UpifW::new(self, 0)
    }
}
#[doc = "Interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`intf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfSpec;
impl crate::RegisterSpec for IntfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf::R`](R) reader structure"]
impl crate::Readable for IntfSpec {}
#[doc = "`write(|w| ..)` method takes [`intf::W`](W) writer structure"]
impl crate::Writable for IntfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for IntfSpec {}
