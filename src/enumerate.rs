use std::fs::File;
use std::io;
use {udev, crate::I2c};

/// Enumerates all available i2c devices on the system.
///
/// # udev dependency
///
/// Requires the `udev` feature enabled to use.
pub struct Enumerator {
    inner: udev::Devices,
}

impl Enumerator {
    /// Create a new enumerator for available displays.
    pub fn new() -> io::Result<Self> {
        let udev = udev::Context::new()?;
        let mut en = udev::Enumerator::new(&udev)?;
        en.match_subsystem("i2c-dev")?;

        Ok(Enumerator {
            inner: en.scan_devices()?,
        })
    }
}

impl Iterator for Enumerator {
    type Item = (I2c<File>, udev::Device);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(dev) = self.inner.next() {
            let i2c = match dev.devnode() {
                Some(devnode) => I2c::from_path(devnode),
                None => continue,
            };

            if let Ok(i2c) = i2c {
                return Some((i2c, dev))
            }
        }

        None
    }
}
