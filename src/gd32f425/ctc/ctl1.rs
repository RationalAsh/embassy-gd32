#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `RLVALUE` reader - CTC counter reload value"]
pub type RlvalueR = crate::FieldReader<u16>;
#[doc = "Field `RLVALUE` writer - CTC counter reload value"]
pub type RlvalueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CKLIM` reader - Clock trim base limit value"]
pub type CklimR = crate::FieldReader;
#[doc = "Field `CKLIM` writer - Clock trim base limit value"]
pub type CklimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Reference signal source prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refpsc {
    #[doc = "0: Reference signal not divided"]
    Div1 = 0,
    #[doc = "1: Reference signal divided by 2"]
    Div2 = 1,
    #[doc = "2: Reference signal divided by 4"]
    Div4 = 2,
    #[doc = "3: Reference signal divided by 8"]
    Div8 = 3,
    #[doc = "4: Reference signal divided by 16"]
    Div16 = 4,
    #[doc = "5: Reference signal divided by 32"]
    Div32 = 5,
    #[doc = "6: Reference signal divided by 64"]
    Div64 = 6,
    #[doc = "7: Reference signal divided by 128"]
    Div128 = 7,
}
impl From<Refpsc> for u8 {
    #[inline(always)]
    fn from(variant: Refpsc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refpsc {
    type Ux = u8;
}
impl crate::IsEnum for Refpsc {}
#[doc = "Field `REFPSC` reader - Reference signal source prescaler"]
pub type RefpscR = crate::FieldReader<Refpsc>;
impl RefpscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refpsc {
        match self.bits {
            0 => Refpsc::Div1,
            1 => Refpsc::Div2,
            2 => Refpsc::Div4,
            3 => Refpsc::Div8,
            4 => Refpsc::Div16,
            5 => Refpsc::Div32,
            6 => Refpsc::Div64,
            7 => Refpsc::Div128,
            _ => unreachable!(),
        }
    }
    #[doc = "Reference signal not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Refpsc::Div1
    }
    #[doc = "Reference signal divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Refpsc::Div2
    }
    #[doc = "Reference signal divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Refpsc::Div4
    }
    #[doc = "Reference signal divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Refpsc::Div8
    }
    #[doc = "Reference signal divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Refpsc::Div16
    }
    #[doc = "Reference signal divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Refpsc::Div32
    }
    #[doc = "Reference signal divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Refpsc::Div64
    }
    #[doc = "Reference signal divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Refpsc::Div128
    }
}
#[doc = "Field `REFPSC` writer - Reference signal source prescaler"]
pub type RefpscW<'a, REG> = crate::FieldWriter<'a, REG, 3, Refpsc, crate::Safe>;
impl<'a, REG> RefpscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reference signal not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Refpsc::Div1)
    }
    #[doc = "Reference signal divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Refpsc::Div2)
    }
    #[doc = "Reference signal divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Refpsc::Div4)
    }
    #[doc = "Reference signal divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Refpsc::Div8)
    }
    #[doc = "Reference signal divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Refpsc::Div16)
    }
    #[doc = "Reference signal divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Refpsc::Div32)
    }
    #[doc = "Reference signal divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Refpsc::Div64)
    }
    #[doc = "Reference signal divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Refpsc::Div128)
    }
}
#[doc = "Reference signal source selection\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refsel {
    #[doc = "0: GPIO (CTC_SYNC) selected"]
    Gpio = 0,
    #[doc = "1: LXTAL clock selected"]
    Lxtal = 1,
    #[doc = "2: Reserved"]
    Reserved1 = 2,
    #[doc = "3: Reserved"]
    Reserved2 = 3,
}
impl From<Refsel> for u8 {
    #[inline(always)]
    fn from(variant: Refsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refsel {
    type Ux = u8;
}
impl crate::IsEnum for Refsel {}
#[doc = "Field `REFSEL` reader - Reference signal source selection"]
pub type RefselR = crate::FieldReader<Refsel>;
impl RefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refsel {
        match self.bits {
            0 => Refsel::Gpio,
            1 => Refsel::Lxtal,
            2 => Refsel::Reserved1,
            3 => Refsel::Reserved2,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO (CTC_SYNC) selected"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Refsel::Gpio
    }
    #[doc = "LXTAL clock selected"]
    #[inline(always)]
    pub fn is_lxtal(&self) -> bool {
        *self == Refsel::Lxtal
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Refsel::Reserved1
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Refsel::Reserved2
    }
}
#[doc = "Field `REFSEL` writer - Reference signal source selection"]
pub type RefselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Refsel, crate::Safe>;
impl<'a, REG> RefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO (CTC_SYNC) selected"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Gpio)
    }
    #[doc = "LXTAL clock selected"]
    #[inline(always)]
    pub fn lxtal(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Lxtal)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Reserved1)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Reserved2)
    }
}
#[doc = "Field `USBSOFSEL` reader - SOF signal selection"]
pub type UsbsofselR = crate::BitReader;
#[doc = "Field `USBSOFSEL` writer - SOF signal selection"]
pub type UsbsofselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Reference signal source polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refpol {
    #[doc = "0: Rising edge selected"]
    RisingEdge = 0,
    #[doc = "1: Falling edge selected"]
    FallingEdge = 1,
}
impl From<Refpol> for bool {
    #[inline(always)]
    fn from(variant: Refpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFPOL` reader - Reference signal source polarity"]
