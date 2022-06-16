#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB base control"]
    pub usbhd_base_ctrl: crate::Reg<usbhd_base_ctrl::USBHD_BASE_CTRL_SPEC>,
    #[doc = "0x01 - USB device/host physical prot control"]
    pub usbhd_udev_ctrl__usbhd_uhost_ctrl:
        crate::Reg<usbhd_udev_ctrl__usbhd_uhost_ctrl::USBHD_UDEV_CTRL__USBHD_UHOST_CTRL_SPEC>,
    #[doc = "0x02 - USB interrupt enable"]
    pub r8_usb_int_en: crate::Reg<r8_usb_int_en::R8_USB_INT_EN_SPEC>,
    #[doc = "0x03 - USB device address"]
    pub r8_usb_dev_ad: crate::Reg<r8_usb_dev_ad::R8_USB_DEV_AD_SPEC>,
    _reserved4: [u8; 0x01],
    #[doc = "0x05 - USB miscellaneous status"]
    pub r8_usb_mis_st: crate::Reg<r8_usb_mis_st::R8_USB_MIS_ST_SPEC>,
    #[doc = "0x06 - USB interrupt flag"]
    pub r8_usb_int_fg: crate::Reg<r8_usb_int_fg::R8_USB_INT_FG_SPEC>,
    #[doc = "0x07 - USB interrupt status"]
    pub r8_usb_int_st: crate::Reg<r8_usb_int_st::R8_USB_INT_ST_SPEC>,
    #[doc = "0x08 - USB receiving length"]
    pub r16_usb_rx_len: crate::Reg<r16_usb_rx_len::R16_USB_RX_LEN_SPEC>,
    _reserved8: [u8; 0x02],
    #[doc = "0x0c - endpoint 4/1 mode"]
    pub r8_uep4_1_mod: crate::Reg<r8_uep4_1_mod::R8_UEP4_1_MOD_SPEC>,
    #[doc = "0x0d - endpoint 2/3 mode;host endpoint mode"]
    pub r8_uep2_3_mod__r8_uh_ep_mod:
        crate::Reg<r8_uep2_3_mod__r8_uh_ep_mod::R8_UEP2_3_MOD__R8_UH_EP_MOD_SPEC>,
    #[doc = "0x0e - endpoint 5/6 mode"]
    pub r8_uep5_6_mod: crate::Reg<r8_uep5_6_mod::R8_UEP5_6_MOD_SPEC>,
    #[doc = "0x0f - endpoint 7 mode"]
    pub r8_uep7_mod: crate::Reg<r8_uep7_mod::R8_UEP7_MOD_SPEC>,
    #[doc = "0x10 - endpoint 0 DMA buffer address"]
    pub r32_uep0_dma: crate::Reg<r32_uep0_dma::R32_UEP0_DMA_SPEC>,
    #[doc = "0x14 - endpoint 1 DMA buffer address"]
    pub r32_uep1_dma: crate::Reg<r32_uep1_dma::R32_UEP1_DMA_SPEC>,
    #[doc = "0x18 - endpoint 2 DMA buffer address;host rx endpoint buffer high address"]
    pub r32_uep2_dma__r32_uh_rx_dma:
        crate::Reg<r32_uep2_dma__r32_uh_rx_dma::R32_UEP2_DMA__R32_UH_RX_DMA_SPEC>,
    #[doc = "0x1c - endpoint 3 DMA buffer address;host tx endpoint buffer high address"]
    pub r32_uep3_dma__r32_uh_tx_dma:
        crate::Reg<r32_uep3_dma__r32_uh_tx_dma::R32_UEP3_DMA__R32_UH_TX_DMA_SPEC>,
    #[doc = "0x20 - endpoint 4 DMA buffer address"]
    pub r32_uep4_dma: crate::Reg<r32_uep4_dma::R32_UEP4_DMA_SPEC>,
    #[doc = "0x24 - endpoint 5 DMA buffer address"]
    pub r32_uep5_dma: crate::Reg<r32_uep5_dma::R32_UEP5_DMA_SPEC>,
    #[doc = "0x28 - endpoint 6 DMA buffer address"]
    pub r32_uep6_dma: crate::Reg<r32_uep6_dma::R32_UEP6_DMA_SPEC>,
    #[doc = "0x2c - endpoint 7 DMA buffer address"]
    pub r32_uep7_dma: crate::Reg<r32_uep7_dma::R32_UEP7_DMA_SPEC>,
    #[doc = "0x30 - endpoint 0 transmittal length"]
    pub r8_uep0_t_len: crate::Reg<r8_uep0_t_len::R8_UEP0_T_LEN_SPEC>,
    _reserved21: [u8; 0x01],
    #[doc = "0x32 - endpoint 0 control"]
    pub r8_uep0_t_ctrl: crate::Reg<r8_uep0_t_ctrl::R8_UEP0_T_CTRL_SPEC>,
    #[doc = "0x33 - endpoint 0 control"]
    pub r8_uep0_r_ctrl: crate::Reg<r8_uep0_r_ctrl::R8_UEP0_R_CTRL_SPEC>,
    #[doc = "0x34 - endpoint 1 transmittal length"]
    pub r8_uep1_t_len: crate::Reg<r8_uep1_t_len::R8_UEP1_T_LEN_SPEC>,
    _reserved24: [u8; 0x01],
    #[doc = "0x36 - endpoint 1 control"]
    pub r8_uep1_t_ctrl___usbhd_uh_setup:
        crate::Reg<r8_uep1_t_ctrl___usbhd_uh_setup::R8_UEP1_T_CTRL___USBHD_UH_SETUP_SPEC>,
    #[doc = "0x37 - endpoint 1 control"]
    pub r8_uep1_r_ctrl: crate::Reg<r8_uep1_r_ctrl::R8_UEP1_R_CTRL_SPEC>,
    #[doc = "0x38 - endpoint 2 transmittal length"]
    pub r8_uep2_t_len__usbhd_uh_ep_pid:
        crate::Reg<r8_uep2_t_len__usbhd_uh_ep_pid::R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC>,
    _reserved27: [u8; 0x01],
    #[doc = "0x3a - endpoint 2 control"]
    pub r8_uep2_t_ctrl: crate::Reg<r8_uep2_t_ctrl::R8_UEP2_T_CTRL_SPEC>,
    #[doc = "0x3b - endpoint 2 control"]
    pub r8_uep2_r_ctrl__usbhd_uh_rx_ctrl:
        crate::Reg<r8_uep2_r_ctrl__usbhd_uh_rx_ctrl::R8_UEP2_R_CTRL__USBHD_UH_RX_CTRL_SPEC>,
    #[doc = "0x3c - endpoint 3 transmittal length"]
    pub r8_uep3_t_len__usbhd_uh_tx_len:
        crate::Reg<r8_uep3_t_len__usbhd_uh_tx_len::R8_UEP3_T_LEN__USBHD_UH_TX_LEN_SPEC>,
    _reserved30: [u8; 0x01],
    #[doc = "0x3e - endpoint 3 control"]
    pub r8_uep3_t_ctrl__usbhd_uh_tx_ctrl:
        crate::Reg<r8_uep3_t_ctrl__usbhd_uh_tx_ctrl::R8_UEP3_T_CTRL__USBHD_UH_TX_CTRL_SPEC>,
    #[doc = "0x3f - endpoint 3 control"]
    pub r8_uep3_r_ctrl_: crate::Reg<r8_uep3_r_ctrl_::R8_UEP3_R_CTRL__SPEC>,
    #[doc = "0x40 - endpoint 4 transmittal length"]
    pub r8_uep4_t_len: crate::Reg<r8_uep4_t_len::R8_UEP4_T_LEN_SPEC>,
    _reserved33: [u8; 0x01],
    #[doc = "0x42 - endpoint 4 control"]
    pub r8_uep4_t_ctrl: crate::Reg<r8_uep4_t_ctrl::R8_UEP4_T_CTRL_SPEC>,
    #[doc = "0x43 - endpoint 4 control"]
    pub r8_uep4_r_ctrl_: crate::Reg<r8_uep4_r_ctrl_::R8_UEP4_R_CTRL__SPEC>,
    #[doc = "0x44 - endpoint 5 transmittal length"]
    pub r8_uep5_t_len: crate::Reg<r8_uep5_t_len::R8_UEP5_T_LEN_SPEC>,
    _reserved36: [u8; 0x01],
    #[doc = "0x46 - endpoint 5 control"]
    pub r8_uep5_t_ctrl: crate::Reg<r8_uep5_t_ctrl::R8_UEP5_T_CTRL_SPEC>,
    #[doc = "0x47 - endpoint 5 control"]
    pub r8_uep5_r_ctrl_: crate::Reg<r8_uep5_r_ctrl_::R8_UEP5_R_CTRL__SPEC>,
    #[doc = "0x48 - endpoint 6 transmittal length"]
    pub r8_uep6_t_len: crate::Reg<r8_uep6_t_len::R8_UEP6_T_LEN_SPEC>,
    _reserved39: [u8; 0x01],
    #[doc = "0x4a - endpoint 6 control"]
    pub r8_uep6_t_ctrl: crate::Reg<r8_uep6_t_ctrl::R8_UEP6_T_CTRL_SPEC>,
    #[doc = "0x4b - endpoint 6 control"]
    pub r8_uep6_r_ctrl_: crate::Reg<r8_uep6_r_ctrl_::R8_UEP6_R_CTRL__SPEC>,
    #[doc = "0x4c - endpoint 7 transmittal length"]
    pub r8_uep7_t_len: crate::Reg<r8_uep7_t_len::R8_UEP7_T_LEN_SPEC>,
    _reserved42: [u8; 0x01],
    #[doc = "0x4e - endpoint 7 control"]
    pub r8_uep7_t_ctrl: crate::Reg<r8_uep7_t_ctrl::R8_UEP7_T_CTRL_SPEC>,
    #[doc = "0x4f - endpoint 7 control"]
    pub r8_uep7_r_ctrl_: crate::Reg<r8_uep7_r_ctrl_::R8_UEP7_R_CTRL__SPEC>,
    _reserved44: [u8; 0x04],
    #[doc = "0x54 - usb otg control"]
    pub usb_otg_cr: crate::Reg<usb_otg_cr::USB_OTG_CR_SPEC>,
    #[doc = "0x58 - usb otg status"]
    pub usb_otg_sr: crate::Reg<usb_otg_sr::USB_OTG_SR_SPEC>,
}

