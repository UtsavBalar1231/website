---
title: "Kernel Memory Management for Drivers"
description: "Master memory allocation and management techniques in the Linux kernel with practical examples for Raspberry Pi 5. Learn about kmalloc, vmalloc, DMA memory, and common debugging techniques."
date: 2025-05-24
author: "Utsav Balar"
difficulty: "beginner"
topics: ["kernel", "device-drivers", "raspberry-pi", "memory-management", "dma"]
series: "Linux Kernel Device Driver"
part: 3
environment:
  hardware: "Raspberry Pi 5"
  kernel: "6.12"
  os: "Raspberry Pi OS (64-bit)"
prerequisites: ["Basic C programming knowledge", "Linux command line familiarity", "Introduction to Linux Kernel Modules (Tutorial 1)", "Character Device Drivers Fundamentals (Tutorial 2)"]
github: "https://github.com/UtsavBalar1231/utsavbalar-linux-kernel-tutorials/tree/master/tutorial-03"
featured: true
draft: false
---


# Kernel Memory Management for Drivers

## Introduction to Kernel Memory Management

Memory management is a critical aspect of kernel programming. Unlike userspace applications, the kernel:

1. Cannot easily extend its memory through mechanisms like swapping
2. Must manage physical memory directly
3. Has no protection against memory errors (which can crash the system)
4. Needs to handle different types of memory for different purposes

  Good memory management practices are essential for driver stability, performance, and security. Memory errors in kernel space can lead to system crashes, data corruption, or security vulnerabilities.

## Memory Allocation Fundamentals

The Linux kernel provides several memory allocation mechanisms, each suited for different scenarios:

- **kmalloc**: For small, physically contiguous memory blocks
- **vmalloc**: For larger, virtually contiguous memory blocks
- **get_free_pages**: For allocating memory in page-sized chunks
- **kmem_cache**: For allocating objects of the same size frequently
- **DMA functions**: For memory that needs to be accessed by devices

Let's dive into each of these mechanisms and understand when to use them.

## kmalloc - Basic Kernel Memory Allocation

`kmalloc()` is the most commonly used memory allocation function in kernel space, similar to `malloc()` in userspace:

```c
#include <linux/slab.h>    /* For kmalloc, kfree */

/* Basic kmalloc example */
void *memory_buffer;
int buffer_size = 1024;  /* 1 KB */

/* Allocate memory */
memory_buffer = kmalloc(buffer_size, GFP_KERNEL);
if (!memory_buffer) {
    pr_err("Failed to allocate memory\n");
    return -ENOMEM;
}

/* Use the memory */
memset(memory_buffer, 0, buffer_size);

/* Free memory when done */
kfree(memory_buffer);
```

### GFP Flags - Memory Allocation Control

The second parameter to `kmalloc()` specifies the "Get Free Page" flags, which control how memory is allocated:

```c
/* May sleep while allocating - most common for general use */
memory_buffer = kmalloc(buffer_size, GFP_KERNEL);

/* Must not sleep - use in interrupt handlers, critical sections */
atomic_buffer = kmalloc(buffer_size, GFP_ATOMIC);

/* For userspace-related allocations (e.g., for copy_to_user) */
user_buffer = kmalloc(buffer_size, GFP_USER);

/* Lower memory zone for DMA operations (32-bit devices) */
dma_buffer = kmalloc(buffer_size, GFP_DMA);

/* Higher memory priority - may cause other memory to be reclaimed */
high_priority = kmalloc(buffer_size, GFP_HIGHUSER);
```

  Using `GFP_KERNEL` in an interrupt context or while holding a spinlock will cause a system lockup! Always use `GFP_ATOMIC` in these contexts.

### kmalloc Limitations

`kmalloc()` has some important limitations to keep in mind:

1. It provides physically contiguous memory (important for hardware access)
2. It's limited to relatively small allocations (typically < 4MB)
3. It may sleep when used with `GFP_KERNEL` flag
4. It returns memory that's accessible via direct mapping in the kernel

  On ARM64 platforms like the Raspberry Pi 5, the kernel's virtual address space is much larger than on 32-bit systems, but there are still limitations on physically contiguous memory allocations.

## vmalloc - Virtually Contiguous Memory

When you need larger memory allocations that don't need to be physically contiguous, `vmalloc()` is the appropriate choice:

```c
#include <linux/vmalloc.h>    /* For vmalloc, vfree */

/* vmalloc example - allocating a larger buffer */
void *large_buffer;
size_t large_size = 8 * 1024 * 1024;  /* 8 MB */

/* Allocate memory */
large_buffer = vmalloc(large_size);
if (!large_buffer) {
    pr_err("Failed to allocate large buffer\n");
    return -ENOMEM;
}

/* Use the memory */
memset(large_buffer, 0, large_size);

/* Free memory when done */
vfree(large_buffer);
```

### When to Use vmalloc

Use `vmalloc()` in the following scenarios:

1. For large allocations (several megabytes or more)
2. When physical contiguity is not required
3. When memory will only be accessed from kernel space (not by hardware)
4. For temporary buffers during initialization or rare operations

  `vmalloc()` always allocates memory that can be accessed by the kernel, but it maps discontinuous physical pages into a contiguous range of kernel virtual addresses.

