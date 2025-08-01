---
title: "Character Device Drivers Fundamentals"
description: "Master the essentials of character device drivers in the Linux kernel with practical examples for Raspberry Pi 5. Learn about device numbers, file operations, and udev integration."
date: 2025-05-24
author: "Utsav Balar"
difficulty: "beginner"
topics: ["kernel", "device-drivers", "raspberry-pi", "character-devices", "file-operations"]
series: "Linux Kernel Device Driver"
part: 2
environment:
  hardware: "Raspberry Pi 5"
  kernel: "6.12"
  os: "Raspberry Pi OS (64-bit)"
prerequisites: ["Basic C programming knowledge", "Linux command line familiarity", "Introduction to Linux Kernel Modules (Tutorial 1)"]
github: "https://github.com/UtsavBalar1231/utsavbalar-linux-kernel-tutorials/tree/master/tutorial-02"
featured: true
draft: false
---


# Character Device Drivers Fundamentals

## What Are Character Devices?

`character devices are one of the three primary device types in Linux (along with block and network devices). They provide a byte-stream interface similar to files, allowing applications to read and write data sequentially. Character devices are ideal for:

- Hardware that streams data (serial ports, sensors)
- Devices without a filesystem (GPIO pins, LEDs)
- Custom interfaces between userspace and kernel space

  Unlike block devices (hard drives, SSDs), character devices don't have a buffer cache and typically process I/O operations directly as they're received.

## Understanding Device Numbers

Each character device in Linux is identified by a unique combination of major and minor numbers:

- **Major Number**: Identifies the driver associated with the device
- **Minor Number**: Distinguishes between different devices controlled by the same driver

Traditionally, major numbers were statically assigned to specific device types, but modern kernels support dynamic allocation for most drivers.

```bash
# View character devices on your system and their major/minor numbers
ls -la /dev/tty*
ls -la /dev/null

# Use the mknod command to manually create a device node (requires root)
sudo mknod /dev/mydev c 42 0  # Creates a character device with major 42, minor 0
```

## The `file_operations` Structure

At the core of every character device driver is the `file_operations` structure. This structure defines the operations that can be performed on the device:

```c
struct file_operations {
    struct module *owner;
    loff_t (*llseek) (struct file *, loff_t, int);
    ssize_t (*read) (struct file *, char __user *, size_t, loff_t *);
    ssize_t (*write) (struct file *, const char __user *, size_t, loff_t *);
    long (*unlocked_ioctl) (struct file *, unsigned int, unsigned long);
    int (*open) (struct inode *, struct file *);
    int (*release) (struct inode *, struct file *);
    /* ... and more ... */
};
```

The most common operations you'll implement are:

- **open**: Initialize the device when it's first accessed
- **release**: Clean up when the last file descriptor is closed
- **read**: Copy data from the device to userspace
- **write**: Copy data from userspace to the device
- **llseek**: Change the current file position
- **unlocked_ioctl**: Implement device-specific commands

## Creating a Simple Character Device Driver

Let's implement a simple character device driver that acts as a memory buffer:

### Step 1: Device Structure

```c
#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/init.h>
#include <linux/fs.h>         /* For register_chrdev, file_operations */
#include <linux/uaccess.h>    /* For copy_to_user, copy_from_user */
#include <linux/device.h>     /* For device_create, class_create */
#include <linux/cdev.h>       /* For cdev_init, cdev_add */

#define DEVICE_NAME "simple_char"
#define CLASS_NAME "simple"
#define BUFFER_SIZE 1024

/* Module metadata */
MODULE_LICENSE("GPL");
MODULE_AUTHOR("Utsav Balar");
MODULE_DESCRIPTION("A simple character device driver example");
MODULE_VERSION("0.1");

