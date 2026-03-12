#[doc = "Register `NPINTEN1` reader"]
pub type R = crate::R<Npinten1Spec>;
#[doc = "Register `NPINTEN1` writer"]
pub type W = crate::W<Npinten1Spec>;
#[doc = "Field `INTRS` reader - Interrupt rising edge status"]
pub type IntrsR = crate::BitReader;
#[doc = "Field `INTRS` writer - Interrupt rising edge status"]
pub type IntrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTHS` reader - Interrupt high-level status"]
pub type InthsR = crate::BitReader;
#[doc = "Field `INTHS` writer - Interrupt high-level status"]
pub type InthsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTFS` reader - Interrupt falling edge status"]
pub type IntfsR = crate::BitReader;
#[doc = "Field `INTFS` writer - Interrupt falling edge status"]
pub type IntfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTREN` reader - Interrupt rising edge detection enable bit"]
pub type IntrenR = crate::BitReader;
#[doc = "Field `INTREN` writer - Interrupt rising edge detection enable bit"]
pub type IntrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTHEN` reader - Interrupt high-level detection enable"]
pub type InthenR = crate::BitReader;
#[doc = "Field `INTHEN` writer - Interrupt high-level detection enable"]
pub type InthenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTFEN` reader - Interrupt falling edge detection enable"]
pub type IntfenR = crate::BitReader;
#[doc = "Field `INTFEN` writer - Interrupt falling edge detection enable"]
pub type IntfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFEPT` reader - FIFO empty flag"]
pub type FfeptR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt rising edge status"]
    #[inline(always)]
    pub fn intrs(&self) -> IntrsR {
        IntrsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt high-level status"]
    #[inline(always)]
    pub fn inths(&self) -> InthsR {
        InthsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt falling edge status"]
    #[inline(always)]
    pub fn intfs(&self) -> IntfsR {
        IntfsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt rising edge detection enable bit"]
    #[inline(always)]
    pub fn intren(&self) -> IntrenR {
        IntrenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt high-level detection enable"]
    #[inline(always)]
    pub fn inthen(&self) -> InthenR {
        InthenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt falling edge detection enable"]
    #[inline(always)]
    pub fn intfen(&self) -> IntfenR {
        IntfenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FIFO empty flag"]
    #[inline(always)]
    pub fn ffept(&self) -> FfeptR {
        FfeptR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt rising edge status"]
    #[inline(always)]
    pub fn intrs(&mut self) -> IntrsW<'_, Npinten1Spec> {
        IntrsW::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt high-level status"]
    #[inline(always)]
    pub fn inths(&mut self) -> InthsW<'_, Npinten1Spec> {
        InthsW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt falling edge status"]
    #[inline(always)]
    pub fn intfs(&mut self) -> IntfsW<'_, Npinten1Spec> {
        IntfsW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt rising edge detection enable bit"]
    #[inline(always)]
    pub fn intren(&mut self) -> IntrenW<'_, Npinten1Spec> {
        IntrenW::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt high-level detection enable"]
    #[inline(always)]
    pub fn inthen(&mut self) -> InthenW<'_, Npinten1Spec> {
        InthenW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt falling edge detection enable"]
    #[inline(always)]
    pub fn intfen(&mut self) -> IntfenW<'_, Npinten1Spec> {
        IntfenW::new(self, 5)
    }
}
#[doc = "NAND flash/PC card interrupt enable register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`npinten1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`npinten1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Npinten1Spec;
impl crate::RegisterSpec for Npinten1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`npinten1::R`](R) reader structure"]
impl crate::Readable for Npinten1Spec {}
#[doc = "`write(|w| ..)` method takes [`npinten1::W`](W) writer structure"]
impl crate::Writable for Npinten1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NPINTEN1 to value 0x40"]
impl crate::Resettable for Npinten1Spec {
    const RESET_VALUE: u32 = 0x40;
}
