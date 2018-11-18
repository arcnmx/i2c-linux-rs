#![deny(missing_docs)]
#![doc(html_root_url = "http://arcnmx.github.io/i2c-linux-rs/")]

//! A safe interface to the Linux I2C and SMBus userspace subsystem.
//!
//! # Example
//!
//! ```rust,no_run
//! extern crate i2c_linux;
//! use i2c_linux::I2c;
//!
//! # fn main_res() -> ::std::io::Result<()> {
//! let mut i2c = I2c::from_path("/dev/i2c-0")?;
//! i2c.smbus_set_slave_address(0x50, false)?;
//! let data = i2c.smbus_read_byte()?;
//! println!("Read I2C data: {}", data);
//! # Ok(())
//! # }
//! # fn main() { main_res().unwrap() }
//! ```
//!
//! # Cargo Features
//!
//! - `i2c` will impl [i2c](https://crates.io/crates/i2c) traits for `I2c`.
//! - `udev` must be enabled to use `Enumerator`.

#[macro_use]
extern crate bitflags;
extern crate resize_slice;
extern crate i2c_linux_sys as i2c;
#[cfg(feature = "i2c")]
extern crate i2c as i2c_gen;
#[cfg(feature = "udev")]
extern crate udev;

use std::time::Duration;
use std::path::Path;
use std::os::unix::io::{AsRawFd, IntoRawFd, FromRawFd, RawFd};
use std::io::{self, Read, Write};
use std::fs::{File, OpenOptions};
use std::{mem, cmp, iter};
use resize_slice::ResizeSlice;

pub use i2c::{SmbusReadWrite as ReadWrite, Functionality};

#[cfg(feature = "udev")]
mod enumerate;

#[cfg(feature = "udev")]
pub use enumerate::Enumerator;

#[cfg(feature = "i2c")]
mod i2c_impl;

/// Part of a combined I2C transaction.
pub enum Message<'a> {
    /// I2C read command
    Read {
        /// The slave address of the device to read from.
        address: u16,
        /// A data buffer to read into.
        data: &'a mut [u8],
        /// Additional flags can modify the operation to work around device quirks.
        flags: ReadFlags,
    },
    /// I2C write command
    Write {
        /// The slave address of the device to write to.
        address: u16,
        /// The data to write.
        data: &'a [u8],
        /// Additional flags can modify the operation to work around device quirks.
        flags: WriteFlags,
    },
}

impl<'a> Message<'a> {
    /// Byte length of the message data buffer.
    pub fn len(&self) -> usize {
        match *self {
            Message::Read { ref data, .. } => data.len(),
            Message::Write { ref data, .. } => data.len(),
        }
    }

    /// Address of the message's slave.
    pub fn address(&self) -> u16 {
        match *self {
            Message::Read { address, .. } => address,
            Message::Write { address, .. } => address,
        }
    }
}

bitflags! {
    /// Flags to work around device quirks.
    #[derive(Default)]
    pub struct ReadFlags: u16 {
        /// This is a 10-bit chip address.
        const TENBIT_ADDR = i2c::I2C_M_TEN;
        /// The first received byte will indicate the remaining length of the transfer.
        const RECEIVE_LEN = i2c::I2C_M_RECV_LEN;
        /// NACK bit is generated for this read.
        ///
        /// Requires `Functionality::PROTOCOL_MANGLING`
        const NACK = i2c::I2C_M_NO_RD_ACK;
        /// Flips the meaning of the read/write address bit for misbehaving devices.
        ///
        /// Requires `Functionality::PROTOCOL_MANGLING`
        const REVERSE_RW = i2c::I2C_M_REV_DIR_ADDR;
        /// Do not generate a START condition or the address start byte. When
        /// used for the first message, a START condition is still generated.
        ///
        /// This can be used to combine multiple buffers into a single I2C transfer,
        /// usually without a direction change.
        ///
        /// Requires `Functionality::NO_START`
        const NO_START = i2c::I2C_M_NOSTART;
        /// Force a STOP condition after this message.
        ///
        /// Requires `Functionality::PROTOCOL_MANGLING`
        const STOP = i2c::I2C_M_STOP;
    }
}