/* Global variables for our device */
static int major_number;              /* Will store our device's major number */
static char device_buffer[BUFFER_SIZE]; /* Memory buffer for the device */
static int buffer_pointer = 0;        /* Current position in buffer */
static struct class *simple_class = NULL;  /* Device class */
static struct device *simple_device = NULL; /* Device */
static struct cdev simple_cdev;        /* Character device structure */

/* Prototypes for device functions */
static int simple_open(struct inode *, struct file *);
static int simple_release(struct inode *, struct file *);
static ssize_t simple_read(struct file *, char __user *, size_t, loff_t *);
static ssize_t simple_write(struct file *, const char __user *, size_t, loff_t *);
static loff_t simple_llseek(struct file *, loff_t, int);
```

### Step 2: File Operations Implementation

```c
/* Define file operations for our device */
static struct file_operations simple_fops = {
    .owner = THIS_MODULE,
    .open = simple_open,
    .release = simple_release,
    .read = simple_read,
    .write = simple_write,
    .llseek = simple_llseek,
};

/* Called when device is opened */
static int simple_open(struct inode *inode, struct file *file)
{
    /* Nothing special to do here */
    printk(KERN_INFO "SIMPLE: Device opened\n");
    return 0;
}

/* Called when device is closed */
static int simple_release(struct inode *inode, struct file *file)
{
    printk(KERN_INFO "SIMPLE: Device closed\n");
    return 0;
}

/* Called when user reads from the device */
static ssize_t simple_read(struct file *file, char __user *user_buffer, 
                          size_t count, loff_t *offset)
{
    int bytes_to_read;
    int bytes_not_copied;
    
    /* Calculate bytes to read */
    bytes_to_read = min((size_t)(BUFFER_SIZE - *offset), count);
    
    if (bytes_to_read <= 0)
        return 0; /* EOF */
    
    /* Copy data to user space */
    bytes_not_copied = copy_to_user(user_buffer, device_buffer + *offset, bytes_to_read);
    
    /* Update file position */
    *offset += (bytes_to_read - bytes_not_copied);
    
    printk(KERN_INFO "SIMPLE: Read %d bytes\n", bytes_to_read - bytes_not_copied);
    
    /* Return number of bytes successfully read */
    return (bytes_to_read - bytes_not_copied);
}

/* Called when user writes to the device */
static ssize_t simple_write(struct file *file, const char __user *user_buffer, 
                           size_t count, loff_t *offset)
{
    int bytes_to_write;
    int bytes_not_copied;
    
    /* Calculate bytes to write */
    bytes_to_write = min((size_t)(BUFFER_SIZE - *offset), count);
    
    if (bytes_to_write <= 0)
        return -ENOSPC; /* No space left on device */
    
    /* Copy data from user space */
    bytes_not_copied = copy_from_user(device_buffer + *offset, user_buffer, bytes_to_write);
    
    /* Update file position */
    *offset += (bytes_to_write - bytes_not_copied);
    
    printk(KERN_INFO "SIMPLE: Wrote %d bytes\n", bytes_to_write - bytes_not_copied);
    
    /* Update buffer_pointer to end of data if needed */
    if (*offset > buffer_pointer)
        buffer_pointer = *offset;
    
    /* Return number of bytes successfully written */
    return (bytes_to_write - bytes_not_copied);
}

