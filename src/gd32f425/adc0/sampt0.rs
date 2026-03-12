#[doc = "Register `SAMPT0` reader"]
pub type R = crate::R<Sampt0Spec>;
#[doc = "Register `SAMPT0` writer"]
pub type W = crate::W<Sampt0Spec>;
#[doc = "Channel 10 sample time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spt10 {
    #[doc = "0: Channel sample time is 3 cycles"]
    Cycles3 = 0,
    #[doc = "1: Channel sample time is 15 cycles"]
    Cycles15 = 1,
    #[doc = "2: Channel sample time is 28 cycles"]
    Cycles28 = 2,
    #[doc = "3: Channel sample time is 56 cycles"]
    Cycles56 = 3,
    #[doc = "4: Channel sample time is 84 cycles"]
    Cycles84 = 4,
    #[doc = "5: Channel sample time is 112 cycles"]
    Cycles112 = 5,
    #[doc = "6: Channel sample time is 144 cycles"]
    Cycles144 = 6,
    #[doc = "7: Channel sample time is 480 cycles"]
    Cycles480 = 7,
}
impl From<Spt10> for u8 {
    #[inline(always)]
    fn from(variant: Spt10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spt10 {
    type Ux = u8;
}
impl crate::IsEnum for Spt10 {}
#[doc = "Field `SPT10` reader - Channel 10 sample time selection"]
pub type Spt10R = crate::FieldReader<Spt10>;
impl Spt10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spt10 {
        match self.bits {
            0 => Spt10::Cycles3,
            1 => Spt10::Cycles15,
            2 => Spt10::Cycles28,
            3 => Spt10::Cycles56,
            4 => Spt10::Cycles84,
            5 => Spt10::Cycles112,
            6 => Spt10::Cycles144,
            7 => Spt10::Cycles480,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel sample time is 3 cycles"]
    #[inline(always)]
    pub fn is_cycles3(&self) -> bool {
        *self == Spt10::Cycles3
    }
    #[doc = "Channel sample time is 15 cycles"]
    #[inline(always)]
    pub fn is_cycles15(&self) -> bool {
        *self == Spt10::Cycles15
    }
    #[doc = "Channel sample time is 28 cycles"]
    #[inline(always)]
    pub fn is_cycles28(&self) -> bool {
        *self == Spt10::Cycles28
    }
    #[doc = "Channel sample time is 56 cycles"]
    #[inline(always)]
    pub fn is_cycles56(&self) -> bool {
        *self == Spt10::Cycles56
    }
    #[doc = "Channel sample time is 84 cycles"]
    #[inline(always)]
    pub fn is_cycles84(&self) -> bool {
        *self == Spt10::Cycles84
    }
    #[doc = "Channel sample time is 112 cycles"]
    #[inline(always)]
    pub fn is_cycles112(&self) -> bool {
        *self == Spt10::Cycles112
    }
    #[doc = "Channel sample time is 144 cycles"]
    #[inline(always)]
    pub fn is_cycles144(&self) -> bool {
        *self == Spt10::Cycles144
    }
    #[doc = "Channel sample time is 480 cycles"]
    #[inline(always)]
    pub fn is_cycles480(&self) -> bool {
        *self == Spt10::Cycles480
    }
}
#[doc = "Field `SPT10` writer - Channel 10 sample time selection"]
pub type Spt10W<'a, REG> = crate::FieldWriter<'a, REG, 3, Spt10, crate::Safe>;
impl<'a, REG> Spt10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel sample time is 3 cycles"]
    #[inline(always)]
    pub fn cycles3(self) -> &'a mut crate::W<REG> {
        self.variant(Spt10::Cycles3)
    }
    #[doc = "Channel sample time is 15 cycles"]
    #[inline(always)]
    pub fn cycles15(self) -> &'a mut crate::W<REG> {
        self.variant(Spt10::Cycles15)
    }
    #[doc = "Channel sample time is 28 cycles"]
    #[inline(always)]
    pub fn cycles28(self) -> &'a mut crate::W<REG> {
        self.variant(Spt10::Cycles28)
    }
    #[doc = "Channel sample time is 56 cycles"]
    #[inline(always)]
    pub fn cycles56(self) -> &'a mut crate::W<REG> {
        self.variant(Spt10::Cycles56)
    }
    #[doc = "Channel sample time is 84 cycles"]
    #[inline(always)]
    pub fn cycles84(self) -> &'a mut crate::W<REG> {
        self.variant(Spt10::Cycles84)
    }
    #[doc = "Channel sample time is 112 cycles"]
    #[inline(always)]
    pub fn cycles112(self) -> &'a mut crate::W<REG> {
        self.variant(Spt10::Cycles112)
    }
    #[doc = "Channel sample time is 144 cycles"]
    #[inline(always)]
    pub fn cycles144(self) -> &'a mut crate::W<REG> {
        self.variant(Spt10::Cycles144)
    }
    #[doc = "Channel sample time is 480 cycles"]
    #[inline(always)]
    pub fn cycles480(self) -> &'a mut crate::W<REG> {
        self.variant(Spt10::Cycles480)
    }
}
#[doc = "Field `SPT11` reader - Channel 11 sample time selection"]
pub use Spt10R as Spt11R;
#[doc = "Field `SPT12` reader - Channel 12 sample time selection"]
pub use Spt10R as Spt12R;
#[doc = "Field `SPT13` reader - Channel 13 sample time selection"]
pub use Spt10R as Spt13R;
#[doc = "Field `SPT14` reader - Channel 14 sample time selection"]
pub use Spt10R as Spt14R;
#[doc = "Field `SPT15` reader - Channel 15 sample time selection"]
pub use Spt10R as Spt15R;
#[doc = "Field `SPT16` reader - Channel 16 sample time selection"]
pub use Spt10R as Spt16R;
#[doc = "Field `SPT17` reader - Channel 17 sample time selection"]
pub use Spt10R as Spt17R;
#[doc = "Field `SPT18` reader - Channel 18 sample time selection"]
pub use Spt10R as Spt18R;
#[doc = "Field `SPT11` writer - Channel 11 sample time selection"]
pub use Spt10W as Spt11W;
#[doc = "Field `SPT12` writer - Channel 12 sample time selection"]
pub use Spt10W as Spt12W;
#[doc = "Field `SPT13` writer - Channel 13 sample time selection"]
pub use Spt10W as Spt13W;
#[doc = "Field `SPT14` writer - Channel 14 sample time selection"]
pub use Spt10W as Spt14W;
#[doc = "Field `SPT15` writer - Channel 15 sample time selection"]
pub use Spt10W as Spt15W;
#[doc = "Field `SPT16` writer - Channel 16 sample time selection"]
pub use Spt10W as Spt16W;
#[doc = "Field `SPT17` writer - Channel 17 sample time selection"]
pub use Spt10W as Spt17W;
#[doc = "Field `SPT18` writer - Channel 18 sample time selection"]
pub use Spt10W as Spt18W;
impl R {
    #[doc = "Bits 0:2 - Channel 10 sample time selection"]
    #[inline(always)]
    pub fn spt10(&self) -> Spt10R {
        Spt10R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Channel 11 sample time selection"]
    #[inline(always)]
    pub fn spt11(&self) -> Spt11R {
        Spt11R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Channel 12 sample time selection"]
    #[inline(always)]
    pub fn spt12(&self) -> Spt12R {
        Spt12R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Channel 13 sample time selection"]
    #[inline(always)]
    pub fn spt13(&self) -> Spt13R {
        Spt13R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel 14 sample time selection"]
    #[inline(always)]
    pub fn spt14(&self) -> Spt14R {
        Spt14R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel 15 sample time selection"]
    #[inline(always)]
    pub fn spt15(&self) -> Spt15R {
        Spt15R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Channel 16 sample time selection"]
    #[inline(always)]
    pub fn spt16(&self) -> Spt16R {
        Spt16R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Channel 17 sample time selection"]
    #[inline(always)]
    pub fn spt17(&self) -> Spt17R {
        Spt17R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Channel 18 sample time selection"]
    #[inline(always)]
    pub fn spt18(&self) -> Spt18R {
        Spt18R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel 10 sample time selection"]
    #[inline(always)]
    pub fn spt10(&mut self) -> Spt10W<'_, Sampt0Spec> {
        Spt10W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Channel 11 sample time selection"]
    #[inline(always)]
    pub fn spt11(&mut self) -> Spt11W<'_, Sampt0Spec> {
        Spt11W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Channel 12 sample time selection"]
    #[inline(always)]
    pub fn spt12(&mut self) -> Spt12W<'_, Sampt0Spec> {
        Spt12W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Channel 13 sample time selection"]
    #[inline(always)]
    pub fn spt13(&mut self) -> Spt13W<'_, Sampt0Spec> {
        Spt13W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Channel 14 sample time selection"]
    #[inline(always)]
    pub fn spt14(&mut self) -> Spt14W<'_, Sampt0Spec> {
        Spt14W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Channel 15 sample time selection"]
    #[inline(always)]
    pub fn spt15(&mut self) -> Spt15W<'_, Sampt0Spec> {
        Spt15W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Channel 16 sample time selection"]
    #[inline(always)]
    pub fn spt16(&mut self) -> Spt16W<'_, Sampt0Spec> {
        Spt16W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Channel 17 sample time selection"]
    #[inline(always)]
    pub fn spt17(&mut self) -> Spt17W<'_, Sampt0Spec> {
        Spt17W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Channel 18 sample time selection"]
    #[inline(always)]
    pub fn spt18(&mut self) -> Spt18W<'_, Sampt0Spec> {
        Spt18W::new(self, 24)
    }
}
#[doc = "Sample time register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sampt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sampt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sampt0Spec;
impl crate::RegisterSpec for Sampt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sampt0::R`](R) reader structure"]
impl crate::Readable for Sampt0Spec {}
#[doc = "`write(|w| ..)` method takes [`sampt0::W`](W) writer structure"]
impl crate::Writable for Sampt0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAMPT0 to value 0"]
impl crate::Resettable for Sampt0Spec {}
