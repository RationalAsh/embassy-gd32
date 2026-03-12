#[doc = "Register `SADDR1` reader"]
pub type R = crate::R<Saddr1Spec>;
#[doc = "Register `SADDR1` writer"]
pub type W = crate::W<Saddr1Spec>;
#[doc = "Dual-Address mode switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Duaden {
    #[doc = "0: Single addressing mode"]
    Single = 0,
    #[doc = "1: Dual addressing mode"]
    Dual = 1,
}
impl From<Duaden> for bool {
    #[inline(always)]
    fn from(variant: Duaden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DUADEN` reader - Dual-Address mode switch"]
pub type DuadenR = crate::BitReader<Duaden>;
impl DuadenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Duaden {
        match self.bits {
            false => Duaden::Single,
            true => Duaden::Dual,
        }
    }
    #[doc = "Single addressing mode"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Duaden::Single
    }
    #[doc = "Dual addressing mode"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == Duaden::Dual
    }
}
#[doc = "Field `DUADEN` writer - Dual-Address mode switch"]
pub type DuadenW<'a, REG> = crate::BitWriter<'a, REG, Duaden>;
impl<'a, REG> DuadenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single addressing mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Duaden::Single)
    }
    #[doc = "Dual addressing mode"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut crate::W<REG> {
        self.variant(Duaden::Dual)
    }
}
#[doc = "Field `ADDRESS2` reader - Second I2C address for the slave in Dual-Address mode"]
pub type Address2R = crate::FieldReader;
#[doc = "Field `ADDRESS2` writer - Second I2C address for the slave in Dual-Address mode"]
pub type Address2W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
impl R {
    #[doc = "Bit 0 - Dual-Address mode switch"]
    #[inline(always)]
    pub fn duaden(&self) -> DuadenR {
        DuadenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Second I2C address for the slave in Dual-Address mode"]
    #[inline(always)]
    pub fn address2(&self) -> Address2R {
        Address2R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Dual-Address mode switch"]
    #[inline(always)]
    pub fn duaden(&mut self) -> DuadenW<'_, Saddr1Spec> {
        DuadenW::new(self, 0)
    }
    #[doc = "Bits 1:7 - Second I2C address for the slave in Dual-Address mode"]
    #[inline(always)]
    pub fn address2(&mut self) -> Address2W<'_, Saddr1Spec> {
        Address2W::new(self, 1)
    }
}
#[doc = "Slave address register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`saddr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Saddr1Spec;
impl crate::RegisterSpec for Saddr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saddr1::R`](R) reader structure"]
impl crate::Readable for Saddr1Spec {}
#[doc = "`write(|w| ..)` method takes [`saddr1::W`](W) writer structure"]
impl crate::Writable for Saddr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SADDR1 to value 0"]
impl crate::Resettable for Saddr1Spec {}
