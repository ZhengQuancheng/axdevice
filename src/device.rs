use crate::AxVmDeviceConfig;

use alloc::format;
use alloc::sync::Arc;
use alloc::vec::Vec;
use arm_vgic::Vgic;

use axaddrspace::GuestPhysAddr;
use axdevice_base::BaseDeviceOps;
use axdevice_base::EmuDeviceType;
use axdevice_base::VCpuIf;
use axerrno::AxResult;
use axvmconfig::EmulatedDeviceConfig;

/// represent A vm own devices
pub struct AxVmDevices {
    /// emu devices
    emu_devices: Vec<Arc<dyn BaseDeviceOps>>,
    // TODO passthrough devices or other type devices ...
}

/// The implemention for AxVmDevices
impl AxVmDevices {
    /// According AxVmDeviceConfig to init the AxVmDevices
    pub fn new(config: AxVmDeviceConfig) -> Self {
        let mut this = Self {
            emu_devices: Vec::new(),
        };
        info!("skjyidfgs");
        Self::init(&mut this, &config.emu_configs);
        this
    }

    /// According the emu_configs to init every  specific device
    fn init(this: &mut Self, emu_configs: &Vec<EmulatedDeviceConfig>) {
        for config in emu_configs {
            let dev = match EmuDeviceType::from_usize(config.emu_type) {
                // todo call specific initialization function of devcise
                // EmuDeviceType::EmuDeviceTConsole => ,
                EmuDeviceType::EmuDeviceTGicdV2 => Ok(Arc::new(Vgic::new())),
                // EmuDeviceType::EmuDeviceTGPPT => ,
                // EmuDeviceType::EmuDeviceTVirtioBlk => ,
                // EmuDeviceType::EmuDeviceTVirtioNet => ,
                // EmuDeviceType::EmuDeviceTVirtioConsole => ,
                // EmuDeviceType::EmuDeviceTIOMMU => ,
                // EmuDeviceType::EmuDeviceTICCSRE => ,
                // EmuDeviceType::EmuDeviceTSGIR => ,
                // EmuDeviceType::EmuDeviceTGICR => ,
                // EmuDeviceType::EmuDeviceTMeta => ,
                _ => Err(format!(
                    "emu type: {} is still not supported",
                    config.emu_type
                )),
            };
            info!("powuref");
            if let Ok(emu_dev) = dev {
                this.emu_devices.push(emu_dev)
            }
        }
    }

    /// Find specific device by ipa
    pub fn find_dev(&self, ipa: GuestPhysAddr) -> Option<Arc<dyn BaseDeviceOps>> {
        self.emu_devices
            .iter()
            .find(|&dev| dev.address_range().contains(ipa))
            .cloned()
    }

    /// Handle the MMIO read by GuestPhysAddr and data width, return the value of the guest want to read
    pub fn handle_mmio_read(
        &self,
        addr: GuestPhysAddr,
        width: usize,
        vcpu: &dyn VCpuIf,
    ) -> AxResult<usize> {
        if let Some(emu_dev) = self.find_dev(addr) {
            info!(
                "emu: {:?} handler read ipa {:#x}",
                emu_dev.address_range(),
                addr
            );
            return emu_dev.handle_read(addr, width, vcpu);
        }
        panic!("emu_handle: no emul handler for data abort ipa {:#x}", addr);
    }

    /// Handle the MMIO write by GuestPhysAddr, data width and the value need to write, call specific device to write the value
    pub fn handle_mmio_write(
        &self,
        addr: GuestPhysAddr,
        width: usize,
        val: usize,
        vcpu: &dyn VCpuIf,
    ) {
        if let Some(emu_dev) = self.find_dev(addr) {
            info!(
                "emu: {:?} handler write ipa {:#x}",
                emu_dev.address_range(),
                addr
            );
            emu_dev.handle_write(addr, width, val, vcpu);
            return;
        }
        panic!(
            "emu_handler: no emul handler for data abort ipa {:#x}",
            addr
        );
    }
}
