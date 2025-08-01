---
title: "Interrupt Handling and Workqueues"
description: "Master the essentials of interrupt handling and deferred work mechanisms in the Linux kernel with practical examples for Raspberry Pi 5. Learn how to process hardware events efficiently while maintaining system responsiveness."
date: 2025-05-26
author: "Utsav Balar"
difficulty: "intermediate"
topics: ["kernel", "device-drivers", "raspberry-pi", "interrupts", "workqueues"]
series: "Linux Kernel Device Driver"
part: 5
environment:
  hardware: "Raspberry Pi 5"
  kernel: "6.12"
  os: "Raspberry Pi OS (64-bit)"
prerequisites: ["Basic C programming knowledge", "Linux command line familiarity", "Introduction to Linux Kernel Modules (Tutorial 1)", "Character Device Drivers Fundamentals (Tutorial 2)", "Kernel Memory Management for Drivers (Tutorial 3)", "Synchronization Primitives (Tutorial 4)"]
github: "https://github.com/UtsavBalar1231/utsavbalar-linux-kernel-tutorials/tree/master/tutorial-05"
featured: true
draft: false
---


# Interrupt Handling and Workqueues

## Introduction to Hardware Interrupts

In embedded systems like the Raspberry Pi, hardware devices need to notify the CPU when events occur. This notification mechanism is called an interrupt. Unlike polling, where the CPU continuously checks device status, interrupts allow the CPU to focus on other tasks until a device requires attention.

Common sources of interrupts on the Raspberry Pi 5 include:

1. GPIO pins (buttons, sensors)
2. Timers and counters
3. Communication interfaces (UART, SPI, I2C)
4. DMA completion
5. External peripheral events

  The Raspberry Pi 5's BCM2712 SoC features a GIC-600 interrupt controller, significantly improved over previous Raspberry Pi models, supporting more interrupt sources and advanced routing capabilities.

## Interrupt Handling Contexts and Flow

Before diving into the code, it's essential to understand the complete interrupt flow in the Linux kernel and the execution contexts involved:

```text
Hardware Event
      |
      v
CPU Detects Interrupt
      |
      v
CPU Jumps to Interrupt Vector
      |
      v
Low-level Handler (arch-specific assembly)
      |
      v
Generic IRQ Handling Code (kernel/irq/handle.c)
      |
      v
Specific Device IRQ Handler (registered with request_irq)
      |
      v
Bottom Half Processing (tasklet, workqueue, threaded IRQ)
      |
      v
User Space Notification (if needed)
```

### Hardware Interrupt Context

When a hardware interrupt occurs, the CPU enters a special execution context with significant restrictions:

**Interrupt Context Constraints**
## Hardware Interrupt Context (Top Half)

- Current user process is interrupted, but remains the "current" process
- Process context switches are disabled
- Local CPU interrupts are typically disabled
- Preemption is disabled
- Cannot access user space memory (copy_to/from_user will fail)
- Cannot sleep or block
- Cannot call any function that might sleep:
  * kmalloc() with GFP_KERNEL flag
  * mutex_lock()
  * schedule()
  * wait_event()
  * msleep() or similar
- Must only use atomic operations and spinlocks
- Should complete as quickly as possible (microseconds, not milliseconds)
- Stack space is limited
- Jitter should be minimized for real-time applications

## Bottom Half Context (Varies by mechanism)

- Tasklet: Still cannot sleep, but runs with interrupts enabled
- Workqueue: Process context, can sleep (most flexible)
- Threaded IRQ: Process context, can sleep, dedicated kernel thread
- Softirq: Still cannot sleep, higher priority than tasklets

### Execution Flow Visualization

To better understand the flow of execution during interrupt handling:

```text
Process A         Hardware       Process B
   |                 |               |
   |                 |               |
   |                 | [IRQ Event]   |
   |                 |               |
   | [Interrupted]   |               |
   |                 |               |
   |     [IRQ Handler Runs]          |
   |                 |               |
   |   [Schedule Tasklet/Work]       |
   |                 |               |
   | [Resume]        |               |
   |                 |               |
   |                 |               |
   |                 |               |  
   |                 |               | [Context Switch]
   |                 |               |
   |                 |               | [Bottom Half Runs]
   |                 |               | (if using workqueue)
   |                 |               |
   v                 v               v

OR if using tasklets:

Process A         Hardware       Tasklet
   |                 |               |
   |                 |               |
   |                 | [IRQ Event]   |
   |                 |               |
   | [Interrupted]   |               |
   |                 |               |
   |     [IRQ Handler Runs]          |
   |                 |               |
   |   [Schedule Tasklet]            |
   |                 |               |
   | [Resume]        |               |
   |                 |               |
   |           [Tasklet Runs]        |
   |           (on software interrupt)|
   |                 |               |
   v                 v               v
```

## Interrupt Context Considerations

Working with interrupt context correctly is one of the most challenging aspects of kernel programming. Here are additional details on the constraints and best practices:

