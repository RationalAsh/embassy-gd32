#[doc = "Register `DSV` reader"]
pub type R = crate::R<DsvSpec>;
#[doc = "Register `DSV` writer"]
pub type W = crate::W<DsvSpec>;
#[doc = "Deep-sleep mode voltage select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dslpvs {
    #[doc = "0: The core voltage is default value in Deep-sleep mode"]
    Default = 0,
    #[doc = "1: The core voltage is (default value-0.1)V in Deep-sleep mode(customers are not recommended to use it)"]
    Default10pct = 1,
    #[doc = "2: The core voltage is (default value-0.2)V in Deep-sleep mode(customers are not recommended to use it)"]
    Default20pct = 2,
    #[doc = "3: The core voltage is (default value-0.3)V in Deep-sleep mode(customers are not recommended to use it)"]
    Default30pct = 3,
}
impl From<Dslpvs> for u8 {
    #[inline(always)]
    fn from(variant: Dslpvs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dslpvs {
    type Ux = u8;
}
impl crate::IsEnum for Dslpvs {}
#[doc = "Field `DSLPVS` reader - Deep-sleep mode voltage select"]
pub type DslpvsR = crate::FieldReader<Dslpvs>;
impl DslpvsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dslpvs> {
        match self.bits {
            0 => Some(Dslpvs::Default),
            1 => Some(Dslpvs::Default10pct),
            2 => Some(Dslpvs::Default20pct),
            3 => Some(Dslpvs::Default30pct),
            _ => None,
        }
    }
    #[doc = "The core voltage is default value in Deep-sleep mode"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Dslpvs::Default
    }
    #[doc = "The core voltage is (default value-0.1)V in Deep-sleep mode(customers are not recommended to use it)"]
    #[inline(always)]
    pub fn is_default10pct(&self) -> bool {
        *self == Dslpvs::Default10pct
    }
    #[doc = "The core voltage is (default value-0.2)V in Deep-sleep mode(customers are not recommended to use it)"]
    #[inline(always)]
    pub fn is_default20pct(&self) -> bool {
        *self == Dslpvs::Default20pct
    }
    #[doc = "The core voltage is (default value-0.3)V in Deep-sleep mode(customers are not recommended to use it)"]
    #[inline(always)]
    pub fn is_default30pct(&self) -> bool {
        *self == Dslpvs::Default30pct
    }
}
#[doc = "Field `DSLPVS` writer - Deep-sleep mode voltage select"]
pub type DslpvsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dslpvs>;
impl<'a, REG> DslpvsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The core voltage is default value in Deep-sleep mode"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Dslpvs::Default)
    }
    #[doc = "The core voltage is (default value-0.1)V in Deep-sleep mode(customers are not recommended to use it)"]
    #[inline(always)]
    pub fn default10pct(self) -> &'a mut crate::W<REG> {
        self.variant(Dslpvs::Default10pct)
    }
    #[doc = "The core voltage is (default value-0.2)V in Deep-sleep mode(customers are not recommended to use it)"]
    #[inline(always)]
    pub fn default20pct(self) -> &'a mut crate::W<REG> {
        self.variant(Dslpvs::Default20pct)
    }
    #[doc = "The core voltage is (default value-0.3)V in Deep-sleep mode(customers are not recommended to use it)"]
    #[inline(always)]
    pub fn default30pct(self) -> &'a mut crate::W<REG> {
        self.variant(Dslpvs::Default30pct)
    }
}
impl R {
    #[doc = "Bits 0:2 - Deep-sleep mode voltage select"]
    #[inline(always)]
    pub fn dslpvs(&self) -> DslpvsR {
        DslpvsR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Deep-sleep mode voltage select"]
    #[inline(always)]
    pub fn dslpvs(&mut self) -> DslpvsW<'_, DsvSpec> {
        DslpvsW::new(self, 0)
    }
}
#[doc = "Deep sleep mode Voltage register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsvSpec;
impl crate::RegisterSpec for DsvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsv::R`](R) reader structure"]
impl crate::Readable for DsvSpec {}
#[doc = "`write(|w| ..)` method takes [`dsv::W`](W) writer structure"]
impl crate::Writable for DsvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSV to value 0"]
impl crate::Resettable for DsvSpec {}