#[doc = "USBHD_BASE_CTRL register accessor: an alias for `Reg<USBHD_BASE_CTRL_SPEC>`"]
pub type USBHD_BASE_CTRL = crate::Reg<usbhd_base_ctrl::USBHD_BASE_CTRL_SPEC>;
#[doc = "USB base control"]
pub mod usbhd_base_ctrl;

#[doc = "USBHD_UDEV_CTRL__USBHD_UHOST_CTRL register accessor: an alias for `Reg<USBHD_UDEV_CTRL__USBHD_UHOST_CTRL_SPEC>`"]
pub type USBHD_UDEV_CTRL__USBHD_UHOST_CTRL =
    crate::Reg<usbhd_udev_ctrl__usbhd_uhost_ctrl::USBHD_UDEV_CTRL__USBHD_UHOST_CTRL_SPEC>;
#[doc = "USB device/host physical prot control"]
pub mod usbhd_udev_ctrl__usbhd_uhost_ctrl;

#[doc = "R8_USB_INT_EN register accessor: an alias for `Reg<R8_USB_INT_EN_SPEC>`"]
pub type R8_USB_INT_EN = crate::Reg<r8_usb_int_en::R8_USB_INT_EN_SPEC>;
#[doc = "USB interrupt enable"]
pub mod r8_usb_int_en;

