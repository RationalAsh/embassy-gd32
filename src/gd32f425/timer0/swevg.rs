#[doc = "Register `SWEVG` writer"]
pub type W = crate::W<SwevgSpec>;
#[doc = "Field `UPG` writer - Update event generation"]
pub type UpgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0G` writer - Channel 0 capture or compare event generation"]
pub type Ch0gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1G` writer - Channel 1 capture or compare event generation"]
pub type Ch1gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2G` writer - Channel 2 capture or compare event generation"]
pub type Ch2gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3G` writer - Channel 3 capture or compare event generation"]
pub type Ch3gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Channel commutation event generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmtg {
    #[doc = "1: Generate a commutation event, updating capture/compare control registers based on CCSE"]
    Commutation = 1,
}
impl From<Cmtg> for bool {
    #[inline(always)]
    fn from(variant: Cmtg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMTG` writer - Channel commutation event generation"]
pub type CmtgW<'a, REG> = crate::BitWriter<'a, REG, Cmtg>;
impl<'a, REG> CmtgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate a commutation event, updating capture/compare control registers based on CCSE"]
    #[inline(always)]
    pub fn commutation(self) -> &'a mut crate::W<REG> {
        self.variant(Cmtg::Commutation)
    }
}
#[doc = "Field `TRGG` writer - Trigger event generation"]
pub type TrggW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Break event generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brkg {
    #[doc = "1: Generate a break event"]
    Break = 1,
}
impl From<Brkg> for bool {
    #[inline(always)]
    fn from(variant: Brkg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKG` writer - Break event generation"]
pub type BrkgW<'a, REG> = crate::BitWriter<'a, REG, Brkg>;
impl<'a, REG> BrkgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate a break event"]
    #[inline(always)]
    pub fn break_(self) -> &'a mut crate::W<REG> {
        self.variant(Brkg::Break)
    }
}
impl W {
    #[doc = "Bit 0 - Update event generation"]
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
    #[doc = "Bit 5 - Channel commutation event generation"]
    #[inline(always)]
    pub fn cmtg(&mut self) -> CmtgW<'_, SwevgSpec> {
        CmtgW::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger event generation"]
    #[inline(always)]
    pub fn trgg(&mut self) -> TrggW<'_, SwevgSpec> {
        TrggW::new(self, 6)
    }
    #[doc = "Bit 7 - Break event generation"]
    #[inline(always)]
    pub fn brkg(&mut self) -> BrkgW<'_, SwevgSpec> {
        BrkgW::new(self, 7)
    }
}
#[doc = "Software event generation register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swevg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
