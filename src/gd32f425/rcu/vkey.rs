#[doc = "Register `VKEY` writer"]
pub type W = crate::W<VkeySpec>;
#[doc = "Field `KEY` writer - The key of RCU_DSV registe"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - The key of RCU_DSV registe"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, VkeySpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "Voltage key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vkey::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VkeySpec;
impl crate::RegisterSpec for VkeySpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`vkey::W`](W) writer structure"]
impl crate::Writable for VkeySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VKEY to value 0"]
impl crate::Resettable for VkeySpec {}
