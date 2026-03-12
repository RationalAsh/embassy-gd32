#[doc = "Register `EC` reader"]
pub type R = crate::R<EcSpec>;
#[doc = "Register `EC` writer"]
pub type W = crate::W<EcSpec>;
#[doc = "Field `PIN` reader - Event output pin selection"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - Event output pin selection"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PORT` reader - Event output port selection"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - Event output port selection"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EOE` reader - Event output enable"]
pub type EoeR = crate::BitReader;
#[doc = "Field `EOE` writer - Event output enable"]
pub type EoeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Event output pin selection"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Event output port selection"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Event output enable"]
    #[inline(always)]
    pub fn eoe(&self) -> EoeR {
        EoeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Event output pin selection"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<'_, EcSpec> {
        PinW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Event output port selection"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<'_, EcSpec> {
        PortW::new(self, 4)
    }
    #[doc = "Bit 7 - Event output enable"]
    #[inline(always)]
    pub fn eoe(&mut self) -> EoeW<'_, EcSpec> {
        EoeW::new(self, 7)
    }
}
#[doc = "Event control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcSpec;
impl crate::RegisterSpec for EcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ec::R`](R) reader structure"]
impl crate::Readable for EcSpec {}
#[doc = "`write(|w| ..)` method takes [`ec::W`](W) writer structure"]
impl crate::Writable for EcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EC to value 0"]
impl crate::Resettable for EcSpec {}
