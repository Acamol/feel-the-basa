FeelTheBasa
---

FeelTheBasa is a tiny bit calculator Windows app for debugging that allows converting between 32-bit decimal (signed/non-signed), hexadecimal, binary, text, IP address, and Linux IOCTL opcode.
Inspired by FeelTheBase (albeit missing some features for now).

![FeelTheBasa Screenshot](https://user-images.githubusercontent.com/40899785/137196645-9e436ae1-fb43-4618-bdf1-157ed0a84ebb.png)


Run
---
You can either download FeelTheBasa from the [releases page](https://github.com/Acamol/feel-the-basa/releases) or build it yourself:
```
git clone https://github.com/Acamol/feel-the-basa.git FeelTheBasa
cd FeelTheBasa
cargo build --release
./target/release/feel_the_basa.exe
```

TODO
---
* Support 64-bit and 128-bit data.
* Use IPv6 address when 128-bit data is used.
* Support 3-bit IOCTL access mode.
