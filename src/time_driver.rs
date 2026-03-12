use core::cell::{Cell, RefCell};
use core::sync::atomic::{compiler_fence, AtomicU32, Ordering};

use cortex_m::peripheral::NVIC;
use critical_section::{CriticalSection, Mutex};
use embassy_time_driver::{Driver, TICK_HZ};
use embassy_time_queue_utils::Queue;
use crate::gd32f425::{Interrupt, Rcu, Timer2};

const IRC16M_HZ: u32 = 16_000_000;
const DEFAULT_HXTAL_HZ: u32 = 8_000_000;
const INTF_UPIF: u32 = 1 << 0;
const INTF_CH0IF: u32 = 1 << 1;
const INTF_CH1IF: u32 = 1 << 2;
const INTF_CH2IF: u32 = 1 << 3;
const INTF_CH3IF: u32 = 1 << 4;
const INTF_TRGIF: u32 = 1 << 6;
const INTF_CH0OF: u32 = 1 << 9;
const INTF_CH1OF: u32 = 1 << 10;
const INTF_CH2OF: u32 = 1 << 11;
const INTF_CH3OF: u32 = 1 << 12;
const INTF_CLEARABLE_MASK: u32 = INTF_UPIF
    | INTF_CH0IF
    | INTF_CH1IF
    | INTF_CH2IF
    | INTF_CH3IF
    | INTF_TRGIF
    | INTF_CH0OF
    | INTF_CH1OF
    | INTF_CH2OF
    | INTF_CH3OF;
const DMAINTEN_CH0IE: u32 = 1 << 1;
// TIMER2 (x=2) is a 16-bit general-purpose timer on GD32F425.
const TIMER2_COUNTER_BITS: u32 = 16;
const TIMER2_COUNTER_MASK: u32 = (1u32 << TIMER2_COUNTER_BITS) - 1;
const MAX_COUNTER_HORIZON: u64 = TIMER2_COUNTER_MASK as u64;

fn ahb_prescaler_div(bits: u8) -> Option<u32> {
    match bits {
        0 => Some(1),
        8 => Some(2),
        9 => Some(4),
        10 => Some(8),
        11 => Some(16),
        12 => Some(64),
        13 => Some(128),
        14 => Some(256),
        15 => Some(512),
        _ => None,
    }
}

fn apb_prescaler_div(bits: u8) -> Option<u32> {
    match bits {
        0 => Some(1),
        4 => Some(2),
        5 => Some(4),
        6 => Some(8),
        7 => Some(16),
        _ => None,
    }
}

fn pllp_div(bits: u8) -> Option<u32> {
    match bits {
        0 => Some(2),
        1 => Some(4),
        2 => Some(8),
        _ => None,
    }
}

fn timer_multiplier(apb_div: u32, timersel_set: bool) -> u32 {
    if timersel_set {
        if apb_div <= 4 {
            apb_div
        } else {
            4
        }
    } else if apb_div <= 2 {
        apb_div
    } else {
        2
    }
}