```c
#include <linux/interrupt.h>
#include <linux/gfp.h>

/* Global pointer to allocated memory */
static void *global_buffer;

/* Top-half function (cannot sleep) */
static irqreturn_t my_interrupt_handler(int irq, void *dev_id)
{
    /* 
     * INCORRECT - GFP_KERNEL might sleep!
     * void *temp = kmalloc(1024, GFP_KERNEL);
     */
    
    /* CORRECT - GFP_ATOMIC won't sleep */
    void *temp = kmalloc(1024, GFP_ATOMIC);
    if (!temp) {
        /* Handle allocation failure - common with GFP_ATOMIC */
        pr_err("Failed to allocate memory in interrupt\n");
        return IRQ_HANDLED;
    }
    
    /* Process data quickly */
    /* ... */
    
    /* Free the temporary buffer */
    kfree(temp);
    
    /* 
     * INCORRECT - mutex might sleep!
     * mutex_lock(&my_mutex);
     */
    
    /* CORRECT - spinlock won't sleep */
    spin_lock(&my_spinlock);
    /* Critical section */
    spin_unlock(&my_spinlock);
    
    return IRQ_HANDLED;
}

/* Bottom-half function (workqueue, can sleep) */
static void my_work_function(struct work_struct *work)
{
    /* This is process context, so we can sleep */
    
    /* CORRECT - GFP_KERNEL is fine here */
    void *buffer = kmalloc(4096, GFP_KERNEL);
    if (!buffer) {
        pr_err("Failed to allocate memory in workqueue\n");
        return;
    }
    
    /* CORRECT - mutex is fine here */
    mutex_lock(&my_mutex);
    /* Critical section */
    mutex_unlock(&my_mutex);
    
    /* CORRECT - can sleep here */
    msleep(1);
    
    /* Process data without time constraints */
    /* ... */
    
    /* Free buffer */
    kfree(buffer);
}
```

### Memory Allocation in Interrupt Context

Memory allocation in interrupt context requires special consideration:

## GFP Flags for Different Contexts

| Flag         | Sleeps? | Best For                      | Drawbacks                   |
|--------------|---------|-------------------------------|----------------------------|
| GFP_KERNEL   | Yes     | Normal process context        | Cannot use in interrupt     |
| GFP_ATOMIC   | No      | Interrupt handlers, atomic    | Higher failure rate, limited|
| GFP_NOWAIT   | No      | Non-interrupt atomic context  | May fail more often         |
| GFP_DMA      | Varies  | DMA-capable memory            | Limited pool of memory      |

```c
/* During module init (can sleep) */
global_buffer = kmalloc(BUFFER_SIZE, GFP_KERNEL);

/* In interrupt handler (can't sleep) */
if (global_buffer) {
    /* Use pre-allocated buffer safely */
}
```

```c
/* Create cache during initialization */
my_cache = kmem_cache_create("my_cache", object_size, 0, 
                            SLAB_HWCACHE_ALIGN, NULL);

/* In interrupt context */
obj = kmem_cache_alloc(my_cache, GFP_ATOMIC);
if (obj) {
    /* Use object */
    /* ... */
    kmem_cache_free(my_cache, obj);
}
```

## Interrupt Handling and Real-Time Considerations

For real-time applications on the Raspberry Pi 5, interrupt handling has significant implications:

## Real-Time Impact of Interrupts

1. **Interrupt Latency**: Time between hardware event and start of handler
   - Affected by: CPU frequency, interrupt controller, priority levels
   - BCM2712 GIC-600 supports prioritization to reduce latency for critical interrupts

2. **Interrupt Jitter**: Variation in interrupt response time
   - Causes: Cache effects, other interrupts, system load
   - Critical for precise timing applications (motor control, audio)

3. **Interrupt Duration**: How long the handler executes
   - Keep top half minimal (microseconds if possible)
   - Defer processing to bottom half

4. **Priority Inversion**: Lower priority interrupt blocking a higher one
   - GIC-600 helps manage this with proper IRQ priorities
   - Use threaded IRQs to move processing to thread context with appropriate priority

## PREEMPT_RT Considerations

When using the PREEMPT_RT patch for real-time Linux on Raspberry Pi 5:

1. Most interrupt handlers become threaded by default
2. Softirqs run in thread context
3. Spinlocks become mutexes (can sleep!)
4. Need special real-time safe code patterns
5. RT-specific debugging tools available

```c
/* Real-time measurement code */
struct rt_metrics {
    ktime_t irq_timestamp;   /* When IRQ arrived */
    ktime_t handler_entry;   /* When handler started */
    ktime_t handler_exit;    /* When handler completed */
    ktime_t thread_entry;    /* When threaded handler started */
    ktime_t thread_exit;     /* When threaded handler completed */
};

static struct rt_metrics metrics[1000];  /* Circular buffer */
static atomic_t metric_idx = ATOMIC_INIT(0);

static irqreturn_t rt_irq_handler(int irq, void *dev_id)
{
    int idx;
    struct rt_metrics *m;
    
    /* Record entry time */
    idx = atomic_inc_return(&metric_idx) % 1000;
    m = &metrics[idx];
    
    m->handler_entry = ktime_get();
    m->irq_timestamp = *(ktime_t*)dev_id;  /* Timestamp from ISR */
    
    /* Minimal processing here */
    
    m->handler_exit = ktime_get();
    
    return IRQ_WAKE_THREAD;  /* Wake threaded handler */
}

static irqreturn_t rt_thread_handler(int irq, void *dev_id)
{
    int idx = (atomic_read(&metric_idx) - 1) % 1000;
    struct rt_metrics *m = &metrics[idx];
    
    m->thread_entry = ktime_get();
    
    /* Main processing here */
    
    m->thread_exit = ktime_get();
    
    return IRQ_HANDLED;
}
```

