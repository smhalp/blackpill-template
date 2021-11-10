# blackpill-template
A fork of: 
```https://github.com/mendelt/bluepill-template```

Cargo generate template for Rust embedded projects based on the stm32f411 black pill board.
Includes a handy openocd.cfg to allow you to use your raspberry pi as a debugger


## Usage
Install cargo-generate if you don't already have it;
```cargo install cargo-generate```

Generate your project;
```cargo generate --git https://github.com/smhalp/blackpill-template```

Configure your project debugger
Edit .cargo/config, set runner to the executable you use as your debugger.

for example:
```runner = "arm-none-eabi-gdb -q -x debug.gdb"```
or
```runner = "gdb-multiarch -q -x debug.gdb"```

Start openocd in a separate terminal window by running `openocd` from the root or your project.
This will make sure it picks up the openocd.cfg file.

Start the project:
```cargo run```
