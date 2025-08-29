// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! Bindings for the GZVM (Geniezone Hypervisor) API.

#![cfg(any(target_os = "android", target_os = "linux"))]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(target_arch = "aarch64")]
pub mod aarch64 {
    pub mod bindings;
    use base::ioctl_io_nr;
    use base::ioctl_iow_nr;
    use base::ioctl_iowr_nr;
    pub use bindings::*;

    ioctl_io_nr!(APDVIRT_CREATE_VM, APDVIRT_IOC_MAGIC, 0x01);
    ioctl_io_nr!(APDVIRT_CHECK_EXTENSION, APDVIRT_IOC_MAGIC, 0x03);
    ioctl_io_nr!(APDVM_CREATE_VCPU, APDVIRT_IOC_MAGIC, 0x41);
    // IRQCHIP is for gicv2 only in ARM64, support in next phase
    //ioctl_io_nr!(APDVM_CREATE_IRQCHIP, APDVIRT_IOC_MAGIC, 0x60);
    ioctl_io_nr!(APDVM_RUN, APDVIRT_IOC_MAGIC, 0x80);

    ioctl_iow_nr!(
        APDVIRT__SET_MEMORY_REGION,
        APDVIRT_IOC_MAGIC,
        0x40,
        gzvm_memory_region
    );

    ioctl_iow_nr!(
        APDVM_SET_USER_MEMORY_REGION,
        APDVIRT_IOC_MAGIC,
        0x46,
        gzvm_userspace_memory_region
    );

    ioctl_iow_nr!(APDVM_IRQ_LINE, APDVIRT_IOC_MAGIC, 0x61, apdvm_irq_level);
    // ioctl_iow_nr!(GZVM_IRQFD, APDVIRT_IOC_MAGIC, 0x76, gzvm_irqfd);
    // ioctl_iow_nr!(GZVM_IOEVENTFD, APDVIRT_IOC_MAGIC, 0x79, gzvm_ioeventfd);
    //ioctl_iow_nr!(APDVM_ENABLE_CAP, APDVIRT_IOC_MAGIC, 0xa3, gzvm_enable_cap);
    ioctl_iow_nr!(APDVM_GET_ONE_REG, APDVIRT_IOC_MAGIC, 0xab, apdvm_one_reg);
    ioctl_iow_nr!(APDVM_SET_ONE_REG, APDVIRT_IOC_MAGIC, 0xac, apdvm_one_reg);
    ioctl_iowr_nr!(APDVM_CREATE_DEVICE, APDVIRT_IOC_MAGIC, 0xe0, apdvm_create_device);
    //ioctl_iow_nr!(APDVM_SET_DTB_CONFIG, APDVIRT_IOC_MAGIC, 0xff, gzvm_dtb_config);
}

#[cfg(target_arch = "aarch64")]
pub use aarch64::*;