bitflags! {
    /// Flags to work around device quirks.
    #[derive(Default)]
    pub struct WriteFlags: u16 {
        /// This is a 10-bit chip address.
        const TENBIT_ADDR = i2c::I2C_M_TEN;
        /// Treat NACK as an ACK and prevent it from interrupting the transfer.
        ///
        /// Requires `Functionality::PROTOCOL_MANGLING`
        const IGNORE_NACK = i2c::I2C_M_IGNORE_NAK;
        /// Flips the meaning of the read/write address bit for misbehaving devices.
        ///
        /// Requires `Functionality::PROTOCOL_MANGLING`
        const REVERSE_RW = i2c::I2C_M_REV_DIR_ADDR;
        /// Do not generate a START condition or the address start byte. When
        /// used for the first message, a START condition is still generated.
        ///
        /// This can be used to combine multiple buffers into a single I2C transfer,
        /// usually without a direction change.
        ///
        /// Requires `Functionality::NO_START`
        const NO_START = i2c::I2C_M_NOSTART;
        /// Force a STOP condition after this message.
        ///
        /// Requires `Functionality::PROTOCOL_MANGLING`
        const STOP = i2c::I2C_M_STOP;
    }
}

/// A safe wrapper around an I2C device.
pub struct I2c<I> {
    inner: I,
    address: Option<u16>,
    address_10bit: bool,
    functionality: Option<Functionality>,
}

impl I2c<File> {
    /// Open an I2C device
    pub fn from_path<P: AsRef<Path>>(p: P) -> io::Result<Self> {
        OpenOptions::new().read(true).write(true)
            .open(p).map(Self::new)
    }
}

impl<I> I2c<I> {
    /// Creates a new I2C handle with the given file descriptor
    pub fn new(device: I) -> Self {
        I2c {
            inner: device,
            address: None,
            address_10bit: false,
            functionality: None,
        }
    }

    /// Consumes the I2C handle to return the inner file descriptor.
    pub fn into_inner(self) -> I {
        self.inner
    }

    /// Borrows the inner file descriptor.
    pub fn inner_ref(&self) -> &I {
        &self.inner
    }

    /// Mutably borrows the inner file descriptor.
    pub fn inner_mut(&mut self) -> &mut I {
        &mut self.inner
    }
}

impl<I: AsRawFd> AsRawFd for I2c<I> {
    fn as_raw_fd(&self) -> RawFd {
        self.inner.as_raw_fd()
    }
}

impl<I: IntoRawFd> IntoRawFd for I2c<I> {
    fn into_raw_fd(self) -> RawFd {
        self.inner.into_raw_fd()
    }
}

impl FromRawFd for I2c<File> {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::new(File::from_raw_fd(fd))
    }
}

// TODO: add assertions for block lengths, return a proper io::Error
impl<I: AsRawFd> I2c<I> {
    fn update_functionality(&mut self) -> Option<Functionality> {
        if let Some(func) = self.functionality.clone() {
            Some(func)
        } else {
            let functionality = self.i2c_functionality().ok();
            self.functionality = functionality.clone();
            functionality
        }
    }

    /// Sets the number of times to retry communication before failing.
    pub fn i2c_set_retries(&self, value: usize) -> io::Result<()> {
        i2c::i2c_set_retries(self.as_raw_fd(), value)
    }

    /// Sets a timeout for I2C operations
    pub fn i2c_set_timeout(&self, duration: Duration) -> io::Result<()> {
        let value = duration.as_secs() as usize * 1000 + duration.subsec_nanos() as usize / 1000000;
        i2c::i2c_set_timeout_ms(self.as_raw_fd(), value as _)
    }

    /// Set the slave address to communicate with.
    pub fn smbus_set_slave_address(&mut self, address: u16, tenbit: bool) -> io::Result<()> {
        if let Some(func) = self.update_functionality() {
            if func.contains(Functionality::TENBIT_ADDR) || tenbit {
                i2c::i2c_set_slave_address_10bit(self.as_raw_fd(), tenbit)?;
            }
        }

        let res = i2c::i2c_set_slave_address(self.as_raw_fd(), address, false);

        if res.is_ok() {
            self.address = Some(address);
            self.address_10bit = tenbit;
        }

        res
    }

    /// Enable or disable SMBus Packet Error Checking.
    pub fn smbus_set_pec(&self, pec: bool) -> io::Result<()> {
        i2c::i2c_pec(self.as_raw_fd(), pec)
    }

    /// Retrieve the capabilities of the I2C device. These should be checked
    /// before attempting to use certain SMBus commands or I2C flags.
    pub fn i2c_functionality(&self) -> io::Result<Functionality> {
        i2c::i2c_get_functionality(self.as_raw_fd())
    }