/* Called when user changes file position with lseek */
static loff_t simple_llseek(struct file *file, loff_t offset, int whence)
{
    loff_t new_pos = 0;
    
    switch(whence) {
        case SEEK_SET: /* Set from start of file */
            new_pos = offset;
            break;
        case SEEK_CUR: /* Set from current position */
            new_pos = file->f_pos + offset;
            break;
        case SEEK_END: /* Set from end of file (buffer_pointer is our EOF) */
            new_pos = buffer_pointer + offset;
            break;
        default:
            return -EINVAL;
    }
    
    /* Check if position is valid */
    if (new_pos < 0 || new_pos > BUFFER_SIZE)
        return -EINVAL;
    
    /* Update file position */
    file->f_pos = new_pos;
    return new_pos;
}
```

### Step 3: Module Initialization and Cleanup

```c
/* Module initialization function */
static int __init simple_init(void)
{
    /* Register a range of character device numbers */
    major_number = register_chrdev(0, DEVICE_NAME, &simple_fops);
    if (major_number < 0) {
        printk(KERN_ALERT "SIMPLE: Failed to register a major number\n");
        return major_number;
    }
    printk(KERN_INFO "SIMPLE: Registered with major number %d\n", major_number);
    
    /* Register device class */
    simple_class = class_create(THIS_MODULE, CLASS_NAME);
    if (IS_ERR(simple_class)) {
        unregister_chrdev(major_number, DEVICE_NAME);
        printk(KERN_ALERT "SIMPLE: Failed to register device class\n");
        return PTR_ERR(simple_class);
    }
    printk(KERN_INFO "SIMPLE: Device class registered\n");
    
    /* Create device node */
    simple_device = device_create(simple_class, NULL, MKDEV(major_number, 0), 
                                  NULL, DEVICE_NAME);
    if (IS_ERR(simple_device)) {
        class_destroy(simple_class);
        unregister_chrdev(major_number, DEVICE_NAME);
        printk(KERN_ALERT "SIMPLE: Failed to create the device\n");
        return PTR_ERR(simple_device);
    }
    printk(KERN_INFO "SIMPLE: Device created successfully\n");
    
    /* Initialize cdev structure and add it to kernel */
    cdev_init(&simple_cdev, &simple_fops);
    simple_cdev.owner = THIS_MODULE;
    
    if (cdev_add(&simple_cdev, MKDEV(major_number, 0), 1) < 0) {
        device_destroy(simple_class, MKDEV(major_number, 0));
        class_destroy(simple_class);
        unregister_chrdev(major_number, DEVICE_NAME);
        printk(KERN_ALERT "SIMPLE: Failed to add cdev\n");
        return -1;
    }
    
    /* Initialize the device buffer */
    memset(device_buffer, 0, BUFFER_SIZE);
    
    printk(KERN_INFO "SIMPLE: Device initialized\n");
    return 0;
}

/* Module cleanup function */
static void __exit simple_exit(void)
{
    /* Remove the cdev */
    cdev_del(&simple_cdev);
    
    /* Remove the device */
    device_destroy(simple_class, MKDEV(major_number, 0));
    
    /* Unregister the device class */
    class_destroy(simple_class);
    
    /* Unregister the major number */
    unregister_chrdev(major_number, DEVICE_NAME);
    
    printk(KERN_INFO "SIMPLE: Device unregistered\n");
}

/* Register module functions */
module_init(simple_init);
module_exit(simple_exit);
```

### Step 4: Create Makefile

```makefile
# If KERNELRELEASE is defined, we've been invoked from the kernel build system
ifneq ($(KERNELRELEASE),)
    obj-m := simple_char.o

# Otherwise, we're being called directly from the command line
else
    # Path to the kernel headers
    KERNEL_DIR ?= /lib/modules/$(shell uname -r)/build
    PWD := $(shell pwd)

# Default target
all:
	$(MAKE) -C $(KERNEL_DIR) M=$(PWD) modules

# Clean target
`Clean:
	$(MAKE) -C $(KERNEL_DIR) M=$(PWD) clean

endif
```

## Understanding Device Registration

Our driver registers a character device by:

1. **Getting a major number**: Using `register_chrdev` or `alloc_chrdev_region` for dynamic allocation
2. **Creating a device class**: With `class_create` to represent our device type in sysfs
3. **Creating a device node**: With `device_create` to create `/dev/simple_char`
4. **Initializing the character device**: Using `cdev_init` and `cdev_add`

This approach ensures proper integration with the kernel's device model and udev.

  Always check return values from registration functions and correctly clean up resources on failure. Memory leaks in kernel space can't be reclaimed until system reboot.

