var N = null;var searchIndex = {};
searchIndex["i2c_linux"]={"doc":"A safe interface to the Linux I2C and SMBus userspace subsystem.","items":[[4,"ReadWrite","i2c_linux","`i2c_smbus_xfer` read or write markers",N,N],[13,"Read","","",0,N],[13,"Write","","",0,N],[3,"Functionality","","To determine what functionality is present",N,N],[3,"Enumerator","","Enumerates all available i2c devices on the system.",N,N],[3,"ReadFlags","","Flags to work around device quirks.",N,N],[3,"WriteFlags","","Flags to work around device quirks.",N,N],[3,"I2c","","A safe wrapper around an I2C device.",N,N],[4,"Message","","Part of a combined I2C transaction.",N,N],[13,"Read","","I2C read command",1,N],[12,"address","i2c_linux::Message","The slave address of the device to read from.",1,N],[12,"data","","A data buffer to read into.",1,N],[12,"flags","","Additional flags can modify the operation to work around device quirks.",1,N],[13,"Write","i2c_linux","I2C write command",1,N],[12,"address","i2c_linux::Message","The slave address of the device to write to.",1,N],[12,"data","","The data to write.",1,N],[12,"flags","","Additional flags can modify the operation to work around device quirks.",1,N],[11,"new","i2c_linux","Create a new enumerator for available displays.",2,[[],["result"]]],[11,"next","","",2,[[["self"]],["option"]]],[11,"set_slave_address","","",3,[[["self"],["u16"],["bool"]],["result"]]],[11,"smbus_write_quick","","",3,[[["self"],["bool"]],["result"]]],[11,"smbus_read_byte","","",3,[[["self"]],["result",["u8"]]]],[11,"smbus_write_byte","","",3,[[["self"],["u8"]],["result"]]],[11,"smbus_read_byte_data","","",3,[[["self"],["u8"]],["result",["u8"]]]],[11,"smbus_write_byte_data","","",3,[[["self"],["u8"],["u8"]],["result"]]],[11,"smbus_read_word_data","","",3,[[["self"],["u8"]],["result",["u16"]]]],[11,"smbus_write_word_data","","",3,[[["self"],["u8"],["u16"]],["result"]]],[11,"smbus_process_call","","",3,[[["self"],["u8"],["u16"]],["result",["u16"]]]],[11,"smbus_read_block_data","","",3,N],[11,"smbus_write_block_data","","",3,N],[11,"smbus_process_call_block","","",3,N],[11,"smbus_set_pec","","",3,[[["self"],["bool"]],["result"]]],[11,"i2c_read_block_data","","",3,N],[11,"i2c_write_block_data","","",3,N],[11,"i2c_transfer_support","","",3,[[["self"]],["result"]]],[11,"i2c_transfer","","",3,N],[11,"from","","",4,[[["i2creadflags"]],["self"]]],[11,"from","","",5,[[["i2cwriteflags"]],["self"]]],[11,"len","","Byte length of the message data buffer.",1,[[["self"]],["usize"]]],[11,"address","","Address of the message's slave.",1,[[["self"]],["u16"]]],[11,"eq","","",4,[[["self"],["readflags"]],["bool"]]],[11,"ne","","",4,[[["self"],["readflags"]],["bool"]]],[11,"clone","","",4,[[["self"]],["readflags"]]],[11,"partial_cmp","","",4,[[["self"],["readflags"]],["option",["ordering"]]]],[11,"lt","","",4,[[["self"],["readflags"]],["bool"]]],[11,"le","","",4,[[["self"],["readflags"]],["bool"]]],[11,"gt","","",4,[[["self"],["readflags"]],["bool"]]],[11,"ge","","",4,[[["self"],["readflags"]],["bool"]]],[11,"cmp","","",4,[[["self"],["readflags"]],["ordering"]]],[11,"hash","","",4,N],[11,"default","","",4,[[],["readflags"]]],[11,"fmt","","",4,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",4,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",4,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",4,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",4,[[["self"],["formatter"]],["result"]]],[18,"TENBIT_ADDR","","This is a 10-bit chip address.",4,N],[18,"RECEIVE_LEN","","The first received byte will indicate the remaining length of the transfer.",4,N],[18,"NACK","","NACK bit is generated for this read.",4,N],[18,"REVERSE_RW","","Flips the meaning of the read/write address bit for misbehaving devices.",4,N],[18,"NO_START","","Do not generate a START condition or the address start byte. When used for the first message, a START condition is still generated.",4,N],[18,"STOP","","Force a STOP condition after this message.",4,N],[11,"empty","","Returns an empty set of flags.",4,[[],["readflags"]]],[11,"all","","Returns the set containing all flags.",4,[[],["readflags"]]],[11,"bits","","Returns the raw value of the flags currently stored.",4,[[["self"]],["u16"]]],[11,"from_bits","","Convert from underlying bit representation, unless that representation contains bits that do not correspond to a flag.",4,[[["u16"]],["option",["readflags"]]]],[11,"from_bits_truncate","","Convert from underlying bit representation, dropping any bits that do not correspond to flags.",4,[[["u16"]],["readflags"]]],[11,"is_empty","","Returns `true` if no flags are currently stored.",4,[[["self"]],["bool"]]],[11,"is_all","","Returns `true` if all flags are currently set.",4,[[["self"]],["bool"]]],[11,"intersects","","Returns `true` if there are flags common to both `self` and `other`.",4,[[["self"],["readflags"]],["bool"]]],[11,"contains","","Returns `true` all of the flags in `other` are contained within `self`.",4,[[["self"],["readflags"]],["bool"]]],[11,"insert","","Inserts the specified flags in-place.",4,[[["self"],["readflags"]]]],[11,"remove","","Removes the specified flags in-place.",4,[[["self"],["readflags"]]]],[11,"toggle","","Toggles the specified flags in-place.",4,[[["self"],["readflags"]]]],[11,"set","","Inserts or removes the specified flags depending on the passed value.",4,[[["self"],["readflags"],["bool"]]]],[11,"bitor","","Returns the union of the two sets of flags.",4,[[["self"],["readflags"]],["readflags"]]],[11,"bitor_assign","","Adds the set of flags.",4,[[["self"],["readflags"]]]],[11,"bitxor","","Returns the left flags, but with all the right flags toggled.",4,[[["self"],["readflags"]],["readflags"]]],[11,"bitxor_assign","","Toggles the set of flags.",4,[[["self"],["readflags"]]]],[11,"bitand","","Returns the intersection between the two sets of flags.",4,[[["self"],["readflags"]],["readflags"]]],[11,"bitand_assign","","Disables all flags disabled in the set.",4,[[["self"],["readflags"]]]],[11,"sub","","Returns the set difference of the two sets of flags.",4,[[["self"],["readflags"]],["readflags"]]],[11,"sub_assign","","Disables all flags enabled in the set.",4,[[["self"],["readflags"]]]],[11,"not","","Returns the complement of this set of flags.",4,[[["self"]],["readflags"]]],[11,"extend","","",4,[[["self"],["t"]]]],[11,"from_iter","","",4,[[["t"]],["readflags"]]],[11,"eq","","",5,[[["self"],["writeflags"]],["bool"]]],[11,"ne","","",5,[[["self"],["writeflags"]],["bool"]]],[11,"clone","","",5,[[["self"]],["writeflags"]]],[11,"partial_cmp","","",5,[[["self"],["writeflags"]],["option",["ordering"]]]],[11,"lt","","",5,[[["self"],["writeflags"]],["bool"]]],[11,"le","","",5,[[["self"],["writeflags"]],["bool"]]],[11,"gt","","",5,[[["self"],["writeflags"]],["bool"]]],[11,"ge","","",5,[[["self"],["writeflags"]],["bool"]]],[11,"cmp","","",5,[[["self"],["writeflags"]],["ordering"]]],[11,"hash","","",5,N],[11,"default","","",5,[[],["writeflags"]]],[11,"fmt","","",5,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",5,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",5,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",5,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",5,[[["self"],["formatter"]],["result"]]],[18,"TENBIT_ADDR","","This is a 10-bit chip address.",5,N],[18,"IGNORE_NACK","","Treat NACK as an ACK and prevent it from interrupting the transfer.",5,N],[18,"REVERSE_RW","","Flips the meaning of the read/write address bit for misbehaving devices.",5,N],[18,"NO_START","","Do not generate a START condition or the address start byte. When used for the first message, a START condition is still generated.",5,N],[18,"STOP","","Force a STOP condition after this message.",5,N],[11,"empty","","Returns an empty set of flags.",5,[[],["writeflags"]]],[11,"all","","Returns the set containing all flags.",5,[[],["writeflags"]]],[11,"bits","","Returns the raw value of the flags currently stored.",5,[[["self"]],["u16"]]],[11,"from_bits","","Convert from underlying bit representation, unless that representation contains bits that do not correspond to a flag.",5,[[["u16"]],["option",["writeflags"]]]],[11,"from_bits_truncate","","Convert from underlying bit representation, dropping any bits that do not correspond to flags.",5,[[["u16"]],["writeflags"]]],[11,"is_empty","","Returns `true` if no flags are currently stored.",5,[[["self"]],["bool"]]],[11,"is_all","","Returns `true` if all flags are currently set.",5,[[["self"]],["bool"]]],[11,"intersects","","Returns `true` if there are flags common to both `self` and `other`.",5,[[["self"],["writeflags"]],["bool"]]],[11,"contains","","Returns `true` all of the flags in `other` are contained within `self`.",5,[[["self"],["writeflags"]],["bool"]]],[11,"insert","","Inserts the specified flags in-place.",5,[[["self"],["writeflags"]]]],[11,"remove","","Removes the specified flags in-place.",5,[[["self"],["writeflags"]]]],[11,"toggle","","Toggles the specified flags in-place.",5,[[["self"],["writeflags"]]]],[11,"set","","Inserts or removes the specified flags depending on the passed value.",5,[[["self"],["writeflags"],["bool"]]]],[11,"bitor","","Returns the union of the two sets of flags.",5,[[["self"],["writeflags"]],["writeflags"]]],[11,"bitor_assign","","Adds the set of flags.",5,[[["self"],["writeflags"]]]],[11,"bitxor","","Returns the left flags, but with all the right flags toggled.",5,[[["self"],["writeflags"]],["writeflags"]]],[11,"bitxor_assign","","Toggles the set of flags.",5,[[["self"],["writeflags"]]]],[11,"bitand","","Returns the intersection between the two sets of flags.",5,[[["self"],["writeflags"]],["writeflags"]]],[11,"bitand_assign","","Disables all flags disabled in the set.",5,[[["self"],["writeflags"]]]],[11,"sub","","Returns the set difference of the two sets of flags.",5,[[["self"],["writeflags"]],["writeflags"]]],[11,"sub_assign","","Disables all flags enabled in the set.",5,[[["self"],["writeflags"]]]],[11,"not","","Returns the complement of this set of flags.",5,[[["self"]],["writeflags"]]],[11,"extend","","",5,[[["self"],["t"]]]],[11,"from_iter","","",5,[[["t"]],["writeflags"]]],[11,"from_path","","Open an I2C device",3,[[["p"]],["result"]]],[11,"new","","Creates a new I2C handle with the given file descriptor",3,[[["i"]],["self"]]],[11,"into_inner","","Consumes the I2C handle to return the inner file descriptor.",3,[[["self"]],["i"]]],[11,"inner_ref","","Borrows the inner file descriptor.",3,[[["self"]],["i"]]],[11,"inner_mut","","Mutably borrows the inner file descriptor.",3,[[["self"]],["i"]]],[11,"as_raw_fd","","",3,[[["self"]],["rawfd"]]],[11,"into_raw_fd","","",3,[[["self"]],["rawfd"]]],[11,"from_raw_fd","","",3,[[["rawfd"]],["self"]]],[11,"i2c_set_retries","","Sets the number of times to retry communication before failing.",3,[[["self"],["usize"]],["result"]]],[11,"i2c_set_timeout","","Sets a timeout for I2C operations",3,[[["self"],["duration"]],["result"]]],[11,"smbus_set_slave_address","","Set the slave address to communicate with.",3,[[["self"],["u16"],["bool"]],["result"]]],[11,"smbus_set_pec","","Enable or disable SMBus Packet Error Checking.",3,[[["self"],["bool"]],["result"]]],[11,"i2c_functionality","","Retrieve the capabilities of the I2C device. These should be checked before attempting to use certain SMBus commands or I2C flags.",3,[[["self"]],["result",["functionality"]]]],[11,"i2c_transfer_flags","","`i2c_transfer` capabilities of the I2C device. These should be checked before attempting to use any of the protocol mangling flags.",3,[[["self"]],["result"]]],[11,"i2c_transfer","","Executes a queue of I2C transfers, separated by repeat START conditions. Data buffers are truncated to the actual read length on completion.",3,N],[11,"smbus_write_quick","","Sends a single bit to the device, in the place of the Rd/Wr address bit.",3,[[["self"],["readwrite"]],["result"]]],[11,"smbus_read_byte","","Reads a single byte from a device without specifying a register.",3,[[["self"]],["result",["u8"]]]],[11,"smbus_write_byte","","Sends a single byte to a device.",3,[[["self"],["u8"]],["result"]]],[11,"smbus_read_byte_data","","Reads a single byte from a device from the designated register.",3,[[["self"],["u8"]],["result",["u8"]]]],[11,"smbus_write_byte_data","","Writes a single byte to a device to the designated register.",3,[[["self"],["u8"],["u8"]],["result"]]],[11,"smbus_read_word_data","","Reads a 16-bit word from the device register.",3,[[["self"],["u8"]],["result",["u16"]]]],[11,"smbus_write_word_data","","Writes a 16-bit word to the device register.",3,[[["self"],["u8"],["u16"]],["result"]]],[11,"smbus_process_call","","Selects a device register, sends a 16-bit word to it, and read 16-bits of data in return.",3,[[["self"],["u8"],["u16"]],["result",["u16"]]]],[11,"smbus_read_block_data","","Read up to 32 bytes from the designated device register.",3,N],[11,"smbus_write_block_data","","Write up to 32 bytes to the designated device register.",3,N],[11,"smbus_block_process_call","","Sends up to 31 bytes of data to the designated device register, and reads up to 31 bytes in return.",3,N],[11,"i2c_read_block_data","","Reads a block of bytes from the designated device register.",3,N],[11,"i2c_write_block_data","","Writes a block of bytes from the designated device register.",3,N],[11,"read","","",3,N],[11,"write","","",3,N],[11,"flush","","",3,[[["self"]],["result"]]],[11,"to_owned","","",0,[[["self"]],["t"]]],[11,"clone_into","","",0,N],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[["self"]],["u"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"try_into","","",0,[[["self"]],["result"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"get_type_id","","",0,[[["self"]],["typeid"]]],[11,"bitor","","Returns the union of the two sets of flags.",6,[[["self"],["functionality"]],["functionality"]]],[11,"cmp","","",6,[[["self"],["functionality"]],["ordering"]]],[11,"cmp","","",0,[[["self"],["smbusreadwrite"]],["ordering"]]],[11,"eq","","",0,[[["self"],["smbusreadwrite"]],["bool"]]],[11,"eq","","",6,[[["self"],["functionality"]],["bool"]]],[11,"ne","","",6,[[["self"],["functionality"]],["bool"]]],[11,"fmt","","",6,[[["self"],["formatter"]],["result",["error"]]]],[11,"fmt","","",0,[[["self"],["formatter"]],["result",["error"]]]],[11,"fmt","","",6,[[["self"],["formatter"]],["result",["error"]]]],[11,"bitxor","","Returns the left flags, but with all the right flags toggled.",6,[[["self"],["functionality"]],["functionality"]]],[11,"bitor_assign","","Adds the set of flags.",6,N],[11,"bitand_assign","","Disables all flags disabled in the set.",6,N],[11,"hash","","",6,N],[11,"hash","","",0,N],[11,"sub_assign","","Disables all flags enabled in the set.",6,N],[11,"not","","Returns the complement of this set of flags.",6,[[["self"]],["functionality"]]],[11,"bitxor_assign","","Toggles the set of flags.",6,N],[11,"fmt","","",6,[[["self"],["formatter"]],["result",["error"]]]],[11,"partial_cmp","","",6,[[["self"],["functionality"]],["option",["ordering"]]]],[11,"lt","","",6,[[["self"],["functionality"]],["bool"]]],[11,"le","","",6,[[["self"],["functionality"]],["bool"]]],[11,"gt","","",6,[[["self"],["functionality"]],["bool"]]],[11,"ge","","",6,[[["self"],["functionality"]],["bool"]]],[11,"partial_cmp","","",0,[[["self"],["smbusreadwrite"]],["option",["ordering"]]]],[11,"extend","","",6,N],[11,"fmt","","",6,[[["self"],["formatter"]],["result",["error"]]]],[11,"from_iter","","",6,[[["t"]],["functionality"]]],[11,"clone","","",0,[[["self"]],["smbusreadwrite"]]],[11,"clone","","",6,[[["self"]],["functionality"]]],[11,"bitand","","Returns the intersection between the two sets of flags.",6,[[["self"],["functionality"]],["functionality"]]],[11,"fmt","","",6,[[["self"],["formatter"]],["result",["error"]]]],[11,"sub","","Returns the set difference of the two sets of flags.",6,[[["self"],["functionality"]],["functionality"]]],[18,"I2C","","Plain i2c-level commands (`I2C_RDWR`)",6,N],[18,"TENBIT_ADDR","","Handles the 10-bit address extensions",6,N],[18,"PROTOCOL_MANGLING","","I2C_M_IGNORE_NAK etc.",6,N],[18,"SMBUS_PEC","","",6,N],[18,"NO_START","","I2C_M_NOSTART",6,N],[18,"SLAVE","","",6,N],[18,"SMBUS_BLOCK_PROC_CALL","","SMBus 2.0",6,N],[18,"SMBUS_QUICK","","",6,N],[18,"SMBUS_READ_BYTE","","",6,N],[18,"SMBUS_WRITE_BYTE","","",6,N],[18,"SMBUS_READ_BYTE_DATA","","",6,N],[18,"SMBUS_WRITE_BYTE_DATA","","",6,N],[18,"SMBUS_READ_WORD_DATA","","",6,N],[18,"SMBUS_WRITE_WORD_DATA","","",6,N],[18,"SMBUS_PROC_CALL","","",6,N],[18,"SMBUS_READ_BLOCK_DATA","","",6,N],[18,"SMBUS_WRITE_BLOCK_DATA","","",6,N],[18,"SMBUS_READ_I2C_BLOCK","","I2C-like block xfer",6,N],[18,"SMBUS_WRITE_I2C_BLOCK","","w/ 1-byte reg. addr.",6,N],[18,"SMBUS_HOST_NOTIFY","","",6,N],[18,"SMBUS_BYTE","","",6,N],[18,"SMBUS_BYTE_DATA","","",6,N],[18,"SMBUS_WORD_DATA","","",6,N],[18,"SMBUS_BLOCK_DATA","","",6,N],[18,"SMBUS_I2C_BLOCK","","",6,N],[18,"SMBUS_EMUL","","",6,N],[11,"empty","","Returns an empty set of flags.",6,[[],["functionality"]]],[11,"all","","Returns the set containing all flags.",6,[[],["functionality"]]],[11,"bits","","Returns the raw value of the flags currently stored.",6,[[["self"]],["u32"]]],[11,"from_bits","","Convert from underlying bit representation, unless that representation contains bits that do not correspond to a flag.",6,[[["u32"]],["option",["functionality"]]]],[11,"from_bits_truncate","","Convert from underlying bit representation, dropping any bits that do not correspond to flags.",6,[[["u32"]],["functionality"]]],[11,"is_empty","","Returns `true` if no flags are currently stored.",6,[[["self"]],["bool"]]],[11,"is_all","","Returns `true` if all flags are currently set.",6,[[["self"]],["bool"]]],[11,"intersects","","Returns `true` if there are flags common to both `self` and `other`.",6,[[["self"],["functionality"]],["bool"]]],[11,"contains","","Returns `true` all of the flags in `other` are contained within `self`.",6,[[["self"],["functionality"]],["bool"]]],[11,"insert","","Inserts the specified flags in-place.",6,N],[11,"remove","","Removes the specified flags in-place.",6,N],[11,"toggle","","Toggles the specified flags in-place.",6,N],[11,"set","","Inserts or removes the specified flags depending on the passed value.",6,N],[11,"to_owned","","",6,[[["self"]],["t"]]],[11,"clone_into","","",6,N],[11,"from","","",6,[[["t"]],["t"]]],[11,"into","","",6,[[["self"]],["u"]]],[11,"try_from","","",6,[[["u"]],["result"]]],[11,"borrow","","",6,[[["self"]],["t"]]],[11,"try_into","","",6,[[["self"]],["result"]]],[11,"borrow_mut","","",6,[[["self"]],["t"]]],[11,"get_type_id","","",6,[[["self"]],["typeid"]]],[11,"from","","",2,[[["t"]],["t"]]],[11,"into_iter","","",2,[[["self"]],["i"]]],[11,"into","","",2,[[["self"]],["u"]]],[11,"try_from","","",2,[[["u"]],["result"]]],[11,"borrow","","",2,[[["self"]],["t"]]],[11,"try_into","","",2,[[["self"]],["result"]]],[11,"borrow_mut","","",2,[[["self"]],["t"]]],[11,"get_type_id","","",2,[[["self"]],["typeid"]]],[11,"to_owned","","",4,[[["self"]],["t"]]],[11,"clone_into","","",4,N],[11,"from","","",4,[[["t"]],["t"]]],[11,"into","","",4,[[["self"]],["u"]]],[11,"try_from","","",4,[[["u"]],["result"]]],[11,"borrow","","",4,[[["self"]],["t"]]],[11,"try_into","","",4,[[["self"]],["result"]]],[11,"borrow_mut","","",4,[[["self"]],["t"]]],[11,"get_type_id","","",4,[[["self"]],["typeid"]]],[11,"to_owned","","",5,[[["self"]],["t"]]],[11,"clone_into","","",5,N],[11,"from","","",5,[[["t"]],["t"]]],[11,"into","","",5,[[["self"]],["u"]]],[11,"try_from","","",5,[[["u"]],["result"]]],[11,"borrow","","",5,[[["self"]],["t"]]],[11,"try_into","","",5,[[["self"]],["result"]]],[11,"borrow_mut","","",5,[[["self"]],["t"]]],[11,"get_type_id","","",5,[[["self"]],["typeid"]]],[11,"from","","",3,[[["t"]],["t"]]],[11,"into","","",3,[[["self"]],["u"]]],[11,"try_from","","",3,[[["u"]],["result"]]],[11,"borrow","","",3,[[["self"]],["t"]]],[11,"try_into","","",3,[[["self"]],["result"]]],[11,"borrow_mut","","",3,[[["self"]],["t"]]],[11,"get_type_id","","",3,[[["self"]],["typeid"]]],[11,"i2c_read","","",3,N],[11,"i2c_write","","",3,N],[11,"from","","",1,[[["t"]],["t"]]],[11,"into","","",1,[[["self"]],["u"]]],[11,"try_from","","",1,[[["u"]],["result"]]],[11,"borrow","","",1,[[["self"]],["t"]]],[11,"try_into","","",1,[[["self"]],["result"]]],[11,"borrow_mut","","",1,[[["self"]],["t"]]],[11,"get_type_id","","",1,[[["self"]],["typeid"]]]],"paths":[[4,"ReadWrite"],[4,"Message"],[3,"Enumerator"],[3,"I2c"],[3,"ReadFlags"],[3,"WriteFlags"],[3,"Functionality"]]};
initSearch(searchIndex);