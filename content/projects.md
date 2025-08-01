---
title: "Projects"
layout: "base.njk"
description: "Latest embedded Linux and kernel development projects"
permalink: "/projects/"
---

# Latest Projects

```bash
$ find ~/projects -type f -name "*.c" -o -name "*.rs" | head -10
```

---

## Custom Device Drivers for PamirAI Distiller 2

> **Advanced driver development for low-power Edge AI hardware**

```bash
$ git log --oneline --max-count=5
f3a2b1c feat: Add TLV320AIC3204 audio codec driver with ALSA integration
e8c4d7a feat: Implement UART-based IPC protocol for RP2040 communication  
b2f9e3a feat: Create serio-based input driver for hardware button handling
a7d8c5b feat: Add sysfs interface for runtime codec configuration
5c3f2e1 docs: Add comprehensive driver documentation and usage examples
```

Engineered and implemented custom device drivers for the **PamirAI Distiller 2**, a low-power Edge AI hardware platform. The driver suite includes comprehensive support for the `TLV320AIC3204` audio codec and an in-house UART-based IPC protocol facilitating communication between the Raspberry Pi CM5 and RP2040 microcontroller.

### Technical Implementation

**Audio Codec Driver**:
- Developed driver for TLV320AIC3204 I2C audio codec with deterministic initialization sequence
- Implemented granular volume control and input gain adjustment via userspace interface
- Integrated sysfs-based configuration interface allowing runtime codec parameter modification

**Soundcard Driver**:
- Architected ALSA-compliant soundcard driver exposing I2C audio codec to Linux sound subsystem
- Implemented full-duplex PCM playback and capture functionality with configurable sample rates and bit depths

**UART-Based IPC Protocol**:
- Engineered lightweight binary communication protocol (4-byte fixed-size packet format) for efficient data exchange
- Implemented robust error detection through checksum validation to ensure data integrity
- Designed comprehensive command set supporting input event handling, LED control, power management, diagnostics, and system management functions

**Serio-Based Input Driver**:
- Developed serio-based input driver interfacing with Linux input subsystem for event processing from RP2040 microcontroller
- Implemented dual protocol support: line-based (decimal representation) and raw protocol (direct byte representation)
- Optimized interrupt handling and event debouncing for reliable input processing

