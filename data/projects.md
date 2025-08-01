---
title: Projects
description: Showcase of my latest embedded Linux and kernel development projects
updated: 2025-05-19
---

# Latest Projects

---

## Custom Device Drivers for PamirAI Distiller 2

> *Advanced driver development for low-power Edge AI hardware*

Engineered and implemented custom device drivers for the **PamirAI Distiller 2**, a low-power Edge AI hardware platform. The driver suite includes comprehensive support for the `TLV320AIC3204` audio codec and an in-house UART-based IPC protocol facilitating communication between the Raspberry Pi CM5 and RP2040 microcontroller.

<summary><strong>Technical Details</strong></summary>

1. **Audio Codec Driver**: 
	- Developed a driver for the TLV320AIC3204 I2C audio codec with deterministic initialization sequence
	- Implemented granular volume control and input gain adjustment via userspace interface
	- Integrated sysfs-based configuration interface allowing runtime codec parameter modification

2. **Soundcard Driver**: 
	- Architected an ALSA-compliant soundcard driver exposing the I2C audio codec to the Linux sound subsystem
	- Implemented full-duplex PCM playback and capture functionality with configurable sample rates and bit depths

3. **UART-Based IPC Protocol**:
	- Engineered a lightweight binary communication protocol (4-byte fixed-size packet format) for efficient data exchange between the Linux host processor and RP2040 microcontroller
	- Implemented robust error detection through checksum validation to ensure data integrity
	- Designed a comprehensive command set supporting input event handling, LED control, power management, diagnostics, and system management functions

4. **Serio-Based Input Driver**:
	- Developed a serio-based input driver interfacing with the Linux input subsystem for event processing from the RP2040 microcontroller
	- Implemented dual protocol support: line-based (decimal representation of button states) and raw protocol (direct byte representation)
	- Optimized interrupt handling and event debouncing for reliable input processing

