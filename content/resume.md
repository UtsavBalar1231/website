---
title: "Resume"
layout: "base.njk"
description: "Professional resume - Embedded Linux & BSP Engineer"
permalink: "/resume/"
---

# Resume

```bash
$ cat ~/.profile/resume.txt
```

## Personal Information

```bash
$ whoami && pwd && date
utsav_balar
/home/utsav_balar/career
Thu Aug  1 12:00:00 IST 2025
```

**Name**: Utsav Balar  
**Location**: Surat, Gujarat, IN 395006  
**Phone**: (+91) 7600-529-280  
**Email**: [utsavbalar1231@gmail.com](mailto:utsavbalar1231@gmail.com)  
**LinkedIn**: [linkedin.com/in/utsavbalar](https://www.linkedin.com/in/utsavbalar)

---

## Professional Summary

```bash
$ grep -r "summary" /etc/profile.d/career.sh
```

Embedded Linux and BSP Engineer pursuing Master's in Computer Science and Engineering, with hands-on experience in developing embedded systems. Proficient in BSP engineering, Linux kernel development, and hardware enablement, while staying up to date with the latest technologies and developments in the Computer Science field.

---

## Work Experience

### Freelance Linux Kernel Developer
**Self, Surat, Gujarat, IN**  
*Jan 2024 - Present*

```bash
$ ps aux | grep -E "(bsp|kernel|driver)" | head -5
- BSP development for multiple platforms (RK3399PRO, RK3588, RK356X, BCM2712)
- Linux kernel customization and device driver integration  
- Board bring-up and peripheral integration
- Custom kernel optimizations for embedded systems
- LLM benchmarking and testing on low-powered Edge hardware
```

- Working on multiple freelance projects involving BSP development, Linux kernel development, device driver development, board bring-up, peripheral integration, and custom kernel optimizations
- Supporting various hardware platforms, including Qualcomm Snapdragon, Raspberry Pi, and Rockchip SoCs
- Integrating custom features, performing driver bring-up, and configuring hardware for various single-board computers

### Linux Kernel and BSP Engineer
**Vicharak, Surat, Gujarat, IN**  
*Dec 2022 - Dec 2023*

```bash
$ ls -la ~/vicharak_projects/
drwxr-xr-x vaaman-rk3399/     # BSP development for RK3399 board
drwxr-xr-x axon-rk3588/       # BSP development for RK3588 board  
drwxr-xr-x multi-distro/      # Support for multiple Linux distributions
drwxr-xr-x windows-arm/       # Windows on ARM investigation
```

- Led BSP development for multiple Rockchip ARM-based boards (RK3399, RK3588), including device tree configuration, U-Boot bootloader integration, and Linux kernel customization
- Developed system images for various Linux distributions (Armbian, Buildroot, Debian, Manjaro, Ubuntu, Yocto) tailored to specific hardware configurations
- Investigated UEFI bootloader functionality, analyzing the boot process and embedded display controller integration
- Assisted with initial Windows on ARM bring-up for Rockchip RK3588, focusing on UEFI and ACPI integration

### Junior Linux Kernel Intern
**Vicharak, Surat, Gujarat, IN**  
*May 2022 - Jul 2022*

```bash
$ git log --oneline --since="2022-05-01" --until="2022-07-31"
a3f7b2c feat: Linux kernel bring-up for custom RK3399 ARM board
e8d4c1a feat: Device tree configuration and peripheral initialization
b5f9e7d feat: Encryption module for secure boot using crypto API
c2a8f4e feat: Userspace authenticator for hardware authentication
```

- Performed Linux kernel bring-up for a custom ARM-based board using Rockchip's RK3399 processor, focusing on device tree configuration and peripheral initialization
- Developed an encryption module for secure boot and hardware authentication using the Linux kernel crypto API and a userspace authenticator, enhancing system security

### Hobby Projects
**Self, Surat, Gujarat, IN**  
*Jan 2019 - Dec 2023*

```bash
$ find ~/hobby_projects -name "*.git" | wc -l
37

$ du -sh ~/hobby_projects/custom_roms/
2.8G    ~/hobby_projects/custom_roms/

$ git log --all --author="Utsav" --oneline | wc -l
2847
```

- Developed a custom Linux kernel for Xiaomi's SM8150 and SM8250 devices, optimizing performance, power management, and integrating custom features
- Built a custom Linux kernel for Samsung's Exynos 9611 with AOSP support, improving device performance and stability
- Created various custom AOSP ROMs with the latest AOSP and Qualcomm patches, integrating custom features and optimizations for enhanced performance and user experience
- Implemented numerous CI/CD pipelines for multiple projects, automating build and testing processes to ensure code quality and reliability

---

## Education

### Master's in Computer Science and Engineering
**National Institute of Technology, Meghalaya, IN**  
*Expected Jun 2025*

```bash
$ cat /etc/os-release | grep -E "(NAME|VERSION)"
PRETTY_NAME="NIT Meghalaya - Computer Science & Engineering"
VERSION="Master's Program (2023-2025)"
```

### Bachelor's in Computer Engineering
**UKA Tarsadia University, Gujarat, IN**  
*2019 - 2023*

```bash
$ ls -la ~/academic_projects/bachelor/
drwxr-xr-x privacy-android-rom/    # De-Googled Android ROM with custom kernel
drwxr-xr-x university-mgmt-sys/    # PHP and MySQL management system
drwxr-xr-x web-auth-plugin/        # JavaScript browser plugin for authentication
```

**Key Projects**:
- Developed a privacy-focused, de-Googled Android ROM with a custom kernel
- Created a university management system using PHP and MySQL, focusing on user authentication and data management
- Assisted in developing a JavaScript-based web plugin for Chrome and Firefox browsers for graphical user authentication

---

## Technical Skills

### Proficient
```rust
let proficient_skills = vec![
    "C", "Rust", "Bash", "AWK",
    "Embedded Linux", "Linux Kernel Development", 
    "BSP Engineering", "Device Drivers",
    "Git", "AOSP"
];
```

### Semi-Proficient
```bash
SEMI_PROFICIENT=(
    "Yocto/Buildroot Project"
    "U-Boot" 
    "ARM Architecture"
)
```

### Familiar With
```c
char *familiar_technologies[] = {
    "Python", "C++", "HTML/CSS", "PHP", 
    "MySQL", "MongoDB", "React", "Node.js", 
    "Figma", "QT QML"
};
```

### Development Tools
```yaml
Tools:
  Compilers: [GCC/Clang Cross-Compiler, GDB]
  Virtualization: [QEMU, Git, Docker]
  CI/CD: [Travis CI, Drone CI, Jenkins]
  Hardware: [Logic Analyzers, Oscilloscopes, JTAG Debuggers]
```

### Soft Skills
```bash
$ echo $SOFT_SKILLS
Problem-Solving, Attention to Detail, Collaboration, Technical Communication
```

---

## Notable Achievements

```bash
$ cat ~/.achievements/professional.log
```

- **Open Source Contributions**: 2800+ commits across various Linux kernel and embedded projects
- **Community Impact**: Custom ROM projects with 10K+ active users across multiple device variants
- **Technical Leadership**: Led BSP development for two commercial single-board computers
- **Industry Recognition**: Collaborated with hardware startups on cutting-edge AI embedded systems
- **Academic Excellence**: Pursuing Master's degree while maintaining active freelance career

---

## Volunteer Work

### Tech Workshops and Lectures
**Vicharak, Surat, Gujarat, IN**  
*Dec 2022 - Dec 2023*

```bash
$ ls -la ~/workshops/
drwxr-xr-x cpu-architecture/      # CPU architecture fundamentals
drwxr-xr-x compiler-internals/    # Compiler design and implementation  
drwxr-xr-x git-version-control/   # Advanced Git workflows
drwxr-xr-x kernel-debugging/      # Linux kernel debugging techniques
```

- Conducted workshops and lectures on CPU architecture, compilers, and Git version control systems
- Mentored junior developers in embedded systems and Linux kernel development
- Organized technical sessions on device driver development and BSP engineering

---

## Languages

```bash
$ locale -a | grep -E "(en|hi|gu)"
en_US.UTF-8    # English (Fluent)
hi_IN.UTF-8    # Hindi (Fluent)  
gu_IN.UTF-8    # Gujarati (Fluent)
```

---

## Interests & Hobbies

### Technical Interests
```bash
$ cat ~/.config/interests/technical.conf
[interests]
primary=["Open-Source Development", "Low-Level Engineering", "Embedded Systems"]
secondary=["Linux Kernel Development", "Computer Architecture", "Compilers"]
emerging=["Artificial Intelligence", "Biology", "Space and Universe"]
```

### Personal Hobbies
```bash
$ ps aux | grep -E "(music|travel|sports)"
- Traveling and exploring new technologies
- Listening to Rock/Indie/EDM/Techno Music  
- Watching recreational programming content
- Playing Cricket and Table Tennis
- Reading technical literature and research papers
```

---

## Contact Information

```bash
$ cat ~/.social_links
[GitHub](https://github.com/UtsavBalar1231)
[LinkedIn](https://linkedin.com/in/utsavbalar)  
[Twitter](https://twitter.com/UtsavTheCunt)
[Telegram](https://t.me/UtsavTheCunt)
Discord: utsav@1231
[Reddit](https://reddit.com/user/UtsavTheCunt)
```

---

```bash
$ echo "Resume last updated: $(date)"
Resume last updated: Thu Aug  1 12:00:00 IST 2025

$ echo "Status: Available for embedded Linux projects and collaborations"
Status: Available for embedded Linux projects and collaborations
```