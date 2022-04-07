
///////////// filename: ws2812.rs ///////////////////////////////

use embedded_time::{
    fixed_point::FixedPoint,};
    
use rp2040_hal::{
    gpio::bank0::Gpio5,
    gpio::{Function, Pin, PinId},
    pio::{PIOBuilder, ShiftDirection, Tx, UninitStateMachine, PIO, SM0},
    pac::PIO0,};

    pub fn init(
        _pin: Pin<Gpio5, Function<PIO0>>,
        pio: &mut PIO<PIO0>,
        sm: UninitStateMachine<(PIO0, SM0)>,
        clock_freq: embedded_time::rate::Hertz,
    ) -> Tx<(PIO0, SM0)> {
        const CYCLES_PER_BIT: u32 = (10) as u32; // 10 is total pio statement cycle
        const FREQ: u32 = 800_000;     
        let program = pio_proc::pio!(32, "
; https://github.com/raspberrypi/pico-examples/blob/master/pio/ws2812/ws2812.pio
.side_set 1
.wrap_target
bitloop:
   out x, 1       side 0 [2]
   jmp !x do_zero side 1 [1]
do_one:
   jmp bitloop    side 1 [4]
do_zero:
   nop            side 0 [4]
.wrap 
");

    let div = clock_freq.integer() as f32 / (FREQ as f32 * CYCLES_PER_BIT as f32);
    let installed = pio.install(&program.program).unwrap();     
    let (mut sm, _, tx) = PIOBuilder::from_program(installed)
       .side_set_pin_base(Gpio5::DYN.num)     
       .autopull(true)
       .pull_threshold(24)
       .out_shift_direction(ShiftDirection::Right) // default is left
       .clock_divisor(div)
       .build(sm);           
    sm.set_pindirs([(Gpio5::DYN.num, rp2040_hal::pio::PinDir::Output)]);
    sm.start();
    tx}
