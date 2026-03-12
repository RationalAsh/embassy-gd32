#[doc = "Register `AHB2RST` reader"]
pub type R = crate::R<Ahb2rstSpec>;
#[doc = "Register `AHB2RST` writer"]
pub type W = crate::W<Ahb2rstSpec>;
#[doc = "DCI reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcirst {
    #[doc = "1: Reset the selected module."]
    Reset = 1,
}
impl From<Dcirst> for bool {
    #[inline(always)]
    fn from(variant: Dcirst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCIRST` reader - DCI reset"]
pub type DcirstR = crate::BitReader<Dcirst>;
impl DcirstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dcirst> {
        match self.bits {
            true => Some(Dcirst::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Dcirst::Reset
    }
}
#[doc = "Field `DCIRST` writer - DCI reset"]
pub type DcirstW<'a, REG> = crate::BitWriter<'a, REG, Dcirst>;
impl<'a, REG> DcirstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Dcirst::Reset)
    }
}
#[doc = "Field `TRNGRST` reader - TRNG reset"]
pub use DcirstR as TrngrstR;
#[doc = "Field `USBFSRST` reader - USBFS reset"]
pub use DcirstR as UsbfsrstR;
#[doc = "Field `TRNGRST` writer - TRNG reset"]
pub use DcirstW as TrngrstW;
#[doc = "Field `USBFSRST` writer - USBFS reset"]
pub use DcirstW as UsbfsrstW;
impl R {
    #[doc = "Bit 0 - DCI reset"]
    #[inline(always)]
    pub fn dcirst(&self) -> DcirstR {
        DcirstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6 - TRNG reset"]
    #[inline(always)]
    pub fn trngrst(&self) -> TrngrstR {
        TrngrstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USBFS reset"]
    #[inline(always)]
    pub fn usbfsrst(&self) -> UsbfsrstR {
        UsbfsrstR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCI reset"]
    #[inline(always)]
    pub fn dcirst(&mut self) -> DcirstW<'_, Ahb2rstSpec> {
        DcirstW::new(self, 0)
    }
    #[doc = "Bit 6 - TRNG reset"]
    #[inline(always)]
    pub fn trngrst(&mut self) -> TrngrstW<'_, Ahb2rstSpec> {
        TrngrstW::new(self, 6)
    }
    #[doc = "Bit 7 - USBFS reset"]
    #[inline(always)]
    pub fn usbfsrst(&mut self) -> UsbfsrstW<'_, Ahb2rstSpec> {
        UsbfsrstW::new(self, 7)
    }
}
#[doc = "AHB2 reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb2rstSpec;
impl crate::RegisterSpec for Ahb2rstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2rst::R`](R) reader structure"]
impl crate::Readable for Ahb2rstSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb2rst::W`](W) writer structure"]
impl crate::Writable for Ahb2rstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB2RST to value 0"]
impl crate::Resettable for Ahb2rstSpec {}
