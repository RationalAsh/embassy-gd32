#[doc = "Register `AHB2SPEN` reader"]
pub type R = crate::R<Ahb2spenSpec>;
#[doc = "Register `AHB2SPEN` writer"]
pub type W = crate::W<Ahb2spenSpec>;
#[doc = "DCI clock enable when sleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcispen {
    #[doc = "0: Disable the selected module in sleep mode."]
    Disable = 0,
    #[doc = "1: Enable the selected module in sleep mode."]
    Enable = 1,
}
impl From<Dcispen> for bool {
    #[inline(always)]
    fn from(variant: Dcispen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCISPEN` reader - DCI clock enable when sleep mode"]
pub type DcispenR = crate::BitReader<Dcispen>;
impl DcispenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcispen {
        match self.bits {
            false => Dcispen::Disable,
            true => Dcispen::Enable,
        }
    }
    #[doc = "Disable the selected module in sleep mode."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dcispen::Disable
    }
    #[doc = "Enable the selected module in sleep mode."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dcispen::Enable
    }
}
#[doc = "Field `DCISPEN` writer - DCI clock enable when sleep mode"]
pub type DcispenW<'a, REG> = crate::BitWriter<'a, REG, Dcispen>;
impl<'a, REG> DcispenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the selected module in sleep mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dcispen::Disable)
    }
    #[doc = "Enable the selected module in sleep mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dcispen::Enable)
    }
}
#[doc = "Field `TRNGSPEN` reader - TRNG clock enable when sleep mode"]
pub use DcispenR as TrngspenR;
#[doc = "Field `USBFSSPEN` reader - USBFS clock enable when sleep mode"]
pub use DcispenR as UsbfsspenR;
#[doc = "Field `TRNGSPEN` writer - TRNG clock enable when sleep mode"]
pub use DcispenW as TrngspenW;
#[doc = "Field `USBFSSPEN` writer - USBFS clock enable when sleep mode"]
pub use DcispenW as UsbfsspenW;
impl R {
    #[doc = "Bit 0 - DCI clock enable when sleep mode"]
    #[inline(always)]
    pub fn dcispen(&self) -> DcispenR {
        DcispenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6 - TRNG clock enable when sleep mode"]
    #[inline(always)]
    pub fn trngspen(&self) -> TrngspenR {
        TrngspenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USBFS clock enable when sleep mode"]
    #[inline(always)]
    pub fn usbfsspen(&self) -> UsbfsspenR {
        UsbfsspenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCI clock enable when sleep mode"]
    #[inline(always)]
    pub fn dcispen(&mut self) -> DcispenW<'_, Ahb2spenSpec> {
        DcispenW::new(self, 0)
    }
    #[doc = "Bit 6 - TRNG clock enable when sleep mode"]
    #[inline(always)]
    pub fn trngspen(&mut self) -> TrngspenW<'_, Ahb2spenSpec> {
        TrngspenW::new(self, 6)
    }
    #[doc = "Bit 7 - USBFS clock enable when sleep mode"]
    #[inline(always)]
    pub fn usbfsspen(&mut self) -> UsbfsspenW<'_, Ahb2spenSpec> {
        UsbfsspenW::new(self, 7)
    }
}
#[doc = "AHB2 sleep mode enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2spen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2spen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb2spenSpec;
impl crate::RegisterSpec for Ahb2spenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2spen::R`](R) reader structure"]
impl crate::Readable for Ahb2spenSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb2spen::W`](W) writer structure"]
impl crate::Writable for Ahb2spenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB2SPEN to value 0xc1"]
impl crate::Resettable for Ahb2spenSpec {
    const RESET_VALUE: u32 = 0xc1;
}
