#[doc = "Register `TG` writer"]
pub type W = crate::W<TgSpec>;
#[doc = "Port toggle bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tg0w {
    #[doc = "1: Toggles the corresponding OCTLx bit"]
    Toggle = 1,
}
impl From<Tg0w> for bool {
    #[inline(always)]
    fn from(variant: Tg0w) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TG0` writer - Port toggle bit"]
pub type Tg0W<'a, REG> = crate::BitWriter<'a, REG, Tg0w>;
impl<'a, REG> Tg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Toggles the corresponding OCTLx bit"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Tg0w::Toggle)
    }
}
#[doc = "Field `TG1` writer - Port toggle bit"]
pub use Tg0W as Tg1W;
#[doc = "Field `TG2` writer - Port toggle bit"]
pub use Tg0W as Tg2W;
#[doc = "Field `TG3` writer - Port toggle bit"]
pub use Tg0W as Tg3W;
#[doc = "Field `TG4` writer - Port toggle bit"]
pub use Tg0W as Tg4W;
#[doc = "Field `TG5` writer - Port toggle bit"]
pub use Tg0W as Tg5W;
#[doc = "Field `TG6` writer - Port toggle bit"]
pub use Tg0W as Tg6W;
#[doc = "Field `TG7` writer - Port toggle bit"]
pub use Tg0W as Tg7W;
#[doc = "Field `TG8` writer - Port toggle bit"]
pub use Tg0W as Tg8W;
#[doc = "Field `TG9` writer - Port toggle bit"]
pub use Tg0W as Tg9W;
#[doc = "Field `TG10` writer - Port toggle bit"]
pub use Tg0W as Tg10W;
#[doc = "Field `TG11` writer - Port toggle bit"]
pub use Tg0W as Tg11W;
#[doc = "Field `TG12` writer - Port toggle bit"]
pub use Tg0W as Tg12W;
#[doc = "Field `TG13` writer - Port toggle bit"]
pub use Tg0W as Tg13W;
#[doc = "Field `TG14` writer - Port toggle bit"]
pub use Tg0W as Tg14W;
#[doc = "Field `TG15` writer - Port toggle bit"]
pub use Tg0W as Tg15W;
impl W {
    #[doc = "Bit 0 - Port toggle bit"]
    #[inline(always)]
    pub fn tg0(&mut self) -> Tg0W<'_, TgSpec> {
        Tg0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port toggle bit"]
    #[inline(always)]
    pub fn tg1(&mut self) -> Tg1W<'_, TgSpec> {
        Tg1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port toggle bit"]
    #[inline(always)]
    pub fn tg2(&mut self) -> Tg2W<'_, TgSpec> {
        Tg2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port toggle bit"]
    #[inline(always)]
    pub fn tg3(&mut self) -> Tg3W<'_, TgSpec> {
        Tg3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port toggle bit"]
    #[inline(always)]
    pub fn tg4(&mut self) -> Tg4W<'_, TgSpec> {
        Tg4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port toggle bit"]
    #[inline(always)]
    pub fn tg5(&mut self) -> Tg5W<'_, TgSpec> {
        Tg5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port toggle bit"]
    #[inline(always)]
    pub fn tg6(&mut self) -> Tg6W<'_, TgSpec> {
        Tg6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port toggle bit"]
    #[inline(always)]
    pub fn tg7(&mut self) -> Tg7W<'_, TgSpec> {
        Tg7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port toggle bit"]
    #[inline(always)]
    pub fn tg8(&mut self) -> Tg8W<'_, TgSpec> {
        Tg8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port toggle bit"]
    #[inline(always)]
    pub fn tg9(&mut self) -> Tg9W<'_, TgSpec> {
        Tg9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port toggle bit"]
    #[inline(always)]
    pub fn tg10(&mut self) -> Tg10W<'_, TgSpec> {
        Tg10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port toggle bit"]
    #[inline(always)]
    pub fn tg11(&mut self) -> Tg11W<'_, TgSpec> {
        Tg11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port toggle bit"]
    #[inline(always)]
    pub fn tg12(&mut self) -> Tg12W<'_, TgSpec> {
        Tg12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port toggle bit"]
    #[inline(always)]
    pub fn tg13(&mut self) -> Tg13W<'_, TgSpec> {
        Tg13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port toggle bit"]
    #[inline(always)]
    pub fn tg14(&mut self) -> Tg14W<'_, TgSpec> {
        Tg14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port toggle bit"]
    #[inline(always)]
    pub fn tg15(&mut self) -> Tg15W<'_, TgSpec> {
        Tg15W::new(self, 15)
    }
}
#[doc = "Port bit toggle register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TgSpec;
impl crate::RegisterSpec for TgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tg::W`](W) writer structure"]
impl crate::Writable for TgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TG to value 0"]
impl crate::Resettable for TgSpec {}