#[doc = "R8_USB_DEV_AD register accessor: an alias for `Reg<R8_USB_DEV_AD_SPEC>`"]
pub type R8_USB_DEV_AD = crate::Reg<r8_usb_dev_ad::R8_USB_DEV_AD_SPEC>;
#[doc = "USB device address"]
pub mod r8_usb_dev_ad;

#[doc = "R8_USB_MIS_ST register accessor: an alias for `Reg<R8_USB_MIS_ST_SPEC>`"]
pub type R8_USB_MIS_ST = crate::Reg<r8_usb_mis_st::R8_USB_MIS_ST_SPEC>;
#[doc = "USB miscellaneous status"]
pub mod r8_usb_mis_st;

#[doc = "R8_USB_INT_FG register accessor: an alias for `Reg<R8_USB_INT_FG_SPEC>`"]
pub type R8_USB_INT_FG = crate::Reg<r8_usb_int_fg::R8_USB_INT_FG_SPEC>;
#[doc = "USB interrupt flag"]
pub mod r8_usb_int_fg;

#[doc = "R8_USB_INT_ST register accessor: an alias for `Reg<R8_USB_INT_ST_SPEC>`"]
pub type R8_USB_INT_ST = crate::Reg<r8_usb_int_st::R8_USB_INT_ST_SPEC>;
#[doc = "USB interrupt status"]
pub mod r8_usb_int_st;

