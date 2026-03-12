#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `CECEN` reader - Enable/disable HDMI-CEC controller"]
pub type CecenR = crate::BitReader;
#[doc = "Field `CECEN` writer - Enable/disable HDMI-CEC controller"]
pub type CecenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOM` reader - Start of sending a message"]
pub type SomR = crate::BitReader;
#[doc = "Field `SOM` writer - Start of sending a message"]
pub type SomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDOM` reader - ENDOM bit value in the next frame in TX mode"]
pub type EndomR = crate::BitReader;
#[doc = "Field `ENDOM` writer - ENDOM bit value in the next frame in TX mode"]
pub type EndomW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable/disable HDMI-CEC controller"]
    #[inline(always)]
    pub fn cecen(&self) -> CecenR {
        CecenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start of sending a message"]
    #[inline(always)]
    pub fn som(&self) -> SomR {
        SomR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ENDOM bit value in the next frame in TX mode"]
    #[inline(always)]
    pub fn endom(&self) -> EndomR {
        EndomR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable/disable HDMI-CEC controller"]
    #[inline(always)]
    pub fn cecen(&mut self) -> CecenW<'_, CtlSpec> {
        CecenW::new(self, 0)
    }
    #[doc = "Bit 1 - Start of sending a message"]
    #[inline(always)]
    pub fn som(&mut self) -> SomW<'_, CtlSpec> {
        SomW::new(self, 1)
    }
    #[doc = "Bit 2 - ENDOM bit value in the next frame in TX mode"]
    #[inline(always)]
    pub fn endom(&mut self) -> EndomW<'_, CtlSpec> {
        EndomW::new(self, 2)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {}
