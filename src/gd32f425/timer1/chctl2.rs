#[doc = "Register `CHCTL2` reader"]
pub type R = crate::R<Chctl2Spec>;
#[doc = "Register `CHCTL2` writer"]
pub type W = crate::W<Chctl2Spec>;
#[doc = "Channel 0 capture/compare function enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0en {
    #[doc = "0: Channel 0 capture / compare disabled."]
    Disabled = 0,
    #[doc = "1: Channel 0 capture / compare enabled."]
    Enabled = 1,
}
impl From<Ch0en> for bool {
    #[inline(always)]
    fn from(variant: Ch0en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0EN` reader - Channel 0 capture/compare function enable"]
pub type Ch0enR = crate::BitReader<Ch0en>;
impl Ch0enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0en {
        match self.bits {
            false => Ch0en::Disabled,
            true => Ch0en::Enabled,
        }
    }
    #[doc = "Channel 0 capture / compare disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch0en::Disabled
    }
    #[doc = "Channel 0 capture / compare enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch0en::Enabled
    }
}
#[doc = "Field `CH0EN` writer - Channel 0 capture/compare function enable"]
pub type Ch0enW<'a, REG> = crate::BitWriter<'a, REG, Ch0en>;
impl<'a, REG> Ch0enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 0 capture / compare disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0en::Disabled)
    }
    #[doc = "Channel 0 capture / compare enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0en::Enabled)
    }
}
#[doc = "Channel 0 capture/compare function polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0p {
    #[doc = "0: Channel 0 capture / compare polarity active high."]
    Active = 0,
    #[doc = "1: Channel 0 capture / compare polarity active low."]
    Inactive = 1,
}
impl From<Ch0p> for bool {
    #[inline(always)]
    fn from(variant: Ch0p) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0P` reader - Channel 0 capture/compare function polarity"]
pub type Ch0pR = crate::BitReader<Ch0p>;
impl Ch0pR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0p {
        match self.bits {
            false => Ch0p::Active,
            true => Ch0p::Inactive,
        }
    }
    #[doc = "Channel 0 capture / compare polarity active high."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ch0p::Active
    }
    #[doc = "Channel 0 capture / compare polarity active low."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ch0p::Inactive
    }
}
#[doc = "Field `CH0P` writer - Channel 0 capture/compare function polarity"]
pub type Ch0pW<'a, REG> = crate::BitWriter<'a, REG, Ch0p>;
impl<'a, REG> Ch0pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 0 capture / compare polarity active high."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0p::Active)
    }
    #[doc = "Channel 0 capture / compare polarity active low."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0p::Inactive)
    }
}
#[doc = "Channel 0 complementary output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0np {
    #[doc = "0: Channel 0 complementary capture / compare polarity active high."]
    Active = 0,
    #[doc = "1: Channel 0 complementary capture / compare polarity active low."]
    Inactive = 1,
}
impl From<Ch0np> for bool {
    #[inline(always)]
    fn from(variant: Ch0np) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0NP` reader - Channel 0 complementary output polarity"]
pub type Ch0npR = crate::BitReader<Ch0np>;
impl Ch0npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0np {
        match self.bits {
            false => Ch0np::Active,
            true => Ch0np::Inactive,
        }
    }
    #[doc = "Channel 0 complementary capture / compare polarity active high."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ch0np::Active
    }
    #[doc = "Channel 0 complementary capture / compare polarity active low."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ch0np::Inactive
    }
}
#[doc = "Field `CH0NP` writer - Channel 0 complementary output polarity"]
pub type Ch0npW<'a, REG> = crate::BitWriter<'a, REG, Ch0np>;
impl<'a, REG> Ch0npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 0 complementary capture / compare polarity active high."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0np::Active)
    }
    #[doc = "Channel 0 complementary capture / compare polarity active low."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0np::Inactive)
    }
}
#[doc = "Channel 1 capture/compare function enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1en {
    #[doc = "0: Channel 1 capture / compare disabled."]
    Disabled = 0,
    #[doc = "1: Channel 1 capture / compare enabled."]
    Enabled = 1,
}
impl From<Ch1en> for bool {
    #[inline(always)]
    fn from(variant: Ch1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1EN` reader - Channel 1 capture/compare function enable"]
pub type Ch1enR = crate::BitReader<Ch1en>;
impl Ch1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1en {
        match self.bits {
            false => Ch1en::Disabled,
            true => Ch1en::Enabled,
        }
    }
    #[doc = "Channel 1 capture / compare disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch1en::Disabled
    }
    #[doc = "Channel 1 capture / compare enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch1en::Enabled
    }
}
#[doc = "Field `CH1EN` writer - Channel 1 capture/compare function enable"]
pub type Ch1enW<'a, REG> = crate::BitWriter<'a, REG, Ch1en>;
impl<'a, REG> Ch1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 1 capture / compare disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1en::Disabled)
    }
    #[doc = "Channel 1 capture / compare enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1en::Enabled)
    }
}
#[doc = "Channel 1 capture/compare function polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1p {
    #[doc = "0: Channel 1 capture / compare polarity active high."]
    Active = 0,
    #[doc = "1: Channel 1 capture / compare polarity active low."]
    Inactive = 1,
}
impl From<Ch1p> for bool {
    #[inline(always)]
    fn from(variant: Ch1p) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1P` reader - Channel 1 capture/compare function polarity"]
pub type Ch1pR = crate::BitReader<Ch1p>;
impl Ch1pR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1p {
        match self.bits {
            false => Ch1p::Active,
            true => Ch1p::Inactive,
        }
    }
    #[doc = "Channel 1 capture / compare polarity active high."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ch1p::Active
    }
    #[doc = "Channel 1 capture / compare polarity active low."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ch1p::Inactive
    }
}
#[doc = "Field `CH1P` writer - Channel 1 capture/compare function polarity"]
pub type Ch1pW<'a, REG> = crate::BitWriter<'a, REG, Ch1p>;
impl<'a, REG> Ch1pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 1 capture / compare polarity active high."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1p::Active)
    }
    #[doc = "Channel 1 capture / compare polarity active low."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1p::Inactive)
    }
}
#[doc = "Channel 1 complementary output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1np {
    #[doc = "0: Channel 1 complementary capture / compare polarity active high."]
    Active = 0,
    #[doc = "1: Channel 1 complementary capture / compare polarity active low."]
    Inactive = 1,
}
impl From<Ch1np> for bool {
    #[inline(always)]
    fn from(variant: Ch1np) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1NP` reader - Channel 1 complementary output polarity"]
pub type Ch1npR = crate::BitReader<Ch1np>;
impl Ch1npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1np {
        match self.bits {
            false => Ch1np::Active,
            true => Ch1np::Inactive,
        }
    }
    #[doc = "Channel 1 complementary capture / compare polarity active high."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ch1np::Active
    }
    #[doc = "Channel 1 complementary capture / compare polarity active low."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ch1np::Inactive
    }
}
#[doc = "Field `CH1NP` writer - Channel 1 complementary output polarity"]
pub type Ch1npW<'a, REG> = crate::BitWriter<'a, REG, Ch1np>;
impl<'a, REG> Ch1npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 1 complementary capture / compare polarity active high."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1np::Active)
    }
    #[doc = "Channel 1 complementary capture / compare polarity active low."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1np::Inactive)
    }
}
#[doc = "Channel 2 capture/compare function enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2en {
    #[doc = "0: Channel 2 capture / compare disabled."]
    Disabled = 0,
    #[doc = "1: Channel 2 capture / compare enabled."]
    Enabled = 1,
}
impl From<Ch2en> for bool {
    #[inline(always)]
    fn from(variant: Ch2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2EN` reader - Channel 2 capture/compare function enable"]
pub type Ch2enR = crate::BitReader<Ch2en>;
impl Ch2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2en {
        match self.bits {
            false => Ch2en::Disabled,
            true => Ch2en::Enabled,
        }
    }
    #[doc = "Channel 2 capture / compare disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch2en::Disabled
    }
    #[doc = "Channel 2 capture / compare enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch2en::Enabled
    }
}
#[doc = "Field `CH2EN` writer - Channel 2 capture/compare function enable"]
pub type Ch2enW<'a, REG> = crate::BitWriter<'a, REG, Ch2en>;
impl<'a, REG> Ch2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 2 capture / compare disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2en::Disabled)
    }
    #[doc = "Channel 2 capture / compare enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2en::Enabled)
    }
}
#[doc = "Channel 2 capture/compare function polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2p {
    #[doc = "0: Channel 2 capture / compare polarity active high."]
    Active = 0,
    #[doc = "1: Channel 2 capture / compare polarity active low."]
    Inactive = 1,
}
impl From<Ch2p> for bool {
    #[inline(always)]
    fn from(variant: Ch2p) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2P` reader - Channel 2 capture/compare function polarity"]
pub type Ch2pR = crate::BitReader<Ch2p>;
impl Ch2pR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2p {
        match self.bits {
            false => Ch2p::Active,
            true => Ch2p::Inactive,
        }
    }
    #[doc = "Channel 2 capture / compare polarity active high."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ch2p::Active
    }
    #[doc = "Channel 2 capture / compare polarity active low."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ch2p::Inactive
    }
}
#[doc = "Field `CH2P` writer - Channel 2 capture/compare function polarity"]
pub type Ch2pW<'a, REG> = crate::BitWriter<'a, REG, Ch2p>;
impl<'a, REG> Ch2pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 2 capture / compare polarity active high."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2p::Active)
    }
    #[doc = "Channel 2 capture / compare polarity active low."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2p::Inactive)
    }
}
#[doc = "Channel 2 complementary output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2np {
    #[doc = "0: Channel 2 complementary capture / compare polarity active high."]
    Active = 0,
    #[doc = "1: Channel 2 complementary capture / compare polarity active low."]
    Inactive = 1,
}
impl From<Ch2np> for bool {
    #[inline(always)]
    fn from(variant: Ch2np) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2NP` reader - Channel 2 complementary output polarity"]
pub type Ch2npR = crate::BitReader<Ch2np>;
impl Ch2npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2np {
        match self.bits {
            false => Ch2np::Active,
            true => Ch2np::Inactive,
        }
    }
    #[doc = "Channel 2 complementary capture / compare polarity active high."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ch2np::Active
    }
    #[doc = "Channel 2 complementary capture / compare polarity active low."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ch2np::Inactive
    }
}
#[doc = "Field `CH2NP` writer - Channel 2 complementary output polarity"]
pub type Ch2npW<'a, REG> = crate::BitWriter<'a, REG, Ch2np>;
impl<'a, REG> Ch2npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 2 complementary capture / compare polarity active high."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2np::Active)
    }
    #[doc = "Channel 2 complementary capture / compare polarity active low."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2np::Inactive)
    }
}
#[doc = "Channel 3 capture/compare function enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3en {
    #[doc = "0: Channel 3 capture / compare disabled."]
    Disabled = 0,
    #[doc = "1: Channel 3 capture / compare enabled."]
    Enabled = 1,
}
impl From<Ch3en> for bool {
    #[inline(always)]
    fn from(variant: Ch3en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3EN` reader - Channel 3 capture/compare function enable"]
pub type Ch3enR = crate::BitReader<Ch3en>;
impl Ch3enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3en {
        match self.bits {
            false => Ch3en::Disabled,
            true => Ch3en::Enabled,
        }
    }
    #[doc = "Channel 3 capture / compare disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch3en::Disabled
    }
    #[doc = "Channel 3 capture / compare enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch3en::Enabled
    }
}
#[doc = "Field `CH3EN` writer - Channel 3 capture/compare function enable"]
pub type Ch3enW<'a, REG> = crate::BitWriter<'a, REG, Ch3en>;
impl<'a, REG> Ch3enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 3 capture / compare disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3en::Disabled)
    }
    #[doc = "Channel 3 capture / compare enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3en::Enabled)
    }
}
#[doc = "Channel 3 capture/compare function polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3p {
    #[doc = "0: Channel 3 capture / compare polarity active high."]
    Active = 0,
    #[doc = "1: Channel 3 capture / compare polarity active low."]
    Inactive = 1,
}
impl From<Ch3p> for bool {
    #[inline(always)]
    fn from(variant: Ch3p) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3P` reader - Channel 3 capture/compare function polarity"]
pub type Ch3pR = crate::BitReader<Ch3p>;
impl Ch3pR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3p {
        match self.bits {
            false => Ch3p::Active,
            true => Ch3p::Inactive,
        }
    }
    #[doc = "Channel 3 capture / compare polarity active high."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ch3p::Active
    }
    #[doc = "Channel 3 capture / compare polarity active low."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ch3p::Inactive
    }
}
#[doc = "Field `CH3P` writer - Channel 3 capture/compare function polarity"]
pub type Ch3pW<'a, REG> = crate::BitWriter<'a, REG, Ch3p>;
impl<'a, REG> Ch3pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 3 capture / compare polarity active high."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3p::Active)
    }
    #[doc = "Channel 3 capture / compare polarity active low."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3p::Inactive)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 capture/compare function enable"]
    #[inline(always)]
    pub fn ch0en(&self) -> Ch0enR {
        Ch0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch0p(&self) -> Ch0pR {
        Ch0pR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 0 complementary output polarity"]
    #[inline(always)]
    pub fn ch0np(&self) -> Ch0npR {
        Ch0npR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 1 capture/compare function enable"]
    #[inline(always)]
    pub fn ch1en(&self) -> Ch1enR {
        Ch1enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 1 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch1p(&self) -> Ch1pR {
        Ch1pR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 1 complementary output polarity"]
    #[inline(always)]
    pub fn ch1np(&self) -> Ch1npR {
        Ch1npR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 2 capture/compare function enable"]
    #[inline(always)]
    pub fn ch2en(&self) -> Ch2enR {
        Ch2enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 2 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch2p(&self) -> Ch2pR {
        Ch2pR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 2 complementary output polarity"]
    #[inline(always)]
    pub fn ch2np(&self) -> Ch2npR {
        Ch2npR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 3 capture/compare function enable"]
    #[inline(always)]
    pub fn ch3en(&self) -> Ch3enR {
        Ch3enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 3 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch3p(&self) -> Ch3pR {
        Ch3pR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 capture/compare function enable"]
    #[inline(always)]
    pub fn ch0en(&mut self) -> Ch0enW<'_, Chctl2Spec> {
        Ch0enW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch0p(&mut self) -> Ch0pW<'_, Chctl2Spec> {
        Ch0pW::new(self, 1)
    }
    #[doc = "Bit 3 - Channel 0 complementary output polarity"]
    #[inline(always)]
    pub fn ch0np(&mut self) -> Ch0npW<'_, Chctl2Spec> {
        Ch0npW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 1 capture/compare function enable"]
    #[inline(always)]
    pub fn ch1en(&mut self) -> Ch1enW<'_, Chctl2Spec> {
        Ch1enW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 1 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch1p(&mut self) -> Ch1pW<'_, Chctl2Spec> {
        Ch1pW::new(self, 5)
    }
    #[doc = "Bit 7 - Channel 1 complementary output polarity"]
    #[inline(always)]
    pub fn ch1np(&mut self) -> Ch1npW<'_, Chctl2Spec> {
        Ch1npW::new(self, 7)
    }
    #[doc = "Bit 8 - Channel 2 capture/compare function enable"]
    #[inline(always)]
    pub fn ch2en(&mut self) -> Ch2enW<'_, Chctl2Spec> {
        Ch2enW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 2 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch2p(&mut self) -> Ch2pW<'_, Chctl2Spec> {
        Ch2pW::new(self, 9)
    }
    #[doc = "Bit 11 - Channel 2 complementary output polarity"]
    #[inline(always)]
    pub fn ch2np(&mut self) -> Ch2npW<'_, Chctl2Spec> {
        Ch2npW::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 3 capture/compare function enable"]
    #[inline(always)]
    pub fn ch3en(&mut self) -> Ch3enW<'_, Chctl2Spec> {
        Ch3enW::new(self, 12)
    }
    #[doc = "Bit 13 - Channel 3 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch3p(&mut self) -> Ch3pW<'_, Chctl2Spec> {
        Ch3pW::new(self, 13)
    }
}
#[doc = "Channel control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`chctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chctl2Spec;
impl crate::RegisterSpec for Chctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctl2::R`](R) reader structure"]
impl crate::Readable for Chctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`chctl2::W`](W) writer structure"]
impl crate::Writable for Chctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHCTL2 to value 0"]
impl crate::Resettable for Chctl2Spec {}