## Interrupt Context vs. Sleeping Context

## Registering an Interrupt Handler

To handle interrupts from a device, you need to register an interrupt handler:

```c
#include <linux/interrupt.h>

/* Interrupt handler function */
static irqreturn_t my_interrupt_handler(int irq, void *dev_id)
{
    /* Handle the interrupt - this is the "top half" */
    
    /* Acknowledge the interrupt to the hardware if necessary */
    
    /* Return IRQ_HANDLED if we handled the interrupt */
    return IRQ_HANDLED;
}

/* Registering the handler */
int result;
result = request_irq(irq_number,        /* IRQ number to handle */
                   my_interrupt_handler, /* Handler function */
                   IRQF_SHARED,          /* Flags (shared, etc.) */
                   "my_device",          /* Device name for /proc/interrupts */
                   dev_data);            /* Data pointer passed to handler */
if (result) {
    pr_err("Cannot register IRQ %d\n", irq_number);
    return result;
}

/* When done, free the interrupt */
free_irq(irq_number, dev_data);
```

### Common IRQ Flags

The flags parameter in `request_irq()` controls how the interrupt is handled:

```c
/* Interrupt can be shared with other devices */
IRQF_SHARED

/* Trigger on rising edge */
IRQF_TRIGGER_RISING

/* Trigger on falling edge */
IRQF_TRIGGER_FALLING

/* Trigger on both edges */
IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING

/* Trigger on high level */
IRQF_TRIGGER_HIGH

/* Trigger on low level */
IRQF_TRIGGER_LOW

/* Combination for common cases */
IRQF_SHARED | IRQF_TRIGGER_RISING
```

### IRQ Return Values

The return value from an interrupt handler indicates whether the interrupt was handled:

```c
/* We handled the interrupt */
return IRQ_HANDLED;

/* We did not handle the interrupt (for shared IRQs) */
return IRQ_NONE;

/* We handled the interrupt and need to wake up processes */
return IRQ_WAKE_THREAD;

/* For threaded interrupts - use this to wake the thread handler */
return IRQ_HANDLED | IRQ_WAKE_THREAD;
```

## Bottom Half Mechanisms

Linux provides several mechanisms for deferring work from the interrupt handler to a more appropriate time:

1. **Softirqs**: Low-level, fixed-purpose handlers with high priority
2. **Tasklets**: Lightweight, dynamically schedulable tasks based on softirqs
3. **Workqueues**: Flexible kernel threads for general deferred work
4. **Threaded IRQs**: Dedicated kernel threads for specific interrupts

Let's explore each of these mechanisms.

### Tasklets

Tasklets are a simple way to defer work:

```c
#include <linux/interrupt.h>

/* Data structure for our device */
struct my_device_data {
    /* ... device-specific fields ... */
    struct tasklet_struct tasklet;
    unsigned long data_buffer[10];
    int data_count;
};

/* Tasklet function - the "bottom half" */
static void my_tasklet_function(unsigned long data)
{
    struct my_device_data *dev_data = (struct my_device_data *)data;
    
    /* Process the data collected in the interrupt handler */
    for (int i = 0; i < dev_data->data_count; i++) {
        /* Process dev_data->data_buffer[i] */
        pr_info("Processing data: %lu\n", dev_data->data_buffer[i]);
    }
    
    /* Reset the counter for next time */
    dev_data->data_count = 0;
}

/* Interrupt handler - the "top half" */
static irqreturn_t my_interrupt_handler(int irq, void *dev_id)
{
    struct my_device_data *dev_data = (struct my_device_data *)dev_id;
    
    /* Collect data quickly */
    if (dev_data->data_count < 10) {
        /* Read from hardware register and store in buffer */
        dev_data->data_buffer[dev_data->data_count++] = readl(dev_data->registers + SOME_OFFSET);
    }
    
    /* Schedule the tasklet to process the data later */
    tasklet_schedule(&dev_data->tasklet);
    
    return IRQ_HANDLED;
}

/* Initialize the device */
int init_my_device(struct my_device_data *dev_data)
{
    /* ... other initialization ... */
    
    /* Initialize the tasklet */
    tasklet_init(&dev_data->tasklet, my_tasklet_function, (unsigned long)dev_data);
    
    /* ... request IRQ, etc. ... */
    
    return 0;
}

/* Cleanup */
void cleanup_my_device(struct my_device_data *dev_data)
{
    /* ... other cleanup ... */
    
    /* Kill any pending tasklet */
    tasklet_kill(&dev_data->tasklet);
    
    /* ... free IRQ, etc. ... */
}
```

