#[repr(align(4))]
pub struct AlignedBitmap {
    pub data: [u8; 65536],
}

#[repr(align(4))]
pub struct AlignedPalette {
    pub data: [u8; 512],
}