pub type RefpolR = crate::BitReader<Refpol>;
impl RefpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refpol {
        match self.bits {
            false => Refpol::RisingEdge,
            true => Refpol::FallingEdge,
        }
    }
    #[doc = "Rising edge selected"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == Refpol::RisingEdge
    }
    #[doc = "Falling edge selected"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == Refpol::FallingEdge
    }
}
#[doc = "Field `REFPOL` writer - Reference signal source polarity"]
pub type RefpolW<'a, REG> = crate::BitWriter<'a, REG, Refpol>;
impl<'a, REG> RefpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge selected"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Refpol::RisingEdge)
    }
    #[doc = "Falling edge selected"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Refpol::FallingEdge)
    }
}
impl R {
    #[doc = "Bits 0:15 - CTC counter reload value"]
    #[inline(always)]
    pub fn rlvalue(&self) -> RlvalueR {
        RlvalueR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Clock trim base limit value"]
    #[inline(always)]
    pub fn cklim(&self) -> CklimR {
        CklimR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - Reference signal source prescaler"]
    #[inline(always)]
    pub fn refpsc(&self) -> RefpscR {
        RefpscR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - Reference signal source selection"]
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - SOF signal selection"]
    #[inline(always)]
    pub fn usbsofsel(&self) -> UsbsofselR {
        UsbsofselR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reference signal source polarity"]
    #[inline(always)]
    pub fn refpol(&self) -> RefpolR {
        RefpolR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CTC counter reload value"]
    #[inline(always)]
    pub fn rlvalue(&mut self) -> RlvalueW<'_, Ctl1Spec> {
        RlvalueW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Clock trim base limit value"]
    #[inline(always)]
    pub fn cklim(&mut self) -> CklimW<'_, Ctl1Spec> {
        CklimW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Reference signal source prescaler"]
    #[inline(always)]
    pub fn refpsc(&mut self) -> RefpscW<'_, Ctl1Spec> {
        RefpscW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Reference signal source selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> RefselW<'_, Ctl1Spec> {
        RefselW::new(self, 28)
    }
    #[doc = "Bit 30 - SOF signal selection"]
    #[inline(always)]
    pub fn usbsofsel(&mut self) -> UsbsofselW<'_, Ctl1Spec> {
        UsbsofselW::new(self, 30)
    }
    #[doc = "Bit 31 - Reference signal source polarity"]
    #[inline(always)]
    pub fn refpol(&mut self) -> RefpolW<'_, Ctl1Spec> {
        RefpolW::new(self, 31)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL1 to value 0x2022_bb7f"]
impl crate::Resettable for Ctl1Spec {
    const RESET_VALUE: u32 = 0x2022_bb7f;
}
