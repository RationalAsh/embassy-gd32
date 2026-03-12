#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Random data ready status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drdyr {
    #[doc = "0: Data is not ready"]
    NotReady = 0,
    #[doc = "1: Data is ready"]
    Ready = 1,
}
impl From<Drdyr> for bool {
    #[inline(always)]
    fn from(variant: Drdyr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRDY` reader - Random data ready status bit"]
pub type DrdyR = crate::BitReader<Drdyr>;
impl DrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drdyr {
        match self.bits {
            false => Drdyr::NotReady,
            true => Drdyr::Ready,
        }
    }
    #[doc = "Data is not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Drdyr::NotReady
    }
    #[doc = "Data is ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Drdyr::Ready
    }
}
#[doc = "Clock error current status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cecsr {
    #[doc = "0: Clock error is not detected at current time. In case of CEIF=1 and CECS=0, it means clock error has been detected before but now is recovered."]
    NotReady = 0,
    #[doc = "1: Clock error is detected at current time. TRNG_CLK frequency is lower than 1/HCLK frequency"]
    Ready = 1,
}
impl From<Cecsr> for bool {
    #[inline(always)]
    fn from(variant: Cecsr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CECS` reader - Clock error current status"]
pub type CecsR = crate::BitReader<Cecsr>;
impl CecsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cecsr {
        match self.bits {
            false => Cecsr::NotReady,
            true => Cecsr::Ready,
        }
    }
    #[doc = "Clock error is not detected at current time. In case of CEIF=1 and CECS=0, it means clock error has been detected before but now is recovered."]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Cecsr::NotReady
    }
    #[doc = "Clock error is detected at current time. TRNG_CLK frequency is lower than 1/HCLK frequency"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Cecsr::Ready
    }
}
#[doc = "Seed error current status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secsr {
    #[doc = "0: The seed is not ready"]
    NotReady = 0,
    #[doc = "1: The seed is ready"]
    Ready = 1,
}
impl From<Secsr> for bool {
    #[inline(always)]
    fn from(variant: Secsr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECS` reader - Seed error current status"]
pub type SecsR = crate::BitReader<Secsr>;
impl SecsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Secsr {
        match self.bits {
            false => Secsr::NotReady,
            true => Secsr::Ready,
        }
    }
    #[doc = "The seed is not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Secsr::NotReady
    }
    #[doc = "The seed is ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Secsr::Ready
    }
}
#[doc = "Clock error interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ceifr {
    #[doc = "0: No fault detected"]
    NoFault = 0,
    #[doc = "1: Clock error has been detected. The bit is cleared by writing 0."]
    Fault = 1,
}
impl From<Ceifr> for bool {
    #[inline(always)]
    fn from(variant: Ceifr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEIF` reader - Clock error interrupt flag"]
pub type CeifR = crate::BitReader<Ceifr>;
impl CeifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ceifr {
        match self.bits {
            false => Ceifr::NoFault,
            true => Ceifr::Fault,
        }
    }
    #[doc = "No fault detected"]
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == Ceifr::NoFault
    }
    #[doc = "Clock error has been detected. The bit is cleared by writing 0."]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == Ceifr::Fault
    }
}
#[doc = "Clock error interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CeifwWO {
    #[doc = "0: Clear the CEIF flag"]
    Clear = 0,
}
impl From<CeifwWO> for bool {
    #[inline(always)]
    fn from(variant: CeifwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEIF` writer - Clock error interrupt flag"]
pub type CeifW<'a, REG> = crate::BitWriter<'a, REG, CeifwWO>;
impl<'a, REG> CeifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the CEIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CeifwWO::Clear)
    }
}
#[doc = "Seed error interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seifr {
    #[doc = "0: No fault detected"]
    NoFault = 0,
    #[doc = "1: Seed error has been detected. The bit is cleared by writing 0."]
    Fault = 1,
}
impl From<Seifr> for bool {
    #[inline(always)]
    fn from(variant: Seifr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEIF` reader - Seed error interrupt flag"]
pub type SeifR = crate::BitReader<Seifr>;
impl SeifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Seifr {
        match self.bits {
            false => Seifr::NoFault,
            true => Seifr::Fault,
        }
    }
    #[doc = "No fault detected"]
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == Seifr::NoFault
    }
    #[doc = "Seed error has been detected. The bit is cleared by writing 0."]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == Seifr::Fault
    }
}
#[doc = "Seed error interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeifwWO {
    #[doc = "0: Clear the SEIF flag"]
    Clear = 0,
}
impl From<SeifwWO> for bool {
    #[inline(always)]
    fn from(variant: SeifwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEIF` writer - Seed error interrupt flag"]
pub type SeifW<'a, REG> = crate::BitWriter<'a, REG, SeifwWO>;
impl<'a, REG> SeifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the SEIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SeifwWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Random data ready status bit"]
    #[inline(always)]
    pub fn drdy(&self) -> DrdyR {
        DrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock error current status"]
    #[inline(always)]
    pub fn cecs(&self) -> CecsR {
        CecsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Seed error current status"]
    #[inline(always)]
    pub fn secs(&self) -> SecsR {
        SecsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock error interrupt flag"]
    #[inline(always)]
    pub fn ceif(&self) -> CeifR {
        CeifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Seed error interrupt flag"]
    #[inline(always)]
    pub fn seif(&self) -> SeifR {
        SeifR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Clock error interrupt flag"]
    #[inline(always)]
    pub fn ceif(&mut self) -> CeifW<'_, StatSpec> {
        CeifW::new(self, 5)
    }
    #[doc = "Bit 6 - Seed error interrupt flag"]
    #[inline(always)]
    pub fn seif(&mut self) -> SeifW<'_, StatSpec> {
        SeifW::new(self, 6)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {}
