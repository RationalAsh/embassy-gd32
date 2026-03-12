#[doc = "Register `PLLI2S` reader"]
pub type R = crate::R<Plli2sSpec>;
#[doc = "Register `PLLI2S` writer"]
pub type W = crate::W<Plli2sSpec>;
#[doc = "Field `PLLI2SPSC` reader - The PLLI2S VCO source clock prescaler"]
pub type Plli2spscR = crate::FieldReader;
#[doc = "Field `PLLI2SPSC` writer - The PLLI2S VCO source clock prescaler"]
pub type Plli2spscW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PLLI2SN` reader - The PLLI2S VCO clock multi factor"]
pub type Plli2snR = crate::FieldReader<u16>;
#[doc = "Field `PLLI2SN` writer - The PLLI2S VCO clock multi factor"]
pub type Plli2snW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PLLI2SQ` reader - The PLLI2S Q output frequency division factor from PLLI2S VCO clock"]
pub type Plli2sqR = crate::FieldReader;
#[doc = "Field `PLLI2SQ` writer - The PLLI2S Q output frequency division factor from PLLI2S VCO clock"]
pub type Plli2sqW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "The PLLI2S R output frequency division factor from PLLI2S VCO clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Plli2sr {
    #[doc = "2: I2S clock frequency divided by 2."]
    Div2 = 2,
    #[doc = "3: I2S clock frequency divided by 3."]
    Div3 = 3,
    #[doc = "4: I2S clock frequency divided by 4."]
    Div4 = 4,
    #[doc = "5: I2S clock frequency divided by 5."]
    Div5 = 5,
    #[doc = "6: I2S clock frequency divided by 6."]
    Div6 = 6,
    #[doc = "7: I2S clock frequency divided by 7."]
    Div7 = 7,
}
impl From<Plli2sr> for u8 {
    #[inline(always)]
    fn from(variant: Plli2sr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Plli2sr {
    type Ux = u8;
}
impl crate::IsEnum for Plli2sr {}
#[doc = "Field `PLLI2SR` reader - The PLLI2S R output frequency division factor from PLLI2S VCO clock"]
pub type Plli2srR = crate::FieldReader<Plli2sr>;
impl Plli2srR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Plli2sr> {
        match self.bits {
            2 => Some(Plli2sr::Div2),
            3 => Some(Plli2sr::Div3),
            4 => Some(Plli2sr::Div4),
            5 => Some(Plli2sr::Div5),
            6 => Some(Plli2sr::Div6),
            7 => Some(Plli2sr::Div7),
            _ => None,
        }
    }
    #[doc = "I2S clock frequency divided by 2."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Plli2sr::Div2
    }
    #[doc = "I2S clock frequency divided by 3."]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == Plli2sr::Div3
    }
    #[doc = "I2S clock frequency divided by 4."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Plli2sr::Div4
    }
    #[doc = "I2S clock frequency divided by 5."]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == Plli2sr::Div5
    }
    #[doc = "I2S clock frequency divided by 6."]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == Plli2sr::Div6
    }
    #[doc = "I2S clock frequency divided by 7."]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == Plli2sr::Div7
    }
}
#[doc = "Field `PLLI2SR` writer - The PLLI2S R output frequency division factor from PLLI2S VCO clock"]
pub type Plli2srW<'a, REG> = crate::FieldWriter<'a, REG, 3, Plli2sr>;
impl<'a, REG> Plli2srW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "I2S clock frequency divided by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Plli2sr::Div2)
    }
    #[doc = "I2S clock frequency divided by 3."]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(Plli2sr::Div3)
    }
    #[doc = "I2S clock frequency divided by 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Plli2sr::Div4)
    }
    #[doc = "I2S clock frequency divided by 5."]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(Plli2sr::Div5)
    }
    #[doc = "I2S clock frequency divided by 6."]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(Plli2sr::Div6)
    }
    #[doc = "I2S clock frequency divided by 7."]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(Plli2sr::Div7)
    }
}
impl R {
    #[doc = "Bits 0:5 - The PLLI2S VCO source clock prescaler"]
    #[inline(always)]
    pub fn plli2spsc(&self) -> Plli2spscR {
        Plli2spscR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:14 - The PLLI2S VCO clock multi factor"]
    #[inline(always)]
    pub fn plli2sn(&self) -> Plli2snR {
        Plli2snR::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 24:27 - The PLLI2S Q output frequency division factor from PLLI2S VCO clock"]
    #[inline(always)]
    pub fn plli2sq(&self) -> Plli2sqR {
        Plli2sqR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - The PLLI2S R output frequency division factor from PLLI2S VCO clock"]
    #[inline(always)]
    pub fn plli2sr(&self) -> Plli2srR {
        Plli2srR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The PLLI2S VCO source clock prescaler"]
    #[inline(always)]
    pub fn plli2spsc(&mut self) -> Plli2spscW<'_, Plli2sSpec> {
        Plli2spscW::new(self, 0)
    }
    #[doc = "Bits 6:14 - The PLLI2S VCO clock multi factor"]
    #[inline(always)]
    pub fn plli2sn(&mut self) -> Plli2snW<'_, Plli2sSpec> {
        Plli2snW::new(self, 6)
    }
    #[doc = "Bits 24:27 - The PLLI2S Q output frequency division factor from PLLI2S VCO clock"]
    #[inline(always)]
    pub fn plli2sq(&mut self) -> Plli2sqW<'_, Plli2sSpec> {
        Plli2sqW::new(self, 24)
    }
    #[doc = "Bits 28:30 - The PLLI2S R output frequency division factor from PLLI2S VCO clock"]
    #[inline(always)]
    pub fn plli2sr(&mut self) -> Plli2srW<'_, Plli2sSpec> {
        Plli2srW::new(self, 28)
    }
}
#[doc = "PLLI2S register (RCU_PLLI2S)\n\nYou can [`read`](crate::Reg::read) this register and get [`plli2s::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plli2s::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Plli2sSpec;
impl crate::RegisterSpec for Plli2sSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plli2s::R`](R) reader structure"]
impl crate::Readable for Plli2sSpec {}
#[doc = "`write(|w| ..)` method takes [`plli2s::W`](W) writer structure"]
impl crate::Writable for Plli2sSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLLI2S to value 0x016e_41b8"]
impl crate::Resettable for Plli2sSpec {
    const RESET_VALUE: u32 = 0x016e_41b8;
}
