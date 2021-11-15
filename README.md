# Rust Blinky for the STM32 F4 via DFU
I wrote this in the process of learning how to use the "HiLetgo STM32 F4VE"
development board, which has the STM32F407VET6 hooked up to a USB port. This
combination makes it easy to program the board by booting into DFU mode and
transferring the ELF file over USB, so I decided to implement good ol' blinky
to demonstrate how.

## Prerequisites
- Rust with the `thumbv7em-none-eabihf` target installed (`rustup target add thumbv7em-none-eabihf`)
  - At the time of creating this repo, I had to use `+nightly` features
- [STM32CubeProgrammer](https://www.st.com/en/development-tools/stm32cubeprog.html)
installed and on your `PATH`
  - I tested this process on Windows, so you may need to tweak the command in 
  [`Makefile.toml`](./Makefile.toml) to fit your dev environment.

## Programming
1. Plug the USB cable into the development board
2. Connect the `BT0` pin to `3V3` and reset the board (tap the `RST`
   button)
   - `BT0` is adjacent to `3V3` on the dev board, so you can just use on a 
     jumper or a dupont wire.
   - [STM AN2606](https://www.st.com/resource/en/application_note/cd00167594-stm32-microcontroller-system-memory-boot-mode-stmicroelectronics.pdf)
     (pp. 119-129) shows that the STM32F407 will enter DFU mode when Boot0
     is high, Boot1 is low, and there is a USB cable detected (but no USART or
     CAN connection).
   - You might need to tap `RST` a few times before DFU mode will start; it can
     be a little finicky.
3. Run `cargo make flash`
4. Disconnect `BT0` from `3V3` and tap `RST` again to reboot into User mode
5. Marvel at the blinking lights

## Boot mode selector jig
If you plan to program the board a lot, you might find it worthwhile to
quickly slap together a simple boot mode selector jig so you aren't constantly
fumbling with taking a jumper on and off and pressing a tiny reset button.

I took a ~50x50mm section of snap-apart protoboard and wired up 4 dupont cables 
to connect to the dev board: `+3V3`, `GND`, `BT0`, and `RST`. There is an easy-
to-access `RST` pin on the bottom right of the `OLD-TFT` header, on the edge
of the board next to the `K0` button. 

The jig has two nice big push buttons, one for `BT0` (which is normally pulled
low by a 20k resistor) and the other for `RST` (which is normally pulled high by 
a 20k resistor). When I want to enter the system boot mode for DFU, I hold down 
the `BT0` button and tap `RST`. You don't have to hold `BT0` the whole time - 
the little fanfare that plays on Windows will let you know you can release `BT0`.