**[View GitHub Repository](https://github.com/UtsavBalar1231/linux/tree/rpi-6.12.y)**

---

## QT Application Development

> *User interface for AI-driven embedded systems*

Designed and implemented a Qt-based application for **PamirAI**, a startup specializing in AI-driven solutions. The application seamlessly integrates with MCP-based AI modules and provides a responsive, efficient interface for monitoring and controlling Edge AI hardware.

<summary><strong>Technical Details</strong></summary>

1. **QML-Based Qt Application**: 
	- Architected a comprehensive PyQt6 QML-based application for the PamirAI Distiller 2
	- Implemented a low-latency interface for bidirectional communication with MCP servers
	- Integrated Whisper-based speech-to-text (STT) processing with optimized audio capture pipeline
	- Developed custom driver and Qt bridge components for E-Ink display rendering and refresh management
	- Implemented hardware-specific key input handling for the RP2040 microcontroller
	- Engineered custom focus and navigation system optimized for three-button hardware interface constraints

**[View GitHub Repository](https://github.com/UtsavBalar1231/distiller-cm5-python)**

---

## Freelance Embedded Linux Development

> *Custom solutions for diverse hardware platforms*

Executed multiple contract projects involving BSP development, Linux kernel customization, and device driver integration for platforms including **Qualcomm Snapdragon**, **Raspberry Pi**, and **Rockchip SoCs**.

<summary><strong>Technical Details</strong></summary>

1. **IMX708 Driver for RADXA Rock 5B**: 
	- Developed a Linux kernel driver for the IMX708 camera sensor with complete V4L2 framework integration
	- Implemented custom ISP parameters and exposure controls for optimal image quality
	- Optimized driver for memory efficiency and reduced CPU utilization

2. **SPI Configuration on Rockchip RK3588 SoC**: 
	- Configured and optimized SPI interfaces for high-throughput communication with external peripherals
	- Developed device tree overlays with comprehensive pin multiplexing and clock configuration
	- Implemented DMA-based transfers for improved performance and reduced CPU overhead

3. **Rockchip Debian to Yocto Migration**:
	- Migrated the Debian-based build system to Yocto for the Khadas Edge 2 platform (RK3588)
	- Engineered custom Yocto layers and recipes for the Khadas hardware ecosystem
	- Ported the Khadas Fenix build framework to integrate with the Yocto build system
	- Implemented device tree modifications and U-Boot integration for bootloader functionality

4. **TC358743 Integration for NVIDIA Jetson**:
	- Integrated the TC358743 HDMI receiver kernel module into the NVIDIA Jetson Orin Nano platform
	- Developed custom device tree modifications for HDMI input support with proper clock and pin configurations
	- Implemented V4L2 capture interface for video stream processing

5. **UART Enablement for Rock 5B with Android Kernel**:
	- Enabled and configured additional UART interfaces on the RADXA Rock 5B platform using the Android kernel
	- Implemented proper pin multiplexing and clock configurations in the device tree
	- Developed userspace interface for UART configuration and testing

6. **BSP Development for AIO-3588Q Board**: 
	- Developed a comprehensive BSP for the AIO-3588Q board, including device tree configurations, bootloader integration, and kernel customizations
	- Implemented proper memory mapping, clock tree configuration, and power management
	- Optimized boot time and system performance through kernel parameter tuning

7. **Android Bring-up for RK3399Pro Custom Board**:
	- Executed Android platform bring-up for a custom RK3399Pro board, including device tree modifications, kernel configurations, and AOSP integration
	- Implemented hardware-specific drivers and interfaces for sensors, display, and connectivity
	- Optimized boot sequence and power management for improved battery life

8. **Buildroot for OrangePi One**:
	- Developed a customized Buildroot configuration for the OrangePi One platform
	- Implemented kernel and root filesystem optimizations for reduced footprint and improved performance
	- Created custom package selection and configuration for specific application requirements

---

## Vicharak Vaaman SBC

> *Complete BSP solution for RK3399-based hardware*

Led comprehensive BSP development for **Vicharak's Vaaman** Single Board Computer (RK3399-based). Managed device tree configurations, U-Boot bootloader integration, and Linux kernel customizations for optimal hardware utilization.

<summary><strong>Technical Details</strong></summary>

1. **Hardware Enablement**: 
	- Implemented complete hardware enablement for the custom RK3399-based Single Board Computer
	- Engineered device tree files with comprehensive configurations for CPU frequency scaling, memory timing, peripherals, and custom hardware components
	- Developed U-Boot integration with proper SPL configuration for booting the kernel and root filesystem
	- Created custom Linux kernel device drivers for GPIO expander and hardware information interfaces
	- Built optimized rootfs images across multiple Linux distributions (Ubuntu, Debian, Yocto, Buildroot, Armbian, and Manjaro)

**[View Product](https://vicharak.in/vaaman)**

---

## Vicharak Axon SBC

> *Performance-optimized BSP for RK3588 platform*

Led BSP development for **Vicharak's Axon** Single Board Computer (RK3588-based). Focused on optimizing performance and power management features through advanced hardware-software integration techniques.

<summary><strong>Technical Details</strong></summary>

1. **Hardware Enablement**: 
	- Implemented comprehensive hardware enablement for the custom RK3588-based Single Board Computer
	- Developed detailed device tree configurations including CPU core management, memory configurations, peripheral interfaces, and custom hardware components
	- Engineered U-Boot integration with proper boot sequence and initialization for kernel and root filesystem loading
	- Created performance-optimized rootfs images for multiple Linux distributions (Ubuntu, Debian, Yocto, Buildroot, Armbian)

**[View Product](https://vicharak.in/axon)**

---

## Custom ROM & Kernel Development

> *Optimized Android experience for Xiaomi devices*

Extensive development work on custom ROM and kernel solutions for **Xiaomi devices**, including Redmi K20 Pro, Mi 11X, Mi 12X, and Poco F2/F3. Focused on optimizing performance characteristics and power management for Qualcomm Snapdragon platforms through custom kernel configurations and scheduler tuning.

**[View GitHub Projects](https://github.com/UtsavBalar1231)**

---

## Open Source Contributions

> *Enhancing the Linux ecosystem*

Active contributor to various open source projects within the Linux ecosystem, focusing on enhancing hardware support for ARM-based platforms and optimizing kernel performance through targeted improvements.

<summary><strong>Contribution Areas</strong></summary>

### Linux Kernel
Submitted patches addressing device driver issues and implementing BSP improvements for various hardware platforms.

### U-Boot
Contributed bootloader customizations and optimizations for embedded platforms, focusing on initialization speed and memory efficiency. 

---