### Workqueues

For more flexible deferred processing, workqueues are ideal:

```c
#include <linux/workqueue.h>

struct my_device_data {
    /* ... device-specific fields ... */
    struct work_struct work;  /* Standard work structure */
    unsigned long data_buffer[10];
    int data_count;
    spinlock_t buffer_lock;   /* Protect data buffer access */
};

/* Work function - may sleep */
static void my_work_function(struct work_struct *work)
{
    struct my_device_data *dev_data = container_of(work, struct my_device_data, work);
    unsigned long flags;
    int count;
    unsigned long local_buffer[10];
    
    /* Copy data with spinlock protection */
    spin_lock_irqsave(&dev_data->buffer_lock, flags);
    count = dev_data->data_count;
    memcpy(local_buffer, dev_data->data_buffer, count * sizeof(unsigned long));
    dev_data->data_count = 0;
    spin_unlock_irqrestore(&dev_data->buffer_lock, flags);
    
    /* Process data (may sleep) */
    for (int i = 0; i < count; i++) {
        /* Process data with no timing constraints */
        pr_info("Processing data: %lu\n", local_buffer[i]);
        
        /* Can use functions that sleep */
        msleep(1);
    }
}

/* Interrupt handler - the "top half" */
static irqreturn_t my_interrupt_handler(int irq, void *dev_id)
{
    struct my_device_data *dev_data = (struct my_device_data *)dev_id;
    unsigned long flags;
    
    /* Collect data quickly with spinlock protection */
    spin_lock_irqsave(&dev_data->buffer_lock, flags);
    if (dev_data->data_count < 10) {
        /* Read from hardware register and store in buffer */
        dev_data->data_buffer[dev_data->data_count++] = readl(dev_data->registers + SOME_OFFSET);
    }
    spin_unlock_irqrestore(&dev_data->buffer_lock, flags);
    
    /* Schedule the work to process the data later */
    schedule_work(&dev_data->work);
    
    return IRQ_HANDLED;
}

/* Initialize the device */
int init_my_device(struct my_device_data *dev_data)
{
    /* ... other initialization ... */
    
    /* Initialize the spinlock */
    spin_lock_init(&dev_data->buffer_lock);
    
    /* Initialize the work structure */
    INIT_WORK(&dev_data->work, my_work_function);
    
    /* ... request IRQ, etc. ... */
    
    return 0;
}

/* Cleanup */
void cleanup_my_device(struct my_device_data *dev_data)
{
    /* ... other cleanup ... */
    
    /* Cancel any pending work */
    cancel_work_sync(&dev_data->work);
    
    /* ... free IRQ, etc. ... */
}
```

### Threaded IRQs

Modern Linux kernels support threaded interrupts, which simplify the top/bottom half separation:

```c
#include <linux/interrupt.h>

/* Top half - fast interrupt handler */
static irqreturn_t my_irq_handler(int irq, void *dev_id)
{
    struct my_device_data *dev_data = (struct my_device_data *)dev_id;
    
    /* Quick hardware acknowledgment and minimal processing */
    /* ... */
    
    /* Wake up the thread function to do the rest */
    return IRQ_WAKE_THREAD;
}

/* Bottom half - threaded handler that can sleep */
static irqreturn_t my_thread_fn(int irq, void *dev_id)
{
    struct my_device_data *dev_data = (struct my_device_data *)dev_id;
    
    /* Process the interrupt - can sleep here */
    /* ... */
    
    return IRQ_HANDLED;
}

/* Register both handlers */
result = request_threaded_irq(irq_number,
                             my_irq_handler,    /* Fast top half */
                             my_thread_fn,      /* Thread bottom half */
                             IRQF_SHARED | IRQF_TRIGGER_RISING,
                             "my_device",
                             dev_data);
```

## Custom Workqueues

For better control, you can create your own workqueue instead of using the system one:

```c
#include <linux/workqueue.h>

/* Declare a pointer to our workqueue */
static struct workqueue_struct *my_workqueue;

/* Initialize the workqueue */
my_workqueue = create_singlethread_workqueue("my_device_wq");
if (!my_workqueue) {
    /* Handle error */
    return -ENOMEM;
}

/* To use our custom workqueue instead of the system one */
queue_work(my_workqueue, &dev_data->work);

/* To clean up */
destroy_workqueue(my_workqueue);
```

## Workqueue Patterns and Best Practices

Workqueues are powerful but can be complex to use effectively. Here are some advanced patterns and best practices:

```c
#include <linux/workqueue.h>
#include <linux/jiffies.h>

struct my_device {
    /* ... other fields ... */
    struct workqueue_struct *wq;            /* Dedicated workqueue */
    struct work_struct normal_work;         /* Standard work */
    struct delayed_work delayed_work;       /* Delayed work */
    struct work_struct flush_work;          /* Special flush work */
    
    spinlock_t lock;                        /* Protect data */
    atomic_t busy_count;                    /* Track pending work */
    wait_queue_head_t flush_wait;           /* For synchronous flush */
    bool flush_complete;                    /* Flag for flush complete */
};

/* Standard work function */
static void my_normal_work_fn(struct work_struct *work)
{
    struct my_device *dev = container_of(work, struct my_device, normal_work);
    
    /* Do standard processing */
    /* ... */
    
    /* Update busy count */
    if (atomic_dec_and_test(&dev->busy_count))
        wake_up(&dev->flush_wait);
}

/* Delayed work function */
static void my_delayed_work_fn(struct work_struct *work)
{
    struct delayed_work *delayed_work = to_delayed_work(work);
    struct my_device *dev = container_of(delayed_work, 
                                       struct my_device, 
                                       delayed_work);
    unsigned long flags;
    
    /* Do timed work */
    /* ... */
    
    /* Check if we need to reschedule */
    spin_lock_irqsave(&dev->lock, flags);
    if (dev->needs_monitoring) {
        /* Reschedule to run again after 1 second */
        queue_delayed_work(dev->wq, &dev->delayed_work, 
                         msecs_to_jiffies(1000));
    }
    spin_unlock_irqrestore(&dev->lock, flags);
    
    /* Update busy count */
    if (atomic_dec_and_test(&dev->busy_count))
        wake_up(&dev->flush_wait);
}

/* Special flush work - used for synchronization */
static void my_flush_work_fn(struct work_struct *work)
{
    struct my_device *dev = container_of(work, struct my_device, flush_work);
    
    /* This work runs after all previous work is done */
    dev->flush_complete = true;
    wake_up(&dev->flush_wait);
}

/* Initialization */
int init_my_device_work(struct my_device *dev)
{
    /* Initialize fields */
    spin_lock_init(&dev->lock);
    atomic_set(&dev->busy_count, 0);
    init_waitqueue_head(&dev->flush_wait);
    dev->flush_complete = false;
    
    /* Create dedicated workqueue */
    dev->wq = alloc_workqueue("my_device_wq", WQ_MEM_RECLAIM, 1);
    if (!dev->wq)
        return -ENOMEM;
    
    /* Initialize work structures */
    INIT_WORK(&dev->normal_work, my_normal_work_fn);
    INIT_DELAYED_WORK(&dev->delayed_work, my_delayed_work_fn);
    INIT_WORK(&dev->flush_work, my_flush_work_fn);
    
    return 0;
}

/* Queue normal work */
void queue_my_device_work(struct my_device *dev)
{
    atomic_inc(&dev->busy_count);
    queue_work(dev->wq, &dev->normal_work);
}

/* Schedule delayed work */
void schedule_my_device_work(struct my_device *dev)
{
    atomic_inc(&dev->busy_count);
    queue_delayed_work(dev->wq, &dev->delayed_work, 
                      msecs_to_jiffies(1000));
}

/* Flush all work synchronously */
void flush_my_device_work(struct my_device *dev)
{
    /* Wait for all current work to complete */
    flush_workqueue(dev->wq);
    
    /* Alternative approach using wait queue */
    dev->flush_complete = false;
    queue_work(dev->wq, &dev->flush_work);
    wait_event(dev->flush_wait, dev->flush_complete);
}

/* Cleanup */
void cleanup_my_device_work(struct my_device *dev)
{
    cancel_work_sync(&dev->normal_work);
    cancel_delayed_work_sync(&dev->delayed_work);
    destroy_workqueue(dev->wq);
}
```

### Workqueue Creation Flags

The workqueue creation flags control important behaviors:

```c
/* Creates a single worker thread for the workqueue */
my_wq = create_singlethread_workqueue("name");

/* Fully featured workqueue creation with flags */
my_wq = alloc_workqueue("name", flags, max_active);

/* Common flags: */

/* Workers may be created on demand and destroyed when idle */
WQ_UNBOUND

/* Workers are not bound to any specific CPU */
WQ_UNBOUND

/* Memory allocation may cause I/O operations (reclaim) */
WQ_MEM_RECLAIM

/* Allow greater queue depth (normally limited to 10x max_active) */
WQ_HIGHPRI

/* Freezable during system suspend */
WQ_FREEZABLE

/* Per-CPU workqueue instead of global */
WQ_PERCPU
```

## Shared IRQ Handling Patterns

When multiple devices share the same IRQ line, special handling is required:

```c
#include <linux/interrupt.h>

struct my_device {
    void __iomem *registers;   /* Memory-mapped registers */
    int irq;                   /* IRQ number */
    /* ... other fields ... */
};

/* Shared interrupt handler */
static irqreturn_t my_shared_irq_handler(int irq, void *dev_id)
{
    struct my_device *dev = (struct my_device *)dev_id;
    u32 status;
    
    /* Read interrupt status register */
    status = readl(dev->registers + STATUS_REG_OFFSET);
    
    /* Check if it's our interrupt */
    if (!(status & OUR_DEVICE_IRQ_MASK)) {
        /* Not our interrupt */
        return IRQ_NONE;
    }
    
    /* It is our interrupt, handle it */
    
    /* Acknowledge the interrupt */
    writel(OUR_DEVICE_IRQ_MASK, dev->registers + IRQ_CLEAR_OFFSET);
    
    /* Process the interrupt */
    /* ... */
    
    return IRQ_HANDLED;
}

/* Initialize and request shared IRQ */
int setup_shared_irq(struct my_device *dev)
{
    int ret;
    
    ret = request_irq(dev->irq, my_shared_irq_handler,
                    IRQF_SHARED, "my_device", dev);
    if (ret) {
        pr_err("Failed to request shared IRQ %d\n", dev->irq);
        return ret;
    }
    
    return 0;
}
```

