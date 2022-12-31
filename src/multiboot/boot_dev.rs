pub struct BootDevice {
	typ: TagType,
	size: u32,
	biosdev: BiosDev,
	partition: u32,
	sub_partition: u32
}

#[repr(u32)]
pub enum BiosDev {
	Floppy1 = 0x0,
	Hdd1 = 0x80
}