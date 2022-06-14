#![doc = "Peripheral access API for CH32V307 microcontrollers (generated using svd2rust)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate riscv;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[doc(hidden)]
pub mod interrupt;
pub use self::interrupt::Interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;

// --------------------------- TRNG ----------------------------------
#[doc = "Random number generator"]
pub struct TRNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRNG {}
impl TRNG {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const trng::RegisterBlock = 0x4002_3c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const trng::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TRNG {
    type Target = trng::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

#[doc = "Random number generator"]
pub mod trng;

// --------------------------- CAN -----------------------------------
#[doc = "Controller area network CAN1"]
pub struct CAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN1 {}
impl CAN1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const can1::RegisterBlock = 0x4000_6400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CAN1 {
    type Target = can1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Controller area network"]
pub mod can1;

#[doc = "Controller area network CAN2"]
pub struct CAN2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN2 {}
impl CAN2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const can1::RegisterBlock = 0x4000_6400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CAN2 {
    type Target = can1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Controller area network"]
pub use can1 as can2;

// --------------------------- SDIO ----------------------------------
#[doc = "Secure digital input/output interface"]
pub struct SDIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDIO {}
impl SDIO {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sdio::RegisterBlock = 0x4001_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sdio::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SDIO {
    type Target = sdio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Secure digital input/output interface"]
pub mod sdio;

// -------------------------- GPIOA ----------------------------------
#[doc = "General purpose I/O"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpioa::RegisterBlock = 0x4001_0800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "General purpose I/O (GPIO port A)"]
pub mod gpioa;

// -------------------------- AFIO ---------------------------------
#[doc = "Alternate function I/O"]
pub struct AFIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AFIO {}
impl AFIO {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const afio::RegisterBlock = 0x4001_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const afio::RegisterBlock {
        Self::PTR
    }
}
impl Deref for AFIO {
    type Target = afio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Alternate function I/O"]
pub mod afio;

// -------------------------- EXTI ---------------------------------
#[doc = "EXTI"]
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const exti::RegisterBlock = 0x4001_0400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const exti::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "EXTI"]
pub mod exti;

// -------------------------- DMA1 ---------------------------------
#[doc = "DMA1 controller"]
pub struct DMA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA1 {}
impl DMA1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dma1::RegisterBlock = 0x4002_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DMA1 {
    type Target = dma1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "DMA1 controller"]
pub mod dma1;

// -------------------------- DMA2 ---------------------------------
#[doc = "DMA2 controller"]
pub struct DMA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA2 {}
impl DMA2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dma2::RegisterBlock = 0x4002_0400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma2::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DMA2 {
    type Target = dma2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for DMA2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA2").finish()
    }
}

#[doc = "DMA2 controller"]
pub mod dma2;

// -------------------------- RTC ----------------------------------
#[doc = "Real time clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtc::RegisterBlock = 0x4000_2800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Real time clock"]
pub mod rtc;

// -------------------------- DAC ----------------------------------
#[doc = "Digital to analog converter"]
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC {}
impl DAC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dac::RegisterBlock = 0x4000_7400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dac::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Digital to analog converter"]
pub mod dac;

// -------------------------- PWR ----------------------------------
#[doc = "Power control"]
pub struct PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWR {}
impl PWR {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pwr::RegisterBlock = 0x4000_7000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwr::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PWR {
    type Target = pwr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Power control"]
pub mod pwr;

// -------------------------- RCC ----------------------------------
#[doc = "Reset and clock control"]
pub struct RCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCC {}
impl RCC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rcc::RegisterBlock = 0x4002_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rcc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RCC {
    type Target = rcc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Reset and clock control"]
pub mod rcc;

// -------------------------- CRC ----------------------------------
#[doc = "CRC calculation unit"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const crc::RegisterBlock = 0x4002_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "CRC calculation unit"]
pub mod crc;

// ------------------------- FLASH ---------------------------------
#[doc = "FLASH"]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const flash::RegisterBlock = 0x4002_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash::RegisterBlock {
        Self::PTR
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "FLASH"]
pub mod flash;

// -----------------------------------------------------------------

#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "TRNG"]
    pub TRNG: TRNG,
    // #[doc = "USB"]
    // pub USB: USB,
    #[doc = "CAN1"]
    pub CAN1: CAN1,
    #[doc = "CAN2"]
    pub CAN2: CAN2,
    // #[doc = "ETHERNET_MAC"]
    // pub ETHERNET_MAC: ETHERNET_MAC,
    // #[doc = "ETHERNET_MMC"]
    // pub ETHERNET_MMC: ETHERNET_MMC,
    // #[doc = "ETHERNET_PTP"]
    // pub ETHERNET_PTP: ETHERNET_PTP,
    // #[doc = "ETHERNET_DMA"]
    // pub ETHERNET_DMA: ETHERNET_DMA,
    #[doc = "SDIO"]
    pub SDIO: SDIO,
    // #[doc = "FSMC"]
    // pub FSMC: FSMC,
    // #[doc = "DVP"]
    // pub DVP: DVP,
    #[doc = "DAC"]
    pub DAC: DAC,
    #[doc = "PWR"]
    pub PWR: PWR,
    #[doc = "RCC"]
    pub RCC: RCC,
    // #[doc = "EXTEND"]
    // pub EXTEND: EXTEND,
    // #[doc = "OPA"]
    // pub OPA: OPA,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    // #[doc = "GPIOB"]
    // pub GPIOB: GPIOB,
    // #[doc = "GPIOC"]
    // pub GPIOC: GPIOC,
    // #[doc = "GPIOD"]
    // pub GPIOD: GPIOD,
    // #[doc = "GPIOE"]
    // pub GPIOE: GPIOE,
    #[doc = "AFIO"]
    pub AFIO: AFIO,
    #[doc = "EXTI"]
    pub EXTI: EXTI,
    #[doc = "DMA1"]
    pub DMA1: DMA1,
    #[doc = "DMA2"]
    pub DMA2: DMA2,
    #[doc = "RTC"]
    pub RTC: RTC,
    // #[doc = "BKP"]
    // pub BKP: BKP,
    // #[doc = "IWDG"]
    // pub IWDG: IWDG,
    // #[doc = "WWDG"]
    // pub WWDG: WWDG,
    // #[doc = "TIM1"]
    // pub TIM1: TIM1,
    // #[doc = "TIM8"]
    // pub TIM8: TIM8,
    // #[doc = "TIM9"]
    // pub TIM9: TIM9,
    // #[doc = "TIM10"]
    // pub TIM10: TIM10,
    // #[doc = "TIM2"]
    // pub TIM2: TIM2,
    // #[doc = "TIM3"]
    // pub TIM3: TIM3,
    // #[doc = "TIM4"]
    // pub TIM4: TIM4,
    // #[doc = "TIM5"]
    // pub TIM5: TIM5,
    // #[doc = "I2C1"]
    // pub I2C1: I2C1,
    // #[doc = "I2C2"]
    // pub I2C2: I2C2,
    // #[doc = "SPI1"]
    // pub SPI1: SPI1,
    // #[doc = "SPI2"]
    // pub SPI2: SPI2,
    // #[doc = "SPI3"]
    // pub SPI3: SPI3,
    // #[doc = "USART1"]
    // pub USART1: USART1,
    // #[doc = "USART2"]
    // pub USART2: USART2,
    // #[doc = "USART3"]
    // pub USART3: USART3,
    // #[doc = "UART4"]
    // pub UART4: UART4,
    // #[doc = "UART5"]
    // pub UART5: UART5,
    // #[doc = "UART6"]
    // pub UART6: UART6,
    // #[doc = "UART7"]
    // pub UART7: UART7,
    // #[doc = "UART8"]
    // pub UART8: UART8,
    // #[doc = "ADC1"]
    // pub ADC1: ADC1,
    // #[doc = "ADC2"]
    // pub ADC2: ADC2,
    // #[doc = "DBG"]
    // pub DBG: DBG,
    // #[doc = "USBHD"]
    // pub USBHD: USBHD,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    // #[doc = "USBOTGFS"]
    // pub USBOTGFS: USBOTGFS,
    // #[doc = "PFIC"]
    // pub PFIC: PFIC,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        riscv::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            TRNG: TRNG {
                _marker: PhantomData,
            },
            // USB: USB {
            //     _marker: PhantomData,
            // },
            CAN1: CAN1 {
                _marker: PhantomData,
            },
            CAN2: CAN2 {
                _marker: PhantomData,
            },
            // ETHERNET_MAC: ETHERNET_MAC {
            //     _marker: PhantomData,
            // },
            // ETHERNET_MMC: ETHERNET_MMC {
            //     _marker: PhantomData,
            // },
            // ETHERNET_PTP: ETHERNET_PTP {
            //     _marker: PhantomData,
            // },
            // ETHERNET_DMA: ETHERNET_DMA {
            //     _marker: PhantomData,
            // },
            SDIO: SDIO {
                _marker: PhantomData,
            },
            // FSMC: FSMC {
            //     _marker: PhantomData,
            // },
            // DVP: DVP {
            //     _marker: PhantomData,
            // },
            DAC: DAC {
                _marker: PhantomData,
            },
            PWR: PWR {
                _marker: PhantomData,
            },
            RCC: RCC {
                _marker: PhantomData,
            },
            // EXTEND: EXTEND {
            //     _marker: PhantomData,
            // },
            // OPA: OPA {
            //     _marker: PhantomData,
            // },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            // GPIOB: GPIOB {
            //     _marker: PhantomData,
            // },
            // GPIOC: GPIOC {
            //     _marker: PhantomData,
            // },
            // GPIOD: GPIOD {
            //     _marker: PhantomData,
            // },
            // GPIOE: GPIOE {
            //     _marker: PhantomData,
            // },
            AFIO: AFIO {
                _marker: PhantomData,
            },
            EXTI: EXTI {
                _marker: PhantomData,
            },
            DMA1: DMA1 {
                _marker: PhantomData,
            },
            DMA2: DMA2 {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            // BKP: BKP {
            //     _marker: PhantomData,
            // },
            // IWDG: IWDG {
            //     _marker: PhantomData,
            // },
            // WWDG: WWDG {
            //     _marker: PhantomData,
            // },
            // TIM1: TIM1 {
            //     _marker: PhantomData,
            // },
            // TIM8: TIM8 {
            //     _marker: PhantomData,
            // },
            // TIM9: TIM9 {
            //     _marker: PhantomData,
            // },
            // TIM10: TIM10 {
            //     _marker: PhantomData,
            // },
            // TIM2: TIM2 {
            //     _marker: PhantomData,
            // },
            // TIM3: TIM3 {
            //     _marker: PhantomData,
            // },
            // TIM4: TIM4 {
            //     _marker: PhantomData,
            // },
            // TIM5: TIM5 {
            //     _marker: PhantomData,
            // },
            // I2C1: I2C1 {
            //     _marker: PhantomData,
            // },
            // I2C2: I2C2 {
            //     _marker: PhantomData,
            // },
            // SPI1: SPI1 {
            //     _marker: PhantomData,
            // },
            // SPI2: SPI2 {
            //     _marker: PhantomData,
            // },
            // SPI3: SPI3 {
            //     _marker: PhantomData,
            // },
            // USART1: USART1 {
            //     _marker: PhantomData,
            // },
            // USART2: USART2 {
            //     _marker: PhantomData,
            // },
            // USART3: USART3 {
            //     _marker: PhantomData,
            // },
            // UART4: UART4 {
            //     _marker: PhantomData,
            // },
            // UART5: UART5 {
            //     _marker: PhantomData,
            // },
            // UART6: UART6 {
            //     _marker: PhantomData,
            // },
            // UART7: UART7 {
            //     _marker: PhantomData,
            // },
            // UART8: UART8 {
            //     _marker: PhantomData,
            // },
            // ADC1: ADC1 {
            //     _marker: PhantomData,
            // },
            // ADC2: ADC2 {
            //     _marker: PhantomData,
            // },
            // DBG: DBG {
            //     _marker: PhantomData,
            // },
            // USBHD: USBHD {
            //     _marker: PhantomData,
            // },
            CRC: CRC {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            // USBOTGFS: USBOTGFS {
            //     _marker: PhantomData,
            // },
            // PFIC: PFIC {
            //     _marker: PhantomData,
            // },
        }
    }
}
