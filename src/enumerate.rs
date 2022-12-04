//! I2c device enumeration via [udev].

use {
    crate::I2c,
    std::{fs::File, io, path::Path},
    udev,
};

/// Enumerates all available i2c devices on the system.
#[derive(Clone)]
pub struct Enumerator {
    inner: udev::Enumerator,
}

impl Enumerator {
    /// Create a new enumerator for available i2c devices.
    pub fn new() -> io::Result<Self> {
        let en = udev::Enumerator::new()?;
        Self::with_udev_enumerator(en)
    }

    /// Manually construct a new enumerator.
    pub fn with_udev_enumerator(mut inner: udev::Enumerator) -> io::Result<Self> {
        inner.match_subsystem("i2c-dev")?;

        Ok(Enumerator { inner })
    }

    /// Iterate over i2c devices.
    pub fn iter(&self) -> io::Result<impl Iterator<Item = EnumeratedDevice>> {
        self.clone().into_iter().map(|i| i.collect::<Vec<_>>().into_iter())
    }

    /// Iterate over i2c devices.
    pub fn into_iter(&mut self) -> io::Result<DeviceIterator> {
        self.inner.scan_devices().map(|devices| DeviceIterator::new(devices))
    }

    /// Retrieve the inner [udev::Enumerator].
    pub fn into_inner(self) -> udev::Enumerator {
        self.inner
    }
}

/// An iterator over enumerated i2c devices.
///
/// Use [Enumerator::into_iter] to construct this.
pub struct DeviceIterator<'a> {
    inner: udev::Devices<'a>,
}

impl<'a> DeviceIterator<'a> {
    /// Manually construct an i2c device iterator.
    pub fn new(inner: udev::Devices<'a>) -> Self {
        Self { inner }
    }
}

impl<'a> Iterator for DeviceIterator<'a> {
    type Item = EnumeratedDevice;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(EnumeratedDevice::new)
    }
}

/// An enumerated i2c device.
pub struct EnumeratedDevice {
    device: udev::Device,
}

impl EnumeratedDevice {
    /// Manually construct an enumerated i2c device.
    pub fn new(device: udev::Device) -> Self {
        Self { device }
    }

    /// I2c device information.
    pub fn device(&self) -> &udev::Device {
        &self.device
    }

    /// The path to the device node of the i2c device, if it exists.
    pub fn path(&self) -> Option<&Path> {
        self.device.devnode()
    }

    /// Open a new handle to the i2c device.
    pub fn open(&self) -> io::Result<I2c<File>> {
        self.path()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "i2c device missing devnode path"))
            .and_then(I2c::from_path)
    }
}
