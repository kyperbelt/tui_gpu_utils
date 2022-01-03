# tui_gpu_utils
A terminal ui for monitoring nvidia gpu clock speeds and temps using nvidia-smi. 

![DisplayDemo](https://raw.githubusercontent.com/kyperbelt/tui_gpu_utils/main/nvidiagpu_cli_tool2.gif)

## Requirements
- `tty` terminal emulator.
- `nvidia-smi` (from nvidia proprietery drivers).

## Installation

### Build it Yourself
- You will need 
  - `git`
  - [latest version](https://www.rust-lang.org/tools/install) of `rustup`.

*Linux*
```bash
git clone https://github.com/kyperbelt/tui_gpu_utils.git
cd tui_gpu_utils
cargo build --release
sudo mv target/release/gpu_utils /usr/local/bin/gpu_utils
```

then from the terminal just run `gpu_utils` and enjoy :)

