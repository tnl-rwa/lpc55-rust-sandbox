#![no_main]
#![no_std]

extern crate panic_semihosting;
use lpc55_hal as hal;

#[rtic::app(device = crate::hal::raw, peripherals = true, dispatchers = [USB0, USB1])]
mod app {
    use cortex_m::asm::wfi;
    use systick_monotonic::*;
    // extern crate panic_halt;
    use core::fmt::Write;
    use cortex_m_semihosting::dbg;
    use embedded_hal::digital::v2::PinState;
    use hal::{
        drivers::pins::Level,
        drivers::{pins, I2cMaster, Pins},
        prelude::*,
        time::{Hertz, Megahertz},
        typestates::pin,
    };
    use lpc55_hal as hal;
    use ssd1306;
    use ssd1306::prelude::*;

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = Systick<10>; // 10 Hz / 100 ms granularity

    type RedLed = hal::Pin<pins::Pio1_6, pin::state::Gpio<pin::gpio::direction::Output>>;
    type GreenLed = hal::Pin<pins::Pio1_7, pin::state::Gpio<pin::gpio::direction::Output>>;
    type BlueLed = hal::Pin<pins::Pio1_4, pin::state::Gpio<pin::gpio::direction::Output>>;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led_idx: i8,
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
        let mut anactrl = hal::Anactrl::from(dp.ANACTRL);
        let mut pmc = hal::Pmc::from(dp.PMC);

        let clocks = hal::ClockRequirements::default()
            .system_frequency(Megahertz(50))
            // .support_flexcomm()
            .configure(&mut anactrl, &mut pmc, &mut syscon)
            .unwrap();

        cortex_m_semihosting::hprintln!("clocks = {:?}", &clocks).ok();

        let token = clocks.support_flexcomm_token().unwrap();

        let i2c = hal::peripherals::flexcomm::Flexcomm4::from((
            dp.FLEXCOMM4,
            dp.I2C4,
            dp.I2S4,
            dp.SPI4,
            dp.USART4,
        ))
        .enabled_as_i2c(&mut syscon, &token);

        let pins = Pins::take().unwrap();
        let scl = pins.pio1_20.into_i2c4_scl_pin(&mut iocon);
        let sda = pins.pio1_21.into_i2c4_sda_pin(&mut iocon);

        // let i2c = I2cMaster::new(i2c, (scl, sda), 400.khz());
        let i2c = I2cMaster::new(i2c, (scl, sda), Hertz(1_000_000));

        // How can I get the display in a task?
        let mut display: TerminalMode<_> = ssd1306::Builder::new()
            .size(DisplaySize::Display128x64)
            .with_i2c_addr(0x3c)
            .connect_i2c(i2c)
            .into();

        display.init().unwrap();
        display.clear().unwrap();
        display
            .write_str(unsafe { core::str::from_utf8_unchecked(b"hello") })
            .unwrap();

        let red_led = pins
            .pio1_6
            .into_gpio_pin(&mut iocon, &mut gpio)
            .into_output(Level::High);
        let green_led = pins
            .pio1_7
            .into_gpio_pin(&mut iocon, &mut gpio)
            .into_output(Level::High);
        let blue_led = pins
            .pio1_4
            .into_gpio_pin(&mut iocon, &mut gpio)
            .into_output(Level::High);

        let systick = cp.SYST;

        // Initialize the monotonic (SysTick rate in QEMU is 12 MHz)
        let mono = Systick::new(systick, 50_000_000);

        switch_led::spawn().unwrap();

        (
            Shared {},
            Local {
                led_idx: 0,
                red_led,
                green_led,
                blue_led,
            },
            init::Monotonics(mono),
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

        dbg!("switch_led",);
        match (*led_idx / 2) as i8 {
            // I would like to call toggle_led(cx.local.red_led) and 
            // toggle_led(cx.local.green_led) etc. How implement that function? 
            0 => cx
                .local
                .red_led
                .set_state(toggle_state(cx.local.red_led.is_set_high().unwrap()))
                .unwrap(),
            1 => cx
                .local
                .green_led
                .set_state(toggle_state(cx.local.green_led.is_set_high().unwrap()))
                .unwrap(),
            2 => cx
                .local
                .blue_led
                .set_state(toggle_state(cx.local.blue_led.is_set_high().unwrap()))
                .unwrap(),
            _ => {
                dbg!("FAAL!");
            }
        };

        *led_idx = (*led_idx + 1) % 6;

        switch_led::spawn_after(1.secs()).unwrap();
    }

    // #[idle(resources = [led, delay, sleep])]
    #[idle()]
    fn idle(_c: idle::Context) -> ! {
        loop {
            wfi();
        }
    }
}