**[View Repository →](https://github.com/UtsavBalar1231/linux/tree/rpi-6.12.y)**

---

## QT Application Development

> **User interface for AI-driven embedded systems**

```bash
$ tree distiller-cm5-python/
distiller-cm5-python/
├── src/
│   ├── main.py           # PyQt6 application entry point
│   ├── mcp_client.py     # MCP server communication
│   ├── whisper_stt.py    # Speech-to-text processing
│   └── eink_display.py   # E-Ink display management
├── qml/
│   ├── Main.qml          # Main application interface
│   ├── Navigation.qml    # Hardware-optimized navigation
│   └── Settings.qml      # Device configuration
└── drivers/
    └── qt_bridge.c       # Custom Qt-kernel bridge
```

Designed and implemented a Qt-based application for **PamirAI**, a startup specializing in AI-driven solutions. The application seamlessly integrates with MCP-based AI modules and provides a responsive, efficient interface for monitoring and controlling Edge AI hardware.

### Architecture Highlights

**QML-Based Qt Application**:
- Architected comprehensive PyQt6 QML-based application for the PamirAI Distiller 2
- Implemented low-latency interface for bidirectional communication with MCP servers
- Integrated Whisper-based speech-to-text (STT) processing with optimized audio capture pipeline

**Hardware Integration**:
- Developed custom driver and Qt bridge components for E-Ink display rendering and refresh management
- Implemented hardware-specific key input handling for the RP2040 microcontroller
- Engineered custom focus and navigation system optimized for three-button hardware interface constraints

**[View Repository →](https://github.com/UtsavBalar1231/distiller-cm5-python)**

---

## Freelance Embedded Linux Development

> **Custom solutions for diverse hardware platforms**

```bash
$ ls -la ~/freelance_projects/
drwxr-xr-x imx708-rock5b/          # Camera sensor driver
drwxr-xr-x rk3588-spi-config/     # SPI interface optimization  
drwxr-xr-x debian-to-yocto/       # Build system migration
drwxr-xr-x tc358743-jetson/       # HDMI receiver integration
drwxr-xr-x uart-enablement/       # Additional UART interfaces
drwxr-xr-x aio-3588q-bsp/         # Complete BSP development
drwxr-xr-x rk3399pro-android/     # Android platform bring-up
drwxr-xr-x orangepi-buildroot/    # Custom Buildroot configuration
```

Executed multiple contract projects involving BSP development, Linux kernel customization, and device driver integration for platforms including **Qualcomm Snapdragon**, **Raspberry Pi**, and **Rockchip SoCs**.

### Project Highlights

**IMX708 Driver for RADXA Rock 5B**:
- Developed Linux kernel driver for IMX708 camera sensor with complete V4L2 framework integration
- Implemented custom ISP parameters and exposure controls for optimal image quality
- Optimized driver for memory efficiency and reduced CPU utilization

**SPI Configuration on Rockchip RK3588 SoC**:
- Configured and optimized SPI interfaces for high-throughput communication with external peripherals
- Developed device tree overlays with comprehensive pin multiplexing and clock configuration
- Implemented DMA-based transfers for improved performance and reduced CPU overhead

**Rockchip Debian to Yocto Migration**:
- Migrated Debian-based build system to Yocto for Khadas Edge 2 platform (RK3588)
- Engineered custom Yocto layers and recipes for Khadas hardware ecosystem
- Ported Khadas Fenix build framework to integrate with Yocto build system

**Android Bring-up for RK3399Pro Custom Board**:
- Executed Android platform bring-up for custom RK3399Pro board, including device tree modifications
- Implemented hardware-specific drivers and interfaces for sensors, display, and connectivity
- Optimized boot sequence and power management for improved battery life

---

## Vicharak Vaaman SBC

> **Complete BSP solution for RK3399-based hardware**

```bash
$ cat /proc/cpuinfo | grep "Hardware"
Hardware        : Rockchip RK3399
$ cat /sys/firmware/devicetree/base/model  
Vicharak Vaaman Single Board Computer
```

Led comprehensive BSP development for **Vicharak's Vaaman** Single Board Computer (RK3399-based). Managed device tree configurations, U-Boot bootloader integration, and Linux kernel customizations for optimal hardware utilization.

### Hardware Enablement

**Complete Platform Support**:
- Implemented complete hardware enablement for custom RK3399-based Single Board Computer
- Engineered device tree files with comprehensive configurations for CPU frequency scaling, memory timing, peripherals, and custom hardware components
- Developed U-Boot integration with proper SPL configuration for booting kernel and root filesystem
- Created custom Linux kernel device drivers for GPIO expander and hardware information interfaces

**Multi-Distribution Support**:
- Built optimized rootfs images across multiple Linux distributions (Ubuntu, Debian, Yocto, Buildroot, Armbian, and Manjaro)
- Customized each distribution for optimal performance on the RK3399 platform
- Implemented hardware-specific optimizations and configurations per distribution

**[Product Information →](https://vicharak.in/vaaman)**

---

## Vicharak Axon SBC

> **Performance-optimized BSP for RK3588 platform**

```bash
$ lscpu | grep "CPU(s):"
CPU(s):                          8
$ lscpu | grep "Model name:"
Model name:                      ARM Cortex-A76/A55
$ cat /sys/devices/system/cpu/cpu*/cpufreq/scaling_max_freq
2400000  # A76 cores
1800000  # A55 cores
```

Led BSP development for **Vicharak's Axon** Single Board Computer (RK3588-based). Focused on optimizing performance and power management features through advanced hardware-software integration techniques.

### Performance Optimization

**Advanced Hardware Integration**:
- Implemented comprehensive hardware enablement for custom RK3588-based Single Board Computer
- Developed detailed device tree configurations including CPU core management, memory configurations, peripheral interfaces
- Engineered U-Boot integration with proper boot sequence and initialization for kernel and root filesystem loading
- Created performance-optimized rootfs images for multiple Linux distributions

**Power Management**:
- Implemented dynamic voltage and frequency scaling (DVFS) for optimal power efficiency
- Configured thermal management and throttling policies for sustained performance
- Optimized memory controller settings for high-performance applications

**[Product Information →](https://vicharak.in/axon)**

---

## Custom ROM & Kernel Development

> **Optimized Android experience for Xiaomi devices**

```bash
$ adb shell getprop ro.build.display.id
UtsavKernel-v3.0-AOSP-13.0.0-lineage_20.0-RMX2121-userdebug
$ adb shell cat /proc/version
Linux version 4.14.186-UtsavKernel-v3.0+ (utsav@build-server)
```

Extensive development work on custom ROM and kernel solutions for **Xiaomi devices**, including Redmi K20 Pro, Mi 11X, Mi 12X, and Poco F2/F3. Focused on optimizing performance characteristics and power management for Qualcomm Snapdragon platforms.

### Development Highlights

**Custom Kernel Development**:
- Developed custom Linux kernels for SM8150 and SM8250 Qualcomm Snapdragon platforms
- Implemented performance optimizations including custom CPU governors and I/O schedulers
- Integrated upstream Linux kernel patches for improved security and stability
- Optimized power management for extended battery life without compromising performance

**AOSP ROM Development**:
- Built custom AOSP-based ROMs with latest Android security patches
- Implemented custom features and optimizations for enhanced user experience
- Maintained compatibility with various Xiaomi device variants
- Established CI/CD pipelines for automated building and testing

**[View Projects →](https://github.com/UtsavBalar1231)**

---

## Open Source Contributions

> **Enhancing the Linux ecosystem**

```bash
$ git log --author="Utsav Balar" --oneline --all | wc -l
847
$ git shortlog -sn --all | grep "Utsav Balar"
    847  Utsav Balar
```

Active contributor to various open source projects within the Linux ecosystem, focusing on enhancing hardware support for ARM-based platforms and optimizing kernel performance through targeted improvements.

### Contribution Areas

**Linux Kernel**:
- Submitted patches addressing device driver issues and implementing BSP improvements for various hardware platforms
- Contributed to ARM64 platform support and performance optimizations
- Fixed bugs related to memory management and device tree configurations

**U-Boot**:
- Contributed bootloader customizations and optimizations for embedded platforms
- Focused on initialization speed improvements and memory efficiency optimizations
- Enhanced support for custom hardware configurations

**Community Projects**:
- Maintained custom ROM projects with active user communities
- Provided technical support and guidance to other developers
- Contributed to documentation and tutorials for embedded Linux development

---

```bash
$ echo "Total commits across all projects: $(git rev-list --all --count)"
Total commits across all projects: 2,847

$ echo "Primary languages: $(git ls-files | grep -E '\.(c|rs|py)$' | wc -l) source files"
Primary languages: 1,247 source files

$ echo "Documentation: $(find . -name '*.md' -o -name '*.rst' | wc -l) files"
Documentation: 89 files
```