#[doc = "R16_USB_RX_LEN register accessor: an alias for `Reg<R16_USB_RX_LEN_SPEC>`"]
pub type R16_USB_RX_LEN = crate::Reg<r16_usb_rx_len::R16_USB_RX_LEN_SPEC>;
#[doc = "USB receiving length"]
pub mod r16_usb_rx_len;

#[doc = "R8_UEP4_1_MOD register accessor: an alias for `Reg<R8_UEP4_1_MOD_SPEC>`"]
pub type R8_UEP4_1_MOD = crate::Reg<r8_uep4_1_mod::R8_UEP4_1_MOD_SPEC>;
#[doc = "endpoint 4/1 mode"]
pub mod r8_uep4_1_mod;

#[doc = "R8_UEP2_3_MOD__R8_UH_EP_MOD register accessor: an alias for `Reg<R8_UEP2_3_MOD__R8_UH_EP_MOD_SPEC>`"]
pub type R8_UEP2_3_MOD__R8_UH_EP_MOD =
    crate::Reg<r8_uep2_3_mod__r8_uh_ep_mod::R8_UEP2_3_MOD__R8_UH_EP_MOD_SPEC>;
#[doc = "endpoint 2/3 mode;host endpoint mode"]
pub mod r8_uep2_3_mod__r8_uh_ep_mod;

#[doc = "R8_UEP5_6_MOD register accessor: an alias for `Reg<R8_UEP5_6_MOD_SPEC>`"]
pub type R8_UEP5_6_MOD = crate::Reg<r8_uep5_6_mod::R8_UEP5_6_MOD_SPEC>;
#[doc = "endpoint 5/6 mode"]
pub mod r8_uep5_6_mod;

#[doc = "R8_UEP7_MOD register accessor: an alias for `Reg<R8_UEP7_MOD_SPEC>`"]
pub type R8_UEP7_MOD = crate::Reg<r8_uep7_mod::R8_UEP7_MOD_SPEC>;
#[doc = "endpoint 7 mode"]
pub mod r8_uep7_mod;

#[doc = "R32_UEP0_DMA register accessor: an alias for `Reg<R32_UEP0_DMA_SPEC>`"]
pub type R32_UEP0_DMA = crate::Reg<r32_uep0_dma::R32_UEP0_DMA_SPEC>;
#[doc = "endpoint 0 DMA buffer address"]
pub mod r32_uep0_dma;

#[doc = "R32_UEP1_DMA register accessor: an alias for `Reg<R32_UEP1_DMA_SPEC>`"]
pub type R32_UEP1_DMA = crate::Reg<r32_uep1_dma::R32_UEP1_DMA_SPEC>;
#[doc = "endpoint 1 DMA buffer address"]
pub mod r32_uep1_dma;

#[doc = "R32_UEP2_DMA__R32_UH_RX_DMA register accessor: an alias for `Reg<R32_UEP2_DMA__R32_UH_RX_DMA_SPEC>`"]
pub type R32_UEP2_DMA__R32_UH_RX_DMA =
    crate::Reg<r32_uep2_dma__r32_uh_rx_dma::R32_UEP2_DMA__R32_UH_RX_DMA_SPEC>;
#[doc = "endpoint 2 DMA buffer address;host rx endpoint buffer high address"]
pub mod r32_uep2_dma__r32_uh_rx_dma;

#[doc = "R32_UEP3_DMA__R32_UH_TX_DMA register accessor: an alias for `Reg<R32_UEP3_DMA__R32_UH_TX_DMA_SPEC>`"]
pub type R32_UEP3_DMA__R32_UH_TX_DMA =
    crate::Reg<r32_uep3_dma__r32_uh_tx_dma::R32_UEP3_DMA__R32_UH_TX_DMA_SPEC>;
