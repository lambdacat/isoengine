// automatically generated by the FlatBuffers compiler, do not modify

use flatbuffers as fb;

#[derive(Clone,Copy)]
#[packed] pub struct AnimInstance {
    t_start: u64,
    duration: u64,
    id: u32,
    repeat: u8,
    __padding0: u8,
    __padding1: u16,
}

impl AnimInstance {
    pub fn new(t_start: u64, duration: u64, id: u32, repeat: bool) -> AnimInstance {
        AnimInstance {
            t_start: fb::Endian::to_le(t_start),
            duration: fb::Endian::to_le(duration),
            id: fb::Endian::to_le(id),
            repeat: fb::Endian::to_le(if repeat { 0u8 } else { 1u8 }),
            __padding0: 0,
            __padding1: 0,
        }
    }

    pub fn t_start(&self) -> u64 { fb::Endian::from_le(self.t_start) }

    pub fn duration(&self) -> u64 { fb::Endian::from_le(self.duration) }

    pub fn id(&self) -> u32 { fb::Endian::from_le(self.id) }

    pub fn repeat(&self) -> bool { fb::Endian::from_le(self.repeat) != 0 }

}