## Dynamic vs. Static Device Numbers

Modern drivers should use dynamic device number allocation:

```c
/* Dynamically allocate a device number */
dev_t dev = 0;
int result = alloc_chrdev_region(&dev, 0, 1, DEVICE_NAME);
if (result < 0) {
    printk(KERN_ALERT "Failed to allocate device number\n");
    return result;
}

/* Extract major and minor numbers */
int major = MAJOR(dev);
int minor = MINOR(dev);
```

This approach allows multiple drivers to coexist without hardcoded major number conflicts.

## File Operations Context

When implementing file operations, it's important to understand:

1. **struct file**: Represents an open file instance
2. **struct inode**: Represents the file on disk
3. **User space pointers**: Must be accessed using `copy_to_user`/`copy_from_user`
4. **Protection against concurrent access**: Use synchronization mechanisms when needed

## Handling User Space Memory

The kernel and user space operate in different memory domains, so you must use special functions to transfer data:

```c
/* NEVER do this - direct access is unsafe and may crash the kernel */
// wrong_way = *user_ptr;  // DON'T DO THIS!

/* Instead, use copy_to_user/copy_from_user */
unsigned long not_copied;

/* Reading from user space */
not_copied = copy_from_user(kernel_buffer, user_buffer, count);
if (not_copied > 0) {
    /* Handle partial copy */
    pr_warn("Could not copy %lu bytes from user space\n", not_copied);
}

/* Writing to user space */
not_copied = copy_to_user(user_buffer, kernel_buffer, count);
if (not_copied > 0) {
    /* Handle partial copy */
    pr_warn("Could not copy %lu bytes to user space\n", not_copied);
}
```

These functions safely handle:
- Page faults during access
- Security checks
- Memory that might not be present
- Architecture-specific memory barriers

## Using udev for Automatic Device Creation

Modern Linux systems use udev for dynamic device management. When our driver creates a device with `device_create()`, udev automatically:

1. Creates appropriate device nodes in `/dev`
2. Sets permissions and ownership
3. Runs scripts to handle device addition/removal events

You can create custom udev rules to:
- Set specific permissions for your device
- Create symbolic links with friendly names
- Run scripts when your device is connected/disconnected

```bash
# Example udev rule for our simple character device
# This sets permissions to allow non-root users in the "gpio" group to access it
KERNEL=="simple_char", SUBSYSTEM=="simple", MODE="0660", GROUP="gpio"
```

## Testing the Driver on Raspberry Pi 5

Let's build and test our character device driver on Raspberry Pi 5:

```bash
# Compile the driver
make

# Load the driver
sudo insmod simple_char.ko

# Check that the device was created
ls -la /dev/simple_char

# Check device messages in kernel log
dmesg | tail

# Test writing to the device
echo "Hello from userspace" | sudo tee /dev/simple_char

# Test reading from the device
sudo cat /dev/simple_char

# Unload the driver
sudo rmmod simple_char
```

## Writing a User Space Test Program

Here's a simple test program to interact with our device:

```c
#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>
#include <string.h>

#define DEVICE_PATH "/dev/simple_char"
#define BUFFER_SIZE 1024

int main() {
    int fd;
    char write_buf[BUFFER_SIZE] = "Testing character device driver";
    char read_buf[BUFFER_SIZE];
    int bytes_written, bytes_read;
    
    /* Open the device */
    fd = open(DEVICE_PATH, O_RDWR);
    if (fd < 0) {
        perror("Failed to open device");
        return EXIT_FAILURE;
    }
    
    printf("Device opened successfully.\n");
    
    /* Write to the device */
    bytes_written = write(fd, write_buf, strlen(write_buf));
    if (bytes_written < 0) {
        perror("Failed to write to device");
        close(fd);
        return EXIT_FAILURE;
    }
    
    printf("Wrote %d bytes to device: %s\n", bytes_written, write_buf);
    
    /* Reset file position to beginning of file */
    lseek(fd, 0, SEEK_SET);
    
    /* Read from the device */
    bytes_read = read(fd, read_buf, BUFFER_SIZE);
    if (bytes_read < 0) {
        perror("Failed to read from device");
        close(fd);
        return EXIT_FAILURE;
    }
    
    read_buf[bytes_read] = '\0'; /* Null terminate the string */
    printf("Read %d bytes from device: %s\n", bytes_read, read_buf);
    
    /* Close the device */
    close(fd);
    printf("Device closed.\n");
    
    return EXIT_SUCCESS;
}
```

