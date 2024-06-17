#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::Input;
use embassy_stm32::gpio::Level;
use embassy_stm32::gpio::Output;
use embassy_stm32::gpio::Pull;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut button = ExtiInput::new(Input::new(p.PC13, Pull::Down), p.EXTI13);
    let mut led = Output::new(p.PB13, Level::Low, embassy_stm32::gpio::Speed::Low);
    info!("Press the blue button to turn the LED on and off");
    loop {
        button.wait_for_rising_edge().await;
        led.set_high();
        info!("Pressed!");
        button.wait_for_falling_edge().await;
        led.set_low();
        info!("Released!");
    }
}
