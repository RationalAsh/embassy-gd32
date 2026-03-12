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
#[doc = "Channel 1 capture/compare interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1ie {
    #[doc = "0: Channel 1 capture/compare interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Channel 1 capture/compare interrupt enabled."]
    Enabled = 1,
}
impl From<Ch1ie> for bool {
    #[inline(always)]
    fn from(variant: Ch1ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1IE` reader - Channel 1 capture/compare interrupt enable"]
pub type Ch1ieR = crate::BitReader<Ch1ie>;
impl Ch1ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1ie {
        match self.bits {
            false => Ch1ie::Disabled,
            true => Ch1ie::Enabled,
        }
    }
    #[doc = "Channel 1 capture/compare interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch1ie::Disabled
    }
    #[doc = "Channel 1 capture/compare interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch1ie::Enabled
    }
}
#[doc = "Field `CH1IE` writer - Channel 1 capture/compare interrupt enable"]
pub type Ch1ieW<'a, REG> = crate::BitWriter<'a, REG, Ch1ie>;
impl<'a, REG> Ch1ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 1 capture/compare interrupt disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1ie::Disabled)
    }
    #[doc = "Channel 1 capture/compare interrupt enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1ie::Enabled)
    }
}
#[doc = "Channel 2 capture/compare interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2ie {
    #[doc = "0: Channel 2 capture/compare interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Channel 2 capture/compare interrupt enabled."]
    Enabled = 1,
}
impl From<Ch2ie> for bool {
    #[inline(always)]
    fn from(variant: Ch2ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2IE` reader - Channel 2 capture/compare interrupt enable"]
pub type Ch2ieR = crate::BitReader<Ch2ie>;
impl Ch2ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2ie {
        match self.bits {
            false => Ch2ie::Disabled,
            true => Ch2ie::Enabled,
        }
    }
    #[doc = "Channel 2 capture/compare interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch2ie::Disabled
    }
    #[doc = "Channel 2 capture/compare interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch2ie::Enabled
    }
}
#[doc = "Field `CH2IE` writer - Channel 2 capture/compare interrupt enable"]
pub type Ch2ieW<'a, REG> = crate::BitWriter<'a, REG, Ch2ie>;
impl<'a, REG> Ch2ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 2 capture/compare interrupt disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2ie::Disabled)
    }
    #[doc = "Channel 2 capture/compare interrupt enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2ie::Enabled)
    }
}
#[doc = "Channel 3 capture/compare interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3ie {
    #[doc = "0: Channel 3 capture/compare interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Channel 3 capture/compare interrupt enabled."]
    Enabled = 1,
}
impl From<Ch3ie> for bool {
    #[inline(always)]
    fn from(variant: Ch3ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3IE` reader - Channel 3 capture/compare interrupt enable"]
pub type Ch3ieR = crate::BitReader<Ch3ie>;
impl Ch3ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3ie {
        match self.bits {
            false => Ch3ie::Disabled,
            true => Ch3ie::Enabled,
        }
    }
    #[doc = "Channel 3 capture/compare interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch3ie::Disabled
    }
    #[doc = "Channel 3 capture/compare interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch3ie::Enabled
    }
}
#[doc = "Field `CH3IE` writer - Channel 3 capture/compare interrupt enable"]
pub type Ch3ieW<'a, REG> = crate::BitWriter<'a, REG, Ch3ie>;
impl<'a, REG> Ch3ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 3 capture/compare interrupt disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3ie::Disabled)
    }
    #[doc = "Channel 3 capture/compare interrupt enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3ie::Enabled)
    }
}
#[doc = "Trigger interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trgie {
    #[doc = "0: Trigger interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Trigger interrupt enabled."]
    Enabled = 1,
}
impl From<Trgie> for bool {
    #[inline(always)]
    fn from(variant: Trgie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGIE` reader - Trigger interrupt enable"]
pub type TrgieR = crate::BitReader<Trgie>;
impl TrgieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgie {
        match self.bits {
            false => Trgie::Disabled,
            true => Trgie::Enabled,
        }
    }
    #[doc = "Trigger interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Trgie::Disabled
    }
    #[doc = "Trigger interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Trgie::Enabled
    }
}
#[doc = "Field `TRGIE` writer - Trigger interrupt enable"]
pub type TrgieW<'a, REG> = crate::BitWriter<'a, REG, Trgie>;
impl<'a, REG> TrgieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger interrupt disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trgie::Disabled)
    }
    #[doc = "Trigger interrupt enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trgie::Enabled)
    }
}
#[doc = "Update DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Upden {
    #[doc = "0: Update DMA request disabled."]
    Disabled = 0,
    #[doc = "1: Update DMA request enabled."]
    Enabled = 1,
}
impl From<Upden> for bool {
    #[inline(always)]
    fn from(variant: Upden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPDEN` reader - Update DMA request enable"]
pub type UpdenR = crate::BitReader<Upden>;
impl UpdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Upden {
        match self.bits {
            false => Upden::Disabled,
            true => Upden::Enabled,
        }
    }
    #[doc = "Update DMA request disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Upden::Disabled
    }
    #[doc = "Update DMA request enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Upden::Enabled
    }
}
#[doc = "Field `UPDEN` writer - Update DMA request enable"]
pub type UpdenW<'a, REG> = crate::BitWriter<'a, REG, Upden>;
impl<'a, REG> UpdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update DMA request disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Upden::Disabled)
    }
    #[doc = "Update DMA request enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Upden::Enabled)
    }
}
#[doc = "Channel 0 capture/compare DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0den {
    #[doc = "0: Channel 0 capture/compare DMA request disabled."]
    Disabled = 0,
    #[doc = "1: Channel 0 capture/compare DMA request enabled."]
    Enabled = 1,
}
impl From<Ch0den> for bool {
    #[inline(always)]
    fn from(variant: Ch0den) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0DEN` reader - Channel 0 capture/compare DMA request enable"]
pub type Ch0denR = crate::BitReader<Ch0den>;
impl Ch0denR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0den {
        match self.bits {
            false => Ch0den::Disabled,
            true => Ch0den::Enabled,
        }
    }
    #[doc = "Channel 0 capture/compare DMA request disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch0den::Disabled
    }
    #[doc = "Channel 0 capture/compare DMA request enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch0den::Enabled
    }
}
#[doc = "Field `CH0DEN` writer - Channel 0 capture/compare DMA request enable"]
pub type Ch0denW<'a, REG> = crate::BitWriter<'a, REG, Ch0den>;
impl<'a, REG> Ch0denW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 0 capture/compare DMA request disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0den::Disabled)
    }
    #[doc = "Channel 0 capture/compare DMA request enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0den::Enabled)
    }
}
#[doc = "Channel 1 capture/compare DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1den {
    #[doc = "0: Channel 1 capture/compare DMA request disabled."]
    Disabled = 0,
    #[doc = "1: Channel 1 capture/compare DMA request enabled."]
    Enabled = 1,
}
impl From<Ch1den> for bool {
    #[inline(always)]
    fn from(variant: Ch1den) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1DEN` reader - Channel 1 capture/compare DMA request enable"]
pub type Ch1denR = crate::BitReader<Ch1den>;
impl Ch1denR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1den {
        match self.bits {
            false => Ch1den::Disabled,
            true => Ch1den::Enabled,
        }
    }
    #[doc = "Channel 1 capture/compare DMA request disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch1den::Disabled
    }
    #[doc = "Channel 1 capture/compare DMA request enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch1den::Enabled
    }
}
#[doc = "Field `CH1DEN` writer - Channel 1 capture/compare DMA request enable"]
pub type Ch1denW<'a, REG> = crate::BitWriter<'a, REG, Ch1den>;
impl<'a, REG> Ch1denW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 1 capture/compare DMA request disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1den::Disabled)
    }
    #[doc = "Channel 1 capture/compare DMA request enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1den::Enabled)
    }
}
#[doc = "Channel 2 capture/compare DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2den {
    #[doc = "0: Channel 2 capture/compare DMA request disabled."]
    Disabled = 0,
    #[doc = "1: Channel 2 capture/compare DMA request enabled."]
    Enabled = 1,
}
impl From<Ch2den> for bool {
    #[inline(always)]
    fn from(variant: Ch2den) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2DEN` reader - Channel 2 capture/compare DMA request enable"]
pub type Ch2denR = crate::BitReader<Ch2den>;
impl Ch2denR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2den {
        match self.bits {
            false => Ch2den::Disabled,
            true => Ch2den::Enabled,
        }
    }
    #[doc = "Channel 2 capture/compare DMA request disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch2den::Disabled
    }
    #[doc = "Channel 2 capture/compare DMA request enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch2den::Enabled
    }
}
#[doc = "Field `CH2DEN` writer - Channel 2 capture/compare DMA request enable"]
pub type Ch2denW<'a, REG> = crate::BitWriter<'a, REG, Ch2den>;
impl<'a, REG> Ch2denW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 2 capture/compare DMA request disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2den::Disabled)
    }
    #[doc = "Channel 2 capture/compare DMA request enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2den::Enabled)
    }
}
#[doc = "Channel 3 capture/compare DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3den {
    #[doc = "0: Channel 3 capture/compare DMA request disabled."]
    Disabled = 0,
    #[doc = "1: Channel 3 capture/compare DMA request enabled."]
    Enabled = 1,
}
impl From<Ch3den> for bool {
    #[inline(always)]
    fn from(variant: Ch3den) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3DEN` reader - Channel 3 capture/compare DMA request enable"]
pub type Ch3denR = crate::BitReader<Ch3den>;
impl Ch3denR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3den {
        match self.bits {
            false => Ch3den::Disabled,
            true => Ch3den::Enabled,
        }
    }
    #[doc = "Channel 3 capture/compare DMA request disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch3den::Disabled
    }
    #[doc = "Channel 3 capture/compare DMA request enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch3den::Enabled
    }
}
#[doc = "Field `CH3DEN` writer - Channel 3 capture/compare DMA request enable"]
pub type Ch3denW<'a, REG> = crate::BitWriter<'a, REG, Ch3den>;
impl<'a, REG> Ch3denW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 3 capture/compare DMA request disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3den::Disabled)
    }
    #[doc = "Channel 3 capture/compare DMA request enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3den::Enabled)
    }
}
#[doc = "Trigger DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trgden {
    #[doc = "0: Trigger DMA request disabled."]
    Disabled = 0,
    #[doc = "1: Trigger DMA request enabled."]
    Enabled = 1,
}
impl From<Trgden> for bool {
    #[inline(always)]
    fn from(variant: Trgden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGDEN` reader - Trigger DMA request enable"]
pub type TrgdenR = crate::BitReader<Trgden>;
impl TrgdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgden {
        match self.bits {
            false => Trgden::Disabled,
            true => Trgden::Enabled,
        }
    }
    #[doc = "Trigger DMA request disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Trgden::Disabled
    }
    #[doc = "Trigger DMA request enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Trgden::Enabled
    }
}
#[doc = "Field `TRGDEN` writer - Trigger DMA request enable"]
pub type TrgdenW<'a, REG> = crate::BitWriter<'a, REG, Trgden>;
impl<'a, REG> TrgdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger DMA request disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trgden::Disabled)
    }
    #[doc = "Trigger DMA request enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trgden::Enabled)
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
    #[doc = "Bit 2 - Channel 1 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch1ie(&self) -> Ch1ieR {
        Ch1ieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 2 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch2ie(&self) -> Ch2ieR {
        Ch2ieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 3 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch3ie(&self) -> Ch3ieR {
        Ch3ieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn trgie(&self) -> TrgieR {
        TrgieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn upden(&self) -> UpdenR {
        UpdenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 0 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch0den(&self) -> Ch0denR {
        Ch0denR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 1 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch1den(&self) -> Ch1denR {
        Ch1denR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 2 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch2den(&self) -> Ch2denR {
        Ch2denR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 3 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch3den(&self) -> Ch3denR {
        Ch3denR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn trgden(&self) -> TrgdenR {
        TrgdenR::new(((self.bits >> 14) & 1) != 0)
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
    #[doc = "Bit 2 - Channel 1 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch1ie(&mut self) -> Ch1ieW<'_, DmaintenSpec> {
        Ch1ieW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 2 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch2ie(&mut self) -> Ch2ieW<'_, DmaintenSpec> {
        Ch2ieW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 3 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch3ie(&mut self) -> Ch3ieW<'_, DmaintenSpec> {
        Ch3ieW::new(self, 4)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn trgie(&mut self) -> TrgieW<'_, DmaintenSpec> {
        TrgieW::new(self, 6)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn upden(&mut self) -> UpdenW<'_, DmaintenSpec> {
        UpdenW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 0 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch0den(&mut self) -> Ch0denW<'_, DmaintenSpec> {
        Ch0denW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 1 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch1den(&mut self) -> Ch1denW<'_, DmaintenSpec> {
        Ch1denW::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 2 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch2den(&mut self) -> Ch2denW<'_, DmaintenSpec> {
        Ch2denW::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 3 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch3den(&mut self) -> Ch3denW<'_, DmaintenSpec> {
        Ch3denW::new(self, 12)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn trgden(&mut self) -> TrgdenW<'_, DmaintenSpec> {
        TrgdenW::new(self, 14)
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