`compile and run the test program:

```bash
# Compile the test program
gcc -o test_device test_device.c

# Run the test program
sudo ./test_device
```

## ARM64 Considerations for Raspberry Pi 5

When working with character device drivers on the Raspberry Pi 5's ARM64 architecture, keep these considerations in mind:

1. **Memory Alignment**: ARM64 is strict about memory alignment, so ensure buffers are properly aligned
2. **Endianness**: ARM64 is little-endian, but some peripheral devices might use different endianness
3. **Cache Coherency**: For memory-mapped I/O, use the appropriate barriers and non-cached memory accesses
4. **64-bit Pointers**: Ensure your driver correctly handles 64-bit pointers and addresses

  The BCM2712 SoC in Raspberry Pi 5 has specific memory management requirements. For memory-mapped I/O or DMA operations, consult the BCM2712 datasheet and Raspberry Pi hardware documentation.

## Raspberry Pi-Specific Device Permissions

By default, most device nodes on Raspberry Pi OS require root access. To make your devices accessible to non-root users:

1. **udev rules**: Create rules in `/etc/udev/rules.d/` as shown earlier
2. **User groups**: Add users to appropriate groups like "gpio", "spi", "i2c", etc.
3. **Device attributes**: Set device attributes in sysfs during driver initialization

```bash
# Add current user to gpio group
sudo usermod -a -G gpio $USER

# Create the gpio group if it doesn't exist
sudo groupadd -f gpio

# Reload udev rules
sudo udevadm control --reload-rules
sudo udevadm trigger
```

## Multiple Devices with the Same Driver

In real-world applications, you'll often want one driver to manage multiple physical or virtual devices. This is accomplished using minor numbers:

```c
#define NUM_DEVICES 4

static dev_t dev_number;        /* Will store first device number */
static struct device *devices[NUM_DEVICES]; /* Array of device structures */

/* In init function */
/* Allocate range of device numbers for NUM_DEVICES */
result = alloc_chrdev_region(&dev_number, 0, NUM_DEVICES, DEVICE_NAME);

/* Create multiple device nodes */
for (i = 0; i < NUM_DEVICES; i++) {
    /* Create device with incrementing minor number */
    devices[i] = device_create(device_class, NULL, 
                              MKDEV(MAJOR(dev_number), MINOR(dev_number) + i),
                              NULL, "%s%d", DEVICE_NAME, i);
}

/* In file operations, use iminor() to determine which device */
static int device_open(struct inode *inode, struct file *file) {
    int minor = iminor(inode);  /* Get minor number */
    /* Now we know which device instance was opened */
    printk(KERN_INFO "Device %d opened\n", minor);
    /* ... */
}
```

This creates devices like `/dev/simple_char0`, `/dev/simple_char1`, etc.

## Character Device Ioctls

For operations beyond simple read/write, use the ioctl (I/O control) interface:

```c
#include <linux/ioctl.h>

/* Define ioctl commands */
#define SIMPLE_RESET     _IO('s', 1)   /* Reset device buffer */
#define SIMPLE_GET_SIZE  _IOR('s', 2, int)  /* Get buffer size */
#define SIMPLE_SET_MODE  _IOW('s', 3, int)  /* Set device mode */

