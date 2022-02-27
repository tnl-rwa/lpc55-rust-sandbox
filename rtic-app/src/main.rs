#![no_main]
#![no_std]

extern crate panic_semihosting;
use lpc55_hal as hal;


#[rtic::app(device = crate::hal::raw, peripherals = true, dispatchers = [USB0, USB1])]
mod app {
    use cortex_m::asm::wfi;
    use systick_monotonic::*;
    // extern crate panic_halt;
    use cortex_m_semihosting::dbg;
    use embedded_hal::digital::v2::PinState;
    use lpc55_hal as hal;
    use hal::{
        prelude::*,
        drivers::pins::Level,
        drivers::pins,
        typestates::pin,
    };

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = Systick<10>; // 10 Hz / 100 ms granularity

    type RedLed = hal::Pin<pins::Pio1_6, pin::state::Gpio<pin::gpio::direction::Output>>;
    type GreenLed = hal::Pin<pins::Pio1_7, pin::state::Gpio<pin::gpio::direction::Output>>;
    type BlueLed = hal::Pin<pins::Pio1_4, pin::state::Gpio<pin::gpio::direction::Output>>;

    
    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led_idx:i8,
        red_led: RedLed,
        green_led: GreenLed,
        blue_led: BlueLed,
    }
    
    #[init]
    fn init(c: init::Context) -> (Shared, Local, init::Monotonics) {
        // dbg!("init");
        let cp = c.core;
        let dp = c.device;

        // setup red LED
        let mut syscon = hal::Syscon::from(dp.SYSCON);
        let mut gpio = hal::Gpio::from(dp.GPIO).enabled(&mut syscon);
        let mut iocon = hal::Iocon::from(dp.IOCON).enabled(&mut syscon);

        let pins = hal::Pins::take().unwrap();
        let red_led = pins.pio1_6
            .into_gpio_pin(&mut iocon, &mut gpio)
            .into_output(Level::High);
        let green_led = pins.pio1_7
            .into_gpio_pin(&mut iocon, &mut gpio)
            .into_output(Level::High);
        let blue_led = pins.pio1_4
            .into_gpio_pin(&mut iocon, &mut gpio)
            .into_output(Level::High);

        let systick = cp.SYST;

        // Initialize the monotonic (SysTick rate in QEMU is 12 MHz)
        let mono = Systick::new(systick, 50_000_000);
    
    

        switch_led::spawn().unwrap();

        (
            Shared{},    
            Local {
                led_idx:0,
                red_led,
                green_led,
                blue_led
            },
            init::Monotonics(mono)
        )

    }

    fn toggle_state(is_high: bool) -> PinState {
        if is_high {
            PinState::Low
        } else {
            PinState::High
        }
    }

    #[task(local = [led_idx, red_led, green_led, blue_led])]
    fn switch_led(cx: switch_led::Context) {
        let led_idx = cx.local.led_idx;

        dbg!("switch_led", );
        match (*led_idx / 2) as i8 {
            0 => cx.local.red_led.set_state(toggle_state(cx.local.red_led.is_set_high().unwrap())).unwrap(),
            1 => cx.local.green_led.set_state(toggle_state(cx.local.green_led.is_set_high().unwrap())).unwrap(),
            2 => cx.local.blue_led.set_state(toggle_state(cx.local.blue_led.is_set_high().unwrap())).unwrap(),
            _ => {dbg!("FAAL!");}
        };
        

        *led_idx = (*led_idx + 1 ) % 6;

        switch_led::spawn_after(1.secs()).unwrap();
        
        // debug::exit(debug::EXIT_SUCCESS);

    }

    // #[idle(resources = [led, delay, sleep])]
    #[idle()]
    fn idle(_c: idle::Context) -> ! {
        loop {
            wfi();
        }
    }
}
