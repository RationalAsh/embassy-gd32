#[doc = "Register `SWEVG` writer"]
pub type W = crate::W<SwevgSpec>;
#[doc = "Update generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Upg {
    #[doc = "1: This bit can be set by software, and cleared by hardware automatically. When this bit is set, the counter is cleared if the center-aligned or up counting mode is selected, else (down counting) it takes the auto-reload value. The prescaler counter is cleared at the same time."]
    Update = 1,
}
impl From<Upg> for bool {
    #[inline(always)]
    fn from(variant: Upg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPG` writer - Update generation"]
pub type UpgW<'a, REG> = crate::BitWriter<'a, REG, Upg>;
impl<'a, REG> UpgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This bit can be set by software, and cleared by hardware automatically. When this bit is set, the counter is cleared if the center-aligned or up counting mode is selected, else (down counting) it takes the auto-reload value. The prescaler counter is cleared at the same time."]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(Upg::Update)
    }
}
#[doc = "Channel 0 capture or compare event generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0g {
    #[doc = "1: Channel 0’s capture or compare event generation."]
    Channel0 = 1,
}
impl From<Ch0g> for bool {
    #[inline(always)]
    fn from(variant: Ch0g) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0G` writer - Channel 0 capture or compare event generation"]
pub type Ch0gW<'a, REG> = crate::BitWriter<'a, REG, Ch0g>;
impl<'a, REG> Ch0gW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 0’s capture or compare event generation."]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0g::Channel0)
    }
}
#[doc = "Channel 1 capture or compare event generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1g {
    #[doc = "1: Channel 1’s capture or compare event generation."]
    Channel1 = 1,
}
impl From<Ch1g> for bool {
    #[inline(always)]
    fn from(variant: Ch1g) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1G` writer - Channel 1 capture or compare event generation"]
pub type Ch1gW<'a, REG> = crate::BitWriter<'a, REG, Ch1g>;
impl<'a, REG> Ch1gW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 1’s capture or compare event generation."]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1g::Channel1)
    }
}
#[doc = "Channel 2 capture or compare event generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2g {
    #[doc = "1: Channel 2’s capture or compare event generation."]
    Channel2 = 1,
}
impl From<Ch2g> for bool {
    #[inline(always)]
    fn from(variant: Ch2g) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2G` writer - Channel 2 capture or compare event generation"]
pub type Ch2gW<'a, REG> = crate::BitWriter<'a, REG, Ch2g>;
impl<'a, REG> Ch2gW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 2’s capture or compare event generation."]
    #[inline(always)]
    pub fn channel2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2g::Channel2)
    }
}
#[doc = "Channel 3 capture or compare event generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3g {
    #[doc = "1: Channel 3’s capture or compare event generation."]
    Channel3 = 1,
}
impl From<Ch3g> for bool {
    #[inline(always)]
    fn from(variant: Ch3g) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3G` writer - Channel 3 capture or compare event generation"]
pub type Ch3gW<'a, REG> = crate::BitWriter<'a, REG, Ch3g>;
impl<'a, REG> Ch3gW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 3’s capture or compare event generation."]
    #[inline(always)]
    pub fn channel3(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3g::Channel3)
    }
}
#[doc = "Trigger event generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trgg {
    #[doc = "1: Trigger event generation This bit is set by software and cleared by hardware automatically. When this bit is set, the TRGIF flag in TIMERx_STAT register is set, related interrupt or DMA transfer can occur if enabled."]
    Trigger = 1,
}
impl From<Trgg> for bool {
    #[inline(always)]
    fn from(variant: Trgg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGG` writer - Trigger event generation"]
pub type TrggW<'a, REG> = crate::BitWriter<'a, REG, Trgg>;
impl<'a, REG> TrggW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger event generation This bit is set by software and cleared by hardware automatically. When this bit is set, the TRGIF flag in TIMERx_STAT register is set, related interrupt or DMA transfer can occur if enabled."]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgg::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    pub fn upg(&mut self) -> UpgW<'_, SwevgSpec> {
        UpgW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 0 capture or compare event generation"]
    #[inline(always)]
    pub fn ch0g(&mut self) -> Ch0gW<'_, SwevgSpec> {
        Ch0gW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 1 capture or compare event generation"]
    #[inline(always)]
    pub fn ch1g(&mut self) -> Ch1gW<'_, SwevgSpec> {
        Ch1gW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 2 capture or compare event generation"]
    #[inline(always)]
    pub fn ch2g(&mut self) -> Ch2gW<'_, SwevgSpec> {
        Ch2gW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 3 capture or compare event generation"]
    #[inline(always)]
    pub fn ch3g(&mut self) -> Ch3gW<'_, SwevgSpec> {
        Ch3gW::new(self, 4)
    }
    #[doc = "Bit 6 - Trigger event generation"]
    #[inline(always)]
    pub fn trgg(&mut self) -> TrggW<'_, SwevgSpec> {
        TrggW::new(self, 6)
    }
}
#[doc = "event generation register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swevg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwevgSpec;
impl crate::RegisterSpec for SwevgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swevg::W`](W) writer structure"]
impl crate::Writable for SwevgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWEVG to value 0"]
impl crate::Resettable for SwevgSpec {}