fn timer2_input_clock_hz(hxtal_hz: u32) -> u32 {
    let rcu = unsafe { Rcu::steal() };
    let cfg0 = rcu.cfg0().read();
    let cfg1 = rcu.cfg1().read();
    let pll = rcu.pll().read();

    let ck_sys_hz = match cfg0.scss().bits() {
        0 => IRC16M_HZ,
        1 => hxtal_hz,
        2 => {
            let pll_source_hz = if pll.pllsel().bit_is_set() {
                hxtal_hz
            } else {
                IRC16M_HZ
            };

            let pllpsc = u32::from(pll.pllpsc().bits());
            if pllpsc == 0 {
                panic!("invalid RCU clock configuration: PLLPSC is 0");
            }

            let plln = u32::from(pll.plln().bits());
            if plln == 0 {
                panic!("invalid RCU clock configuration: PLLN is 0");
            }

            let pllp = pllp_div(pll.pllp().bits())
                .unwrap_or_else(|| panic!("invalid RCU clock configuration: invalid PLLP"));

            let pll_input_hz = pll_source_hz / pllpsc;
            let pll_vco_hz = pll_input_hz
                .checked_mul(plln)
                .expect("invalid RCU clock configuration: PLL VCO overflow");
            pll_vco_hz / pllp
        }
        scss => panic!(
            "invalid RCU clock configuration: unsupported SCSS value {}",
            scss
        ),
    };

    let ahb_div = ahb_prescaler_div(cfg0.ahbpsc().bits())
        .unwrap_or_else(|| panic!("invalid RCU clock configuration: invalid AHB prescaler"));
    let apb1_div = apb_prescaler_div(cfg0.apb1psc().bits())
        .unwrap_or_else(|| panic!("invalid RCU clock configuration: invalid APB1 prescaler"));

    let ck_ahb_hz = ck_sys_hz / ahb_div;
    let ck_apb1_hz = ck_ahb_hz / apb1_div;

    let timer_apb1_mul = timer_multiplier(apb1_div, cfg1.timersel().bit_is_set());
    ck_apb1_hz
        .checked_mul(timer_apb1_mul)
        .expect("invalid RCU clock configuration: APB1 timer clock overflow")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AlarmArmDecision {
    FireNow,
    ArmCompare,
    Defer,
}

fn classify_alarm_arm(now: u64, timestamp: u64) -> AlarmArmDecision {
    if timestamp <= now {
        AlarmArmDecision::FireNow
    } else if timestamp - now <= MAX_COUNTER_HORIZON {
        AlarmArmDecision::ArmCompare
    } else {
        AlarmArmDecision::Defer
    }
}

struct AlarmState {
    timestamp: Cell<u64>,
}

unsafe impl Send for AlarmState {}

impl AlarmState {
    const fn new() -> Self {
        Self {
            timestamp: Cell::new(u64::MAX),
        }
    }
}

struct EmbassyTimerDriver {
    high_word: AtomicU32,
    alarm: Mutex<AlarmState>,
    queue: Mutex<RefCell<Queue>>,
}

embassy_time_driver::time_driver_impl!(static DRIVER: EmbassyTimerDriver = EmbassyTimerDriver {
    high_word: AtomicU32::new(0),
    alarm: Mutex::new(AlarmState::new()),
    queue: Mutex::new(RefCell::new(Queue::new())),
});

impl EmbassyTimerDriver {
    fn init(&'static self, cs: CriticalSection) {
        self.high_word.store(0, Ordering::Relaxed);
        self.alarm.borrow(cs).timestamp.set(u64::MAX);

        let timer_clock_hz = u64::from(timer2_input_clock_hz(DEFAULT_HXTAL_HZ));
        let tick_hz = TICK_HZ;
        let divider = timer_clock_hz / tick_hz;
        if divider == 0 || timer_clock_hz % tick_hz != 0 {
            panic!(
                "TIMER2 clock {}Hz is not divisible by tick rate {}Hz",
                timer_clock_hz, tick_hz
            );
        }
        let psc = divider - 1;
        if psc > u64::from(u16::MAX) {
            panic!("TIMER2 prescaler overflow: {}", psc);
        }
        let psc = psc as u32;

        let rcu = unsafe { Rcu::steal() };
        rcu.apb1en().modify(|_, w| w.timer2en().set_bit());
        rcu.apb1rst().modify(|_, w| w.timer2rst().set_bit());
        rcu.apb1rst().modify(|_, w| w.timer2rst().clear_bit());

        let timer = unsafe { Timer2::steal() };
        timer.ctl0().modify(|_, w| w.cen().clear_bit());
        timer.smcfg().modify(|_, w| w.smc().disabled());
        timer.psc().write(|w| unsafe { w.bits(psc) });
        timer
            .car()
            .write(|w| unsafe { w.bits(TIMER2_COUNTER_MASK) });
        timer.cnt().write(|w| unsafe { w.bits(0) });
        timer.ch0cv().write(|w| unsafe { w.bits(0) });

        // Latch prescaler and auto-reload immediately.
        timer.swevg().write(|w| w.upg().set_bit());

        Self::clear_interrupt_flags(&timer, timer.intf().read().bits());
        timer
            .dmainten()
            .write(|w| w.upie().set_bit().ch0ie().clear_bit());

        NVIC::unpend(Interrupt::TIMER2);
        unsafe { NVIC::unmask(Interrupt::TIMER2) };

        timer.ctl0().modify(|_, w| w.cen().set_bit());
    }

    fn clear_interrupt_flags(timer: &Timer2, status: u32) {
        // INTF bits are write-0-to-clear. Write 1 to untouched flags and 0 to active ones.
        let clear_bits = (!status) & INTF_CLEARABLE_MASK;
        timer.intf().write(|w| unsafe { w.bits(clear_bits) });
    }

    fn set_alarm(&self, cs: CriticalSection, timestamp: u64) -> bool {
        let timer = unsafe { Timer2::steal() };
        self.alarm.borrow(cs).timestamp.set(timestamp);

        let now = self.now();
        match classify_alarm_arm(now, timestamp) {
            AlarmArmDecision::FireNow => {
                timer.dmainten().modify(|_, w| w.ch0ie().clear_bit());
                self.alarm.borrow(cs).timestamp.set(u64::MAX);
                return false;
            }
            AlarmArmDecision::ArmCompare => {
                timer
                    .ch0cv()
                    .write(|w| unsafe { w.bits((timestamp as u32) & TIMER2_COUNTER_MASK) });
                timer.dmainten().modify(|_, w| w.ch0ie().set_bit());
            }
            AlarmArmDecision::Defer => {
                timer
                    .ch0cv()
                    .write(|w| unsafe { w.bits((timestamp as u32) & TIMER2_COUNTER_MASK) });
                timer.dmainten().modify(|_, w| w.ch0ie().clear_bit());
            }
        }

        if timestamp <= self.now() {
            timer.dmainten().modify(|_, w| w.ch0ie().clear_bit());
            self.alarm.borrow(cs).timestamp.set(u64::MAX);
            return false;
        }

        true
    }

    fn on_overflow(&self, cs: CriticalSection) {
        self.high_word.fetch_add(1, Ordering::Relaxed);
        let at = self.alarm.borrow(cs).timestamp.get();
        if at != u64::MAX && !self.set_alarm(cs, at) {
            self.trigger_alarm(cs);
        }
    }

    fn trigger_alarm(&self, cs: CriticalSection) {
        let mut queue = self.queue.borrow(cs).borrow_mut();
        let mut next = queue.next_expiration(self.now());

        while !self.set_alarm(cs, next) {
            next = queue.next_expiration(self.now());
        }
    }
}

impl Driver for EmbassyTimerDriver {
    fn now(&self) -> u64 {
        let timer = unsafe { Timer2::steal() };

        loop {
            let high_before = self.high_word.load(Ordering::Relaxed);
            compiler_fence(Ordering::Acquire);
            let upif_before = timer.intf().read().upif().bit_is_set();
            let cnt = timer.cnt().read().cnt().bits();
            let upif_after = timer.intf().read().upif().bit_is_set();
            compiler_fence(Ordering::Acquire);
            let high_after = self.high_word.load(Ordering::Relaxed);

            if high_before != high_after || upif_before != upif_after {
                continue;
            }

            let high = if upif_after {
                high_before.wrapping_add(1)
            } else {
                high_before
            };

            let cnt = cnt & TIMER2_COUNTER_MASK;
            return ((high as u64) << TIMER2_COUNTER_BITS) | u64::from(cnt);
        }
    }

    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
        critical_section::with(|cs| {
            let mut queue = self.queue.borrow(cs).borrow_mut();
            if queue.schedule_wake(at, waker) {
                let mut next = queue.next_expiration(self.now());
                while !self.set_alarm(cs, next) {
                    next = queue.next_expiration(self.now());
                }
            }
        });
    }
}