### vmalloc Overhead

`vmalloc()` has more overhead than `kmalloc()`:

1. It requires setting up page tables for the mapping
2. It uses more memory due to page table overhead
3. Access to vmalloc memory can be slightly slower due to TLB effects
4. Each allocation has at least one-page (4KB) overhead

## get_free_pages - Page-Level Allocations

For allocations that need to be page-aligned and in multiples of page size, you can use the page allocator directly:

```c
#include <linux/gfp.h>     /* For __get_free_pages */

/* Page allocation example */
unsigned long page_buffer;
int order = 2;  /* 2^2 = 4 pages = 16KB on systems with 4KB pages */

/* Allocate pages */
page_buffer = __get_free_pages(GFP_KERNEL, order);
if (!page_buffer) {
    pr_err("Failed to allocate pages\n");
    return -ENOMEM;
}

/* Use the memory (cast to needed type) */
memset((void *)page_buffer, 0, PAGE_SIZE << order);

/* Free pages when done */
free_pages(page_buffer, order);
```

### Understanding Order in Page Allocation

The `order` parameter in `__get_free_pages()` specifies the number of contiguous pages as a power of 2:

- order = 0: 2^0 = 1 page (typically 4KB)
- order = 1: 2^1 = 2 pages (typically 8KB)
- order = 2: 2^2 = 4 pages (typically 16KB)
- ...and so on

  Higher order allocations become exponentially harder to satisfy as free memory becomes fragmented. Orders above 3 or 4 may fail frequently on busy systems.

## kmem_cache - Object Cache Allocations

For drivers that need to allocate many objects of the same size frequently, `kmem_cache` provides an efficient solution:

```c
#include <linux/slab.h>    /* For kmem_cache_* functions */

/* Define a structure for our objects */
struct my_device_data {
    int id;
    char name[64];
    struct list_head list;
    /* ... other members ... */
};

/* Global variable for our cache */
static struct kmem_cache *my_device_cache;

/* Initialize the cache during module init */
static int __init my_init(void)
{
    /* Create cache with name "my_device_cache" for our objects */
    my_device_cache = kmem_cache_create("my_device_cache",
                                        sizeof(struct my_device_data),
                                        0, /* alignment */
                                        0, /* flags */
                                        NULL); /* constructor */
    if (!my_device_cache)
        return -ENOMEM;
        
    /* Other initialization... */
    return 0;
}

/* Allocate an object from the cache */
static struct my_device_data *allocate_device_data(void)
{
    struct my_device_data *data;
    
    data = kmem_cache_alloc(my_device_cache, GFP_KERNEL);
    if (!data)
        return NULL;
        
    /* Initialize the object */
    data->id = next_id++;
    memset(data->name, 0, sizeof(data->name));
    INIT_LIST_HEAD(&data->list);
    
    return data;
}

/* Free an object back to the cache */
static void free_device_data(struct my_device_data *data)
{
    if (data)
        kmem_cache_free(my_device_cache, data);
}

/* Clean up the cache during module exit */
static void __exit my_exit(void)
{
    /* Other cleanup... */
    
    /* Destroy cache when no longer needed */
    if (my_device_cache)
        kmem_cache_destroy(my_device_cache);
}

module_init(my_init);
module_exit(my_exit);
```

### Benefits of kmem_cache

Using a slab allocator provides several advantages:

1. Reduced memory fragmentation
2. Improved allocation and deallocation performance
3. Better memory locality for similarly-sized objects
4. Optional object initialization via constructor functions
5. Ability to track and debug allocations by cache name 

## DMA Memory Allocation

Device drivers often need to allocate memory for Direct Memory Access (DMA) operations, where hardware devices directly read from or write to system memory:

```c
#include <linux/dma-mapping.h>

/* DMA allocation example */
void *dma_buffer;
dma_addr_t dma_handle;
size_t dma_size = 4096;  /* 4 KB */
struct device *dev;      /* Your device structure */

/* Allocate coherent (uncached) DMA memory */
dma_buffer = dma_alloc_coherent(dev, dma_size, &dma_handle, GFP_KERNEL);
if (!dma_buffer) {
    dev_err(dev, "Failed to allocate DMA buffer\n");
    return -ENOMEM;
}

/* Now dma_buffer can be used from CPU side */
memset(dma_buffer, 0, dma_size);

/* dma_handle can be passed to the device for DMA operations */
/* Program your device registers with dma_handle... */

/* Free the DMA memory when done */
dma_free_coherent(dev, dma_size, dma_buffer, dma_handle);
```

### Understanding DMA Addresses

In DMA operations, we deal with two kinds of addresses:

1. **Virtual addresses** (like `dma_buffer`): Used by the CPU to access memory
2. **DMA/Physical addresses** (like `dma_handle`): Used by devices to access memory

  On some architectures like ARM64, the DMA address might not be the same as the physical address due to an IOMMU (IO Memory Management Unit). The DMA API abstracts these details.

### Coherent vs. Streaming DMA

The Linux kernel provides two main types of DMA mappings:

#### 1. Coherent DMA (Consistent DMA)

