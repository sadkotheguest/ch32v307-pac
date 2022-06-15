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

// ---------------------------- USB ----------------------------------
#[doc = "Universal serial bus full-speed device interface"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usb::RegisterBlock = 0x4000_5c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB").finish()
    }
}
#[doc = "Universal serial bus full-speed device interface"]
pub mod usb;

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

// ------------------------- ETH_MAC ---------------------------------
#[doc = "Ethernet: media access control"]
pub struct ETHERNET_MAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETHERNET_MAC {}
impl ETHERNET_MAC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ethernet_mac::RegisterBlock = 0x4002_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ethernet_mac::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ETHERNET_MAC {
    type Target = ethernet_mac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Ethernet: media access control"]
pub mod ethernet_mac;

// ------------------------- ETH_MMC --------------------------------
#[doc = "Ethernet: MAC management counters"]
pub struct ETHERNET_MMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETHERNET_MMC {}
impl ETHERNET_MMC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ethernet_mmc::RegisterBlock = 0x4002_8100 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ethernet_mmc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ETHERNET_MMC {
    type Target = ethernet_mmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Ethernet: MAC management counters"]
pub mod ethernet_mmc;

// ------------------------- ETH_PTP --------------------------------
#[doc = "Ethernet: Precision time protocol"]
pub struct ETHERNET_PTP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETHERNET_PTP {}
impl ETHERNET_PTP {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ethernet_ptp::RegisterBlock = 0x4002_8700 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ethernet_ptp::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ETHERNET_PTP {
    type Target = ethernet_ptp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Ethernet: Precision time protocol"]
pub mod ethernet_ptp;

// ------------------------- ETH_DMA --------------------------------
#[doc = "Ethernet: DMA controller operation"]
pub struct ETHERNET_DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETHERNET_DMA {}
impl ETHERNET_DMA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ethernet_dma::RegisterBlock = 0x4002_9000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ethernet_dma::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ETHERNET_DMA {
    type Target = ethernet_dma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ETHERNET_DMA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETHERNET_DMA").finish()
    }
}
#[doc = "Ethernet: DMA controller operation"]
pub mod ethernet_dma;

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

// --------------------------- FSMC ----------------------------------
#[doc = "Flexible static memory controller"]
pub struct FSMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FSMC {}
impl FSMC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const fsmc::RegisterBlock = 0xa000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fsmc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for FSMC {
    type Target = fsmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Flexible static memory controller"]
pub mod fsmc;

// ---------------------------- DVP ----------------------------------
#[doc = "Digital Video Port"]
pub struct DVP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DVP {}
impl DVP {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dvp::RegisterBlock = 0x5005_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dvp::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DVP {
    type Target = dvp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Digital Video Port"]
pub mod dvp;

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

// ------------------- Advanced timers -----------------------------
#[doc = "Advanced timer"]
pub struct TIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM1 {}
impl TIM1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tim1::RegisterBlock = 0x4001_2c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIM1 {
    type Target = tim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Advanced timer"]
pub mod tim1;
//-----
#[doc = "Advanced timer"]
pub struct TIM8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM8 {}
impl TIM8 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tim1::RegisterBlock = 0x4000_0400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIM8 {
    type Target = tim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Advanced timer"]
pub use tim1 as tim8;
//-----
#[doc = "Advanced timer"]
pub struct TIM9 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM9 {}
impl TIM9 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tim1::RegisterBlock = 0x4001_4c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIM9 {
    type Target = tim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Advanced timer"]
pub use tim1 as tim9;
//-----
#[doc = "Advanced timer"]
pub struct TIM10 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM10 {}
impl TIM10 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tim1::RegisterBlock = 0x4001_5000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIM10 {
    type Target = tim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Advanced timer"]
pub use tim1 as tim10;

// ---------------- General purpose timers -------------------------
#[doc = "General purpose timer"]
pub struct TIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM2 {}
impl TIM2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tim2::RegisterBlock = 0x4000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim2::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIM2 {
    type Target = tim2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIM2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM2").finish()
    }
}
#[doc = "General purpose timer"]
pub mod tim2;
//-----
#[doc = "General purpose timer"]
pub struct TIM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM3 {}
impl TIM3 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tim2::RegisterBlock = 0x4000_0400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim2::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIM3 {
    type Target = tim2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "General purpose timer"]
pub use tim2 as tim3;
//-----
#[doc = "General purpose timer"]
pub struct TIM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM4 {}
impl TIM4 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tim2::RegisterBlock = 0x4000_0800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim2::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIM4 {
    type Target = tim2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "General purpose timer"]
pub use tim2 as tim4;
//-----
#[doc = "General purpose timer"]
pub struct TIM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM5 {}
impl TIM5 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tim2::RegisterBlock = 0x4000_0c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim2::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIM5 {
    type Target = tim2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "General purpose timer"]
pub use tim2 as tim5;

// -------------------------- I2C ----------------------------------
#[doc = "Inter integrated circuit"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2c1::RegisterBlock = 0x4000_5400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Inter integrated circuit"]
pub mod i2c1;

#[doc = "Inter integrated circuit"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2c1::RegisterBlock = 0x4000_5800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2C2 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Inter integrated circuit"]
pub use i2c1 as i2c2;

// -------------------------- SPI1 ----------------------------------
#[doc = "Serial peripheral interface"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi1::RegisterBlock = 0x4001_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Serial peripheral interface"]
pub mod spi1;
//-----
#[doc = "Serial peripheral interface"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi2::RegisterBlock = 0x4000_3800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi2::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI2 {
    type Target = spi2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI2").finish()
    }
}
#[doc = "Serial peripheral interface"]
pub mod spi2;
//-----
#[doc = "Serial peripheral interface"]
pub struct SPI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI3 {}
impl SPI3 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi2::RegisterBlock = 0x4000_3c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi2::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI3 {
    type Target = spi2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI3").finish()
    }
}
#[doc = "Serial peripheral interface"]
pub use spi2 as spi3;

// -------------------------- USART --------------------------------
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usart1::RegisterBlock = 0x4001_3800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod usart1;
//-----
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usart1::RegisterBlock = 0x4000_4400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USART2 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub use usart1 as usart2;
//-----
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usart1::RegisterBlock = 0x4000_4800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USART3 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub use usart1 as usart3;
//-----
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct UART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART4 {}
impl UART4 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usart1::RegisterBlock = 0x4000_4c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART4 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART4").finish()
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub use usart1 as uart4;
//-----
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct UART5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART5 {}
impl UART5 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usart1::RegisterBlock = 0x4000_5000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART5 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub use usart1 as uart5;
//-----
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct UART6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART6 {}
impl UART6 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usart1::RegisterBlock = 0x4000_1800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART6 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub use usart1 as uart6;
//-----
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct UART7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART7 {}
impl UART7 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usart1::RegisterBlock = 0x4000_1c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART7 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub use usart1 as uart7;
//-----
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct UART8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART8 {}
impl UART8 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usart1::RegisterBlock = 0x4000_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART8 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub use usart1 as uart8;

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
    #[doc = "USB"]
    pub USB: USB,
    #[doc = "CAN1"]
    pub CAN1: CAN1,
    #[doc = "CAN2"]
    pub CAN2: CAN2,
    #[doc = "ETHERNET_MAC"]
    pub ETHERNET_MAC: ETHERNET_MAC,
    #[doc = "ETHERNET_MMC"]
    pub ETHERNET_MMC: ETHERNET_MMC,
    #[doc = "ETHERNET_PTP"]
    pub ETHERNET_PTP: ETHERNET_PTP,
    #[doc = "ETHERNET_DMA"]
    pub ETHERNET_DMA: ETHERNET_DMA,
    #[doc = "SDIO"]
    pub SDIO: SDIO,
    #[doc = "FSMC"]
    pub FSMC: FSMC,
    #[doc = "DVP"]
    pub DVP: DVP,
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
    #[doc = "TIM1"]
    pub TIM1: TIM1,
    #[doc = "TIM8"]
    pub TIM8: TIM8,
    #[doc = "TIM9"]
    pub TIM9: TIM9,
    #[doc = "TIM10"]
    pub TIM10: TIM10,
    #[doc = "TIM2"]
    pub TIM2: TIM2,
    #[doc = "TIM3"]
    pub TIM3: TIM3,
    #[doc = "TIM4"]
    pub TIM4: TIM4,
    #[doc = "TIM5"]
    pub TIM5: TIM5,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "SPI3"]
    pub SPI3: SPI3,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART3"]
    pub USART3: USART3,
    #[doc = "UART4"]
    pub UART4: UART4,
    #[doc = "UART5"]
    pub UART5: UART5,
    #[doc = "UART6"]
    pub UART6: UART6,
    #[doc = "UART7"]
    pub UART7: UART7,
    #[doc = "UART8"]
    pub UART8: UART8,
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
            USB: USB {
                _marker: PhantomData,
            },
            CAN1: CAN1 {
                _marker: PhantomData,
            },
            CAN2: CAN2 {
                _marker: PhantomData,
            },
            ETHERNET_MAC: ETHERNET_MAC {
                _marker: PhantomData,
            },
            ETHERNET_MMC: ETHERNET_MMC {
                _marker: PhantomData,
            },
            ETHERNET_PTP: ETHERNET_PTP {
                _marker: PhantomData,
            },
            ETHERNET_DMA: ETHERNET_DMA {
                _marker: PhantomData,
            },
            SDIO: SDIO {
                _marker: PhantomData,
            },
            FSMC: FSMC {
                _marker: PhantomData,
            },
            DVP: DVP {
                _marker: PhantomData,
            },
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
            TIM1: TIM1 {
                _marker: PhantomData,
            },
            TIM8: TIM8 {
                _marker: PhantomData,
            },
            TIM9: TIM9 {
                _marker: PhantomData,
            },
            TIM10: TIM10 {
                _marker: PhantomData,
            },
            TIM2: TIM2 {
                _marker: PhantomData,
            },
            TIM3: TIM3 {
                _marker: PhantomData,
            },
            TIM4: TIM4 {
                _marker: PhantomData,
            },
            TIM5: TIM5 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            SPI3: SPI3 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            USART3: USART3 {
                _marker: PhantomData,
            },
            UART4: UART4 {
                _marker: PhantomData,
            },
            UART5: UART5 {
                _marker: PhantomData,
            },
            UART6: UART6 {
                _marker: PhantomData,
            },
            UART7: UART7 {
                _marker: PhantomData,
            },
            UART8: UART8 {
                _marker: PhantomData,
            },
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