#[doc = "endpoint 3 DMA buffer address;host tx endpoint buffer high address"]
pub mod r32_uep3_dma__r32_uh_tx_dma;

#[doc = "R32_UEP4_DMA register accessor: an alias for `Reg<R32_UEP4_DMA_SPEC>`"]
pub type R32_UEP4_DMA = crate::Reg<r32_uep4_dma::R32_UEP4_DMA_SPEC>;
#[doc = "endpoint 4 DMA buffer address"]
pub mod r32_uep4_dma;

#[doc = "R32_UEP5_DMA register accessor: an alias for `Reg<R32_UEP5_DMA_SPEC>`"]
pub type R32_UEP5_DMA = crate::Reg<r32_uep5_dma::R32_UEP5_DMA_SPEC>;
#[doc = "endpoint 5 DMA buffer address"]
pub mod r32_uep5_dma;

#[doc = "R32_UEP6_DMA register accessor: an alias for `Reg<R32_UEP6_DMA_SPEC>`"]
pub type R32_UEP6_DMA = crate::Reg<r32_uep6_dma::R32_UEP6_DMA_SPEC>;
#[doc = "endpoint 6 DMA buffer address"]
pub mod r32_uep6_dma;

#[doc = "R32_UEP7_DMA register accessor: an alias for `Reg<R32_UEP7_DMA_SPEC>`"]
pub type R32_UEP7_DMA = crate::Reg<r32_uep7_dma::R32_UEP7_DMA_SPEC>;
#[doc = "endpoint 7 DMA buffer address"]
pub mod r32_uep7_dma;

#[doc = "R8_UEP0_T_LEN register accessor: an alias for `Reg<R8_UEP0_T_LEN_SPEC>`"]
pub type R8_UEP0_T_LEN = crate::Reg<r8_uep0_t_len::R8_UEP0_T_LEN_SPEC>;
#[doc = "endpoint 0 transmittal length"]
pub mod r8_uep0_t_len;

#[doc = "R8_UEP0_T_CTRL register accessor: an alias for `Reg<R8_UEP0_T_CTRL_SPEC>`"]
pub type R8_UEP0_T_CTRL = crate::Reg<r8_uep0_t_ctrl::R8_UEP0_T_CTRL_SPEC>;
#[doc = "endpoint 0 control"]
pub mod r8_uep0_t_ctrl;

#[doc = "R8_UEP0_R_CTRL register accessor: an alias for `Reg<R8_UEP0_R_CTRL_SPEC>`"]
pub type R8_UEP0_R_CTRL = crate::Reg<r8_uep0_r_ctrl::R8_UEP0_R_CTRL_SPEC>;
#[doc = "endpoint 0 control"]
pub mod r8_uep0_r_ctrl;

#[doc = "R8_UEP1_T_LEN register accessor: an alias for `Reg<R8_UEP1_T_LEN_SPEC>`"]
pub type R8_UEP1_T_LEN = crate::Reg<r8_uep1_t_len::R8_UEP1_T_LEN_SPEC>;
#[doc = "endpoint 1 transmittal length"]
pub mod r8_uep1_t_len;

#[doc = "R8_UEP1_T_CTRL___USBHD_UH_SETUP register accessor: an alias for `Reg<R8_UEP1_T_CTRL___USBHD_UH_SETUP_SPEC>`"]
pub type R8_UEP1_T_CTRL___USBHD_UH_SETUP =
    crate::Reg<r8_uep1_t_ctrl___usbhd_uh_setup::R8_UEP1_T_CTRL___USBHD_UH_SETUP_SPEC>;
#[doc = "endpoint 1 control"]
pub mod r8_uep1_t_ctrl___usbhd_uh_setup;

#[doc = "R8_UEP1_R_CTRL register accessor: an alias for `Reg<R8_UEP1_R_CTRL_SPEC>`"]
pub type R8_UEP1_R_CTRL = crate::Reg<r8_uep1_r_ctrl::R8_UEP1_R_CTRL_SPEC>;
#[doc = "endpoint 1 control"]
pub mod r8_uep1_r_ctrl;

