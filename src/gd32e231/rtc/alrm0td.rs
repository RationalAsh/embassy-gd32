#[doc = "Register `ALRM0TD` reader"]
pub type R = crate::R<Alrm0tdSpec>;
#[doc = "Register `ALRM0TD` writer"]
pub type W = crate::W<Alrm0tdSpec>;
#[doc = "Field `SCU` reader - Second units in BCD format."]
pub type ScuR = crate::FieldReader;
#[doc = "Field `SCU` writer - Second units in BCD format."]
pub type ScuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCT` reader - Second tens in BCD format."]
pub type SctR = crate::FieldReader;
#[doc = "Field `SCT` writer - Second tens in BCD format."]
pub type SctW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSKS` reader - Alarm seconds mask"]
pub type MsksR = crate::BitReader;
#[doc = "Field `MSKS` writer - Alarm seconds mask"]
pub type MsksW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MNU` reader - Minute units in BCD format."]
pub type MnuR = crate::FieldReader;
#[doc = "Field `MNU` writer - Minute units in BCD format."]
pub type MnuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MNT` reader - Minute tens in BCD format."]
pub type MntR = crate::FieldReader;
#[doc = "Field `MNT` writer - Minute tens in BCD format."]
pub type MntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSKM` reader - Alarm minutes mask"]
pub type MskmR = crate::BitReader;
#[doc = "Field `MSKM` writer - Alarm minutes mask"]
pub type MskmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRU` reader - Hour units in BCD format."]
pub type HruR = crate::FieldReader;
#[doc = "Field `HRU` writer - Hour units in BCD format."]
pub type HruW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HRT` reader - Hour tens in BCD format."]
pub type HrtR = crate::FieldReader;
#[doc = "Field `HRT` writer - Hour tens in BCD format."]
pub type HrtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PM` reader - AM/PM notation"]
pub type PmR = crate::BitReader;
#[doc = "Field `PM` writer - AM/PM notation"]
pub type PmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSKH` reader - Alarm hours mask"]
pub type MskhR = crate::BitReader;
#[doc = "Field `MSKH` writer - Alarm hours mask"]
pub type MskhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAYU` reader - Date units or day in BCD format."]
pub type DayuR = crate::FieldReader;
#[doc = "Field `DAYU` writer - Date units or day in BCD format."]
pub type DayuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DAYT` reader - Date tens in BCD format."]
pub type DaytR = crate::FieldReader;
#[doc = "Field `DAYT` writer - Date tens in BCD format."]
pub type DaytW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DOWS` reader - Week day selection"]
pub type DowsR = crate::BitReader;
#[doc = "Field `DOWS` writer - Week day selection"]
pub type DowsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSKD` reader - Alarm date mask"]
pub type MskdR = crate::BitReader;
#[doc = "Field `MSKD` writer - Alarm date mask"]
pub type MskdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Second units in BCD format."]
    #[inline(always)]
    pub fn scu(&self) -> ScuR {
        ScuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format."]
    #[inline(always)]
    pub fn sct(&self) -> SctR {
        SctR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Alarm seconds mask"]
    #[inline(always)]
    pub fn msks(&self) -> MsksR {
        MsksR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format."]
    #[inline(always)]
    pub fn mnu(&self) -> MnuR {
        MnuR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format."]
    #[inline(always)]
    pub fn mnt(&self) -> MntR {
        MntR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Alarm minutes mask"]
    #[inline(always)]
    pub fn mskm(&self) -> MskmR {
        MskmR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format."]
    #[inline(always)]
    pub fn hru(&self) -> HruR {
        HruR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format."]
    #[inline(always)]
    pub fn hrt(&self) -> HrtR {
        HrtR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Alarm hours mask"]
    #[inline(always)]
    pub fn mskh(&self) -> MskhR {
        MskhR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Date units or day in BCD format."]
    #[inline(always)]
    pub fn dayu(&self) -> DayuR {
        DayuR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Date tens in BCD format."]
    #[inline(always)]
    pub fn dayt(&self) -> DaytR {
        DaytR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Week day selection"]
    #[inline(always)]
    pub fn dows(&self) -> DowsR {
        DowsR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Alarm date mask"]
    #[inline(always)]
    pub fn mskd(&self) -> MskdR {
        MskdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Second units in BCD format."]
    #[inline(always)]
    pub fn scu(&mut self) -> ScuW<'_, Alrm0tdSpec> {
        ScuW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format."]
    #[inline(always)]
    pub fn sct(&mut self) -> SctW<'_, Alrm0tdSpec> {
        SctW::new(self, 4)
    }
    #[doc = "Bit 7 - Alarm seconds mask"]
    #[inline(always)]
    pub fn msks(&mut self) -> MsksW<'_, Alrm0tdSpec> {
        MsksW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format."]
    #[inline(always)]
    pub fn mnu(&mut self) -> MnuW<'_, Alrm0tdSpec> {
        MnuW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format."]
    #[inline(always)]
    pub fn mnt(&mut self) -> MntW<'_, Alrm0tdSpec> {
        MntW::new(self, 12)
    }
    #[doc = "Bit 15 - Alarm minutes mask"]
    #[inline(always)]
    pub fn mskm(&mut self) -> MskmW<'_, Alrm0tdSpec> {
        MskmW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format."]
    #[inline(always)]
    pub fn hru(&mut self) -> HruW<'_, Alrm0tdSpec> {
        HruW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format."]
    #[inline(always)]
    pub fn hrt(&mut self) -> HrtW<'_, Alrm0tdSpec> {
        HrtW::new(self, 20)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn pm(&mut self) -> PmW<'_, Alrm0tdSpec> {
        PmW::new(self, 22)
    }
    #[doc = "Bit 23 - Alarm hours mask"]
    #[inline(always)]
    pub fn mskh(&mut self) -> MskhW<'_, Alrm0tdSpec> {
        MskhW::new(self, 23)
    }
    #[doc = "Bits 24:27 - Date units or day in BCD format."]
    #[inline(always)]
    pub fn dayu(&mut self) -> DayuW<'_, Alrm0tdSpec> {
        DayuW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Date tens in BCD format."]
    #[inline(always)]
    pub fn dayt(&mut self) -> DaytW<'_, Alrm0tdSpec> {
        DaytW::new(self, 28)
    }
    #[doc = "Bit 30 - Week day selection"]
    #[inline(always)]
    pub fn dows(&mut self) -> DowsW<'_, Alrm0tdSpec> {
        DowsW::new(self, 30)
    }
    #[doc = "Bit 31 - Alarm date mask"]
    #[inline(always)]
    pub fn mskd(&mut self) -> MskdW<'_, Alrm0tdSpec> {
        MskdW::new(self, 31)
    }
}
#[doc = "alarm A register\n\nYou can [`read`](crate::Reg::read) this register and get [`alrm0td::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrm0td::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Alrm0tdSpec;
impl crate::RegisterSpec for Alrm0tdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alrm0td::R`](R) reader structure"]
impl crate::Readable for Alrm0tdSpec {}
#[doc = "`write(|w| ..)` method takes [`alrm0td::W`](W) writer structure"]
impl crate::Writable for Alrm0tdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALRM0TD to value 0"]
impl crate::Resettable for Alrm0tdSpec {}