- Permanently mapped for the lifetime of the allocation
- Both CPU and device can access the memory simultaneously
- No need to flush caches or synchronize
- Used for control structures and data that's frequently accessed by both CPU and device
- Higher overhead, potentially slower access

```c
/* Already shown above */
dma_buffer = dma_alloc_coherent(dev, dma_size, &dma_handle, GFP_KERNEL);
/* ... use the buffer ... */
dma_free_coherent(dev, dma_size, dma_buffer, dma_handle);
```

#### 2. Streaming DMA

- Temporarily mapped for a specific DMA operation
- More efficient for one-time or infrequent transfers
- Requires explicit mapping/unmapping for each operation
- Better performance for large data transfers
- Must synchronize/flush caches explicitly

```c
/* Allocate regular memory first */
void *buffer = kmalloc(buffer_size, GFP_KERNEL);
if (!buffer)
    return -ENOMEM;

/* For DMA FROM device TO memory (read operation) */
dma_handle = dma_map_single(dev, buffer, buffer_size, DMA_FROM_DEVICE);
if (dma_mapping_error(dev, dma_handle)) {
    kfree(buffer);
    return -ENOMEM;
}

/* Program the device to start DMA using dma_handle */
start_device_dma_read(dev, dma_handle, buffer_size);

/* Wait for DMA completion (device specific) */
wait_for_dma_completion(dev);

/* Unmap after DMA operation is complete */
dma_unmap_single(dev, dma_handle, buffer_size, DMA_FROM_DEVICE);

/* Now CPU can safely access the data in buffer */
process_data(buffer, buffer_size);

/* Free the regular memory when done */
kfree(buffer);
```

### DMA Directions

When using streaming DMA, you specify the direction of the transfer:

- **DMA_TO_DEVICE**: CPU writes data, device reads it (e.g., sending data to a peripheral)
- **DMA_FROM_DEVICE**: Device writes data, CPU reads it (e.g., receiving data from a peripheral)
- **DMA_BIDIRECTIONAL**: Both device and CPU read and write the memory

  Using the wrong DMA direction can lead to data corruption due to cache coherency issues. Always use the correct direction flag!

### Raspberry Pi 5 DMA Considerations

The Raspberry Pi 5's BCM2712 SoC has specific DMA characteristics:

1. It has a dedicated DMA controller with multiple channels
2. The BCM2712 supports 36-bit physical addressing (64GB memory range)
3. There are DMA mask limitations for some peripherals
4. Some peripherals require specific alignment for DMA buffers

```c
/* Set the DMA mask to 36 bits for BCM2712 peripherals */
if (dma_set_mask_and_coherent(dev, DMA_BIT_MASK(36))) {
    dev_warn(dev, "No suitable DMA available, falling back to 32-bit mask\n");
    
    /* Try with 32-bit mask as fallback */
    if (dma_set_mask_and_coherent(dev, DMA_BIT_MASK(32))) {
        dev_err(dev, "No DMA available\n");
        return -ENODEV;
    }
}
```

## Memory Mapping and User-Kernel Space Interaction

Often, device drivers need to allow userspace applications to directly access device memory or driver buffers. This is done using memory mapping:

```c
/* Global driver buffer */
static void *driver_buffer;
static dma_addr_t driver_dma_handle;
static size_t driver_buffer_size = 1024 * 1024;  /* 1 MB */

/* mmap file operation implementation */
static int my_device_mmap(struct file *file, struct vm_area_struct *vma)
{
    unsigned long size = vma->vm_end - vma->vm_start;
    
    /* Check if requested mapping size exceeds our buffer */
    if (size > driver_buffer_size)
        return -EINVAL;
    
    /* Map kernel memory to userspace */
    if (remap_pfn_range(vma,
                       vma->vm_start,
                       virt_to_phys(driver_buffer) >> PAGE_SHIFT,
                       size,
                       vma->vm_page_prot)) {
        return -EAGAIN;
    }
    
    return 0;
}

/* Device file operations structure */
static const struct file_operations my_device_fops = {
    .owner = THIS_MODULE,
    .open = my_device_open,
    .release = my_device_release,
    .read = my_device_read,
    .write = my_device_write,
    .mmap = my_device_mmap,
};

/* Initialization */
static int __init my_device_init(void)
{
    /* ... other initialization ... */
    
    /* Allocate DMA-capable memory */
    driver_buffer = dma_alloc_coherent(dev, driver_buffer_size,
                                      &driver_dma_handle, GFP_KERNEL);
    if (!driver_buffer)
        return -ENOMEM;
    
    /* ... register character device ... */
    
    return 0;
}

/* Cleanup */
static void __exit my_device_exit(void)
{
    /* ... unregister character device ... */
    
    /* Free DMA memory */
    if (driver_buffer)
        dma_free_coherent(dev, driver_buffer_size,
                         driver_buffer, driver_dma_handle);
}
```

### User Space Application Example

Here's how a userspace application would use the memory mapping:

```c
#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/mman.h>

#define BUFFER_SIZE (1024 * 1024)  /* 1 MB */

int main()
{
    int fd;
    void *mapped_memory;
    
    /* Open the device */
    fd = open("/dev/mydevice", O_RDWR);
    if (fd < 0) {
        perror("Failed to open device");
        return -1;
    }
    
    /* Map the device memory to user space */
    mapped_memory = mmap(NULL, BUFFER_SIZE, PROT_READ | PROT_WRITE,
                          MAP_SHARED, fd, 0);
    if (mapped_memory == MAP_FAILED) {
        perror("Failed to map memory");
        close(fd);
        return -1;
    }
    
    /* Now we can access the device memory directly */
    printf("Memory mapped at address %p\n", mapped_memory);
    
    /* Example: Write a pattern to the memory */
    unsigned char *ptr = (unsigned char *)mapped_memory;
    for (int i = 0; i < 100; i++) {
        ptr[i] = i & 0xFF;
    }
    
    /* Read back the values */
    for (int i = 0; i < 100; i++) {
        printf("%d ", ptr[i]);
    }
    printf("\n");
    
    /* Clean up */
    munmap(mapped_memory, BUFFER_SIZE);
    close(fd);
    
    return 0;
}
```

## Memory Debugging Techniques

Memory bugs in kernel code can be particularly difficult to diagnose. Here are some useful techniques for debugging kernel memory issues:

### 1. Kmemleak - Kernel Memory Leak Detector

Kmemleak helps detect memory leaks in kernel code:

```bash
# Enable kmemleak at boot time (add to kernel command line)
sudo nano /boot/cmdline.txt
# Add 'kmemleak=on' to the end of the line
# Save and reboot

# Trigger a scan
echo scan > /sys/kernel/debug/kmemleak

# View detected leaks
cat /sys/kernel/debug/kmemleak
```

### 2. KASAN - Kernel Address Sanitizer

KASAN is a powerful dynamic memory error detector for the kernel, similar to AddressSanitizer for userspace:

  KASAN requires a specially configured kernel. When building a custom kernel, enable CONFIG_KASAN in the kernel configuration.

KASAN detects:
- Out-of-bounds accesses
- Use-after-free
- Double-free errors
- Uninitialized memory reads

```bash
# KASAN reports appear in kernel logs
dmesg | grep KASAN
```

### 3. Memory Usage Statistics

Monitor memory usage and allocations:

```bash
# View overall memory stats
cat /proc/meminfo

# View slab allocator statistics
cat /proc/slabinfo

# View vmalloc statistics
cat /proc/vmallocinfo

# Check for memory fragmentation
cat /proc/buddyinfo
```

### 4. Common Kernel Memory Bugs

  These are critical bugs that can cause system instability or security vulnerabilities:

1. **Memory Leaks**: Forgetting to free allocated memory
2. **Use-After-Free**: Accessing memory after it has been freed
3. **Buffer Overflows**: Writing past the end of allocated memory
4. **Double Free**: Freeing the same memory twice
5. **Freeing Wrong Memory**: Passing the wrong pointer to kfree/vfree
6. **Memory Type Mismatch**: Using kfree on vmalloc memory or vice versa
7. **Invalid DMA Directions**: Using the wrong direction flag in DMA operations

## Complete Example: Memory Management Driver

Let's put everything together in a complete driver example. This driver demonstrates various memory allocation techniques:

```c
#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/init.h>
#include <linux/fs.h>
#include <linux/cdev.h>
#include <linux/device.h>
#include <linux/slab.h>
#include <linux/vmalloc.h>
#include <linux/mm.h>
#include <linux/dma-mapping.h>
#include <linux/uaccess.h>

#define DEVICE_NAME "memdriver"
#define CLASS_NAME "memclass"
#define MAX_DEVICES 1

/* Module metadata */
MODULE_LICENSE("GPL");
MODULE_AUTHOR("Utsav Balar");
MODULE_DESCRIPTION("Memory Management Demonstration Driver");
MODULE_VERSION("0.1");

/* Device structure */
struct mem_device {
    struct cdev cdev;
    void *kmalloc_buffer;
    void *vmalloc_buffer;
    void *dma_buffer;
    dma_addr_t dma_handle;
    struct kmem_cache *obj_cache;
    struct device *device;
    
    size_t kmalloc_size;
    size_t vmalloc_size;
    size_t dma_size;
    size_t obj_size;
};

/* Structure for objects in our slab cache */
struct mem_object {
    int id;
    char data[64];
    struct list_head list;
};

/* Global variables */
static int major_number;
static struct class *mem_class = NULL;
static struct mem_device *mem_devices = NULL;
static LIST_HEAD(object_list);
static DEFINE_MUTEX(object_list_lock);

/* Memory sizes */
#define KMALLOC_SIZE (4 * 1024)       /* 4 KB */
#define VMALLOC_SIZE (1 * 1024 * 1024) /* 1 MB */
#define DMA_SIZE (16 * 1024)          /* 16 KB */
#define OBJECT_SIZE sizeof(struct mem_object)

/* IOCTL commands */
#define MEM_IOCTL_MAGIC 'M'
#define MEM_IOCTL_ALLOC_OBJECT _IO(MEM_IOCTL_MAGIC, 1)
#define MEM_IOCTL_FREE_OBJECT  _IOW(MEM_IOCTL_MAGIC, 2, int)

/* File operations */
static int mem_open(struct inode *inode, struct file *file)
{
    struct mem_device *mem_dev;
    
    mem_dev = container_of(inode->i_cdev, struct mem_device, cdev);
    file->private_data = mem_dev;
    
    pr_info("MEMDRIVER: Device opened\n");
    return 0;
}

static int mem_release(struct inode *inode, struct file *file)
{
    pr_info("MEMDRIVER: Device closed\n");
    return 0;
}

static ssize_t mem_read(struct file *file, char __user *user_buffer,
                        size_t count, loff_t *offset)
{
    struct mem_device *mem_dev = file->private_data;
    unsigned long bytes_not_copied;
    size_t bytes_to_read;
    
    /* Choose buffer based on offset range */
    void *src_buffer;
    size_t max_size;
    
    if (*offset < mem_dev->kmalloc_size) {
        /* Read from kmalloc buffer */
        src_buffer = mem_dev->kmalloc_buffer;
        max_size = mem_dev->kmalloc_size;
    } else if (*offset < mem_dev->kmalloc_size + mem_dev->vmalloc_size) {
        /* Read from vmalloc buffer */
        src_buffer = mem_dev->vmalloc_buffer;
        *offset -= mem_dev->kmalloc_size;
        max_size = mem_dev->vmalloc_size;
    } else if (*offset < mem_dev->kmalloc_size + mem_dev->vmalloc_size + mem_dev->dma_size) {
        /* Read from DMA buffer */
        src_buffer = mem_dev->dma_buffer;
        *offset -= (mem_dev->kmalloc_size + mem_dev->vmalloc_size);
        max_size = mem_dev->dma_size;
    } else {
        return 0; /* EOF */
    }
    
    /* Calculate how many bytes to read */
    bytes_to_read = min((size_t)(max_size - *offset), count);
    
    if (bytes_to_read == 0)
        return 0;
    
    /* Copy data to user */
    bytes_not_copied = copy_to_user(user_buffer, src_buffer + *offset, bytes_to_read);
    
    /* Update offset and return bytes successfully read */
    *offset += (bytes_to_read - bytes_not_copied);
    return bytes_to_read - bytes_not_copied;
}

static ssize_t mem_write(struct file *file, const char __user *user_buffer,
                         size_t count, loff_t *offset)
{
    struct mem_device *mem_dev = file->private_data;
    unsigned long bytes_not_copied;
    size_t bytes_to_write;
    
    /* Choose buffer based on offset range */
    void *dest_buffer;
    size_t max_size;
    
    if (*offset < mem_dev->kmalloc_size) {
        /* Write to kmalloc buffer */
        dest_buffer = mem_dev->kmalloc_buffer;
        max_size = mem_dev->kmalloc_size;
    } else if (*offset < mem_dev->kmalloc_size + mem_dev->vmalloc_size) {
        /* Write to vmalloc buffer */
        dest_buffer = mem_dev->vmalloc_buffer;
        *offset -= mem_dev->kmalloc_size;
        max_size = mem_dev->vmalloc_size;
    } else if (*offset < mem_dev->kmalloc_size + mem_dev->vmalloc_size + mem_dev->dma_size) {
        /* Write to DMA buffer */
        dest_buffer = mem_dev->dma_buffer;
        *offset -= (mem_dev->kmalloc_size + mem_dev->vmalloc_size);
        max_size = mem_dev->dma_size;
    } else {
        return -ENOSPC; /* No space left */
    }
    
    /* Calculate how many bytes to write */
    bytes_to_write = min((size_t)(max_size - *offset), count);
    
    if (bytes_to_write == 0)
        return -ENOSPC;
    
    /* Copy data from user */
    bytes_not_copied = copy_from_user(dest_buffer + *offset, user_buffer, bytes_to_write);
    
    /* Update offset and return bytes successfully written */
    *offset += (bytes_to_write - bytes_not_copied);
    return bytes_to_write - bytes_not_copied;
}

static long mem_ioctl(struct file *file, unsigned int cmd, unsigned long arg)
{
    struct mem_device *mem_dev = file->private_data;
    struct mem_object *obj;
    int id, found = 0;
    
    switch (cmd) {
    case MEM_IOCTL_ALLOC_OBJECT:
        /* Allocate a new object from cache */
        obj = kmem_cache_alloc(mem_dev->obj_cache, GFP_KERNEL);
        if (!obj)
            return -ENOMEM;
        
        /* Initialize object */
        obj->id = jiffies & 0xFFFF; /* Simple ID generation */
        snprintf(obj->data, sizeof(obj->data), "Object ID: %d", obj->id);
        
        /* Add to list */
        mutex_lock(&object_list_lock);
        list_add_tail(&obj->list, &object_list);
        mutex_unlock(&object_list_lock);
        
        pr_info("MEMDRIVER: Allocated object ID %d\n", obj->id);
        return obj->id;
        
    case MEM_IOCTL_FREE_OBJECT:
        /* Get object ID from user */
        if (copy_from_user(&id, (int __user *)arg, sizeof(int)))
            return -EFAULT;
        
        /* Find and remove object */
        mutex_lock(&object_list_lock);
        list_for_each_entry(obj, &object_list, list) {
            if (obj->id == id) {
                list_del(&obj->list);
                kmem_cache_free(mem_dev->obj_cache, obj);
                found = 1;
                break;
            }
        }
        mutex_unlock(&object_list_lock);
        
        if (!found)
            return -ENOENT;
            
        pr_info("MEMDRIVER: Freed object ID %d\n", id);
        return 0;
        
    default:
        return -ENOTTY; /* Unknown command */
    }
}

static int mem_mmap(struct file *file, struct vm_area_struct *vma)
{
    struct mem_device *mem_dev = file->private_data;
    unsigned long offset = vma->vm_pgoff << PAGE_SHIFT;
    unsigned long size = vma->vm_end - vma->vm_start;
    
    /* Check which area to map based on offset */
    if (offset == 0 && size <= mem_dev->kmalloc_size) {
        /* Map kmalloc buffer */
        if (remap_pfn_range(vma, vma->vm_start,
                          virt_to_phys(mem_dev->kmalloc_buffer) >> PAGE_SHIFT,
                          size, vma->vm_page_prot)) {
            return -EAGAIN;
        }
    } else if (offset == mem_dev->kmalloc_size && size <= mem_dev->dma_size) {
        /* Map DMA buffer - using physical address via dma_handle */
        if (remap_pfn_range(vma, vma->vm_start,
                          mem_dev->dma_handle >> PAGE_SHIFT,
                          size, vma->vm_page_prot)) {
            return -EAGAIN;
        }
    } else {
        /* Cannot map vmalloc directly - would need special handling */
        return -EINVAL;
    }
    
    return 0;
}

static struct file_operations mem_fops = {
    .owner = THIS_MODULE,
    .open = mem_open,
    .release = mem_release,
    .read = mem_read,
    .write = mem_write,
    .unlocked_ioctl = mem_ioctl,
    .mmap = mem_mmap,
};

static int __init mem_init(void)
{
    int ret, i;
    
    /* Dynamically allocate a major number */
    major_number = register_chrdev(0, DEVICE_NAME, &mem_fops);
    if (major_number < 0) {
        pr_err("MEMDRIVER: Failed to register major number\n");
        return major_number;
    }
    
    /* Create device class */
    mem_class = class_create(THIS_MODULE, CLASS_NAME);
    if (IS_ERR(mem_class)) {
        unregister_chrdev(major_number, DEVICE_NAME);
        pr_err("MEMDRIVER: Failed to create device class\n");
        return PTR_ERR(mem_class);
    }
    
    /* Allocate memory for our device structure */
    mem_devices = kmalloc(MAX_DEVICES * sizeof(struct mem_device), GFP_KERNEL);
    if (!mem_devices) {
        class_destroy(mem_class);
        unregister_chrdev(major_number, DEVICE_NAME);
        pr_err("MEMDRIVER: Failed to allocate memory for device structure\n");
        return -ENOMEM;
    }
    
    /* Initialize and set up each device */
    for (i = 0; i < MAX_DEVICES; i++) {
        /* Create device */
        mem_devices[i].device = device_create(mem_class, NULL, 
                                            MKDEV(major_number, i),
                                            NULL, DEVICE_NAME "%d", i);
        if (IS_ERR(mem_devices[i].device)) {
            ret = PTR_ERR(mem_devices[i].device);
            pr_err("MEMDRIVER: Failed to create device\n");
            goto fail_device;
        }
        
        /* Initialize cdev structure and add it */
        cdev_init(&mem_devices[i].cdev, &mem_fops);
        mem_devices[i].cdev.owner = THIS_MODULE;
        ret = cdev_add(&mem_devices[i].cdev, MKDEV(major_number, i), 1);
        if (ret < 0) {
            pr_err("MEMDRIVER: Failed to add cdev\n");
            goto fail_cdev;
        }
        
        /* Allocate memory regions */
        mem_devices[i].kmalloc_size = KMALLOC_SIZE;
        mem_devices[i].vmalloc_size = VMALLOC_SIZE;
        mem_devices[i].dma_size = DMA_SIZE;
        mem_devices[i].obj_size = OBJECT_SIZE;
        
        /* Allocate kmalloc buffer */
        mem_devices[i].kmalloc_buffer = kmalloc(KMALLOC_SIZE, GFP_KERNEL);
        if (!mem_devices[i].kmalloc_buffer) {
            pr_err("MEMDRIVER: Failed to allocate kmalloc buffer\n");
            ret = -ENOMEM;
            goto fail_kmalloc;
        }
        memset(mem_devices[i].kmalloc_buffer, 0, KMALLOC_SIZE);
        
        /* Allocate vmalloc buffer */
        mem_devices[i].vmalloc_buffer = vmalloc(VMALLOC_SIZE);
        if (!mem_devices[i].vmalloc_buffer) {
            pr_err("MEMDRIVER: Failed to allocate vmalloc buffer\n");
            ret = -ENOMEM;
            goto fail_vmalloc;
        }
        memset(mem_devices[i].vmalloc_buffer, 0, VMALLOC_SIZE);
        
        /* Allocate DMA buffer */
        mem_devices[i].dma_buffer = dma_alloc_coherent(mem_devices[i].device,
                                                     DMA_SIZE,
                                                     &mem_devices[i].dma_handle,
                                                     GFP_KERNEL);
        if (!mem_devices[i].dma_buffer) {
            pr_err("MEMDRIVER: Failed to allocate DMA buffer\n");
            ret = -ENOMEM;
            goto fail_dma;
        }
        memset(mem_devices[i].dma_buffer, 0, DMA_SIZE);
        
        /* Create object cache */
        mem_devices[i].obj_cache = kmem_cache_create("memdriver_cache",
                                                   OBJECT_SIZE, 0, 0, NULL);
        if (!mem_devices[i].obj_cache) {
            pr_err("MEMDRIVER: Failed to create object cache\n");
            ret = -ENOMEM;
            goto fail_cache;
        }
    }
    
    pr_info("MEMDRIVER: Initialized with major number %d\n", major_number);
    return 0;
    
    /* Error handling */
fail_cache:
    dma_free_coherent(mem_devices[i].device, DMA_SIZE,
                     mem_devices[i].dma_buffer, mem_devices[i].dma_handle);
fail_dma:
    vfree(mem_devices[i].vmalloc_buffer);
fail_vmalloc:
    kfree(mem_devices[i].kmalloc_buffer);
fail_kmalloc:
    cdev_del(&mem_devices[i].cdev);
fail_cdev:
    device_destroy(mem_class, MKDEV(major_number, i));
fail_device:
    /* Clean up any previously created devices */
    while (--i >= 0) {
        if (mem_devices[i].obj_cache)
            kmem_cache_destroy(mem_devices[i].obj_cache);
        if (mem_devices[i].dma_buffer)
            dma_free_coherent(mem_devices[i].device, DMA_SIZE,
                             mem_devices[i].dma_buffer, mem_devices[i].dma_handle);
        if (mem_devices[i].vmalloc_buffer)
            vfree(mem_devices[i].vmalloc_buffer);
        if (mem_devices[i].kmalloc_buffer)
            kfree(mem_devices[i].kmalloc_buffer);
        cdev_del(&mem_devices[i].cdev);
        device_destroy(mem_class, MKDEV(major_number, i));
    }
    kfree(mem_devices);
    class_destroy(mem_class);
    unregister_chrdev(major_number, DEVICE_NAME);
    return ret;
}

static void __exit mem_exit(void)
{
    int i;
    struct mem_object *obj, *temp;
    
    /* Free all objects in the list */
    mutex_lock(&object_list_lock);
    list_for_each_entry_safe(obj, temp, &object_list, list) {
        list_del(&obj->list);
        kmem_cache_free(mem_devices[0].obj_cache, obj);
    }
    mutex_unlock(&object_list_lock);
    
    /* Clean up devices */
    for (i = 0; i < MAX_DEVICES; i++) {
        if (mem_devices[i].obj_cache)
            kmem_cache_destroy(mem_devices[i].obj_cache);
        if (mem_devices[i].dma_buffer)
            dma_free_coherent(mem_devices[i].device, DMA_SIZE,
                             mem_devices[i].dma_buffer, mem_devices[i].dma_handle);
        if (mem_devices[i].vmalloc_buffer)
            vfree(mem_devices[i].vmalloc_buffer);
        if (mem_devices[i].kmalloc_buffer)
            kfree(mem_devices[i].kmalloc_buffer);
        cdev_del(&mem_devices[i].cdev);
        device_destroy(mem_class, MKDEV(major_number, i));
    }
    
    kfree(mem_devices);
    class_destroy(mem_class);
    unregister_chrdev(major_number, DEVICE_NAME);
    
    pr_info("MEMDRIVER: Module unloaded\n");
}

module_init(mem_init);
module_exit(mem_exit);
```

