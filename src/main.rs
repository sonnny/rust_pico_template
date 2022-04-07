// rust demo of ws2812 using pio on pico using function instead of struct on a separate file
/////////////// filename: main.rs /////////////////////////////

#![no_std]
#![no_main]

mod ws2812;

use cortex_m_rt::entry;
use embedded_time::rate::*;
use panic_halt as _;
use rp_pico::hal::prelude::*;
use rp_pico::hal::pac;
use rp_pico::hal;

const WHITE: u32 = 0x808080; // lower brightness to save current
const RED:   u32 = 0x008000;
const BLUE:  u32 = 0x800000;
const GREEN: u32 = 0x000080;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,).ok().unwrap();
    let sio = hal::Sio::new(pac.SIO);
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,);
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);    
    let mut tx = ws2812::init(  // example of calling function that returns tx on pio
        pins.gpio5.into_mode(), 
        &mut pio, 
        sm0,
        clocks.peripheral_clock.freq(),);
    
    loop {
       tx.write(RED);
       delay.delay_ms(1000);
       tx.write(GREEN);
       delay.delay_ms(1000);
       tx.write(BLUE);
       delay.delay_ms(1000);
       tx.write(WHITE);
       delay.delay_ms(1000);}}
