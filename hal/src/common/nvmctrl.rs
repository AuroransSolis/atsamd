use crate::target_device::NVMCTRL;

static mut NVMCTRL: bool = false;

macro_rules! memdef {
    ($($item:item);+) => {
        $(
            $item;
        )+
    }
}

#[cfg(any(
    feature = "samd11x",
    feature = "samd21x",
    feature = "samd51x",
    feature = "same51x",
    feature = "same53x",
    feature = "same54x"
))]
pub const FLASH_START: usize = 0x20000000;

#[cfg(any(feature = "samd21xb", feature = "samd21xc", feature = "samd21xd", feature = "samd21xl"))]
pub const RWW_START: usize = 0x00400000;

#[cfg(any(feature = "samd11x", feature = "samd21x"))]
pub const PAGE_SIZE: usize = 64;

#[cfg(any(feature = "samd51x", feature = "same51x", feature = "same53x", feature = "same54x"))]
pub const PAGE_SIZE: usize = 512;

#[cfg(and(feature = "samd11x", feature = "samd11x14"))]
pub const FLASH_PAGES: usize = 256;

#[cfg(and(feature = "samd21x", feature = "samd21x18"))]
pub const FLASH_PAGES: usize = 4096;

#[cfg(and(feature = "samd21x", feature = "samd21x17"))]
pub const FLASH_PAGES: usize = 2048;

#[cfg(and(
    any(
        feature = "samd21xb",
        feature = "samd21xc",
        feature = "samd21xd",
        feature = "samd21xl"
    ),
    feature = "samd21x17"
))]
pub const RWW_PAGES: usize = 64;

#[cfg(and(feature = "samd21x", feature = "samd21x16"))]
pub const FLASH_PAGES: usize = 1024;

#[cfg(and(
    any(
        feature = "samd21xb",
        feature = "samd21xc",
        feature = "samd21xd",
        feature = "samd21xl"
    ),
    feature = "samd21x16"
))]
pub const RWW_PAGES: usize = 32;

#[cfg(and(feature = "samd21x", feature = "samd21x15"))]
pub const FLASH_PAGES: usize = 512;

#[cfg(and(
    any(
        feature = "samd21xb",
        feature = "samd21xc",
        feature = "samd21xd",
        feature = "samd21xl"
    ),
    feature = "samd21x15"
))]
pub const RWW_PAGES: usize = 16;

#[cfg(any(
    feature = "samd11x",
    feature = "samd21xa",
))]
memdef! {
    pub struct NvmCtrl {
        nvmctrl: NVMCTRL,
        mem: &mut [[u16; PAGE_SIZE]; FLASH_PAGES],
        pagebuf: [[u16; PAGE_SIZE]; 4],
    }

    impl NvmCtrl {
        pub fn take(nvmctrl: NVMCTRL) -> Option<Self> {
            if unsafe { NVMCTRL } {
                None
            } else {
                Some(unsafe { NvmCtrl::steal(nvmctrl) })
            }
        }

        pub unsafe fn steal(nvmctrl: NVMCTRL) -> Self {
            NVMCTRL = true;
            NvmCtrl {
                nvmctrl,
                mem: &mut *(FLASH_START as *mut u8 as *mut [[u8; PAGE_SIZE]; FLASH_PAGES])
            }
        }
    }
}

#[cfg(any(feature = "samd21xb", feature = "samd21xc", feature = "samd21xd", feature = "samd21xl"))]
memdef! {
    pub struct NvmCtrl {
        nvmctrl: NVMCTRL,
        mem: &mut [[u8; PAGE_SIZE]; FLASH_PAGES],
        rww: &mut [[u8; RWW_PAGE_SIZE]; RWW_PAGES],
    }

    impl NvmCtrl {
        pub fn take(nvmctrl: NVMCTRL) -> Option<Self> {
            if unsafe { NVMCTRL } {
                None
            } else {
                Some(unsafe { NvmCtrl::steal(nvmctrl) })
            }
        }

        pub unsafe fn steal(nvmctrl: NVMCTRL) -> Self {
            NVMCTRL = true;
            NvmCtrl {
                nvmctrl,
                mem: &mut *(FLASH_START as *mut u8 as *mut [[u8; PAGE_SIZE]; FLASH_PAGES])
                rww: &mut *(RWW_START as *mut u8 as *mut [[u8; RWW_PAGE_SIZE]; RWW_PAGES])
            }
        }
    }
}
