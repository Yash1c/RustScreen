# RustScreen

**RustScreen**: Your old laptop, reborn as a monitor.  
This lightweight Rust-based OS is designed to turn any old machine into a dedicated screen-sharing monitor.  

> ⚠️ **Still in development.** Use at your own risk. Features are experimental and evolving.

## Purpose

The goal of RustScreen is to provide a simple, lightweight system for screen monitoring and sharing, ideal for repurposing old hardware into a dedicated display device.  

## Getting Started

1. Clone this repository:

```bash
git clone https://github.com/Yash1c/RustScreen.git
cd RustScreen
````

2. Build the bootable ISO (if not already provided):

```bash
# Update your system firstly
sudo apt update

# Make sure grub-mkrescue and xorriso are installed
sudo apt install xorriso grub-mkrescue
grub-mkrescue -o RustScreen.iso iso-root/
```

3. Run RustScreen in a virtual machine (recommended for testing):

```bash
# Using QEMU for quick testing
qemu-system-x86_64 -m 512 -cdrom RustScreen.iso -boot d -serial stdio
```

* `-m 512` → allocate 512 MB RAM
* `-serial stdio` → redirect console output to terminal

This allows you to test the ISO without writing it to a physical device.

## Notes

* The ISO is experimental and may not boot on all hardware.
* For real hardware deployment, create a bootable USB using tools like `dd` or `Rufus`.
* Contributions and feedback are welcome.

