#[doc = "Register `TMP0` reader"]
pub type R = crate::R<Tmp0Spec>;
#[doc = "Register `TMP0` writer"]
pub type W = crate::W<Tmp0Spec>;
#[doc = "Field `DLENC` reader - Data length code"]
pub type DlencR = crate::FieldReader;
#[doc = "Field `DLENC` writer - Data length code"]
pub type DlencW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TSEN` reader - Time stamp enable"]
pub type TsenR = crate::BitReader;
#[doc = "Field `TSEN` writer - Time stamp enable"]
pub type TsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS` reader - Time stamp"]
pub type TsR = crate::FieldReader<u16>;
#[doc = "Field `TS` writer - Time stamp"]
pub type TsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - Data length code"]
    #[inline(always)]
    pub fn dlenc(&self) -> DlencR {
        DlencR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Time stamp enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TsenR {
        TsenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Time stamp"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data length code"]
    #[inline(always)]
    pub fn dlenc(&mut self) -> DlencW<'_, Tmp0Spec> {
        DlencW::new(self, 0)
    }
    #[doc = "Bit 8 - Time stamp enable"]
    #[inline(always)]
    pub fn tsen(&mut self) -> TsenW<'_, Tmp0Spec> {
        TsenW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Time stamp"]
    #[inline(always)]
    pub fn ts(&mut self) -> TsW<'_, Tmp0Spec> {
        TsW::new(self, 16)
    }
}
#[doc = "Transmit mailbox property register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tmp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmp0Spec;
impl crate::RegisterSpec for Tmp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmp0::R`](R) reader structure"]
impl crate::Readable for Tmp0Spec {}
#[doc = "`write(|w| ..)` method takes [`tmp0::W`](W) writer structure"]
impl crate::Writable for Tmp0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMP0 to value 0"]
impl crate::Resettable for Tmp0Spec {}