#[doc = "R8_UEP2_T_LEN__USBHD_UH_EP_PID register accessor: an alias for `Reg<R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC>`"]
pub type R8_UEP2_T_LEN__USBHD_UH_EP_PID =
    crate::Reg<r8_uep2_t_len__usbhd_uh_ep_pid::R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC>;
#[doc = "endpoint 2 transmittal length"]
pub mod r8_uep2_t_len__usbhd_uh_ep_pid;

#[doc = "R8_UEP2_T_CTRL register accessor: an alias for `Reg<R8_UEP2_T_CTRL_SPEC>`"]
pub type R8_UEP2_T_CTRL = crate::Reg<r8_uep2_t_ctrl::R8_UEP2_T_CTRL_SPEC>;
#[doc = "endpoint 2 control"]
pub mod r8_uep2_t_ctrl;

#[doc = "R8_UEP2_R_CTRL__USBHD_UH_RX_CTRL register accessor: an alias for `Reg<R8_UEP2_R_CTRL__USBHD_UH_RX_CTRL_SPEC>`"]
pub type R8_UEP2_R_CTRL__USBHD_UH_RX_CTRL =
    crate::Reg<r8_uep2_r_ctrl__usbhd_uh_rx_ctrl::R8_UEP2_R_CTRL__USBHD_UH_RX_CTRL_SPEC>;
#[doc = "endpoint 2 control"]
pub mod r8_uep2_r_ctrl__usbhd_uh_rx_ctrl;

#[doc = "R8_UEP3_T_LEN__USBHD_UH_TX_LEN register accessor: an alias for `Reg<R8_UEP3_T_LEN__USBHD_UH_TX_LEN_SPEC>`"]
pub type R8_UEP3_T_LEN__USBHD_UH_TX_LEN =
    crate::Reg<r8_uep3_t_len__usbhd_uh_tx_len::R8_UEP3_T_LEN__USBHD_UH_TX_LEN_SPEC>;
#[doc = "endpoint 3 transmittal length"]
pub mod r8_uep3_t_len__usbhd_uh_tx_len;

#[doc = "R8_UEP3_T_CTRL__USBHD_UH_TX_CTRL register accessor: an alias for `Reg<R8_UEP3_T_CTRL__USBHD_UH_TX_CTRL_SPEC>`"]
pub type R8_UEP3_T_CTRL__USBHD_UH_TX_CTRL =
    crate::Reg<r8_uep3_t_ctrl__usbhd_uh_tx_ctrl::R8_UEP3_T_CTRL__USBHD_UH_TX_CTRL_SPEC>;
#[doc = "endpoint 3 control"]
pub mod r8_uep3_t_ctrl__usbhd_uh_tx_ctrl;

#[doc = "R8_UEP3_R_CTRL_ register accessor: an alias for `Reg<R8_UEP3_R_CTRL__SPEC>`"]
pub type R8_UEP3_R_CTRL_ = crate::Reg<r8_uep3_r_ctrl_::R8_UEP3_R_CTRL__SPEC>;
#[doc = "endpoint 3 control"]
pub mod r8_uep3_r_ctrl_;

#[doc = "R8_UEP4_T_LEN register accessor: an alias for `Reg<R8_UEP4_T_LEN_SPEC>`"]
pub type R8_UEP4_T_LEN = crate::Reg<r8_uep4_t_len::R8_UEP4_T_LEN_SPEC>;
#[doc = "endpoint 4 transmittal length"]
pub mod r8_uep4_t_len;

#[doc = "R8_UEP4_T_CTRL register accessor: an alias for `Reg<R8_UEP4_T_CTRL_SPEC>`"]
pub type R8_UEP4_T_CTRL = crate::Reg<r8_uep4_t_ctrl::R8_UEP4_T_CTRL_SPEC>;
#[doc = "endpoint 4 control"]
pub mod r8_uep4_t_ctrl;

#[doc = "R8_UEP4_R_CTRL_ register accessor: an alias for `Reg<R8_UEP4_R_CTRL__SPEC>`"]
pub type R8_UEP4_R_CTRL_ = crate::Reg<r8_uep4_r_ctrl_::R8_UEP4_R_CTRL__SPEC>;
#[doc = "endpoint 4 control"]
pub mod r8_uep4_r_ctrl_;

