#[doc = "Register `CHCTL2` reader"]
pub type R = crate::R<Chctl2Spec>;
#[doc = "Register `CHCTL2` writer"]
pub type W = crate::W<Chctl2Spec>;
#[doc = "Field `CH0EN` reader - Channel 0 capture/compare function enable"]
pub type Ch0enR = crate::BitReader;
#[doc = "Field `CH0EN` writer - Channel 0 capture/compare function enable"]
pub type Ch0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0P` reader - Channel 0 capture/compare function polarity"]
pub type Ch0pR = crate::BitReader;
#[doc = "Field `CH0P` writer - Channel 0 capture/compare function polarity"]
pub type Ch0pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Channel 0 complementary output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0nen {
    #[doc = "0: Channel 0 complementary output disabled"]
    Disabled = 0,
    #[doc = "1: Channel 0 complementary output enabled"]
    Enabled = 1,
}
impl From<Ch0nen> for bool {
    #[inline(always)]
    fn from(variant: Ch0nen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0NEN` reader - Channel 0 complementary output enable"]
pub type Ch0nenR = crate::BitReader<Ch0nen>;
impl Ch0nenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0nen {
        match self.bits {
            false => Ch0nen::Disabled,
            true => Ch0nen::Enabled,
        }
    }
    #[doc = "Channel 0 complementary output disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch0nen::Disabled
    }
    #[doc = "Channel 0 complementary output enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch0nen::Enabled
    }
}
#[doc = "Field `CH0NEN` writer - Channel 0 complementary output enable"]
pub type Ch0nenW<'a, REG> = crate::BitWriter<'a, REG, Ch0nen>;
impl<'a, REG> Ch0nenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 0 complementary output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0nen::Disabled)
    }
    #[doc = "Channel 0 complementary output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0nen::Enabled)
    }
}
#[doc = "Field `CH0NP` reader - Channel 0 complementary output polarity"]
pub type Ch0npR = crate::BitReader;
#[doc = "Field `CH0NP` writer - Channel 0 complementary output polarity"]
pub type Ch0npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1EN` reader - Channel 1 capture/compare function enable"]
pub type Ch1enR = crate::BitReader;
#[doc = "Field `CH1EN` writer - Channel 1 capture/compare function enable"]
pub type Ch1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1P` reader - Channel 1 capture/compare function polarity"]
pub type Ch1pR = crate::BitReader;
#[doc = "Field `CH1P` writer - Channel 1 capture/compare function polarity"]
pub type Ch1pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Channel 1 complementary output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1nen {
    #[doc = "0: Channel 1 complementary output disabled"]
    Disabled = 0,
    #[doc = "1: Channel 1 complementary output enabled"]
    Enabled = 1,
}
impl From<Ch1nen> for bool {
    #[inline(always)]
    fn from(variant: Ch1nen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1NEN` reader - Channel 1 complementary output enable"]
pub type Ch1nenR = crate::BitReader<Ch1nen>;
impl Ch1nenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1nen {
        match self.bits {
            false => Ch1nen::Disabled,
            true => Ch1nen::Enabled,
        }
    }
    #[doc = "Channel 1 complementary output disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch1nen::Disabled
    }
    #[doc = "Channel 1 complementary output enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch1nen::Enabled
    }
}
#[doc = "Field `CH1NEN` writer - Channel 1 complementary output enable"]
pub type Ch1nenW<'a, REG> = crate::BitWriter<'a, REG, Ch1nen>;
impl<'a, REG> Ch1nenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 1 complementary output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1nen::Disabled)
    }
    #[doc = "Channel 1 complementary output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1nen::Enabled)
    }
}
#[doc = "Field `CH1NP` reader - Channel 1 complementary output polarity"]
pub type Ch1npR = crate::BitReader;
#[doc = "Field `CH1NP` writer - Channel 1 complementary output polarity"]
pub type Ch1npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2EN` reader - Channel 2 capture/compare function enable"]
pub type Ch2enR = crate::BitReader;
#[doc = "Field `CH2EN` writer - Channel 2 capture/compare function enable"]
pub type Ch2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2P` reader - Channel 2 capture/compare function polarity"]
pub type Ch2pR = crate::BitReader;
#[doc = "Field `CH2P` writer - Channel 2 capture/compare function polarity"]
pub type Ch2pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Channel 2 complementary output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2nen {
    #[doc = "0: Channel 2 complementary output disabled"]
    Disabled = 0,
    #[doc = "1: Channel 2 complementary output enabled"]
    Enabled = 1,
}
impl From<Ch2nen> for bool {
    #[inline(always)]
    fn from(variant: Ch2nen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2NEN` reader - Channel 2 complementary output enable"]
pub type Ch2nenR = crate::BitReader<Ch2nen>;
impl Ch2nenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2nen {
        match self.bits {
            false => Ch2nen::Disabled,
            true => Ch2nen::Enabled,
        }
    }
    #[doc = "Channel 2 complementary output disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch2nen::Disabled
    }
    #[doc = "Channel 2 complementary output enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch2nen::Enabled
    }
}
#[doc = "Field `CH2NEN` writer - Channel 2 complementary output enable"]
pub type Ch2nenW<'a, REG> = crate::BitWriter<'a, REG, Ch2nen>;
impl<'a, REG> Ch2nenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 2 complementary output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2nen::Disabled)
    }
    #[doc = "Channel 2 complementary output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2nen::Enabled)
    }
}
#[doc = "Field `CH2NP` reader - Channel 2 complementary output polarity"]
pub type Ch2npR = crate::BitReader;
#[doc = "Field `CH2NP` writer - Channel 2 complementary output polarity"]
pub type Ch2npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3EN` reader - Channel 3 capture/compare function enable"]
pub type Ch3enR = crate::BitReader;
#[doc = "Field `CH3EN` writer - Channel 3 capture/compare function enable"]
pub type Ch3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3P` reader - Channel 3 capture/compare function polarity"]
pub type Ch3pR = crate::BitReader;
#[doc = "Field `CH3P` writer - Channel 3 capture/compare function polarity"]
pub type Ch3pW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - Channel 0 complementary output enable"]
    #[inline(always)]
    pub fn ch0nen(&self) -> Ch0nenR {
        Ch0nenR::new(((self.bits >> 2) & 1) != 0)
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
    #[doc = "Bit 6 - Channel 1 complementary output enable"]
    #[inline(always)]
    pub fn ch1nen(&self) -> Ch1nenR {
        Ch1nenR::new(((self.bits >> 6) & 1) != 0)
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
    #[doc = "Bit 10 - Channel 2 complementary output enable"]
    #[inline(always)]
    pub fn ch2nen(&self) -> Ch2nenR {
        Ch2nenR::new(((self.bits >> 10) & 1) != 0)
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
    #[doc = "Bit 2 - Channel 0 complementary output enable"]
    #[inline(always)]
    pub fn ch0nen(&mut self) -> Ch0nenW<'_, Chctl2Spec> {
        Ch0nenW::new(self, 2)
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
    #[doc = "Bit 6 - Channel 1 complementary output enable"]
    #[inline(always)]
    pub fn ch1nen(&mut self) -> Ch1nenW<'_, Chctl2Spec> {
        Ch1nenW::new(self, 6)
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
    #[doc = "Bit 10 - Channel 2 complementary output enable"]
    #[inline(always)]
    pub fn ch2nen(&mut self) -> Ch2nenW<'_, Chctl2Spec> {
        Ch2nenW::new(self, 10)
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
