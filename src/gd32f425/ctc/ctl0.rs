#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Clock trim ok interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckokie {
    #[doc = "0: CKOKIF interrupt disabled"]
    Disabled = 0,
    #[doc = "1: CKOKIF interrupt enabled"]
    Enabled = 1,
}
impl From<Ckokie> for bool {
    #[inline(always)]
    fn from(variant: Ckokie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKOKIE` reader - Clock trim ok interrupt enable"]
pub type CkokieR = crate::BitReader<Ckokie>;
impl CkokieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckokie {
        match self.bits {
            false => Ckokie::Disabled,
            true => Ckokie::Enabled,
        }
    }
    #[doc = "CKOKIF interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ckokie::Disabled
    }
    #[doc = "CKOKIF interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ckokie::Enabled
    }
}
#[doc = "Field `CKOKIE` writer - Clock trim ok interrupt enable"]
pub type CkokieW<'a, REG> = crate::BitWriter<'a, REG, Ckokie>;
impl<'a, REG> CkokieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CKOKIF interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ckokie::Disabled)
    }
    #[doc = "CKOKIF interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ckokie::Enabled)
    }
}
#[doc = "Clock trim warning interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckwarnie {
    #[doc = "0: CKWARNIF interrupt disabled"]
    Disabled = 0,
    #[doc = "1: CKWARNIF interrupt enabled"]
    Enabled = 1,
}
impl From<Ckwarnie> for bool {
    #[inline(always)]
    fn from(variant: Ckwarnie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKWARNIE` reader - Clock trim warning interrupt enable"]
pub type CkwarnieR = crate::BitReader<Ckwarnie>;
impl CkwarnieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckwarnie {
        match self.bits {
            false => Ckwarnie::Disabled,
            true => Ckwarnie::Enabled,
        }
    }
    #[doc = "CKWARNIF interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ckwarnie::Disabled
    }
    #[doc = "CKWARNIF interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ckwarnie::Enabled
    }
}
#[doc = "Field `CKWARNIE` writer - Clock trim warning interrupt enable"]
pub type CkwarnieW<'a, REG> = crate::BitWriter<'a, REG, Ckwarnie>;
impl<'a, REG> CkwarnieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CKWARNIF interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ckwarnie::Disabled)
    }
    #[doc = "CKWARNIF interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ckwarnie::Enabled)
    }
}
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errie {
    #[doc = "0: ERRIF interrupt disabled"]
    Disabled = 0,
    #[doc = "1: ERRIF interrupt enabled"]
    Enabled = 1,
}
impl From<Errie> for bool {
    #[inline(always)]
    fn from(variant: Errie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ErrieR = crate::BitReader<Errie>;
impl ErrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errie {
        match self.bits {
            false => Errie::Disabled,
            true => Errie::Enabled,
        }
    }
    #[doc = "ERRIF interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Errie::Disabled
    }
    #[doc = "ERRIF interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Errie::Enabled
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG, Errie>;
impl<'a, REG> ErrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ERRIF interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Errie::Disabled)
    }
    #[doc = "ERRIF interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Errie::Enabled)
    }
}
#[doc = "EREFIF interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erefie {
    #[doc = "0: EREFIF interrupt disabled"]
    Disabled = 0,
    #[doc = "1: EREFIF interrupt enabled"]
    Enabled = 1,
}
impl From<Erefie> for bool {
    #[inline(always)]
    fn from(variant: Erefie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EREFIE` reader - EREFIF interrupt enable"]
pub type ErefieR = crate::BitReader<Erefie>;
impl ErefieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erefie {
        match self.bits {
            false => Erefie::Disabled,
            true => Erefie::Enabled,
        }
    }
    #[doc = "EREFIF interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Erefie::Disabled
    }
    #[doc = "EREFIF interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Erefie::Enabled
    }
}
#[doc = "Field `EREFIE` writer - EREFIF interrupt enable"]
pub type ErefieW<'a, REG> = crate::BitWriter<'a, REG, Erefie>;
impl<'a, REG> ErefieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EREFIF interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Erefie::Disabled)
    }
    #[doc = "EREFIF interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Erefie::Enabled)
    }
}
#[doc = "CTC counter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cnten {
    #[doc = "0: CTC trim counter disabled"]
    Disabled = 0,
    #[doc = "1: CTC trim counter enabled"]
    Enabled = 1,
}
impl From<Cnten> for bool {
    #[inline(always)]
    fn from(variant: Cnten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTEN` reader - CTC counter enable"]
pub type CntenR = crate::BitReader<Cnten>;
impl CntenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cnten {
        match self.bits {
            false => Cnten::Disabled,
            true => Cnten::Enabled,
        }
    }
    #[doc = "CTC trim counter disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cnten::Disabled
    }
    #[doc = "CTC trim counter enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cnten::Enabled
    }
}
#[doc = "Field `CNTEN` writer - CTC counter enable"]
pub type CntenW<'a, REG> = crate::BitWriter<'a, REG, Cnten>;
impl<'a, REG> CntenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CTC trim counter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cnten::Disabled)
    }
    #[doc = "CTC trim counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cnten::Enabled)
    }
}
#[doc = "Hardware automatically trim mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autotrim {
    #[doc = "0: Hardware automatic trim disabled"]
    Disabled = 0,
    #[doc = "1: Hardware automatic trim enabled"]
    Enabled = 1,
}
impl From<Autotrim> for bool {
    #[inline(always)]
    fn from(variant: Autotrim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOTRIM` reader - Hardware automatically trim mode"]
pub type AutotrimR = crate::BitReader<Autotrim>;
impl AutotrimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autotrim {
        match self.bits {
            false => Autotrim::Disabled,
            true => Autotrim::Enabled,
        }
    }
    #[doc = "Hardware automatic trim disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Autotrim::Disabled
    }
    #[doc = "Hardware automatic trim enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Autotrim::Enabled
    }
}
#[doc = "Field `AUTOTRIM` writer - Hardware automatically trim mode"]
pub type AutotrimW<'a, REG> = crate::BitWriter<'a, REG, Autotrim>;
impl<'a, REG> AutotrimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware automatic trim disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Autotrim::Disabled)
    }
    #[doc = "Hardware automatic trim enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Autotrim::Enabled)
    }
}
#[doc = "Software reference source sync pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swrefpul {
    #[doc = "1: Generates a software reference source sync pulse."]
    Generate = 1,
}
impl From<Swrefpul> for bool {
    #[inline(always)]
    fn from(variant: Swrefpul) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWREFPUL` reader - Software reference source sync pulse"]
pub type SwrefpulR = crate::BitReader<Swrefpul>;
impl SwrefpulR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Swrefpul> {
        match self.bits {
            true => Some(Swrefpul::Generate),
            _ => None,
        }
    }
    #[doc = "Generates a software reference source sync pulse."]
    #[inline(always)]
    pub fn is_generate(&self) -> bool {
        *self == Swrefpul::Generate
    }
}
#[doc = "Field `SWREFPUL` writer - Software reference source sync pulse"]
pub type SwrefpulW<'a, REG> = crate::BitWriter<'a, REG, Swrefpul>;
impl<'a, REG> SwrefpulW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generates a software reference source sync pulse."]
    #[inline(always)]
    pub fn generate(self) -> &'a mut crate::W<REG> {
        self.variant(Swrefpul::Generate)
    }
}
#[doc = "Field `TRIMVALUE` reader - IRC48M trim value"]
pub type TrimvalueR = crate::FieldReader;
#[doc = "Field `TRIMVALUE` writer - IRC48M trim value"]
pub type TrimvalueW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Clock trim ok interrupt enable"]
    #[inline(always)]
    pub fn ckokie(&self) -> CkokieR {
        CkokieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock trim warning interrupt enable"]
    #[inline(always)]
    pub fn ckwarnie(&self) -> CkwarnieR {
        CkwarnieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EREFIF interrupt enable"]
    #[inline(always)]
    pub fn erefie(&self) -> ErefieR {
        ErefieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - CTC counter enable"]
    #[inline(always)]
    pub fn cnten(&self) -> CntenR {
        CntenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hardware automatically trim mode"]
    #[inline(always)]
    pub fn autotrim(&self) -> AutotrimR {
        AutotrimR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software reference source sync pulse"]
    #[inline(always)]
    pub fn swrefpul(&self) -> SwrefpulR {
        SwrefpulR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - IRC48M trim value"]
    #[inline(always)]
    pub fn trimvalue(&self) -> TrimvalueR {
        TrimvalueR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock trim ok interrupt enable"]
    #[inline(always)]
    pub fn ckokie(&mut self) -> CkokieW<'_, Ctl0Spec> {
        CkokieW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock trim warning interrupt enable"]
    #[inline(always)]
    pub fn ckwarnie(&mut self) -> CkwarnieW<'_, Ctl0Spec> {
        CkwarnieW::new(self, 1)
    }
    #[doc = "Bit 2 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ErrieW<'_, Ctl0Spec> {
        ErrieW::new(self, 2)
    }
    #[doc = "Bit 3 - EREFIF interrupt enable"]
    #[inline(always)]
    pub fn erefie(&mut self) -> ErefieW<'_, Ctl0Spec> {
        ErefieW::new(self, 3)
    }
    #[doc = "Bit 5 - CTC counter enable"]
    #[inline(always)]
    pub fn cnten(&mut self) -> CntenW<'_, Ctl0Spec> {
        CntenW::new(self, 5)
    }
    #[doc = "Bit 6 - Hardware automatically trim mode"]
    #[inline(always)]
    pub fn autotrim(&mut self) -> AutotrimW<'_, Ctl0Spec> {
        AutotrimW::new(self, 6)
    }
    #[doc = "Bit 7 - Software reference source sync pulse"]
    #[inline(always)]
    pub fn swrefpul(&mut self) -> SwrefpulW<'_, Ctl0Spec> {
        SwrefpulW::new(self, 7)
    }
    #[doc = "Bits 8:13 - IRC48M trim value"]
    #[inline(always)]
    pub fn trimvalue(&mut self) -> TrimvalueW<'_, Ctl0Spec> {
        TrimvalueW::new(self, 8)
    }
}
#[doc = "Control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL0 to value 0x2000"]
impl crate::Resettable for Ctl0Spec {
    const RESET_VALUE: u32 = 0x2000;
}
