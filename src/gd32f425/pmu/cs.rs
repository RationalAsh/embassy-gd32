#[doc = "Register `CS` reader"]
pub type R = crate::R<CsSpec>;
#[doc = "Register `CS` writer"]
pub type W = crate::W<CsSpec>;
#[doc = "Wakeup flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wufr {
    #[doc = "0: No wakeup event received"]
    NoEvent = 0,
    #[doc = "1: Wakeup event occurred"]
    EventOccurred = 1,
}
impl From<Wufr> for bool {
    #[inline(always)]
    fn from(variant: Wufr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF` reader - Wakeup flag"]
pub type WufR = crate::BitReader<Wufr>;
impl WufR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wufr {
        match self.bits {
            false => Wufr::NoEvent,
            true => Wufr::EventOccurred,
        }
    }
    #[doc = "No wakeup event received"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == Wufr::NoEvent
    }
    #[doc = "Wakeup event occurred"]
    #[inline(always)]
    pub fn is_event_occurred(&self) -> bool {
        *self == Wufr::EventOccurred
    }
}
#[doc = "Standby flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stbfr {
    #[doc = "0: The device has not entered Standby mode"]
    NotEntered = 0,
    #[doc = "1: The device has been in Standby mode"]
    Entered = 1,
}
impl From<Stbfr> for bool {
    #[inline(always)]
    fn from(variant: Stbfr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STBF` reader - Standby flag"]
pub type StbfR = crate::BitReader<Stbfr>;
impl StbfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stbfr {
        match self.bits {
            false => Stbfr::NotEntered,
            true => Stbfr::Entered,
        }
    }
    #[doc = "The device has not entered Standby mode"]
    #[inline(always)]
    pub fn is_not_entered(&self) -> bool {
        *self == Stbfr::NotEntered
    }
    #[doc = "The device has been in Standby mode"]
    #[inline(always)]
    pub fn is_entered(&self) -> bool {
        *self == Stbfr::Entered
    }
}
#[doc = "Low Voltage Detector Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvdfr {
    #[doc = "0: No Low Voltage event occurred (VDD > threshold)"]
    NoEvent = 0,
    #[doc = "1: Low Voltage event occurred (VDD ≤ threshold)"]
    EventOccurred = 1,
}
impl From<Lvdfr> for bool {
    #[inline(always)]
    fn from(variant: Lvdfr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVDF` reader - Low Voltage Detector Status Flag"]
pub type LvdfR = crate::BitReader<Lvdfr>;
impl LvdfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvdfr {
        match self.bits {
            false => Lvdfr::NoEvent,
            true => Lvdfr::EventOccurred,
        }
    }
    #[doc = "No Low Voltage event occurred (VDD > threshold)"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == Lvdfr::NoEvent
    }
    #[doc = "Low Voltage event occurred (VDD ≤ threshold)"]
    #[inline(always)]
    pub fn is_event_occurred(&self) -> bool {
        *self == Lvdfr::EventOccurred
    }
}
#[doc = "Backup SRAM LDO ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bldorfr {
    #[doc = "0: Backup SRAM LDO not ready"]
    NotReady = 0,
    #[doc = "1: Backup SRAM LDO ready"]
    Ready = 1,
}
impl From<Bldorfr> for bool {
    #[inline(always)]
    fn from(variant: Bldorfr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLDORF` reader - Backup SRAM LDO ready flag"]
pub type BldorfR = crate::BitReader<Bldorfr>;
impl BldorfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bldorfr {
        match self.bits {
            false => Bldorfr::NotReady,
            true => Bldorfr::Ready,
        }
    }
    #[doc = "Backup SRAM LDO not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Bldorfr::NotReady
    }
    #[doc = "Backup SRAM LDO ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Bldorfr::Ready
    }
}
#[doc = "Enable WKUP pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupen {
    #[doc = "0: WKUP pin function disabled"]
    Disabled = 0,
    #[doc = "1: WKUP pin function enabled"]
    Enabled = 1,
}
impl From<Wupen> for bool {
    #[inline(always)]
    fn from(variant: Wupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPEN` reader - Enable WKUP pin"]
pub type WupenR = crate::BitReader<Wupen>;
impl WupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupen {
        match self.bits {
            false => Wupen::Disabled,
            true => Wupen::Enabled,
        }
    }
    #[doc = "WKUP pin function disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Wupen::Disabled
    }
    #[doc = "WKUP pin function enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Wupen::Enabled
    }
}
#[doc = "Field `WUPEN` writer - Enable WKUP pin"]
pub type WupenW<'a, REG> = crate::BitWriter<'a, REG, Wupen>;
impl<'a, REG> WupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WKUP pin function disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wupen::Disabled)
    }
    #[doc = "WKUP pin function enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wupen::Enabled)
    }
}
#[doc = "Backup SRAM LDO on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bldoon {
    #[doc = "0: Backup SRAM LDO closed"]
    Closed = 0,
    #[doc = "1: Open the Backup SRAM LDO"]
    Open = 1,
}
impl From<Bldoon> for bool {
    #[inline(always)]
    fn from(variant: Bldoon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLDOON` reader - Backup SRAM LDO on"]
pub type BldoonR = crate::BitReader<Bldoon>;
impl BldoonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bldoon {
        match self.bits {
            false => Bldoon::Closed,
            true => Bldoon::Open,
        }
    }
    #[doc = "Backup SRAM LDO closed"]
    #[inline(always)]
    pub fn is_closed(&self) -> bool {
        *self == Bldoon::Closed
    }
    #[doc = "Open the Backup SRAM LDO"]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == Bldoon::Open
    }
}
#[doc = "Field `BLDOON` writer - Backup SRAM LDO on"]
pub type BldoonW<'a, REG> = crate::BitWriter<'a, REG, Bldoon>;
impl<'a, REG> BldoonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Backup SRAM LDO closed"]
    #[inline(always)]
    pub fn closed(self) -> &'a mut crate::W<REG> {
        self.variant(Bldoon::Closed)
    }
    #[doc = "Open the Backup SRAM LDO"]
    #[inline(always)]
    pub fn open(self) -> &'a mut crate::W<REG> {
        self.variant(Bldoon::Open)
    }
}
#[doc = "LDO voltage select ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ldovsrfr {
    #[doc = "0: LDO voltage select not ready"]
    NotReady = 0,
    #[doc = "1: LDO voltage select ready"]
    Ready = 1,
}
impl From<Ldovsrfr> for bool {
    #[inline(always)]
    fn from(variant: Ldovsrfr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDOVSRF` reader - LDO voltage select ready flag"]
pub type LdovsrfR = crate::BitReader<Ldovsrfr>;
impl LdovsrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ldovsrfr {
        match self.bits {
            false => Ldovsrfr::NotReady,
            true => Ldovsrfr::Ready,
        }
    }
    #[doc = "LDO voltage select not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Ldovsrfr::NotReady
    }
    #[doc = "LDO voltage select ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Ldovsrfr::Ready
    }
}
#[doc = "High-driver ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdrfr {
    #[doc = "0: High-driver not ready"]
    NotReady = 0,
    #[doc = "1: High-driver ready"]
    Ready = 1,
}
impl From<Hdrfr> for bool {
    #[inline(always)]
    fn from(variant: Hdrfr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDRF` reader - High-driver ready flag"]
pub type HdrfR = crate::BitReader<Hdrfr>;
impl HdrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdrfr {
        match self.bits {
            false => Hdrfr::NotReady,
            true => Hdrfr::Ready,
        }
    }
    #[doc = "High-driver not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Hdrfr::NotReady
    }
    #[doc = "High-driver ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Hdrfr::Ready
    }
}
#[doc = "High-driver switch ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdsrfr {
    #[doc = "0: High-driver switch not ready"]
    NotReady = 0,
    #[doc = "1: High-driver switch ready"]
    Ready = 1,
}
impl From<Hdsrfr> for bool {
    #[inline(always)]
    fn from(variant: Hdsrfr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDSRF` reader - High-driver switch ready flag"]
pub type HdsrfR = crate::BitReader<Hdsrfr>;
impl HdsrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdsrfr {
        match self.bits {
            false => Hdsrfr::NotReady,
            true => Hdsrfr::Ready,
        }
    }
    #[doc = "High-driver switch not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Hdsrfr::NotReady
    }
    #[doc = "High-driver switch ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Hdsrfr::Ready
    }
}
#[doc = "Low-driver mode ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ldrf {
    #[doc = "0: Normal driver in Deep-sleep mode"]
    NormalDriver = 0,
    #[doc = "3: Low-driver mode in Deep-sleep mode"]
    LowDriver = 3,
}
impl From<Ldrf> for u8 {
    #[inline(always)]
    fn from(variant: Ldrf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ldrf {
    type Ux = u8;
}
impl crate::IsEnum for Ldrf {}
#[doc = "Field `LDRF` reader - Low-driver mode ready flag"]
pub type LdrfR = crate::FieldReader<Ldrf>;
impl LdrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ldrf> {
        match self.bits {
            0 => Some(Ldrf::NormalDriver),
            3 => Some(Ldrf::LowDriver),
            _ => None,
        }
    }
    #[doc = "Normal driver in Deep-sleep mode"]
    #[inline(always)]
    pub fn is_normal_driver(&self) -> bool {
        *self == Ldrf::NormalDriver
    }
    #[doc = "Low-driver mode in Deep-sleep mode"]
    #[inline(always)]
    pub fn is_low_driver(&self) -> bool {
        *self == Ldrf::LowDriver
    }
}
#[doc = "Field `LDRF` writer - Low-driver mode ready flag"]
pub type LdrfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ldrf>;
impl<'a, REG> LdrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal driver in Deep-sleep mode"]
    #[inline(always)]
    pub fn normal_driver(self) -> &'a mut crate::W<REG> {
        self.variant(Ldrf::NormalDriver)
    }
    #[doc = "Low-driver mode in Deep-sleep mode"]
    #[inline(always)]
    pub fn low_driver(self) -> &'a mut crate::W<REG> {
        self.variant(Ldrf::LowDriver)
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup flag"]
    #[inline(always)]
    pub fn wuf(&self) -> WufR {
        WufR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Standby flag"]
    #[inline(always)]
    pub fn stbf(&self) -> StbfR {
        StbfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low Voltage Detector Status Flag"]
    #[inline(always)]
    pub fn lvdf(&self) -> LvdfR {
        LvdfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Backup SRAM LDO ready flag"]
    #[inline(always)]
    pub fn bldorf(&self) -> BldorfR {
        BldorfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable WKUP pin"]
    #[inline(always)]
    pub fn wupen(&self) -> WupenR {
        WupenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Backup SRAM LDO on"]
    #[inline(always)]
    pub fn bldoon(&self) -> BldoonR {
        BldoonR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - LDO voltage select ready flag"]
    #[inline(always)]
    pub fn ldovsrf(&self) -> LdovsrfR {
        LdovsrfR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - High-driver ready flag"]
    #[inline(always)]
    pub fn hdrf(&self) -> HdrfR {
        HdrfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - High-driver switch ready flag"]
    #[inline(always)]
    pub fn hdsrf(&self) -> HdsrfR {
        HdsrfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Low-driver mode ready flag"]
    #[inline(always)]
    pub fn ldrf(&self) -> LdrfR {
        LdrfR::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Enable WKUP pin"]
    #[inline(always)]
    pub fn wupen(&mut self) -> WupenW<'_, CsSpec> {
        WupenW::new(self, 8)
    }
    #[doc = "Bit 9 - Backup SRAM LDO on"]
    #[inline(always)]
    pub fn bldoon(&mut self) -> BldoonW<'_, CsSpec> {
        BldoonW::new(self, 9)
    }
    #[doc = "Bits 18:19 - Low-driver mode ready flag"]
    #[inline(always)]
    pub fn ldrf(&mut self) -> LdrfW<'_, CsSpec> {
        LdrfW::new(self, 18)
    }
}
#[doc = "power control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsSpec;
impl crate::RegisterSpec for CsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs::R`](R) reader structure"]
impl crate::Readable for CsSpec {}
#[doc = "`write(|w| ..)` method takes [`cs::W`](W) writer structure"]
impl crate::Writable for CsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CS to value 0"]
impl crate::Resettable for CsSpec {}