    /// `i2c_transfer` capabilities of the I2C device. These should be checked
    /// before attempting to use any of the protocol mangling flags.
    pub fn i2c_transfer_flags(&self) -> io::Result<(ReadFlags, WriteFlags)> {
        let func = self.i2c_functionality()?;
        let (mut read, mut write) = (ReadFlags::empty(), WriteFlags::empty());
        if func.contains(Functionality::PROTOCOL_MANGLING) {
            read.set(ReadFlags::NACK, true);
            read.set(ReadFlags::REVERSE_RW, true);
            read.set(ReadFlags::STOP, true);
            write.set(WriteFlags::IGNORE_NACK, true);
            write.set(WriteFlags::REVERSE_RW, true);
            write.set(WriteFlags::STOP, true);
        }
        if func.contains(Functionality::NO_START) {
            read.set(ReadFlags::NO_START, true);
            write.set(WriteFlags::NO_START, true);
        }
        if func.contains(Functionality::TENBIT_ADDR) {
            read.set(ReadFlags::TENBIT_ADDR, true);
            write.set(WriteFlags::TENBIT_ADDR, true);
        }
        Ok((read, write))
    }

    /// Executes a queue of I2C transfers, separated by repeat START conditions.
    /// Data buffers are truncated to the actual read length on completion.
    ///
    /// See the `I2C_RDWR` ioctl for more information.
    pub fn i2c_transfer(&mut self, messages: &mut [Message]) -> io::Result<()> {
        let mut message_buffer: [i2c::i2c_msg; i2c::I2C_RDWR_IOCTL_MAX_MSGS] = unsafe {
            mem::uninitialized()
        };
        assert!(messages.len() <= message_buffer.len());

        message_buffer.iter_mut().zip(messages.iter_mut())
            .for_each(|(out, msg)| *out = match *msg {
                Message::Read { address, ref mut data, flags } => i2c::i2c_msg {
                    addr: address,
                    flags: i2c::Flags::from_bits_truncate(flags.bits()) | i2c::Flags::RD,
                    len: data.len() as _,
                    buf: data.as_mut_ptr(),
                },
                Message::Write { address, ref data, flags } => i2c::i2c_msg {
                    addr: address,
                    flags: i2c::Flags::from_bits_truncate(flags.bits()),
                    len: data.len() as _,
                    buf: data.as_ptr() as *mut _,
                },
            });

        let res = unsafe {
            i2c::i2c_rdwr(self.as_raw_fd(), &mut message_buffer[..messages.len()])?;
        };

        message_buffer.iter().zip(messages.iter_mut())
            .for_each(|(msg, out)| match *out {
                Message::Read { ref mut data, .. } => data.resize_to(msg.len as usize),
                Message::Write { .. } => (),
            });

        Ok(res)
    }

    /// Sends a single bit to the device, in the place of the Rd/Wr address bit.
    pub fn smbus_write_quick(&mut self, value: ReadWrite) -> io::Result<()> {
        i2c::i2c_smbus_write_quick(self.as_raw_fd(), value)
    }

    /// Reads a single byte from a device without specifying a register.
    ///
    /// Some devices are so simple that this interface is enough; for others, it
    /// is a shorthand if you want to read the same register as in the previous
    /// SMBus command.
    pub fn smbus_read_byte(&mut self) -> io::Result<u8> {
        i2c::i2c_smbus_read_byte(self.as_raw_fd())
    }

    /// Sends a single byte to a device.
    pub fn smbus_write_byte(&mut self, value: u8) -> io::Result<()> {
        i2c::i2c_smbus_write_byte(self.as_raw_fd(), value)
    }

    /// Reads a single byte from a device from the designated register.
    pub fn smbus_read_byte_data(&mut self, command: u8) -> io::Result<u8> {
        i2c::i2c_smbus_read_byte_data(self.as_raw_fd(), command)
    }

    /// Writes a single byte to a device to the designated register.
    pub fn smbus_write_byte_data(&mut self, command: u8, value: u8) -> io::Result<()> {
        i2c::i2c_smbus_write_byte_data(self.as_raw_fd(), command, value)
    }

    /// Reads a 16-bit word from the device register.
    pub fn smbus_read_word_data(&mut self, command: u8) -> io::Result<u16> {
        i2c::i2c_smbus_read_word_data(self.as_raw_fd(), command)
    }

    /// Writes a 16-bit word to the device register.
    pub fn smbus_write_word_data(&mut self, command: u8, value: u16) -> io::Result<()> {
        i2c::i2c_smbus_write_word_data(self.as_raw_fd(), command, value)
    }

    /// Selects a device register, sends a 16-bit word to it, and read 16-bits
    /// of data in return.
    pub fn smbus_process_call(&mut self, command: u8, value: u16) -> io::Result<u16> {
        i2c::i2c_smbus_process_call(self.as_raw_fd(), command, value)
    }