### Building and Testing the Driver

```makefile
obj-m := memdriver.o

KERNEL_DIR ?= /lib/modules/$(shell uname -r)/build
PWD := $(shell pwd)

all:
	$(MAKE) -C $(KERNEL_DIR) M=$(PWD) modules

clean:
	$(MAKE) -C $(KERNEL_DIR) M=$(PWD) clean
```

```bash
# Build the driver
make

# Load the module
sudo insmod memdriver.ko

# Check that the device node was created
ls -la /dev/memdriver*

# Test writing to the driver
echo "Testing memory allocation" | sudo tee /dev/memdriver0

# Test reading from the driver
sudo cat /dev/memdriver0

# Unload the module
sudo rmmod memdriver
```

## User-Space Test Program

Create a simple test program to interact with our memory driver:

```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/ioctl.h>
#include <sys/mman.h>

/* IOCTL commands - must match the ones in the driver */
#define MEM_IOCTL_MAGIC 'M'
#define MEM_IOCTL_ALLOC_OBJECT _IO(MEM_IOCTL_MAGIC, 1)
#define MEM_IOCTL_FREE_OBJECT  _IOW(MEM_IOCTL_MAGIC, 2, int)

#define KMALLOC_SIZE (4 * 1024)       /* 4 KB */
#define DMA_SIZE (16 * 1024)          /* 16 KB */

int main(int argc, char *argv[])
{
    int fd, ret, i;
    char buffer[128];
    int object_ids[10] = {0};
    void *kmalloc_mapped = NULL;
    void *dma_mapped = NULL;
    
    /* Open the device */
    fd = open("/dev/memdriver0", O_RDWR);
    if (fd < 0) {
        perror("Failed to open device");
        return 1;
    }
    
    printf("Device opened successfully\n");
    
    /* Test writing to device */
    strcpy(buffer, "Hello from user space!");
    ret = write(fd, buffer, strlen(buffer));
    if (ret < 0) {
        perror("Failed to write to device");
        close(fd);
        return 1;
    }
    
    printf("Wrote %d bytes to device\n", ret);
    
    /* Test reading from device */
    memset(buffer, 0, sizeof(buffer));
    ret = read(fd, buffer, sizeof(buffer) - 1);
    if (ret < 0) {
        perror("Failed to read from device");
        close(fd);
        return 1;
    }
    
    printf("Read %d bytes from device: %s\n", ret, buffer);
    
    /* Test IOCTL to allocate objects */
    printf("Allocating objects using kmem_cache...\n");
    for (i = 0; i < 5; i++) {
        ret = ioctl(fd, MEM_IOCTL_ALLOC_OBJECT);
        if (ret < 0) {
            perror("Failed to allocate object");
        } else {
            object_ids[i] = ret;
            printf("Allocated object ID: %d\n", ret);
        }
    }
    
    /* Test IOCTL to free objects */
    printf("Freeing objects...\n");
    for (i = 0; i < 5; i++) {
        if (object_ids[i] > 0) {
            ret = ioctl(fd, MEM_IOCTL_FREE_OBJECT, &object_ids[i]);
            if (ret < 0) {
                perror("Failed to free object");
            } else {
                printf("Freed object ID: %d\n", object_ids[i]);
            }
        }
    }
    
    /* Test memory mapping of kmalloc buffer */
    printf("Testing mmap of kmalloc buffer...\n");
    kmalloc_mapped = mmap(NULL, KMALLOC_SIZE, PROT_READ | PROT_WRITE,
                         MAP_SHARED, fd, 0);
    if (kmalloc_mapped == MAP_FAILED) {
        perror("Failed to map kmalloc memory");
    } else {
        printf("Mapped kmalloc buffer at %p\n", kmalloc_mapped);
        
        /* Write a pattern to the mapped memory */
        unsigned char *ptr = (unsigned char *)kmalloc_mapped;
        for (i = 0; i < 16; i++) {
            ptr[i] = i + 'A';
        }
        ptr[i] = '\0';
        
        printf("Wrote pattern to mapped memory: %s\n", ptr);
        
        /* Unmap the memory */
        munmap(kmalloc_mapped, KMALLOC_SIZE);
    }
    
    /* Test memory mapping of DMA buffer */
    printf("Testing mmap of DMA buffer...\n");
    dma_mapped = mmap(NULL, DMA_SIZE, PROT_READ | PROT_WRITE,
                     MAP_SHARED, fd, KMALLOC_SIZE);
    if (dma_mapped == MAP_FAILED) {
        perror("Failed to map DMA memory");
    } else {
        printf("Mapped DMA buffer at %p\n", dma_mapped);
        
        /* Write a pattern to the mapped memory */
        unsigned char *ptr = (unsigned char *)dma_mapped;
        for (i = 0; i < 16; i++) {
            ptr[i] = i + '0';
        }
        ptr[i] = '\0';
        
        printf("Wrote pattern to mapped DMA memory: %s\n", ptr);
        
        /* Unmap the memory */
        munmap(dma_mapped, DMA_SIZE);
    }
    
    /* Close the device */
    close(fd);
    printf("Device closed\n");
    
    return 0;
}
```

