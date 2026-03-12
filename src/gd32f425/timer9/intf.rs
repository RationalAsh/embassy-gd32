#[doc = "Register `INTF` reader"]
pub type R = crate::R<IntfSpec>;
#[doc = "Register `INTF` writer"]
pub type W = crate::W<IntfSpec>;
#[doc = "Update interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Upif {
    #[doc = "0: No update interrupt occurred."]
    Clear = 0,
    #[doc = "1: Update interrupt pending."]
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
    #[doc = "No update interrupt occurred."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Upif::Clear
    }
    #[doc = "Update interrupt pending."]
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
    #[doc = "No update interrupt occurred."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Upif::Clear)
    }
    #[doc = "Update interrupt pending."]
    #[inline(always)]
    pub fn update_pending(self) -> &'a mut crate::W<REG> {
        self.variant(Upif::UpdatePending)
    }
}
#[doc = "Channel 0 capture/compare interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0if {
    #[doc = "0: No channel 0 capture/compare interrupt enable."]
    Clear = 0,
    #[doc = "1: Channel 0 capture/compare interrupt enable."]
    CapturePending = 1,
}
impl From<Ch0if> for bool {
    #[inline(always)]
    fn from(variant: Ch0if) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0IF` reader - Channel 0 capture/compare interrupt flag"]
pub type Ch0ifR = crate::BitReader<Ch0if>;
impl Ch0ifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0if {
        match self.bits {
            false => Ch0if::Clear,
            true => Ch0if::CapturePending,
        }
    }
    #[doc = "No channel 0 capture/compare interrupt enable."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Ch0if::Clear
    }
    #[doc = "Channel 0 capture/compare interrupt enable."]
    #[inline(always)]
    pub fn is_capture_pending(&self) -> bool {
        *self == Ch0if::CapturePending
    }
}
#[doc = "Field `CH0IF` writer - Channel 0 capture/compare interrupt flag"]
pub type Ch0ifW<'a, REG> = crate::BitWriter<'a, REG, Ch0if>;
impl<'a, REG> Ch0ifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel 0 capture/compare interrupt enable."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0if::Clear)
    }
    #[doc = "Channel 0 capture/compare interrupt enable."]
    #[inline(always)]
    pub fn capture_pending(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0if::CapturePending)
    }
}
#[doc = "Channel 0 over capture flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0of {
    #[doc = "0: No channel 0 overflow/underflow interrupt occurred."]
    Clear = 0,
    #[doc = "1: Channel 0 overflow/underflow interrupt pending."]
    OverflowPending = 1,
}
impl From<Ch0of> for bool {
    #[inline(always)]
    fn from(variant: Ch0of) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0OF` reader - Channel 0 over capture flag"]
pub type Ch0ofR = crate::BitReader<Ch0of>;
impl Ch0ofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0of {
        match self.bits {
            false => Ch0of::Clear,
            true => Ch0of::OverflowPending,
        }
    }
    #[doc = "No channel 0 overflow/underflow interrupt occurred."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Ch0of::Clear
    }
    #[doc = "Channel 0 overflow/underflow interrupt pending."]
    #[inline(always)]
    pub fn is_overflow_pending(&self) -> bool {
        *self == Ch0of::OverflowPending
    }
}
#[doc = "Field `CH0OF` writer - Channel 0 over capture flag"]
pub type Ch0ofW<'a, REG> = crate::BitWriter<'a, REG, Ch0of>;
impl<'a, REG> Ch0ofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel 0 overflow/underflow interrupt occurred."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0of::Clear)
    }
    #[doc = "Channel 0 overflow/underflow interrupt pending."]
    #[inline(always)]
    pub fn overflow_pending(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0of::OverflowPending)
    }
}
impl R {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn upif(&self) -> UpifR {
        UpifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ch0if(&self) -> Ch0ifR {
        Ch0ifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 0 over capture flag"]
    #[inline(always)]
    pub fn ch0of(&self) -> Ch0ofR {
        Ch0ofR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn upif(&mut self) -> UpifW<'_, IntfSpec> {
        UpifW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ch0if(&mut self) -> Ch0ifW<'_, IntfSpec> {
        Ch0ifW::new(self, 1)
    }
    #[doc = "Bit 9 - Channel 0 over capture flag"]
    #[inline(always)]
    pub fn ch0of(&mut self) -> Ch0ofW<'_, IntfSpec> {
        Ch0ofW::new(self, 9)
    }
}
#[doc = "interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`intf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