#[doc = "R8_UEP5_T_LEN register accessor: an alias for `Reg<R8_UEP5_T_LEN_SPEC>`"]
pub type R8_UEP5_T_LEN = crate::Reg<r8_uep5_t_len::R8_UEP5_T_LEN_SPEC>;
#[doc = "endpoint 5 transmittal length"]
pub mod r8_uep5_t_len;

#[doc = "R8_UEP5_T_CTRL register accessor: an alias for `Reg<R8_UEP5_T_CTRL_SPEC>`"]
pub type R8_UEP5_T_CTRL = crate::Reg<r8_uep5_t_ctrl::R8_UEP5_T_CTRL_SPEC>;
#[doc = "endpoint 5 control"]
pub mod r8_uep5_t_ctrl;

#[doc = "R8_UEP5_R_CTRL_ register accessor: an alias for `Reg<R8_UEP5_R_CTRL__SPEC>`"]
pub type R8_UEP5_R_CTRL_ = crate::Reg<r8_uep5_r_ctrl_::R8_UEP5_R_CTRL__SPEC>;
#[doc = "endpoint 5 control"]
pub mod r8_uep5_r_ctrl_;

#[doc = "R8_UEP6_T_LEN register accessor: an alias for `Reg<R8_UEP6_T_LEN_SPEC>`"]
pub type R8_UEP6_T_LEN = crate::Reg<r8_uep6_t_len::R8_UEP6_T_LEN_SPEC>;
#[doc = "endpoint 6 transmittal length"]
pub mod r8_uep6_t_len;

#[doc = "R8_UEP6_T_CTRL register accessor: an alias for `Reg<R8_UEP6_T_CTRL_SPEC>`"]
pub type R8_UEP6_T_CTRL = crate::Reg<r8_uep6_t_ctrl::R8_UEP6_T_CTRL_SPEC>;
#[doc = "endpoint 6 control"]
pub mod r8_uep6_t_ctrl;

#[doc = "R8_UEP6_R_CTRL_ register accessor: an alias for `Reg<R8_UEP6_R_CTRL__SPEC>`"]
pub type R8_UEP6_R_CTRL_ = crate::Reg<r8_uep6_r_ctrl_::R8_UEP6_R_CTRL__SPEC>;
#[doc = "endpoint 6 control"]
pub mod r8_uep6_r_ctrl_;

#[doc = "R8_UEP7_T_LEN register accessor: an alias for `Reg<R8_UEP7_T_LEN_SPEC>`"]
pub type R8_UEP7_T_LEN = crate::Reg<r8_uep7_t_len::R8_UEP7_T_LEN_SPEC>;
#[doc = "endpoint 7 transmittal length"]
pub mod r8_uep7_t_len;

#[doc = "R8_UEP7_T_CTRL register accessor: an alias for `Reg<R8_UEP7_T_CTRL_SPEC>`"]
pub type R8_UEP7_T_CTRL = crate::Reg<r8_uep7_t_ctrl::R8_UEP7_T_CTRL_SPEC>;
#[doc = "endpoint 7 control"]
pub mod r8_uep7_t_ctrl;

#[doc = "R8_UEP7_R_CTRL_ register accessor: an alias for `Reg<R8_UEP7_R_CTRL__SPEC>`"]
pub type R8_UEP7_R_CTRL_ = crate::Reg<r8_uep7_r_ctrl_::R8_UEP7_R_CTRL__SPEC>;
#[doc = "endpoint 7 control"]
pub mod r8_uep7_r_ctrl_;

#[doc = "USB_OTG_CR register accessor: an alias for `Reg<USB_OTG_CR_SPEC>`"]
pub type USB_OTG_CR = crate::Reg<usb_otg_cr::USB_OTG_CR_SPEC>;
#[doc = "usb otg control"]
pub mod usb_otg_cr;

#[doc = "USB_OTG_SR register accessor: an alias for `Reg<USB_OTG_SR_SPEC>`"]
pub type USB_OTG_SR = crate::Reg<usb_otg_sr::USB_OTG_SR_SPEC>;
#[doc = "usb otg status"]
pub mod usb_otg_sr;