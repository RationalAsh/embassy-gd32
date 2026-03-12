#[doc = "Register `AHB2EN` reader"]
pub type R = crate::R<Ahb2enSpec>;
#[doc = "Register `AHB2EN` writer"]
pub type W = crate::W<Ahb2enSpec>;
#[doc = "DCI clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcien {
    #[doc = "0: Disable the selected module clock."]
    Disable = 0,
    #[doc = "1: Enable the selected module clock."]
    Enable = 1,
}
impl From<Dcien> for bool {
    #[inline(always)]
    fn from(variant: Dcien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCIEN` reader - DCI clock enable"]
pub type DcienR = crate::BitReader<Dcien>;
impl DcienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcien {
        match self.bits {
            false => Dcien::Disable,
            true => Dcien::Enable,
        }
    }
    #[doc = "Disable the selected module clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dcien::Disable
    }
    #[doc = "Enable the selected module clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dcien::Enable
    }
}
#[doc = "Field `DCIEN` writer - DCI clock enable"]
pub type DcienW<'a, REG> = crate::BitWriter<'a, REG, Dcien>;
impl<'a, REG> DcienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the selected module clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dcien::Disable)
    }
    #[doc = "Enable the selected module clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dcien::Enable)
    }
}
#[doc = "Field `TRNGEN` reader - TRNG clock enable"]
pub use DcienR as TrngenR;
#[doc = "Field `USBFSEN` reader - USBFS clock enable"]
pub use DcienR as UsbfsenR;
#[doc = "Field `TRNGEN` writer - TRNG clock enable"]
pub use DcienW as TrngenW;
#[doc = "Field `USBFSEN` writer - USBFS clock enable"]
pub use DcienW as UsbfsenW;
impl R {
    #[doc = "Bit 0 - DCI clock enable"]
    #[inline(always)]
    pub fn dcien(&self) -> DcienR {
        DcienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6 - TRNG clock enable"]
    #[inline(always)]
    pub fn trngen(&self) -> TrngenR {
        TrngenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USBFS clock enable"]
    #[inline(always)]
    pub fn usbfsen(&self) -> UsbfsenR {
        UsbfsenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCI clock enable"]
    #[inline(always)]
    pub fn dcien(&mut self) -> DcienW<'_, Ahb2enSpec> {
        DcienW::new(self, 0)
    }
    #[doc = "Bit 6 - TRNG clock enable"]
    #[inline(always)]
    pub fn trngen(&mut self) -> TrngenW<'_, Ahb2enSpec> {
        TrngenW::new(self, 6)
    }
    #[doc = "Bit 7 - USBFS clock enable"]
    #[inline(always)]
    pub fn usbfsen(&mut self) -> UsbfsenW<'_, Ahb2enSpec> {
        UsbfsenW::new(self, 7)
    }
}
#[doc = "AHB2 enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb2enSpec;
impl crate::RegisterSpec for Ahb2enSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2en::R`](R) reader structure"]
impl crate::Readable for Ahb2enSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb2en::W`](W) writer structure"]
impl crate::Writable for Ahb2enSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB2EN to value 0"]
impl crate::Resettable for Ahb2enSpec {}