/* Implement the ioctl handler */
static long simple_ioctl(struct file *file, unsigned int cmd, unsigned long arg)
{
    int mode, ret = 0;
    
    switch (cmd) {
        case SIMPLE_RESET:
            /* Reset the device buffer */
            memset(device_buffer, 0, BUFFER_SIZE);
            buffer_pointer = 0;
            break;
            
        case SIMPLE_GET_SIZE:
            /* Return the buffer size to user */
            ret = put_user(BUFFER_SIZE, (int __user *)arg);
            break;
            
        case SIMPLE_SET_MODE:
            /* Get mode from user */
            ret = get_user(mode, (int __user *)arg);
            if (ret == 0) {
                /* Handle mode setting */
                printk(KERN_INFO "SIMPLE: Mode set to %d\n", mode);
            }
            break;
            
        default:
            return -ENOTTY; /* Unknown command */
    }
    
    return ret;
}

/* Add to file_operations structure */
static struct file_operations simple_fops = {
    .owner = THIS_MODULE,
    .open = simple_open,
    .release = simple_release,
    .read = simple_read,
    .write = simple_write,
    .llseek = simple_llseek,
    .unlocked_ioctl = simple_ioctl,  /* Add ioctl handler */
};
```

In user space, use ioctl() to call these functions:

```c
#include <sys/ioctl.h>
#include <fcntl.h>
#include <stdio.h>

/* Same ioctl definitions as in the kernel module */
#define SIMPLE_RESET     _IO('s', 1)
#define SIMPLE_GET_SIZE  _IOR('s', 2, int)
#define SIMPLE_SET_MODE  _IOW('s', 3, int)

int main() {
    int fd = open("/dev/simple_char", O_RDWR);
    int size, mode = 1;
    
    /* Reset the device */
    ioctl(fd, SIMPLE_RESET);
    
    /* Get the buffer size */
    ioctl(fd, SIMPLE_GET_SIZE, &size);
    printf("Buffer size: %d\n", size);
    
    /* Set the device mode */
    ioctl(fd, SIMPLE_SET_MODE, &mode);
    
    close(fd);
    return 0;
}
```

## Best Practices for Character Device Drivers

1. **Error Handling**: Always check return values and properly clean up on errors
2. **Resource Management**: Release all resources in reverse order of acquisition
3. **Concurrency**: Use appropriate locking mechanisms when multiple users access the device
4. **Permissions**: Set appropriate device permissions for security
5. **Documentation**: Document your driver interface for user space developers
6. **Use the Kernel's API**: Leverage existing kernel subsystems when possible
7. **Defensive Programming**: Validate all inputs from user space

## Summary

In this tutorial, you've learned:

1. **Character Device Fundamentals**: How character devices work in Linux
2. **Device Registration**: How to register devices with the kernel
3. **File Operations**: How to implement read, write, seek, and other operations
4. **User Space Interface**: Safe communication between kernel and user space
5. **udev Integration**: Automatic device creation and permission management
6. **Testing**: Building and testing drivers on Raspberry Pi 5
7. **ARM64 Considerations**: Platform-specific issues to consider

With these foundations, you're now equipped to develop character device drivers for various hardware interfaces on the Raspberry Pi 5.

## Next Steps

In the next tutorial, we'll explore kernel memory management—a critical skill for device driver developers. We'll discuss different allocation methods, DMA memory, and user-kernel memory sharing techniques specific to the Raspberry Pi 5 platform.

## References

1. [Linux Device Drivers, 3rd Edition](https://lwn.net/Kernel/LDD3/)
2. [Linux Kernel Documentation: The Linux Kernel API](https://www.kernel.org/doc/html/latest/core-api/index.html)
3. [Character Device Drivers in the Linux Kernel](https://www.kernel.org/doc/html/latest/driver-api/driver-model/index.html)
4. [Raspberry Pi Hardware Documentation](https://www.raspberrypi.com/documentation/computers/processors.html)
5. [ARM64 Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest)

---

*Last updated: May 31, 2025* 
