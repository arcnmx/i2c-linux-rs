use std::os::unix::io::AsRawFd;
use std::io;
use std::mem::MaybeUninit;
use resize_slice::ResizeSlice;
use i2c::{ReadFlags as I2cReadFlags, WriteFlags as I2cWriteFlags};
use i2c_linux_sys::{Flags, i2c_msg, i2c_rdwr, I2C_RDWR_IOCTL_MAX_MSGS};
use super::{I2c, ReadFlags, WriteFlags, ReadWrite};

impl<I: AsRawFd> i2c::Master for I2c<I> {
    type Error = io::Error;
}

impl<I: AsRawFd> i2c::Address for I2c<I> {
    fn set_slave_address(&mut self, addr: u16, tenbit: bool) -> Result<(), Self::Error> {
        I2c::smbus_set_slave_address(self, addr, tenbit)
    }
}

impl<I: AsRawFd> i2c::Smbus for I2c<I> {
    fn smbus_write_quick(&mut self, value: bool) -> Result<(), Self::Error> {
        I2c::smbus_write_quick(self, if value { ReadWrite::Read } else { ReadWrite::Write })
    }

    fn smbus_read_byte(&mut self) -> Result<u8, Self::Error> {
        I2c::smbus_read_byte(self)
    }

    fn smbus_write_byte(&mut self, value: u8) -> Result<(), Self::Error> {
        I2c::smbus_write_byte(self, value)
    }

    fn smbus_read_byte_data(&mut self, command: u8) -> Result<u8, Self::Error> {
        I2c::smbus_read_byte_data(self, command)
    }

    fn smbus_write_byte_data(&mut self, command: u8, value: u8) -> Result<(), Self::Error> {
        I2c::smbus_write_byte_data(self, command, value)
    }

    fn smbus_read_word_data(&mut self, command: u8) -> Result<u16, Self::Error> {
        I2c::smbus_read_word_data(self, command)
    }

    fn smbus_write_word_data(&mut self, command: u8, value: u16) -> Result<(), Self::Error> {
        I2c::smbus_write_word_data(self, command, value)
    }

    fn smbus_process_call(&mut self, command: u8, value: u16) -> Result<u16, Self::Error> {
        I2c::smbus_process_call(self, command, value)
    }

    fn smbus_read_block_data(&mut self, command: u8, value: &mut [u8]) -> Result<usize, Self::Error> {
        I2c::smbus_read_block_data(self, command, value)
    }

    fn smbus_write_block_data(&mut self, command: u8, value: &[u8]) -> Result<(), Self::Error> {
        I2c::smbus_write_block_data(self, command, value)
    }
}

impl<I: AsRawFd> i2c::Smbus20 for I2c<I> {
    fn smbus_process_call_block(&mut self, command: u8, write: &[u8], read: &mut [u8]) -> Result<usize, Self::Error> {
        I2c::smbus_block_process_call(self, command, write, read)
    }
}

impl<I: AsRawFd> i2c::SmbusPec for I2c<I> {
    fn smbus_set_pec(&mut self, pec: bool) -> Result<(), Self::Error> {
        I2c::smbus_set_pec(self, pec)
    }
}

impl<I: AsRawFd> i2c::BlockTransfer for I2c<I> {
    fn i2c_read_block_data(&mut self, command: u8, value: &mut [u8]) -> Result<usize, Self::Error> {
        I2c::i2c_read_block_data(self, command, value)
    }

    fn i2c_write_block_data(&mut self, command: u8, value: &[u8]) -> Result<(), Self::Error> {
        I2c::i2c_write_block_data(self, command, value)
    }
}

impl<I: AsRawFd> i2c::BulkTransfer for I2c<I> {
    fn i2c_transfer_support(&mut self) -> Result<(i2c::ReadFlags, i2c::WriteFlags), Self::Error> {
        I2c::i2c_transfer_flags(self).map(|(read, write)| (read.into(), write.into()))
    }

    fn i2c_transfer(&mut self, messages: &mut [i2c::Message]) -> Result<(), Self::Error> {
        let mut message_buffer = [MaybeUninit::<i2c_msg>::uninit(); I2C_RDWR_IOCTL_MAX_MSGS];
        assert!(messages.len() <= message_buffer.len());

        for (out, msg) in message_buffer.iter_mut().zip(messages.iter_mut()) {
            out.write(match *msg {
                i2c::Message::Read { address, ref mut data, flags } => i2c_msg {
                    addr: address,
                    flags: Flags::from_bits_truncate(ReadFlags::from(flags).bits()) | Flags::RD,
                    len: data.len() as _,
                    buf: data.as_mut_ptr(),
                },
                i2c::Message::Write { address, ref data, flags } => i2c_msg {
                    addr: address,
                    flags: Flags::from_bits_truncate(WriteFlags::from(flags).bits()),
                    len: data.len() as _,
                    buf: data.as_ptr() as *mut _,
                },
            });
        }
        let messages_raw = unsafe {
            crate::transmute_slice_mut(&mut message_buffer[..messages.len()])
        };

        let res = unsafe {
            i2c_rdwr(self.as_raw_fd(), messages_raw)?;
        };

        for (msg, out) in messages_raw.iter().zip(messages.iter_mut()) {
            match *out {
                i2c::Message::Read { ref mut data, .. } => data.resize_to(msg.len as usize),
                i2c::Message::Write { .. } => (),
            }
        }

        Ok(res)
    }
}

macro_rules! impl_flags {
    ($my:ident >< $f:ident => { $($tt:tt)* }) => {
        impl_flags! { @impl $my >< $f => { $($tt)* } }
        impl_flags! { @impl $f >< $my => { $($tt)* } }
    };
    (@impl $my:ident >< $f:ident => {
        $($flag:ident,)*
    }) => {
        impl From<$my> for $f {
            fn from(f: $my) -> Self {
                let mut out = Self::empty();
                $(
                if f.contains($my::$flag) {
                    out.set($f::$flag, true);
                }
                )*
                out
            }
        }
    };
}

impl_flags! { ReadFlags >< I2cReadFlags => {
    RECEIVE_LEN,
    NACK,
    REVERSE_RW,
    NO_START,
    STOP,
} }

impl_flags! { WriteFlags >< I2cWriteFlags => {
    IGNORE_NACK,
    REVERSE_RW,
    NO_START,
    STOP,
} }
