#[doc = "Register `DMAINTEN` reader"]
pub type R = crate::R<DmaintenSpec>;
#[doc = "Register `DMAINTEN` writer"]
pub type W = crate::W<DmaintenSpec>;
#[doc = "Update interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Upie {
    #[doc = "0: Update interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Update interrupt enabled."]
    Enabled = 1,
}
impl From<Upie> for bool {
    #[inline(always)]
    fn from(variant: Upie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPIE` reader - Update interrupt enable"]
pub type UpieR = crate::BitReader<Upie>;
impl UpieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Upie {
        match self.bits {
            false => Upie::Disabled,
            true => Upie::Enabled,
        }
    }
    #[doc = "Update interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Upie::Disabled
    }
    #[doc = "Update interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Upie::Enabled
    }
}
#[doc = "Field `UPIE` writer - Update interrupt enable"]
pub type UpieW<'a, REG> = crate::BitWriter<'a, REG, Upie>;
impl<'a, REG> UpieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update interrupt disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Upie::Disabled)
    }
    #[doc = "Update interrupt enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Upie::Enabled)
    }
}
#[doc = "Channel 0 capture/compare interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0ie {
    #[doc = "0: Channel 0 capture/compare interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Channel 0 capture/compare interrupt enabled."]
    Enabled = 1,
}
impl From<Ch0ie> for bool {
    #[inline(always)]
    fn from(variant: Ch0ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0IE` reader - Channel 0 capture/compare interrupt enable"]
pub type Ch0ieR = crate::BitReader<Ch0ie>;
impl Ch0ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0ie {
        match self.bits {
            false => Ch0ie::Disabled,
            true => Ch0ie::Enabled,
        }
    }
    #[doc = "Channel 0 capture/compare interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch0ie::Disabled
    }
    #[doc = "Channel 0 capture/compare interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch0ie::Enabled
    }
}
#[doc = "Field `CH0IE` writer - Channel 0 capture/compare interrupt enable"]
pub type Ch0ieW<'a, REG> = crate::BitWriter<'a, REG, Ch0ie>;
impl<'a, REG> Ch0ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 0 capture/compare interrupt disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0ie::Disabled)
    }
    #[doc = "Channel 0 capture/compare interrupt enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0ie::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn upie(&self) -> UpieR {
        UpieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch0ie(&self) -> Ch0ieR {
        Ch0ieR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn upie(&mut self) -> UpieW<'_, DmaintenSpec> {
        UpieW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch0ie(&mut self) -> Ch0ieW<'_, DmaintenSpec> {
        Ch0ieW::new(self, 1)
    }
}
#[doc = "DMA/Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmainten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmainten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaintenSpec;
impl crate::RegisterSpec for DmaintenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmainten::R`](R) reader structure"]
impl crate::Readable for DmaintenSpec {}
#[doc = "`write(|w| ..)` method takes [`dmainten::W`](W) writer structure"]
impl crate::Writable for DmaintenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMAINTEN to value 0"]
impl crate::Resettable for DmaintenSpec {}