    /// Read up to 32 bytes from the designated device register.
    ///
    /// Returns the amount of data read.
    pub fn smbus_read_block_data(&mut self, command: u8, value: &mut [u8]) -> io::Result<usize> {
        let len = cmp::min(value.len(), i2c::I2C_SMBUS_BLOCK_MAX);
        i2c::i2c_smbus_read_block_data(self.as_raw_fd(), command, &mut value[..len])
    }

    /// Write up to 32 bytes to the designated device register.
    pub fn smbus_write_block_data(&mut self, command: u8, value: &[u8]) -> io::Result<()> {
        i2c::i2c_smbus_write_block_data(self.as_raw_fd(), command, value)
    }

    /// Sends up to 31 bytes of data to the designated device register, and reads
    /// up to 31 bytes in return.
    ///
    /// This was introduced in SMBus 2.0
    pub fn smbus_block_process_call(&mut self, command: u8, write: &[u8], read: &mut [u8]) -> io::Result<usize> {
        let read_len = cmp::min(read.len(), i2c::I2C_SMBUS_BLOCK_MAX);
        i2c::i2c_smbus_block_process_call(self.as_raw_fd(), command, write, &mut read[..read_len])
    }

    /// Reads a block of bytes from the designated device register.
    ///
    /// Unlike smbus_read_block_data this does not receive a data length. This
    /// is limited to 32 bytes due to the use of the Linux SMBus interface. Use
    /// `i2c_transfer()` if more data is needed. `write()`+`read()` may also be
    /// an option, though will produce an I2C STOP condition between the
    /// transfers, which may be undesirable.
    pub fn i2c_read_block_data(&mut self, command: u8, value: &mut [u8]) -> io::Result<usize> {
        // Compatibility/emulation
        if let Some(func) = self.update_functionality() {
            if !func.contains(Functionality::SMBUS_READ_I2C_BLOCK) || value.len() > i2c::I2C_SMBUS_BLOCK_MAX {
                if func.contains(Functionality::I2C) {
                    if let Some(address) = self.address {
                        let mut msgs = [
                            Message::Write {
                                address: address,
                                data: &[command],
                                flags: if self.address_10bit { WriteFlags::TENBIT_ADDR } else { WriteFlags::default() },
                            },
                            Message::Read {
                                address: address,
                                data: value,
                                flags: if self.address_10bit { ReadFlags::TENBIT_ADDR } else { ReadFlags::default() },
                            },
                        ];
                        return self.i2c_transfer(&mut msgs)
                            .map(|_| msgs[1].len())
                    }
                }
            }
        }

        let len = cmp::min(value.len(), i2c::I2C_SMBUS_BLOCK_MAX);
        i2c::i2c_smbus_read_i2c_block_data(self.as_raw_fd(), command, &mut value[..len])
    }

    /// Writes a block of bytes from the designated device register.
    ///
    /// Unlike smbus_write_block_data this does not transfer the data length.
    /// This is limited to 32 bytes due to the use of the Linux SMBus interface.
    /// Use `i2c_transfer()` or `write()` instead if more data is needed.
    pub fn i2c_write_block_data(&mut self, command: u8, value: &[u8]) -> io::Result<()> {
        // Compatibility/emulation
        if let Some(func) = self.update_functionality() {
            if !func.contains(Functionality::SMBUS_WRITE_I2C_BLOCK) || value.len() > i2c::I2C_SMBUS_BLOCK_MAX {
                if func.contains(Functionality::I2C) {
                    if let Some(address) = self.address {
                        let flags = if self.address_10bit { WriteFlags::TENBIT_ADDR } else { WriteFlags::default() };
                        return if func.contains(Functionality::NO_START) {
                            self.i2c_transfer(&mut [
                                Message::Write {
                                    address: address,
                                    data: &[command],
                                    flags: flags,
                                },
                                Message::Write {
                                    address: address,
                                    data: value,
                                    flags: flags | WriteFlags::NO_START,
                                },
                            ])
                        } else {
                            self.i2c_transfer(&mut [
                                Message::Write {
                                    address: address,
                                    data: &iter::once(command).chain(value.iter().cloned()).collect::<Vec<_>>(),
                                    flags: flags,
                                },
                            ])
                        }
                    } else {
                        // could also just use i2c_transfer, not much difference
                    }
                }
            }
        }

        i2c::i2c_smbus_write_i2c_block_data(self.as_raw_fd(), command, value)
    }
}

impl<I: Read> Read for I2c<I> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<I: Write> Write for I2c<I> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