## IRQ Affinity and Multicore Handling

The Raspberry Pi 5's quad-core CPU allows for IRQ affinity configuration, which can improve performance by directing specific interrupts to specific cores:

```c
#include <linux/interrupt.h>
#include <linux/cpumask.h>

/* Set IRQ affinity to a specific CPU */
int set_irq_affinity(unsigned int irq, unsigned int cpu)
{
    struct cpumask mask;
    
    /* Create a mask with only the specified CPU */
    cpumask_clear(&mask);
    cpumask_set_cpu(cpu, &mask);
    
    /* Set the IRQ affinity */
    return irq_set_affinity(irq, &mask);
}

/* Distributing IRQs across CPUs in init function */
static int __init my_driver_init(void)
{
    int i;
    int num_cpus = num_online_cpus();
    
    /* Request multiple IRQs for different channels */
    for (i = 0; i < NUM_CHANNELS; i++) {
        int ret;
        
        ret = request_irq(channel_irqs[i], channel_irq_handler,
                        IRQF_SHARED, "my_channel", &channels[i]);
        if (ret)
            goto err_free_irqs;
            
        /* Distribute IRQs across available CPUs */
        set_irq_affinity(channel_irqs[i], i % num_cpus);
    }
    
    return 0;
    
err_free_irqs:
    /* Cleanup on error */
    while (--i >= 0)
        free_irq(channel_irqs[i], &channels[i]);
        
    return ret;
}
```

**Multicore Interrupt Handling**

## Interrupt Performance on Raspberry Pi 5 Quad-Core System

### Cache Considerations
- Each core has its own L1 and L2 cache
- Shared L3 cache (2MB)
- Moving data between cores causes cache coherency traffic
- Keep interrupt processing on the same core when possible

### IRQ Balancing Strategies
1. **Dedicated Core**: Reserve one core for interrupt handling
   - Good for high-frequency interrupts
   - Reduces cache thrashing
   - Other cores can handle user processes

2. **Affinity Matching**: Match IRQ to the core running related processes
   - Good for device drivers with specific user process affinity
   - Example: GPU interrupts handled on same core as graphics processes

3. **Load Spreading**: Distribute IRQs across all cores
   - Good for multiple independent devices
   - Avoids overloading a single core
   - Can use the kernel's IRQ balancer

4. **Time-Critical IRQs**: Assign highest priority IRQs to a dedicated core
   - Audio/video timing-sensitive interrupts
   - Real-time control applications

### Practical Tips
- Use `irq_set_affinity()` to assign IRQs to specific cores
- Monitor interrupt load with `/proc/interrupts`
- Adjust based on actual system measurements
- Consider power management implications (core sleep states)

## Interrupt Mitigation and Rate Limiting

For devices that generate high-frequency interrupts, mitigation techniques can reduce system load:

```c
#include <linux/interrupt.h>
#include <linux/jiffies.h>

struct my_device {
    /* ... other fields ... */
    unsigned long last_irq_time;
    unsigned int irq_count;
    struct delayed_work rate_work;
};

/* Delayed work function for processing batched events */
static void process_batched_events(struct work_struct *work)
{
    struct delayed_work *delayed_work = to_delayed_work(work);
    struct my_device *dev = container_of(delayed_work, 
                                       struct my_device, 
                                       rate_work);
    
    /* Process all accumulated events */
    pr_info("Processing %u batched events\n", dev->irq_count);
    
    /* Reset counter */
    dev->irq_count = 0;
}

/* Rate-limited interrupt handler */
static irqreturn_t rate_limited_irq_handler(int irq, void *dev_id)
{
    struct my_device *dev = (struct my_device *)dev_id;
    unsigned long now = jiffies;
    
    /* Acknowledge the hardware interrupt */
    /* ... */
    
    /* Increment the counter */
    dev->irq_count++;
    
    /* If this is the first interrupt or sufficient time has passed */
    if (dev->irq_count == 1 || 
        time_after(now, dev->last_irq_time + msecs_to_jiffies(50))) {
        
        /* Cancel any pending work */
        cancel_delayed_work(&dev->rate_work);
        
        /* Schedule processing after a delay (coalescing window) */
        queue_delayed_work(system_wq, &dev->rate_work, 
                         msecs_to_jiffies(10));
        
        /* Update timestamp */
        dev->last_irq_time = now;
    }
    
    return IRQ_HANDLED;
}

/* Alternative approach using timer-based throttling */
static irqreturn_t throttled_irq_handler(int irq, void *dev_id)
{
    struct my_device *dev = (struct my_device *)dev_id;
    unsigned long now = jiffies;
    
    /* Acknowledge the interrupt */
    /* ... */
    
    /* Check if we should process this interrupt */
    if (time_after(now, dev->last_irq_time + msecs_to_jiffies(5))) {
        /* Process the interrupt */
        /* ... */
        
        /* Update timestamp */
        dev->last_irq_time = now;
    } else {
        /* Too soon after the last one, just count it */
        dev->irq_count++;
    }
    
    return IRQ_HANDLED;
}
```