```bash
# Compile the test program
gcc -o memdriver_test memdriver_test.c

# Run the test program
sudo ./memdriver_test
```

## Summary

In this tutorial, we've covered Linux kernel memory management for device drivers:

1. **Allocation Methods**:
   - `kmalloc()` for small, physically contiguous memory
   - `vmalloc()` for larger, virtually contiguous memory
   - `__get_free_pages()` for page-sized allocations
   - `kmem_cache` for object-based allocations
   - DMA memory for device-accessible memory

2. **Memory Mapping**:
   - Kernel to userspace memory mapping
   - `remap_pfn_range()` for exposing kernel memory
   - Userspace `mmap()` for accessing kernel memory

3. **DMA Operations**:
   - Coherent vs. streaming DMA
   - DMA direction flags
   - ARM64/Raspberry Pi 5 specific considerations

4. **Debugging Techniques**:
   - Kmemleak for memory leak detection
   - KASAN for memory error detection
   - Kernel memory statistics tools

The included driver example demonstrates all these concepts in a working module that you can build and test on your Raspberry Pi 5.

  **ARM64 Considerations**: The BCM2712 SoC in the Raspberry Pi 5 has specific memory characteristics. When developing real drivers, always consult the SoC documentation for hardware-specific details about memory access, DMA capabilities, and address limitations.

In the next tutorial, we'll build on these memory management concepts and explore synchronization primitives for multi-threaded driver code. 