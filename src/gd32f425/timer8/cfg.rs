#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Write CHxVAL register selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chvsel {
    #[doc = "0: No effect"]
    Disabled = 0,
    #[doc = "1: If write CHxVAL value equals current value, write access is ignored"]
    Enabled = 1,
}
impl From<Chvsel> for bool {
    #[inline(always)]
    fn from(variant: Chvsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHVSEL` reader - Write CHxVAL register selection"]
pub type ChvselR = crate::BitReader<Chvsel>;
impl ChvselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chvsel {
        match self.bits {
            false => Chvsel::Disabled,
            true => Chvsel::Enabled,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Chvsel::Disabled
    }
    #[doc = "If write CHxVAL value equals current value, write access is ignored"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Chvsel::Enabled
    }
}
#[doc = "Field `CHVSEL` writer - Write CHxVAL register selection"]
pub type ChvselW<'a, REG> = crate::BitWriter<'a, REG, Chvsel>;
impl<'a, REG> ChvselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Chvsel::Disabled)
    }
    #[doc = "If write CHxVAL value equals current value, write access is ignored"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Chvsel::Enabled)
    }
}
impl R {
    #[doc = "Bit 1 - Write CHxVAL register selection"]
    #[inline(always)]
    pub fn chvsel(&self) -> ChvselR {
        ChvselR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Write CHxVAL register selection"]
    #[inline(always)]
    pub fn chvsel(&mut self) -> ChvselW<'_, CfgSpec> {
        ChvselW::new(self, 1)
    }
}
#[doc = "configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {}
