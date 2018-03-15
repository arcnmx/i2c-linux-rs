#![deny(missing_docs)]
#![doc(html_root_url = "http://arcnmx.github.io/i2c-linux-rs/")]

//! A safe interface to the Linux I2C subsystem from userspace.
//!
//! # Example
//!
//! ```rust,no_run
//! # fn main_res() -> ::std::io::Result<()> {
//! let mut i2c = I2c::from_path("/dev/i2c-0")?;
//! i2c.i2c_slave_address(0x50)?;
//! let data = i2c.smbus_read_byte()?;
//! println!("Read I2C data: {}", data);
//! # }
//! # fn main() { main_res().unwrap() }
//! ```

#[macro_use]
extern crate bitflags;
extern crate resize_slice;
extern crate i2c_linux_sys as i2c;

use std::time::Duration;
use std::path::Path;
use std::os::unix::io::{AsRawFd, IntoRawFd, FromRawFd, RawFd};
use std::io::{self, Read, Write};
use std::fs::{File, OpenOptions};
use std::mem;
use resize_slice::ResizeSlice;

pub use i2c::{SmbusReadWrite as ReadWrite, Functionality};

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

bitflags! {
	/// Flags to work around device quirks.
	#[derive(Default)]
	pub struct ReadFlags: u16 {
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
		const STOP = i2c::I2C_M_STOP;
	}
}

/// A safe wrapper around an I2C device.
pub struct I2c<I> {
	inner: I,
	message_buffer: Box<[i2c::i2c_msg; i2c::I2C_RDWR_IOCTL_MAX_MSGS]>,
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
			message_buffer: Box::new(unsafe { mem::zeroed() }),
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

impl<I: AsRawFd> I2c<I> {
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
	pub fn smbus_set_slave_address(&self, address: u16) -> io::Result<()> {
		if address & 0xff00 != 0 {
			i2c::i2c_set_slave_address_10bit(self.as_raw_fd(), true)?
		} else {
			i2c::i2c_set_slave_address_10bit(self.as_raw_fd(), false)?
		}

		i2c::i2c_set_slave_address(self.as_raw_fd(), address, false)
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

	/// Executes a queue of I2C transfers, separated by repeat START conditions.
	/// Data buffers are truncated to the actual read length on completion.
	///
	/// See the `I2C_RDWR` ioctl for more information.
	pub fn i2c_transfer(&mut self, messages: &mut [Message]) -> io::Result<()> {
		assert!(messages.len() <= self.message_buffer.len());

		self.message_buffer.iter_mut().zip(messages.iter_mut())
			.for_each(|(out, msg)| *out = match *msg {
				Message::Read { address, ref mut data, flags } => i2c::i2c_msg {
					addr: address,
					flags: i2c::Flags::from_bits_truncate(flags.bits()),
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
			i2c::i2c_rdwr(self.as_raw_fd(), &mut self.message_buffer[..messages.len()])?;
		};

		self.message_buffer.iter().zip(messages.iter_mut())
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

	/// Deads a single byte from a device without specifying a register.
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
		i2c::i2c_smbus_read_block_data(self.as_raw_fd(), command, value)
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
		i2c::i2c_smbus_block_process_call(self.as_raw_fd(), command, write, read)
	}

	/// Reads a block of bytes from the designated device register.
	///
	/// Unlike smbus_read_block_data this does not receive a data length.
	pub fn i2c_read_block_data(&mut self, command: u8, value: &mut [u8]) -> io::Result<usize> {
		i2c::i2c_smbus_read_i2c_block_data(self.as_raw_fd(), command, value)
	}

	/// Writes a block of bytes from the designated device register.
	///
	/// Unlike smbus_write_block_data this does not transfer the data length.
	pub fn i2c_write_block_data(&mut self, command: u8, value: &[u8]) -> io::Result<()> {
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
