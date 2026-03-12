#[doc = "Register `MAC_FCTH` reader"]
pub type R = crate::R<MacFcthSpec>;
#[doc = "Register `MAC_FCTH` writer"]
pub type W = crate::W<MacFcthSpec>;
#[doc = "Field `RFA` reader - Threshold of active flow control"]
pub type RfaR = crate::FieldReader;
#[doc = "Field `RFA` writer - Threshold of active flow control"]
pub type RfaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RFD` reader - Threshold of deactive flow control"]
pub type RfdR = crate::FieldReader;
#[doc = "Field `RFD` writer - Threshold of deactive flow control"]
pub type RfdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Threshold of active flow control"]
    #[inline(always)]
    pub fn rfa(&self) -> RfaR {
        RfaR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Threshold of deactive flow control"]
    #[inline(always)]
    pub fn rfd(&self) -> RfdR {
        RfdR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Threshold of active flow control"]
    #[inline(always)]
    pub fn rfa(&mut self) -> RfaW<'_, MacFcthSpec> {
        RfaW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Threshold of deactive flow control"]
    #[inline(always)]
    pub fn rfd(&mut self) -> RfdW<'_, MacFcthSpec> {
        RfdW::new(self, 4)
    }
}
#[doc = "Ethernet MAC flow control threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_fcth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_fcth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacFcthSpec;
impl crate::RegisterSpec for MacFcthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_fcth::R`](R) reader structure"]
impl crate::Readable for MacFcthSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_fcth::W`](W) writer structure"]
impl crate::Writable for MacFcthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC_FCTH to value 0x15"]
impl crate::Resettable for MacFcthSpec {
    const RESET_VALUE: u32 = 0x15;
}