## Event and Interrupt Chaining

For more complex interrupt scenarios, you might need to chain handlers or forward events to other subsystems:

```c
#include <linux/interrupt.h>
#include <linux/input.h>

struct my_button_device {
    struct input_dev *input;   /* Input subsystem device */
    int gpio;                  /* GPIO pin */
    int irq;                   /* IRQ number */
    bool state;                /* Current button state */
};

/* GPIO IRQ handler that forwards to input subsystem */
static irqreturn_t button_irq_handler(int irq, void *dev_id)
{
    struct my_button_device *button_dev = dev_id;
    bool new_state;
    
    /* Read the GPIO pin state */
    new_state = gpio_get_value(button_dev->gpio) ? true : false;
    
    /* Only report if state changed (debounced) */
    if (new_state != button_dev->state) {
        /* Report the event to input subsystem */
        input_report_key(button_dev->input, KEY_ENTER, new_state);
        input_sync(button_dev->input);
        
        /* Update stored state */
        button_dev->state = new_state;
    }
    
    return IRQ_HANDLED;
}

/* Initialize the button device with input subsystem */
static int setup_button_device(struct my_button_device *button_dev)
{
    int ret;
    
    /* Allocate input device */
    button_dev->input = input_allocate_device();
    if (!button_dev->input)
        return -ENOMEM;
    
    /* Configure input device */
    button_dev->input->name = "GPIO Button";
    button_dev->input->phys = "gpio/input0";
    button_dev->input->id.bustype = BUS_HOST;
    
    /* Set capabilities */
    set_bit(EV_KEY, button_dev->input->evbit);
    set_bit(KEY_ENTER, button_dev->input->keybit);
    
    /* Register with input subsystem */
    ret = input_register_device(button_dev->input);
    if (ret) {
        input_free_device(button_dev->input);
        return ret;
    }
    
    /* Request GPIO and IRQ */
    ret = gpio_request(button_dev->gpio, "button-gpio");
    if (ret) {
        input_unregister_device(button_dev->input);
        return ret;
    }
    
    ret = gpio_direction_input(button_dev->gpio);
    if (ret) {
        gpio_free(button_dev->gpio);
        input_unregister_device(button_dev->input);
        return ret;
    }
    
    button_dev->irq = gpio_to_irq(button_dev->gpio);
    ret = request_irq(button_dev->irq, button_irq_handler,
                    IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING,
                    "button-irq", button_dev);
    if (ret) {
        gpio_free(button_dev->gpio);
        input_unregister_device(button_dev->input);
        return ret;
    }
    
    /* Read initial state */
    button_dev->state = gpio_get_value(button_dev->gpio) ? true : false;
    
    return 0;
}
```

## Interrupt Handler Debugging Techniques

Debugging interrupt handlers can be challenging due to their asynchronous nature and special execution context. Here are some practical techniques:

```c
#include <linux/interrupt.h>
#include <linux/debugfs.h>
#include <linux/seq_file.h>

/* Statistics structure */
struct irq_stats {
    unsigned long count;           /* Total interrupt count */
    ktime_t last_time;             /* Last interrupt timestamp */
    ktime_t min_interval;          /* Minimum interval between interrupts */
    ktime_t max_interval;          /* Maximum interval between interrupts */
    unsigned long spurious_count;  /* Spurious interrupt count */
    
    /* Histogram of intervals */
    unsigned long intervals[10];   /* Buckets for different intervals */
};

struct my_debug_device {
    int irq;
    struct irq_stats stats;
    struct dentry *debugfs_dir;
    struct dentry *stats_file;
};

/* DebugFS show function */
static int irq_stats_show(struct seq_file *s, void *unused)
{
    struct my_debug_device *dev = s->private;
    int i;
    
    seq_printf(s, "IRQ Statistics for IRQ %d:\n", dev->irq);
    seq_printf(s, "Total interrupts: %lu\n", dev->stats.count);
    seq_printf(s, "Spurious interrupts: %lu\n", dev->stats.spurious_count);
    
    if (dev->stats.count > 0) {
        seq_printf(s, "Last interrupt: %lld ns\n", 
                 ktime_to_ns(dev->stats.last_time));
        seq_printf(s, "Min interval: %lld ns\n", 
                 ktime_to_ns(dev->stats.min_interval));
        seq_printf(s, "Max interval: %lld ns\n", 
                 ktime_to_ns(dev->stats.max_interval));
    }
    
    seq_puts(s, "Interval histogram (ns):\n");
    for (i = 0; i < 10; i++) {
        seq_printf(s, "  %d-%d: %lu\n", 
                 i * 100, (i + 1) * 100, dev->stats.intervals[i]);
    }
    
    return 0;
}

/* DebugFS open function */
static int irq_stats_open(struct inode *inode, struct file *file)
{
    return single_open(file, irq_stats_show, inode->i_private);
}

/* DebugFS file operations */
static const struct file_operations irq_stats_fops = {
    .open = irq_stats_open,
    .read = seq_read,
    .llseek = seq_lseek,
    .release = single_release,
};

/* Debuggable interrupt handler */
static irqreturn_t debug_irq_handler(int irq, void *dev_id)
{
    struct my_debug_device *dev = dev_id;
    ktime_t now = ktime_get();
    ktime_t interval;
    u32 status;
    int bucket;
    
    /* Read status register to check if it's our interrupt */
    status = read_status_register();
    
    if (!(status & OUR_DEVICE_IRQ_MASK)) {
        /* Not our interrupt, it's spurious */
        dev->stats.spurious_count++;
        return IRQ_NONE;
    }
    
    /* Update statistics */
    if (dev->stats.count > 0) {
        interval = ktime_sub(now, dev->stats.last_time);
        
        /* Update min/max */
        if (ktime_to_ns(dev->stats.min_interval) == 0 ||
            ktime_to_ns(interval) < ktime_to_ns(dev->stats.min_interval))
            dev->stats.min_interval = interval;
            
        if (ktime_to_ns(interval) > ktime_to_ns(dev->stats.max_interval))
            dev->stats.max_interval = interval;
            
        /* Update histogram */
        bucket = ktime_to_ns(interval) / 100;
        if (bucket < 10)
            dev->stats.intervals[bucket]++;
    }
    
    dev->stats.count++;
    dev->stats.last_time = now;
    
    /* Actual interrupt handling */
    /* ... */
    
    return IRQ_HANDLED;
}

/* Setup debugging for an IRQ */
static int setup_irq_debugging(struct my_debug_device *dev)
{
    /* Initialize statistics */
    memset(&dev->stats, 0, sizeof(dev->stats));
    dev->stats.min_interval = ktime_set(0, 0);
    dev->stats.max_interval = ktime_set(0, 0);
    
    /* Create debugfs entries */
    dev->debugfs_dir = debugfs_create_dir("my_irq_debug", NULL);
    if (IS_ERR_OR_NULL(dev->debugfs_dir))
        return -ENODEV;
        
    dev->stats_file = debugfs_create_file("statistics", 0444,
                                       dev->debugfs_dir, dev,
                                       &irq_stats_fops);
    if (IS_ERR_OR_NULL(dev->stats_file)) {
        debugfs_remove_recursive(dev->debugfs_dir);
        return -ENODEV;
    }
    
    return 0;
}
```

## Practical GPIO Interrupt Handling on Raspberry Pi 5

// ... rest of existing content ... 

## Additional Resources

For further exploration of interrupt handling and workqueues in the Linux kernel, consider these resources:

1. **Books and Documentation**
   - "Linux Kernel Development" by Robert Love (chapters on Interrupts and Bottom Halves)
   - "Essential Linux Device Drivers" by Sreekrishnan Venkateswaran
   - [Linux Kernel Documentation: Core API](https://www.kernel.org/doc/html/latest/core-api/index.html)

2. **ARM and Raspberry Pi Specific**
   - [ARM GIC Architecture Specification](https://developer.arm.com/documentation/ihi0069/latest/)
   - [BCM2712 Technical Reference Manual](https://datasheets.raspberrypi.com/bcm2712/bcm2712-peripherals.pdf)
   - [Raspberry Pi Linux Kernel Source](https://github.com/raspberrypi/linux)
   - [Raspberry Pi Hardware Documentation](https://www.raspberrypi.com/documentation/computers/processors.html)

3. **Tutorials and Examples**
   - [Free Electrons/Bootlin Kernel and Driver Development Course](https://bootlin.com/training/kernel/)
   - [Linux Foundation Training: Linux Kernel Internals and Development](https://training.linuxfoundation.org/training/linux-kernel-internals-and-development/)
   - [Embedded Linux Conference Presentations](https://elinux.org/Events)

4. **Tools for Interrupt Analysis**
   - [ftrace and trace-cmd](https://www.kernel.org/doc/html/latest/trace/ftrace.html)
   - [perf: Linux profiling tool](https://perf.wiki.kernel.org/index.php/Main_Page)
   - [BCC/eBPF Tools for Linux Performance Analysis](https://github.com/iovisor/bcc)

5. **Real-Time Considerations**
   - [PREEMPT_RT Patch Documentation](https://wiki.linuxfoundation.org/realtime/documentation/start)
   - [Real-Time Linux Wiki](https://wiki.linuxfoundation.org/realtime/start)
   - [Cyclictest for Measuring Interrupt Latency](https://wiki.linuxfoundation.org/realtime/documentation/howto/tools/cyclictest)

These resources will help you deepen your understanding of interrupt handling, hardware interactions, and efficient asynchronous processing in the Linux kernel, particularly on ARM-based platforms like the Raspberry Pi 5. 
