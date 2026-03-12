#[doc = "Register `PLLSAI` reader"]
pub type R = crate::R<PllsaiSpec>;
#[doc = "Register `PLLSAI` writer"]
pub type W = crate::W<PllsaiSpec>;
#[doc = "Field `PLLSAIN` reader - The PLLSAI VCO clock multi factor"]
pub type PllsainR = crate::FieldReader<u16>;
#[doc = "Field `PLLSAIN` writer - The PLLSAI VCO clock multi factor"]
pub type PllsainW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "The PLLSAI P output frequency division factor from PLLSAI VCO clock\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pllsaip {
    #[doc = "0: PLLSAI P Clock is PLLSAIVCO divided by 2."]
    Div2 = 0,
    #[doc = "1: PLLSAI P Clock is PLLSAIVCO divided by 4."]
    Div4 = 1,
    #[doc = "2: PLLSAI P Clock is PLLSAIVCO divided by 6."]
    Div6 = 2,
    #[doc = "3: PLLSAI P Clock is PLLSAIVCO divided by 8."]
    Div8 = 3,
}
impl From<Pllsaip> for u8 {
    #[inline(always)]
    fn from(variant: Pllsaip) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pllsaip {
    type Ux = u8;
}
impl crate::IsEnum for Pllsaip {}
#[doc = "Field `PLLSAIP` reader - The PLLSAI P output frequency division factor from PLLSAI VCO clock"]
pub type PllsaipR = crate::FieldReader<Pllsaip>;
impl PllsaipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllsaip {
        match self.bits {
            0 => Pllsaip::Div2,
            1 => Pllsaip::Div4,
            2 => Pllsaip::Div6,
            3 => Pllsaip::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "PLLSAI P Clock is PLLSAIVCO divided by 2."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Pllsaip::Div2
    }
    #[doc = "PLLSAI P Clock is PLLSAIVCO divided by 4."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Pllsaip::Div4
    }
    #[doc = "PLLSAI P Clock is PLLSAIVCO divided by 6."]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == Pllsaip::Div6
    }
    #[doc = "PLLSAI P Clock is PLLSAIVCO divided by 8."]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Pllsaip::Div8
    }
}
#[doc = "Field `PLLSAIP` writer - The PLLSAI P output frequency division factor from PLLSAI VCO clock"]
pub type PllsaipW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pllsaip, crate::Safe>;
impl<'a, REG> PllsaipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLLSAI P Clock is PLLSAIVCO divided by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsaip::Div2)
    }
    #[doc = "PLLSAI P Clock is PLLSAIVCO divided by 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsaip::Div4)
    }
    #[doc = "PLLSAI P Clock is PLLSAIVCO divided by 6."]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsaip::Div6)
    }
    #[doc = "PLLSAI P Clock is PLLSAIVCO divided by 8."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsaip::Div8)
    }
}
#[doc = "Field `PLLSAIQ` reader - The PLLI2S Q output frequency division factor from PLLI2S VCO clock"]
pub type PllsaiqR = crate::FieldReader;
#[doc = "Field `PLLSAIQ` writer - The PLLI2S Q output frequency division factor from PLLI2S VCO clock"]
pub type PllsaiqW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "The PLLSAI R output frequency division factor from PLLSAI VCO clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pllsair {
    #[doc = "2: TLI clock frequency is PLLSAIVCO divided by 2."]
    Div2 = 2,
    #[doc = "3: TLI clock frequency is PLLSAIVCO divided by 3."]
    Div3 = 3,
    #[doc = "4: TLI clock frequency is PLLSAIVCO divided by 4."]
    Div4 = 4,
    #[doc = "5: TLI clock frequency is PLLSAIVCO divided by 5."]
    Div5 = 5,
    #[doc = "6: TLI clock frequency is PLLSAIVCO divided by 6."]
    Div6 = 6,
    #[doc = "7: TLI clock frequency is PLLSAIVCO divided by 7."]
    Div7 = 7,
}
impl From<Pllsair> for u8 {
    #[inline(always)]
    fn from(variant: Pllsair) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pllsair {
    type Ux = u8;
}
impl crate::IsEnum for Pllsair {}
#[doc = "Field `PLLSAIR` reader - The PLLSAI R output frequency division factor from PLLSAI VCO clock"]
pub type PllsairR = crate::FieldReader<Pllsair>;
impl PllsairR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pllsair> {
        match self.bits {
            2 => Some(Pllsair::Div2),
            3 => Some(Pllsair::Div3),
            4 => Some(Pllsair::Div4),
            5 => Some(Pllsair::Div5),
            6 => Some(Pllsair::Div6),
            7 => Some(Pllsair::Div7),
            _ => None,
        }
    }
    #[doc = "TLI clock frequency is PLLSAIVCO divided by 2."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Pllsair::Div2
    }
    #[doc = "TLI clock frequency is PLLSAIVCO divided by 3."]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == Pllsair::Div3
    }
    #[doc = "TLI clock frequency is PLLSAIVCO divided by 4."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Pllsair::Div4
    }
    #[doc = "TLI clock frequency is PLLSAIVCO divided by 5."]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == Pllsair::Div5
    }
    #[doc = "TLI clock frequency is PLLSAIVCO divided by 6."]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == Pllsair::Div6
    }
    #[doc = "TLI clock frequency is PLLSAIVCO divided by 7."]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == Pllsair::Div7
    }
}
#[doc = "Field `PLLSAIR` writer - The PLLSAI R output frequency division factor from PLLSAI VCO clock"]
pub type PllsairW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pllsair>;
impl<'a, REG> PllsairW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TLI clock frequency is PLLSAIVCO divided by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsair::Div2)
    }
    #[doc = "TLI clock frequency is PLLSAIVCO divided by 3."]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsair::Div3)
    }
    #[doc = "TLI clock frequency is PLLSAIVCO divided by 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsair::Div4)
    }
    #[doc = "TLI clock frequency is PLLSAIVCO divided by 5."]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsair::Div5)
    }
    #[doc = "TLI clock frequency is PLLSAIVCO divided by 6."]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsair::Div6)
    }
    #[doc = "TLI clock frequency is PLLSAIVCO divided by 7."]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsair::Div7)
    }
}
impl R {
    #[doc = "Bits 6:14 - The PLLSAI VCO clock multi factor"]
    #[inline(always)]
    pub fn pllsain(&self) -> PllsainR {
        PllsainR::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:17 - The PLLSAI P output frequency division factor from PLLSAI VCO clock"]
    #[inline(always)]
    pub fn pllsaip(&self) -> PllsaipR {
        PllsaipR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:27 - The PLLI2S Q output frequency division factor from PLLI2S VCO clock"]
    #[inline(always)]
    pub fn pllsaiq(&self) -> PllsaiqR {
        PllsaiqR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - The PLLSAI R output frequency division factor from PLLSAI VCO clock"]
    #[inline(always)]
    pub fn pllsair(&self) -> PllsairR {
        PllsairR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 6:14 - The PLLSAI VCO clock multi factor"]
    #[inline(always)]
    pub fn pllsain(&mut self) -> PllsainW<'_, PllsaiSpec> {
        PllsainW::new(self, 6)
    }
    #[doc = "Bits 16:17 - The PLLSAI P output frequency division factor from PLLSAI VCO clock"]
    #[inline(always)]
    pub fn pllsaip(&mut self) -> PllsaipW<'_, PllsaiSpec> {
        PllsaipW::new(self, 16)
    }
    #[doc = "Bits 24:27 - The PLLI2S Q output frequency division factor from PLLI2S VCO clock"]
    #[inline(always)]
    pub fn pllsaiq(&mut self) -> PllsaiqW<'_, PllsaiSpec> {
        PllsaiqW::new(self, 24)
    }
    #[doc = "Bits 28:30 - The PLLSAI R output frequency division factor from PLLSAI VCO clock"]
    #[inline(always)]
    pub fn pllsair(&mut self) -> PllsairW<'_, PllsaiSpec> {
        PllsairW::new(self, 28)
    }
}
#[doc = "PLLSAI register (RCU_PLLSAI)\n\nYou can [`read`](crate::Reg::read) this register and get [`pllsai::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllsai::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllsaiSpec;
impl crate::RegisterSpec for PllsaiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllsai::R`](R) reader structure"]
impl crate::Readable for PllsaiSpec {}
#[doc = "`write(|w| ..)` method takes [`pllsai::W`](W) writer structure"]
impl crate::Writable for PllsaiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLLSAI to value 0x016e_41c2"]
impl crate::Resettable for PllsaiSpec {
    const RESET_VALUE: u32 = 0x016e_41c2;
}
