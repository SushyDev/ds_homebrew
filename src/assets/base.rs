use core::ffi::c_void;

const SPRITE_SIZE: usize = 32 * 32; // 256 bytes for a 16x16 sprite
const BITMAP_SIZE: usize = 256 * 256; // 65536 bytes for a 256x256 bitmap
const PALETTE_SIZE: usize = 256 * 2; // 512 bytes for a 256 color palette

#[repr(align(4))]
pub struct AlignedData<const N: usize> {
    pub data: [u8; N],
}

pub type AlignedBitMap = AlignedData<BITMAP_SIZE>;
pub type AlignedPalette = AlignedData<PALETTE_SIZE>;
pub type AlignedSprite = AlignedData<SPRITE_SIZE>;

impl<const N: usize> AlignedData<N> {
    pub const fn new(data: [u8; N]) -> Self {
        Self { data }
    }

    pub const fn get_data(&self) -> *const u8 {
        self.data.as_ptr()
    }

    pub const fn get_data_c(&self) -> *const c_void {
        self.get_data() as *const c_void
    }

    pub const fn get_len(&self) -> usize {
        self.data.len()
    }

    pub const fn get_len_c(&self) -> usize {
        self.get_len() as usize
    }
}