pub fn init() {
    critical_section::with(|cs| DRIVER.init(cs));
}

pub fn on_interrupt() {
    let timer = unsafe { Timer2::steal() };

    critical_section::with(|cs| {
        let status = timer.intf().read().bits();
        let enabled = timer.dmainten().read().bits();

        EmbassyTimerDriver::clear_interrupt_flags(&timer, status);

        if (status & INTF_UPIF) != 0 {
            DRIVER.on_overflow(cs);
        }

        if (status & INTF_CH0IF) != 0 && (enabled & DMAINTEN_CH0IE) != 0 {
            DRIVER.trigger_alarm(cs);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::{classify_alarm_arm, AlarmArmDecision};

    #[test]
    fn alarm_arming_past_timestamp_fires_now() {
        assert_eq!(classify_alarm_arm(100, 99), AlarmArmDecision::FireNow);
        assert_eq!(classify_alarm_arm(100, 100), AlarmArmDecision::FireNow);
    }

    #[test]
    fn alarm_arming_near_future_arms_compare() {
        assert_eq!(classify_alarm_arm(100, 101), AlarmArmDecision::ArmCompare);
        assert_eq!(
            classify_alarm_arm(100, 100 + u16::MAX as u64),
            AlarmArmDecision::ArmCompare
        );
    }

    #[test]
    fn alarm_arming_far_future_is_deferred() {
        assert_eq!(
            classify_alarm_arm(100, 101 + u16::MAX as u64),
            AlarmArmDecision::Defer
        );
    }
}
