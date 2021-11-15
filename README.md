# Rust Blinky for the STM32 F4 via DFU (no STLink!)
I wrote this in the process of learning how to use the HiLetgo STM32 F4VE
development board, which has the STM32F407VET6 hooked up to a USB port. This
combination makes it easy to program the board by booting into DFU mode and
transferring the ELF file over USB, without needing an STLink dongle. I 
decided to implement good ol' blinky to demonstrate how.

## Prerequisites
- [STM32CubeProgrammer](https://www.st.com/en/development-tools/stm32cubeprog.html)
installed and on your `PATH`
- Rust with the `thumbv7em-none-eabihf` target installed (`rustup target add thumbv7em-none-eabihf`)
  - At the time of creating this repo, I had to use `+nightly` features
  - The DFU programming method works with binaries produced by any language, not just
  Rust
- `cargo install cargo-make` (optional)
  - I tested this process on Windows, so you may need to tweak the command in 
  [`Makefile.toml`](./Makefile.toml) to fit your dev environment


## Programming
1. Plug the USB cable into the development board
   - You may need to use [Zadig](https://zadig.akeo.ie/) to install the 
     libusb driver to get your computer to recognize the board
3. Connect the `BT0` pin to `3V3` and reset the board (tap the `RST`
   button)
   - `BT0` is adjacent to `3V3` on the dev board, so you can just use a 
     jumper or a dupont wire
   - You might need to tap `RST` a couple times before DFU mode will start; it can
     be a little finicky
   - [STM AN2606](https://www.st.com/resource/en/application_note/cd00167594-stm32-microcontroller-system-memory-boot-mode-stmicroelectronics.pdf)
     (pp. 119-129) shows that the STM32F407 will enter DFU mode when Boot0
     is high, Boot1 is low, and there is a USB cable detected (but no USART or
     CAN connection)
3. Run `cargo make flash`
   - You can also open the STM32CubeProgrammer GUI and download your ELF there
   (e.g., if you want to use the DFU method with a binary you produced in some
   other programming language)
5. Disconnect `BT0` from `3V3` and tap `RST` again to reboot into User mode
6. Marvel at the blinking lights

## Boot mode selector jig
If you plan to program the board a lot, you might find it worthwhile to
quickly slap together a simple boot mode selector jig so you aren't constantly
fumbling with taking a jumper on and off and pressing a tiny reset button.

I took a ~50x50mm section of snap-apart protoboard and wired up 4 dupont cables 
to connect to the dev board: `+3V3`, `GND`, `BT0`, and `RST`. There is an 
easy-to-access `RST` pin on the bottom right of the `OLD-TFT` header, on the edge
of the board next to the `K0` button. 

The jig has two nice big push buttons, one for `BT0` (which is normally pulled
low by a 20k resistor) and the other for `RST` (which is normally pulled high by 
a 20k resistor). When I want to enter the system boot mode for DFU, I hold down
the `BT0` button (connecting it to `3V3`) and tap `RST` (connecting it to `GND`).
You don't have to hold `BT0` the whole time - the little fanfare that plays on
Windows when a new USB device is connected will let you know you can release `BT0`.

![image](https://user-images.githubusercontent.com/89292/141735295-442d5c44-02bf-4f22-a772-0e768db576b5.png)

