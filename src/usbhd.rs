#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB base control"]
    pub usb_ctrl: crate::Reg<usb_ctrl::USB_CTRL_SPEC>,
    #[doc = "0x01 - USB HOST control"]
    pub uhost_ctrl: crate::Reg<uhost_ctrl::UHOST_CTRL_SPEC>,
    #[doc = "0x02 - USB interrupt enable"]
    pub usb_int_en: crate::Reg<usb_int_en::USB_INT_EN_SPEC>,
    #[doc = "0x03 - USB device address"]
    pub usb_dev_ad: crate::Reg<usb_dev_ad::USB_DEV_AD_SPEC>,
    #[doc = "0x04 - USB_FRAME_NO"]
    pub usb_frame_no: crate::Reg<usb_frame_no::USB_FRAME_NO_SPEC>,
    #[doc = "0x06 - indicate USB suspend status"]
    pub usb_usb_suspend: crate::Reg<usb_usb_suspend::USB_USB_SUSPEND_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x08 - USB_SPEED_TYPE"]
    pub usb_speed_type: crate::Reg<usb_speed_type::USB_SPEED_TYPE_SPEC>,
    #[doc = "0x09 - USB miscellaneous status"]
    pub usb_mis_st: crate::Reg<usb_mis_st::USB_MIS_ST_SPEC>,
    #[doc = "0x0a - USB interrupt flag"]
    pub usb_int_fg: crate::Reg<usb_int_fg::USB_INT_FG_SPEC>,
    #[doc = "0x0b - USB interrupt status"]
    pub usb_int_st: crate::Reg<usb_int_st::USB_INT_ST_SPEC>,
    #[doc = "0x0c - USB receiving length"]
    pub usb_rx_len: crate::Reg<usb_rx_len::USB_RX_LEN_SPEC>,
    _reserved11: [u8; 0x02],
    #[doc = "0x10 - USB endpoint configuration"]
    pub uep_config: crate::Reg<uep_config::UEP_CONFIG_SPEC>,
    #[doc = "0x14 - USB endpoint type"]
    pub uep_type: crate::Reg<uep_type::UEP_TYPE_SPEC>,
    #[doc = "0x18 - USB endpoint buffer mode"]
    pub uep_buf_mod: crate::Reg<uep_buf_mod::UEP_BUF_MOD_SPEC>,
    #[doc = "0x1c - B endpoint 0 DMA buffer address"]
    pub uep0_dma: crate::Reg<uep0_dma::UEP0_DMA_SPEC>,
    #[doc = "0x20 - endpoint 1 DMA RX buffer address"]
    pub uep1_rx_dma: crate::Reg<uep1_rx_dma::UEP1_RX_DMA_SPEC>,
    #[doc = "0x24 - endpoint 2 DMA RX buffer address/UH_RX_DMA"]
    pub uep2_rx_dma__uh_rx_dma: crate::Reg<uep2_rx_dma__uh_rx_dma::UEP2_RX_DMA__UH_RX_DMA_SPEC>,
    #[doc = "0x28 - endpoint 3 DMA RX buffer address"]
    pub uep3_rx_dma: crate::Reg<uep3_rx_dma::UEP3_RX_DMA_SPEC>,
    #[doc = "0x2c - endpoint 4 DMA RX buffer address"]
    pub uep4_rx_dma: crate::Reg<uep4_rx_dma::UEP4_RX_DMA_SPEC>,
    #[doc = "0x30 - endpoint 5 DMA RX buffer address"]
    pub uep5_rx_dma: crate::Reg<uep5_rx_dma::UEP5_RX_DMA_SPEC>,
    #[doc = "0x34 - endpoint 6 DMA RX buffer address"]
    pub uep6_rx_dma: crate::Reg<uep6_rx_dma::UEP6_RX_DMA_SPEC>,
    #[doc = "0x38 - endpoint 7 DMA RX buffer address"]
    pub uep7_rx_dma: crate::Reg<uep7_rx_dma::UEP7_RX_DMA_SPEC>,
    #[doc = "0x3c - endpoint 8 DMA RX buffer address"]
    pub uep8_rx_dma: crate::Reg<uep8_rx_dma::UEP8_RX_DMA_SPEC>,
    #[doc = "0x40 - endpoint 9 DMA RX buffer address"]
    pub uep9_rx_dma: crate::Reg<uep9_rx_dma::UEP9_RX_DMA_SPEC>,
    #[doc = "0x44 - endpoint 10 DMA RX buffer address"]
    pub uep10_rx_dma: crate::Reg<uep10_rx_dma::UEP10_RX_DMA_SPEC>,
    #[doc = "0x48 - endpoint 11 DMA RX buffer address"]
    pub uep11_rx_dma: crate::Reg<uep11_rx_dma::UEP11_RX_DMA_SPEC>,
    #[doc = "0x4c - endpoint 12 DMA RX buffer address"]
    pub uep12_rx_dma: crate::Reg<uep12_rx_dma::UEP12_RX_DMA_SPEC>,
    #[doc = "0x50 - endpoint 13 DMA RX buffer address"]
    pub uep13_rx_dma: crate::Reg<uep13_rx_dma::UEP13_RX_DMA_SPEC>,
    #[doc = "0x54 - endpoint 14 DMA RX buffer address"]
    pub uep14_rx_dma: crate::Reg<uep14_rx_dma::UEP14_RX_DMA_SPEC>,
    #[doc = "0x58 - endpoint 15 DMA RX buffer address"]
    pub uep15_rx_dma: crate::Reg<uep15_rx_dma::UEP15_RX_DMA_SPEC>,
    #[doc = "0x5c - endpoint 1 DMA TX buffer address"]
    pub uep1_tx_dma: crate::Reg<uep1_tx_dma::UEP1_TX_DMA_SPEC>,
    #[doc = "0x60 - endpoint 2 DMA TX buffer address"]
    pub uep2_tx_dma: crate::Reg<uep2_tx_dma::UEP2_TX_DMA_SPEC>,
    #[doc = "0x64 - endpoint 3 DMA TX buffer address"]
    pub uep3_tx_dma__uh_tx_dma: crate::Reg<uep3_tx_dma__uh_tx_dma::UEP3_TX_DMA__UH_TX_DMA_SPEC>,
    #[doc = "0x68 - endpoint 4 DMA TX buffer address"]
    pub uep4_tx_dma: crate::Reg<uep4_tx_dma::UEP4_TX_DMA_SPEC>,
    #[doc = "0x6c - endpoint 5 DMA TX buffer address"]
    pub uep5_tx_dma: crate::Reg<uep5_tx_dma::UEP5_TX_DMA_SPEC>,
    #[doc = "0x70 - endpoint 6 DMA TX buffer address"]
    pub uep6_tx_dma: crate::Reg<uep6_tx_dma::UEP6_TX_DMA_SPEC>,
    #[doc = "0x74 - endpoint 7 DMA TX buffer address"]
    pub uep7_tx_dma: crate::Reg<uep7_tx_dma::UEP7_TX_DMA_SPEC>,
    #[doc = "0x78 - endpoint 8 DMA TX buffer address"]
    pub uep8_tx_dma: crate::Reg<uep8_tx_dma::UEP8_TX_DMA_SPEC>,
    #[doc = "0x7c - endpoint 9 DMA TX buffer address"]
    pub uep9_tx_dma: crate::Reg<uep9_tx_dma::UEP9_TX_DMA_SPEC>,
    #[doc = "0x80 - endpoint 10 DMA TX buffer address"]
    pub uep10_tx_dma: crate::Reg<uep10_tx_dma::UEP10_TX_DMA_SPEC>,
    #[doc = "0x84 - endpoint 11 DMA TX buffer address"]
    pub uep11_tx_dma: crate::Reg<uep11_tx_dma::UEP11_TX_DMA_SPEC>,
    #[doc = "0x88 - endpoint 12 DMA TX buffer address"]
    pub uep12_tx_dma____uh_split_data:
        crate::Reg<uep12_tx_dma____uh_split_data::UEP12_TX_DMA____UH_SPLIT_DATA_SPEC>,
    #[doc = "0x8c - endpoint 13 DMA TX buffer address"]
    pub uep13_tx_dma: crate::Reg<uep13_tx_dma::UEP13_TX_DMA_SPEC>,
    #[doc = "0x90 - endpoint 14 DMA TX buffer address"]
    pub uep14_tx_dma: crate::Reg<uep14_tx_dma::UEP14_TX_DMA_SPEC>,
    #[doc = "0x94 - endpoint 15 DMA TX buffer address"]
    pub uep15_tx_dma: crate::Reg<uep15_tx_dma::UEP15_TX_DMA_SPEC>,
    #[doc = "0x98 - endpoint 0 max acceptable length"]
    pub uep0_max_len: crate::Reg<uep0_max_len::UEP0_MAX_LEN_SPEC>,
    _reserved46: [u8; 0x02],
    #[doc = "0x9c - endpoint 1 max acceptable length"]
    pub uep1_max_len: crate::Reg<uep1_max_len::UEP1_MAX_LEN_SPEC>,
    _reserved47: [u8; 0x02],
    #[doc = "0xa0 - endpoint 2 max acceptable length"]
    pub uep2_max_len__uh_rx_max_len:
        crate::Reg<uep2_max_len__uh_rx_max_len::UEP2_MAX_LEN__UH_RX_MAX_LEN_SPEC>,
    _reserved48: [u8; 0x02],
    #[doc = "0xa4 - endpoint 3 MAX_LEN TX"]
    pub uep3_max_len: crate::Reg<uep3_max_len::UEP3_MAX_LEN_SPEC>,
    _reserved49: [u8; 0x02],
    #[doc = "0xa8 - endpoint 4 max acceptable length"]
    pub uep4_max_len: crate::Reg<uep4_max_len::UEP4_MAX_LEN_SPEC>,
    _reserved50: [u8; 0x02],
    #[doc = "0xac - endpoint 5 max acceptable length"]
    pub uep5_max_len: crate::Reg<uep5_max_len::UEP5_MAX_LEN_SPEC>,
    _reserved51: [u8; 0x02],
    #[doc = "0xb0 - endpoint 6 max acceptable length"]
    pub uep6_max_len: crate::Reg<uep6_max_len::UEP6_MAX_LEN_SPEC>,
    _reserved52: [u8; 0x02],
    #[doc = "0xb4 - endpoint 7 max acceptable length"]
    pub uep7_max_len: crate::Reg<uep7_max_len::UEP7_MAX_LEN_SPEC>,
    _reserved53: [u8; 0x02],
    #[doc = "0xb8 - endpoint 8 max acceptable length"]
    pub uep8_max_len: crate::Reg<uep8_max_len::UEP8_MAX_LEN_SPEC>,
    _reserved54: [u8; 0x02],
    #[doc = "0xbc - endpoint 9 max acceptable length"]
    pub uep9_max_len: crate::Reg<uep9_max_len::UEP9_MAX_LEN_SPEC>,
    _reserved55: [u8; 0x02],
    #[doc = "0xc0 - endpoint 10 max acceptable length"]
    pub uep10_max_len: crate::Reg<uep10_max_len::UEP10_MAX_LEN_SPEC>,
    _reserved56: [u8; 0x02],
    #[doc = "0xc4 - endpoint 11 max acceptable length"]
    pub uep11_max_len: crate::Reg<uep11_max_len::UEP11_MAX_LEN_SPEC>,
    _reserved57: [u8; 0x02],
    #[doc = "0xc8 - endpoint 12 max acceptable length"]
    pub uep12_max_len: crate::Reg<uep12_max_len::UEP12_MAX_LEN_SPEC>,
    _reserved58: [u8; 0x02],
    #[doc = "0xcc - endpoint 13 max acceptable length"]
    pub uep13_max_len: crate::Reg<uep13_max_len::UEP13_MAX_LEN_SPEC>,
    _reserved59: [u8; 0x02],
    #[doc = "0xd0 - endpoint 14 max acceptable length"]
    pub uep14_max_len: crate::Reg<uep14_max_len::UEP14_MAX_LEN_SPEC>,
    _reserved60: [u8; 0x02],
    #[doc = "0xd4 - endpoint 15 max acceptable length"]
    pub uep15_max_len: crate::Reg<uep15_max_len::UEP15_MAX_LEN_SPEC>,
    _reserved61: [u8; 0x02],
    #[doc = "0xd8 - endpoint 0 send the length"]
    pub uep0_t_len: crate::Reg<uep0_t_len::UEP0_T_LEN_SPEC>,
    _reserved_62_uep1_t_len: [u8; 0x04],
    _reserved_63_uep1_r_ctrl: [u8; 0x04],
    _reserved_64_uep2_t_ctrl: [u8; 0x04],
    _reserved_65_uep4_t_len: [u8; 0x04],
    _reserved_66_uep5_t_len: [u8; 0x04],
    _reserved_67_uep6_t_len: [u8; 0x04],
    _reserved_68_uep7_t_len: [u8; 0x04],
    _reserved_69_uep8_t_len: [u8; 0x04],
    _reserved_70_uep9_t_len: [u8; 0x04],
    _reserved_71_uep10_t_len: [u8; 0x04],
    _reserved_72_uep1: [u8; 0x04],
    _reserved_73_uep1: [u8; 0x04],
    _reserved_74_uep1: [u8; 0x04],
    _reserved_75_uep1: [u8; 0x04],
    _reserved_76_uep1: [u8; 0x04],
    _reserved_77_uep15: [u8; 0x03],
}
impl RegisterBlock {
    #[doc = "0xda - endpoint 0 send control"]
    #[inline(always)]
    pub fn uep0_t_ctrl(&self) -> &crate::Reg<uep0_t_ctrl::UEP0_T_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(218usize)
                as *const crate::Reg<uep0_t_ctrl::UEP0_T_CTRL_SPEC>)
        }
    }
    #[doc = "0xdb - endpoint 0 send control"]
    #[inline(always)]
    pub fn uep0_r_ctrl(&self) -> &crate::Reg<uep0_r_ctrl::UEP0_R_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(219usize)
                as *const crate::Reg<uep0_r_ctrl::UEP0_R_CTRL_SPEC>)
        }
    }
    #[doc = "0xdc - endpoint 1 send the length"]
    #[inline(always)]
    pub fn uep1_t_len(&self) -> &crate::Reg<uep1_t_len::UEP1_T_LEN_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(220usize)
                as *const crate::Reg<uep1_t_len::UEP1_T_LEN_SPEC>)
        }
    }
    #[doc = "0xde - endpoint 1 send control"]
    #[inline(always)]
    pub fn uep1_t_ctrl(&self) -> &crate::Reg<uep1_t_ctrl::UEP1_T_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(222usize)
                as *const crate::Reg<uep1_t_ctrl::UEP1_T_CTRL_SPEC>)
        }
    }
    #[doc = "0xdf - endpoint 1 send control"]
    #[inline(always)]
    pub fn uep1_r_ctrl(&self) -> &crate::Reg<uep1_r_ctrl::UEP1_R_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(223usize)
                as *const crate::Reg<uep1_r_ctrl::UEP1_R_CTRL_SPEC>)
        }
    }
    #[doc = "0xe0 - endpoint 2 send the length"]
    #[inline(always)]
    pub fn uep2_t_len__uh_ep_pid(
        &self,
    ) -> &crate::Reg<uep2_t_len__uh_ep_pid::UEP2_T_LEN__UH_EP_PID_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(224usize)
                as *const crate::Reg<uep2_t_len__uh_ep_pid::UEP2_T_LEN__UH_EP_PID_SPEC>)
        }
    }
    #[doc = "0xe2 - endpoint 2 send control"]
    #[inline(always)]
    pub fn uep2_t_ctrl(&self) -> &crate::Reg<uep2_t_ctrl::UEP2_T_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(226usize)
                as *const crate::Reg<uep2_t_ctrl::UEP2_T_CTRL_SPEC>)
        }
    }
    #[doc = "0xe3 - endpoint 2 send control"]
    #[inline(always)]
    pub fn uep2_r_ctrl__uh_rx_ctrl(
        &self,
    ) -> &crate::Reg<uep2_r_ctrl__uh_rx_ctrl::UEP2_R_CTRL__UH_RX_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(227usize)
                as *const crate::Reg<uep2_r_ctrl__uh_rx_ctrl::UEP2_R_CTRL__UH_RX_CTRL_SPEC>)
        }
    }
    #[doc = "0xe4 - endpoint 3 send the length"]
    #[inline(always)]
    pub fn uep3_t_len___uh_tx_len_h(
        &self,
    ) -> &crate::Reg<uep3_t_len___uh_tx_len_h::UEP3_T_LEN___UH_TX_LEN_H_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(228usize)
                as *const crate::Reg<uep3_t_len___uh_tx_len_h::UEP3_T_LEN___UH_TX_LEN_H_SPEC>)
        }
    }
    #[doc = "0xe6 - endpoint 3 send control"]
    #[inline(always)]
    pub fn uep3_t_ctrl___uh_tx_ctrl(
        &self,
    ) -> &crate::Reg<uep3_t_ctrl___uh_tx_ctrl::UEP3_T_CTRL___UH_TX_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(230usize)
                as *const crate::Reg<uep3_t_ctrl___uh_tx_ctrl::UEP3_T_CTRL___UH_TX_CTRL_SPEC>)
        }
    }
    #[doc = "0xe7 - endpoint 3 send control"]
    #[inline(always)]
    pub fn uep3_r_ctrl(&self) -> &crate::Reg<uep3_r_ctrl::UEP3_R_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(231usize)
                as *const crate::Reg<uep3_r_ctrl::UEP3_R_CTRL_SPEC>)
        }
    }
    #[doc = "0xe8 - endpoint 4 send the length"]
    #[inline(always)]
    pub fn uep4_t_len(&self) -> &crate::Reg<uep4_t_len::UEP4_T_LEN_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(232usize)
                as *const crate::Reg<uep4_t_len::UEP4_T_LEN_SPEC>)
        }
    }
    #[doc = "0xea - endpoint 4 send control"]
    #[inline(always)]
    pub fn uep4_t_ctrl(&self) -> &crate::Reg<uep4_t_ctrl::UEP4_T_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(234usize)
                as *const crate::Reg<uep4_t_ctrl::UEP4_T_CTRL_SPEC>)
        }
    }
    #[doc = "0xeb - endpoint 4 send control"]
    #[inline(always)]
    pub fn uep4_r_ctrl(&self) -> &crate::Reg<uep4_r_ctrl::UEP4_R_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(235usize)
                as *const crate::Reg<uep4_r_ctrl::UEP4_R_CTRL_SPEC>)
        }
    }
    #[doc = "0xec - endpoint 5 send the length"]
    #[inline(always)]
    pub fn uep5_t_len(&self) -> &crate::Reg<uep5_t_len::UEP5_T_LEN_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(236usize)
                as *const crate::Reg<uep5_t_len::UEP5_T_LEN_SPEC>)
        }
    }
    #[doc = "0xee - endpoint 5 send control"]
    #[inline(always)]
    pub fn uep5_t_ctrl(&self) -> &crate::Reg<uep5_t_ctrl::UEP5_T_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(238usize)
                as *const crate::Reg<uep5_t_ctrl::UEP5_T_CTRL_SPEC>)
        }
    }
    #[doc = "0xef - endpoint 5 send control"]
    #[inline(always)]
    pub fn uep5_r_ctrl(&self) -> &crate::Reg<uep5_r_ctrl::UEP5_R_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(239usize)
                as *const crate::Reg<uep5_r_ctrl::UEP5_R_CTRL_SPEC>)
        }
    }
    #[doc = "0xf0 - endpoint 6 send the length"]
    #[inline(always)]
    pub fn uep6_t_len(&self) -> &crate::Reg<uep6_t_len::UEP6_T_LEN_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(240usize)
                as *const crate::Reg<uep6_t_len::UEP6_T_LEN_SPEC>)
        }
    }
    #[doc = "0xf2 - endpoint 6 send control"]
    #[inline(always)]
    pub fn uep6_t_ctrl(&self) -> &crate::Reg<uep6_t_ctrl::UEP6_T_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(242usize)
                as *const crate::Reg<uep6_t_ctrl::UEP6_T_CTRL_SPEC>)
        }
    }
    #[doc = "0xf3 - endpoint 6 send control"]
    #[inline(always)]
    pub fn uep6_r_ctrl(&self) -> &crate::Reg<uep6_r_ctrl::UEP6_R_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(243usize)
                as *const crate::Reg<uep6_r_ctrl::UEP6_R_CTRL_SPEC>)
        }
    }
    #[doc = "0xf4 - endpoint 7 send the length"]
    #[inline(always)]
    pub fn uep7_t_len(&self) -> &crate::Reg<uep7_t_len::UEP7_T_LEN_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(244usize)
                as *const crate::Reg<uep7_t_len::UEP7_T_LEN_SPEC>)
        }
    }
    #[doc = "0xf6 - endpoint 7 send control"]
    #[inline(always)]
    pub fn uep7_t_ctrl(&self) -> &crate::Reg<uep7_t_ctrl::UEP7_T_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(246usize)
                as *const crate::Reg<uep7_t_ctrl::UEP7_T_CTRL_SPEC>)
        }
    }
    #[doc = "0xf7 - endpoint 7 send control"]
    #[inline(always)]
    pub fn uep7_r_ctrl(&self) -> &crate::Reg<uep7_r_ctrl::UEP7_R_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(247usize)
                as *const crate::Reg<uep7_r_ctrl::UEP7_R_CTRL_SPEC>)
        }
    }
    #[doc = "0xf8 - endpoint 8 send the length"]
    #[inline(always)]
    pub fn uep8_t_len(&self) -> &crate::Reg<uep8_t_len::UEP8_T_LEN_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(248usize)
                as *const crate::Reg<uep8_t_len::UEP8_T_LEN_SPEC>)
        }
    }
    #[doc = "0xfa - endpoint 8 send control"]
    #[inline(always)]
    pub fn uep8_t_ctrl(&self) -> &crate::Reg<uep8_t_ctrl::UEP8_T_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(250usize)
                as *const crate::Reg<uep8_t_ctrl::UEP8_T_CTRL_SPEC>)
        }
    }
    #[doc = "0xfb - endpoint 8 send control"]
    #[inline(always)]
    pub fn uep8_r_ctrl(&self) -> &crate::Reg<uep8_r_ctrl::UEP8_R_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(251usize)
                as *const crate::Reg<uep8_r_ctrl::UEP8_R_CTRL_SPEC>)
        }
    }
    #[doc = "0xfc - endpoint9 send the length"]
    #[inline(always)]
    pub fn uep9_t_len(&self) -> &crate::Reg<uep9_t_len::UEP9_T_LEN_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(252usize)
                as *const crate::Reg<uep9_t_len::UEP9_T_LEN_SPEC>)
        }
    }
    #[doc = "0xfe - endpoint 9 send control"]
    #[inline(always)]
    pub fn uep9_t_ctrl(&self) -> &crate::Reg<uep9_t_ctrl::UEP9_T_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(254usize)
                as *const crate::Reg<uep9_t_ctrl::UEP9_T_CTRL_SPEC>)
        }
    }
    #[doc = "0xff - endpoint 9 send control"]
    #[inline(always)]
    pub fn uep9_r_ctrl(&self) -> &crate::Reg<uep9_r_ctrl::UEP9_R_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(255usize)
                as *const crate::Reg<uep9_r_ctrl::UEP9_R_CTRL_SPEC>)
        }
    }
    #[doc = "0x100 - endpoint 10 send the length"]
    #[inline(always)]
    pub fn uep10_t_len(&self) -> &crate::Reg<uep10_t_len::UEP10_T_LEN_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(256usize)
                as *const crate::Reg<uep10_t_len::UEP10_T_LEN_SPEC>)
        }
    }
    #[doc = "0x102 - endpoint 10 send control"]
    #[inline(always)]
    pub fn uep10_t_ctrl(&self) -> &crate::Reg<uep10_t_ctrl::UEP10_T_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(258usize)
                as *const crate::Reg<uep10_t_ctrl::UEP10_T_CTRL_SPEC>)
        }
    }
    #[doc = "0x103 - endpoint 10 send control"]
    #[inline(always)]
    pub fn uep10_r_ctrl(&self) -> &crate::Reg<uep10_r_ctrl::UEP10_R_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(259usize)
                as *const crate::Reg<uep10_r_ctrl::UEP10_R_CTRL_SPEC>)
        }
    }
    #[doc = "0x104 - endpoint 11 send the length"]
    #[inline(always)]
    pub fn uep11_t_len(&self) -> &crate::Reg<uep11_t_len::UEP11_T_LEN_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(260usize)
                as *const crate::Reg<uep11_t_len::UEP11_T_LEN_SPEC>)
        }
    }
    #[doc = "0x106 - endpoint 11 send control"]
    #[inline(always)]
    pub fn uep11_t_ctrl(&self) -> &crate::Reg<uep11_t_ctrl::UEP11_T_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(262usize)
                as *const crate::Reg<uep11_t_ctrl::UEP11_T_CTRL_SPEC>)
        }
    }
    #[doc = "0x107 - endpoint 11 send control"]
    #[inline(always)]
    pub fn uep11_r_ctrl(&self) -> &crate::Reg<uep11_r_ctrl::UEP11_R_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(263usize)
                as *const crate::Reg<uep11_r_ctrl::UEP11_R_CTRL_SPEC>)
        }
    }
    #[doc = "0x108 - endpoint 12 send the length"]
    #[inline(always)]
    pub fn uep12_t_len(&self) -> &crate::Reg<uep12_t_len::UEP12_T_LEN_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(264usize)
                as *const crate::Reg<uep12_t_len::UEP12_T_LEN_SPEC>)
        }
    }
    #[doc = "0x10a - endpoint 12 send control"]
    #[inline(always)]
    pub fn uep12_t_ctrl(&self) -> &crate::Reg<uep12_t_ctrl::UEP12_T_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(266usize)
                as *const crate::Reg<uep12_t_ctrl::UEP12_T_CTRL_SPEC>)
        }
    }
    #[doc = "0x10b - endpoint 12 send control"]
    #[inline(always)]
    pub fn uep12_r_ctrl(&self) -> &crate::Reg<uep12_r_ctrl::UEP12_R_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(267usize)
                as *const crate::Reg<uep12_r_ctrl::UEP12_R_CTRL_SPEC>)
        }
    }
    #[doc = "0x10c - endpoint 13 send the length"]
    #[inline(always)]
    pub fn uep13_t_len(&self) -> &crate::Reg<uep13_t_len::UEP13_T_LEN_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(268usize)
                as *const crate::Reg<uep13_t_len::UEP13_T_LEN_SPEC>)
        }
    }
    #[doc = "0x10e - endpoint 13 send control"]
    #[inline(always)]
    pub fn uep13_t_ctrl(&self) -> &crate::Reg<uep13_t_ctrl::UEP13_T_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(270usize)
                as *const crate::Reg<uep13_t_ctrl::UEP13_T_CTRL_SPEC>)
        }
    }
    #[doc = "0x10f - endpoint 13 send control"]
    #[inline(always)]
    pub fn uep13_r_ctrl(&self) -> &crate::Reg<uep13_r_ctrl::UEP13_R_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(271usize)
                as *const crate::Reg<uep13_r_ctrl::UEP13_R_CTRL_SPEC>)
        }
    }
    #[doc = "0x110 - endpoint 14 send the length"]
    #[inline(always)]
    pub fn uep14_t_len(&self) -> &crate::Reg<uep14_t_len::UEP14_T_LEN_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(272usize)
                as *const crate::Reg<uep14_t_len::UEP14_T_LEN_SPEC>)
        }
    }
    #[doc = "0x112 - endpoint 14 send control"]
    #[inline(always)]
    pub fn uep14_t_ctrl(&self) -> &crate::Reg<uep14_t_ctrl::UEP14_T_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(274usize)
                as *const crate::Reg<uep14_t_ctrl::UEP14_T_CTRL_SPEC>)
        }
    }
    #[doc = "0x113 - endpoint 14 send control"]
    #[inline(always)]
    pub fn uep14_r_ctrl(&self) -> &crate::Reg<uep14_r_ctrl::UEP14_R_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(275usize)
                as *const crate::Reg<uep14_r_ctrl::UEP14_R_CTRL_SPEC>)
        }
    }
    #[doc = "0x114 - endpoint 15 send the length"]
    #[inline(always)]
    pub fn uep15_t_len(&self) -> &crate::Reg<uep15_t_len::UEP15_T_LEN_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(276usize)
                as *const crate::Reg<uep15_t_len::UEP15_T_LEN_SPEC>)
        }
    }
    #[doc = "0x116 - endpoint 15 send control"]
    #[inline(always)]
    pub fn uep15_t_ctrl(&self) -> &crate::Reg<uep15_t_ctrl::UEP15_T_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(278usize)
                as *const crate::Reg<uep15_t_ctrl::UEP15_T_CTRL_SPEC>)
        }
    }
    #[doc = "0x117 - endpoint 15 send control"]
    #[inline(always)]
    pub fn uep15_r_ctrl(&self) -> &crate::Reg<uep15_r_ctrl::UEP15_R_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(279usize)
                as *const crate::Reg<uep15_r_ctrl::UEP15_R_CTRL_SPEC>)
        }
    }
}

#[doc = "USB_CTRL register accessor: an alias for `Reg<USB_CTRL_SPEC>`"]
pub type USB_CTRL = crate::Reg<usb_ctrl::USB_CTRL_SPEC>;
#[doc = "USB base control"]
pub mod usb_ctrl;

#[doc = "UHOST_CTRL register accessor: an alias for `Reg<UHOST_CTRL_SPEC>`"]
pub type UHOST_CTRL = crate::Reg<uhost_ctrl::UHOST_CTRL_SPEC>;
#[doc = "USB HOST control"]
pub mod uhost_ctrl;

#[doc = "USB_INT_EN register accessor: an alias for `Reg<USB_INT_EN_SPEC>`"]
pub type USB_INT_EN = crate::Reg<usb_int_en::USB_INT_EN_SPEC>;
#[doc = "USB interrupt enable"]
pub mod usb_int_en;

#[doc = "USB_DEV_AD register accessor: an alias for `Reg<USB_DEV_AD_SPEC>`"]
pub type USB_DEV_AD = crate::Reg<usb_dev_ad::USB_DEV_AD_SPEC>;
#[doc = "USB device address"]
pub mod usb_dev_ad;

#[doc = "USB_FRAME_NO register accessor: an alias for `Reg<USB_FRAME_NO_SPEC>`"]
pub type USB_FRAME_NO = crate::Reg<usb_frame_no::USB_FRAME_NO_SPEC>;
#[doc = "USB_FRAME_NO"]
pub mod usb_frame_no;

#[doc = "USB_USB_SUSPEND register accessor: an alias for `Reg<USB_USB_SUSPEND_SPEC>`"]
pub type USB_USB_SUSPEND = crate::Reg<usb_usb_suspend::USB_USB_SUSPEND_SPEC>;
#[doc = "indicate USB suspend status"]
pub mod usb_usb_suspend;

#[doc = "USB_SPEED_TYPE register accessor: an alias for `Reg<USB_SPEED_TYPE_SPEC>`"]
pub type USB_SPEED_TYPE = crate::Reg<usb_speed_type::USB_SPEED_TYPE_SPEC>;
#[doc = "USB_SPEED_TYPE"]
pub mod usb_speed_type;

#[doc = "USB_MIS_ST register accessor: an alias for `Reg<USB_MIS_ST_SPEC>`"]
pub type USB_MIS_ST = crate::Reg<usb_mis_st::USB_MIS_ST_SPEC>;
#[doc = "USB miscellaneous status"]
pub mod usb_mis_st;

#[doc = "USB_INT_FG register accessor: an alias for `Reg<USB_INT_FG_SPEC>`"]
pub type USB_INT_FG = crate::Reg<usb_int_fg::USB_INT_FG_SPEC>;
#[doc = "USB interrupt flag"]
pub mod usb_int_fg;

#[doc = "USB_INT_ST register accessor: an alias for `Reg<USB_INT_ST_SPEC>`"]
pub type USB_INT_ST = crate::Reg<usb_int_st::USB_INT_ST_SPEC>;
#[doc = "USB interrupt status"]
pub mod usb_int_st;

#[doc = "USB_RX_LEN register accessor: an alias for `Reg<USB_RX_LEN_SPEC>`"]
pub type USB_RX_LEN = crate::Reg<usb_rx_len::USB_RX_LEN_SPEC>;
#[doc = "USB receiving length"]
pub mod usb_rx_len;

#[doc = "UEP_CONFIG register accessor: an alias for `Reg<UEP_CONFIG_SPEC>`"]
pub type UEP_CONFIG = crate::Reg<uep_config::UEP_CONFIG_SPEC>;
#[doc = "USB endpoint configuration"]
pub mod uep_config;

#[doc = "UEP_TYPE register accessor: an alias for `Reg<UEP_TYPE_SPEC>`"]
pub type UEP_TYPE = crate::Reg<uep_type::UEP_TYPE_SPEC>;
#[doc = "USB endpoint type"]
pub mod uep_type;

#[doc = "UEP_BUF_MOD register accessor: an alias for `Reg<UEP_BUF_MOD_SPEC>`"]
pub type UEP_BUF_MOD = crate::Reg<uep_buf_mod::UEP_BUF_MOD_SPEC>;
#[doc = "USB endpoint buffer mode"]
pub mod uep_buf_mod;

#[doc = "UEP0_DMA register accessor: an alias for `Reg<UEP0_DMA_SPEC>`"]
pub type UEP0_DMA = crate::Reg<uep0_dma::UEP0_DMA_SPEC>;
#[doc = "B endpoint 0 DMA buffer address"]
pub mod uep0_dma;

#[doc = "UEP1_RX_DMA register accessor: an alias for `Reg<UEP1_RX_DMA_SPEC>`"]
pub type UEP1_RX_DMA = crate::Reg<uep1_rx_dma::UEP1_RX_DMA_SPEC>;
#[doc = "endpoint 1 DMA RX buffer address"]
pub mod uep1_rx_dma;

#[doc = "UEP2_RX_DMA__UH_RX_DMA register accessor: an alias for `Reg<UEP2_RX_DMA__UH_RX_DMA_SPEC>`"]
pub type UEP2_RX_DMA__UH_RX_DMA =
    crate::Reg<uep2_rx_dma__uh_rx_dma::UEP2_RX_DMA__UH_RX_DMA_SPEC>;
#[doc = "endpoint 2 DMA RX buffer address/UH_RX_DMA"]
pub mod uep2_rx_dma__uh_rx_dma;

#[doc = "UEP3_RX_DMA register accessor: an alias for `Reg<UEP3_RX_DMA_SPEC>`"]
pub type UEP3_RX_DMA = crate::Reg<uep3_rx_dma::UEP3_RX_DMA_SPEC>;
#[doc = "endpoint 3 DMA RX buffer address"]
pub mod uep3_rx_dma;

#[doc = "UEP4_RX_DMA register accessor: an alias for `Reg<UEP4_RX_DMA_SPEC>`"]
pub type UEP4_RX_DMA = crate::Reg<uep4_rx_dma::UEP4_RX_DMA_SPEC>;
#[doc = "endpoint 4 DMA RX buffer address"]
pub mod uep4_rx_dma;

#[doc = "UEP5_RX_DMA register accessor: an alias for `Reg<UEP5_RX_DMA_SPEC>`"]
pub type UEP5_RX_DMA = crate::Reg<uep5_rx_dma::UEP5_RX_DMA_SPEC>;
#[doc = "endpoint 5 DMA RX buffer address"]
pub mod uep5_rx_dma;

#[doc = "UEP6_RX_DMA register accessor: an alias for `Reg<UEP6_RX_DMA_SPEC>`"]
pub type UEP6_RX_DMA = crate::Reg<uep6_rx_dma::UEP6_RX_DMA_SPEC>;
#[doc = "endpoint 6 DMA RX buffer address"]
pub mod uep6_rx_dma;

#[doc = "UEP7_RX_DMA register accessor: an alias for `Reg<UEP7_RX_DMA_SPEC>`"]
pub type UEP7_RX_DMA = crate::Reg<uep7_rx_dma::UEP7_RX_DMA_SPEC>;
#[doc = "endpoint 7 DMA RX buffer address"]
pub mod uep7_rx_dma;

#[doc = "UEP8_RX_DMA register accessor: an alias for `Reg<UEP8_RX_DMA_SPEC>`"]
pub type UEP8_RX_DMA = crate::Reg<uep8_rx_dma::UEP8_RX_DMA_SPEC>;
#[doc = "endpoint 8 DMA RX buffer address"]
pub mod uep8_rx_dma;

#[doc = "UEP9_RX_DMA register accessor: an alias for `Reg<UEP9_RX_DMA_SPEC>`"]
pub type UEP9_RX_DMA = crate::Reg<uep9_rx_dma::UEP9_RX_DMA_SPEC>;
#[doc = "endpoint 9 DMA RX buffer address"]
pub mod uep9_rx_dma;

#[doc = "UEP10_RX_DMA register accessor: an alias for `Reg<UEP10_RX_DMA_SPEC>`"]
pub type UEP10_RX_DMA = crate::Reg<uep10_rx_dma::UEP10_RX_DMA_SPEC>;
#[doc = "endpoint 10 DMA RX buffer address"]
pub mod uep10_rx_dma;

#[doc = "UEP11_RX_DMA register accessor: an alias for `Reg<UEP11_RX_DMA_SPEC>`"]
pub type UEP11_RX_DMA = crate::Reg<uep11_rx_dma::UEP11_RX_DMA_SPEC>;
#[doc = "endpoint 11 DMA RX buffer address"]
pub mod uep11_rx_dma;

#[doc = "UEP12_RX_DMA register accessor: an alias for `Reg<UEP12_RX_DMA_SPEC>`"]
pub type UEP12_RX_DMA = crate::Reg<uep12_rx_dma::UEP12_RX_DMA_SPEC>;
#[doc = "endpoint 12 DMA RX buffer address"]
pub mod uep12_rx_dma;

#[doc = "UEP13_RX_DMA register accessor: an alias for `Reg<UEP13_RX_DMA_SPEC>`"]
pub type UEP13_RX_DMA = crate::Reg<uep13_rx_dma::UEP13_RX_DMA_SPEC>;
#[doc = "endpoint 13 DMA RX buffer address"]
pub mod uep13_rx_dma;

#[doc = "UEP14_RX_DMA register accessor: an alias for `Reg<UEP14_RX_DMA_SPEC>`"]
pub type UEP14_RX_DMA = crate::Reg<uep14_rx_dma::UEP14_RX_DMA_SPEC>;
#[doc = "endpoint 14 DMA RX buffer address"]
pub mod uep14_rx_dma;

#[doc = "UEP15_RX_DMA register accessor: an alias for `Reg<UEP15_RX_DMA_SPEC>`"]
pub type UEP15_RX_DMA = crate::Reg<uep15_rx_dma::UEP15_RX_DMA_SPEC>;
#[doc = "endpoint 15 DMA RX buffer address"]
pub mod uep15_rx_dma;

#[doc = "UEP1_TX_DMA register accessor: an alias for `Reg<UEP1_TX_DMA_SPEC>`"]
pub type UEP1_TX_DMA = crate::Reg<uep1_tx_dma::UEP1_TX_DMA_SPEC>;
#[doc = "endpoint 1 DMA TX buffer address"]
pub mod uep1_tx_dma;

#[doc = "UEP2_TX_DMA register accessor: an alias for `Reg<UEP2_TX_DMA_SPEC>`"]
pub type UEP2_TX_DMA = crate::Reg<uep2_tx_dma::UEP2_TX_DMA_SPEC>;
#[doc = "endpoint 2 DMA TX buffer address"]
pub mod uep2_tx_dma;

#[doc = "UEP3_TX_DMA__UH_TX_DMA register accessor: an alias for `Reg<UEP3_TX_DMA__UH_TX_DMA_SPEC>`"]
pub type UEP3_TX_DMA__UH_TX_DMA =
    crate::Reg<uep3_tx_dma__uh_tx_dma::UEP3_TX_DMA__UH_TX_DMA_SPEC>;
#[doc = "endpoint 3 DMA TX buffer address"]
pub mod uep3_tx_dma__uh_tx_dma;

#[doc = "UEP4_TX_DMA register accessor: an alias for `Reg<UEP4_TX_DMA_SPEC>`"]
pub type UEP4_TX_DMA = crate::Reg<uep4_tx_dma::UEP4_TX_DMA_SPEC>;
#[doc = "endpoint 4 DMA TX buffer address"]
pub mod uep4_tx_dma;

#[doc = "UEP5_TX_DMA register accessor: an alias for `Reg<UEP5_TX_DMA_SPEC>`"]
pub type UEP5_TX_DMA = crate::Reg<uep5_tx_dma::UEP5_TX_DMA_SPEC>;
#[doc = "endpoint 5 DMA TX buffer address"]
pub mod uep5_tx_dma;

#[doc = "UEP6_TX_DMA register accessor: an alias for `Reg<UEP6_TX_DMA_SPEC>`"]
pub type UEP6_TX_DMA = crate::Reg<uep6_tx_dma::UEP6_TX_DMA_SPEC>;
#[doc = "endpoint 6 DMA TX buffer address"]
pub mod uep6_tx_dma;

#[doc = "UEP7_TX_DMA register accessor: an alias for `Reg<UEP7_TX_DMA_SPEC>`"]
pub type UEP7_TX_DMA = crate::Reg<uep7_tx_dma::UEP7_TX_DMA_SPEC>;
#[doc = "endpoint 7 DMA TX buffer address"]
pub mod uep7_tx_dma;

#[doc = "UEP8_TX_DMA register accessor: an alias for `Reg<UEP8_TX_DMA_SPEC>`"]
pub type UEP8_TX_DMA = crate::Reg<uep8_tx_dma::UEP8_TX_DMA_SPEC>;
#[doc = "endpoint 8 DMA TX buffer address"]
pub mod uep8_tx_dma;

#[doc = "UEP9_TX_DMA register accessor: an alias for `Reg<UEP9_TX_DMA_SPEC>`"]
pub type UEP9_TX_DMA = crate::Reg<uep9_tx_dma::UEP9_TX_DMA_SPEC>;
#[doc = "endpoint 9 DMA TX buffer address"]
pub mod uep9_tx_dma;

#[doc = "UEP10_TX_DMA register accessor: an alias for `Reg<UEP10_TX_DMA_SPEC>`"]
pub type UEP10_TX_DMA = crate::Reg<uep10_tx_dma::UEP10_TX_DMA_SPEC>;
#[doc = "endpoint 10 DMA TX buffer address"]
pub mod uep10_tx_dma;

#[doc = "UEP11_TX_DMA register accessor: an alias for `Reg<UEP11_TX_DMA_SPEC>`"]
pub type UEP11_TX_DMA = crate::Reg<uep11_tx_dma::UEP11_TX_DMA_SPEC>;
#[doc = "endpoint 11 DMA TX buffer address"]
pub mod uep11_tx_dma;

#[doc = "UEP12_TX_DMA____UH_SPLIT_DATA register accessor: an alias for `Reg<UEP12_TX_DMA____UH_SPLIT_DATA_SPEC>`"]
pub type UEP12_TX_DMA____UH_SPLIT_DATA =
    crate::Reg<uep12_tx_dma____uh_split_data::UEP12_TX_DMA____UH_SPLIT_DATA_SPEC>;
#[doc = "endpoint 12 DMA TX buffer address"]
pub mod uep12_tx_dma____uh_split_data;

#[doc = "UEP13_TX_DMA register accessor: an alias for `Reg<UEP13_TX_DMA_SPEC>`"]
pub type UEP13_TX_DMA = crate::Reg<uep13_tx_dma::UEP13_TX_DMA_SPEC>;
#[doc = "endpoint 13 DMA TX buffer address"]
pub mod uep13_tx_dma;

#[doc = "UEP14_TX_DMA register accessor: an alias for `Reg<UEP14_TX_DMA_SPEC>`"]
pub type UEP14_TX_DMA = crate::Reg<uep14_tx_dma::UEP14_TX_DMA_SPEC>;
#[doc = "endpoint 14 DMA TX buffer address"]
pub mod uep14_tx_dma;

#[doc = "UEP15_TX_DMA register accessor: an alias for `Reg<UEP15_TX_DMA_SPEC>`"]
pub type UEP15_TX_DMA = crate::Reg<uep15_tx_dma::UEP15_TX_DMA_SPEC>;
#[doc = "endpoint 15 DMA TX buffer address"]
pub mod uep15_tx_dma;

#[doc = "UEP0_MAX_LEN register accessor: an alias for `Reg<UEP0_MAX_LEN_SPEC>`"]
pub type UEP0_MAX_LEN = crate::Reg<uep0_max_len::UEP0_MAX_LEN_SPEC>;
#[doc = "endpoint 0 max acceptable length"]
pub mod uep0_max_len;

#[doc = "UEP1_MAX_LEN register accessor: an alias for `Reg<UEP1_MAX_LEN_SPEC>`"]
pub type UEP1_MAX_LEN = crate::Reg<uep1_max_len::UEP1_MAX_LEN_SPEC>;
#[doc = "endpoint 1 max acceptable length"]
pub mod uep1_max_len;

#[doc = "UEP2_MAX_LEN__UH_RX_MAX_LEN register accessor: an alias for `Reg<UEP2_MAX_LEN__UH_RX_MAX_LEN_SPEC>`"]
pub type UEP2_MAX_LEN__UH_RX_MAX_LEN =
    crate::Reg<uep2_max_len__uh_rx_max_len::UEP2_MAX_LEN__UH_RX_MAX_LEN_SPEC>;
#[doc = "endpoint 2 max acceptable length"]
pub mod uep2_max_len__uh_rx_max_len;

#[doc = "UEP3_MAX_LEN register accessor: an alias for `Reg<UEP3_MAX_LEN_SPEC>`"]
pub type UEP3_MAX_LEN = crate::Reg<uep3_max_len::UEP3_MAX_LEN_SPEC>;
#[doc = "endpoint 3 MAX_LEN TX"]
pub mod uep3_max_len;

#[doc = "UEP4_MAX_LEN register accessor: an alias for `Reg<UEP4_MAX_LEN_SPEC>`"]
pub type UEP4_MAX_LEN = crate::Reg<uep4_max_len::UEP4_MAX_LEN_SPEC>;
#[doc = "endpoint 4 max acceptable length"]
pub mod uep4_max_len;

#[doc = "UEP5_MAX_LEN register accessor: an alias for `Reg<UEP5_MAX_LEN_SPEC>`"]
pub type UEP5_MAX_LEN = crate::Reg<uep5_max_len::UEP5_MAX_LEN_SPEC>;
#[doc = "endpoint 5 max acceptable length"]
pub mod uep5_max_len;

#[doc = "UEP6_MAX_LEN register accessor: an alias for `Reg<UEP6_MAX_LEN_SPEC>`"]
pub type UEP6_MAX_LEN = crate::Reg<uep6_max_len::UEP6_MAX_LEN_SPEC>;
#[doc = "endpoint 6 max acceptable length"]
pub mod uep6_max_len;

#[doc = "UEP7_MAX_LEN register accessor: an alias for `Reg<UEP7_MAX_LEN_SPEC>`"]
pub type UEP7_MAX_LEN = crate::Reg<uep7_max_len::UEP7_MAX_LEN_SPEC>;
#[doc = "endpoint 7 max acceptable length"]
pub mod uep7_max_len;

#[doc = "UEP8_MAX_LEN register accessor: an alias for `Reg<UEP8_MAX_LEN_SPEC>`"]
pub type UEP8_MAX_LEN = crate::Reg<uep8_max_len::UEP8_MAX_LEN_SPEC>;
#[doc = "endpoint 8 max acceptable length"]
pub mod uep8_max_len;

#[doc = "UEP9_MAX_LEN register accessor: an alias for `Reg<UEP9_MAX_LEN_SPEC>`"]
pub type UEP9_MAX_LEN = crate::Reg<uep9_max_len::UEP9_MAX_LEN_SPEC>;
#[doc = "endpoint 9 max acceptable length"]
pub mod uep9_max_len;

#[doc = "UEP10_MAX_LEN register accessor: an alias for `Reg<UEP10_MAX_LEN_SPEC>`"]
pub type UEP10_MAX_LEN = crate::Reg<uep10_max_len::UEP10_MAX_LEN_SPEC>;
#[doc = "endpoint 10 max acceptable length"]
pub mod uep10_max_len;

#[doc = "UEP11_MAX_LEN register accessor: an alias for `Reg<UEP11_MAX_LEN_SPEC>`"]
pub type UEP11_MAX_LEN = crate::Reg<uep11_max_len::UEP11_MAX_LEN_SPEC>;
#[doc = "endpoint 11 max acceptable length"]
pub mod uep11_max_len;

#[doc = "UEP12_MAX_LEN register accessor: an alias for `Reg<UEP12_MAX_LEN_SPEC>`"]
pub type UEP12_MAX_LEN = crate::Reg<uep12_max_len::UEP12_MAX_LEN_SPEC>;
#[doc = "endpoint 12 max acceptable length"]
pub mod uep12_max_len;

#[doc = "UEP13_MAX_LEN register accessor: an alias for `Reg<UEP13_MAX_LEN_SPEC>`"]
pub type UEP13_MAX_LEN = crate::Reg<uep13_max_len::UEP13_MAX_LEN_SPEC>;
#[doc = "endpoint 13 max acceptable length"]
pub mod uep13_max_len;

#[doc = "UEP14_MAX_LEN register accessor: an alias for `Reg<UEP14_MAX_LEN_SPEC>`"]
pub type UEP14_MAX_LEN = crate::Reg<uep14_max_len::UEP14_MAX_LEN_SPEC>;
#[doc = "endpoint 14 max acceptable length"]
pub mod uep14_max_len;

#[doc = "UEP15_MAX_LEN register accessor: an alias for `Reg<UEP15_MAX_LEN_SPEC>`"]
pub type UEP15_MAX_LEN = crate::Reg<uep15_max_len::UEP15_MAX_LEN_SPEC>;
#[doc = "endpoint 15 max acceptable length"]
pub mod uep15_max_len;

#[doc = "UEP0_T_LEN register accessor: an alias for `Reg<UEP0_T_LEN_SPEC>`"]
pub type UEP0_T_LEN = crate::Reg<uep0_t_len::UEP0_T_LEN_SPEC>;
#[doc = "endpoint 0 send the length"]
pub mod uep0_t_len;

#[doc = "UEP0_T_CTRL register accessor: an alias for `Reg<UEP0_T_CTRL_SPEC>`"]
pub type UEP0_T_CTRL = crate::Reg<uep0_t_ctrl::UEP0_T_CTRL_SPEC>;
#[doc = "endpoint 0 send control"]
pub mod uep0_t_ctrl;

#[doc = "UEP0_R_CTRL register accessor: an alias for `Reg<UEP0_R_CTRL_SPEC>`"]
pub type UEP0_R_CTRL = crate::Reg<uep0_r_ctrl::UEP0_R_CTRL_SPEC>;
#[doc = "endpoint 0 send control"]
pub mod uep0_r_ctrl;

#[doc = "UEP1_T_LEN register accessor: an alias for `Reg<UEP1_T_LEN_SPEC>`"]
pub type UEP1_T_LEN = crate::Reg<uep1_t_len::UEP1_T_LEN_SPEC>;
#[doc = "endpoint 1 send the length"]
pub mod uep1_t_len;

#[doc = "UEP1_T_CTRL register accessor: an alias for `Reg<UEP1_T_CTRL_SPEC>`"]
pub type UEP1_T_CTRL = crate::Reg<uep1_t_ctrl::UEP1_T_CTRL_SPEC>;
#[doc = "endpoint 1 send control"]
pub mod uep1_t_ctrl;

#[doc = "UEP1_R_CTRL register accessor: an alias for `Reg<UEP1_R_CTRL_SPEC>`"]
pub type UEP1_R_CTRL = crate::Reg<uep1_r_ctrl::UEP1_R_CTRL_SPEC>;
#[doc = "endpoint 1 send control"]
pub mod uep1_r_ctrl;

#[doc = "UEP2_T_LEN__UH_EP_PID register accessor: an alias for `Reg<UEP2_T_LEN__UH_EP_PID_SPEC>`"]
pub type UEP2_T_LEN__UH_EP_PID = crate::Reg<uep2_t_len__uh_ep_pid::UEP2_T_LEN__UH_EP_PID_SPEC>;
#[doc = "endpoint 2 send the length"]
pub mod uep2_t_len__uh_ep_pid;

#[doc = "UEP2_T_CTRL register accessor: an alias for `Reg<UEP2_T_CTRL_SPEC>`"]
pub type UEP2_T_CTRL = crate::Reg<uep2_t_ctrl::UEP2_T_CTRL_SPEC>;
#[doc = "endpoint 2 send control"]
pub mod uep2_t_ctrl;

#[doc = "UEP2_R_CTRL__UH_RX_CTRL register accessor: an alias for `Reg<UEP2_R_CTRL__UH_RX_CTRL_SPEC>`"]
pub type UEP2_R_CTRL__UH_RX_CTRL =
    crate::Reg<uep2_r_ctrl__uh_rx_ctrl::UEP2_R_CTRL__UH_RX_CTRL_SPEC>;
#[doc = "endpoint 2 send control"]
pub mod uep2_r_ctrl__uh_rx_ctrl;

#[doc = "UEP3_T_LEN___UH_TX_LEN_H register accessor: an alias for `Reg<UEP3_T_LEN___UH_TX_LEN_H_SPEC>`"]
pub type UEP3_T_LEN___UH_TX_LEN_H =
    crate::Reg<uep3_t_len___uh_tx_len_h::UEP3_T_LEN___UH_TX_LEN_H_SPEC>;
#[doc = "endpoint 3 send the length"]
pub mod uep3_t_len___uh_tx_len_h;

#[doc = "UEP3_T_CTRL___UH_TX_CTRL register accessor: an alias for `Reg<UEP3_T_CTRL___UH_TX_CTRL_SPEC>`"]
pub type UEP3_T_CTRL___UH_TX_CTRL =
    crate::Reg<uep3_t_ctrl___uh_tx_ctrl::UEP3_T_CTRL___UH_TX_CTRL_SPEC>;
#[doc = "endpoint 3 send control"]
pub mod uep3_t_ctrl___uh_tx_ctrl;

#[doc = "UEP3_R_CTRL register accessor: an alias for `Reg<UEP3_R_CTRL_SPEC>`"]
pub type UEP3_R_CTRL = crate::Reg<uep3_r_ctrl::UEP3_R_CTRL_SPEC>;
#[doc = "endpoint 3 send control"]
pub mod uep3_r_ctrl;

#[doc = "UEP4_T_LEN register accessor: an alias for `Reg<UEP4_T_LEN_SPEC>`"]
pub type UEP4_T_LEN = crate::Reg<uep4_t_len::UEP4_T_LEN_SPEC>;
#[doc = "endpoint 4 send the length"]
pub mod uep4_t_len;

#[doc = "UEP4_T_CTRL register accessor: an alias for `Reg<UEP4_T_CTRL_SPEC>`"]
pub type UEP4_T_CTRL = crate::Reg<uep4_t_ctrl::UEP4_T_CTRL_SPEC>;
#[doc = "endpoint 4 send control"]
pub mod uep4_t_ctrl;

#[doc = "UEP4_R_CTRL register accessor: an alias for `Reg<UEP4_R_CTRL_SPEC>`"]
pub type UEP4_R_CTRL = crate::Reg<uep4_r_ctrl::UEP4_R_CTRL_SPEC>;
#[doc = "endpoint 4 send control"]
pub mod uep4_r_ctrl;

#[doc = "UEP5_T_LEN register accessor: an alias for `Reg<UEP5_T_LEN_SPEC>`"]
pub type UEP5_T_LEN = crate::Reg<uep5_t_len::UEP5_T_LEN_SPEC>;
#[doc = "endpoint 5 send the length"]
pub mod uep5_t_len;

#[doc = "UEP5_T_CTRL register accessor: an alias for `Reg<UEP5_T_CTRL_SPEC>`"]
pub type UEP5_T_CTRL = crate::Reg<uep5_t_ctrl::UEP5_T_CTRL_SPEC>;
#[doc = "endpoint 5 send control"]
pub mod uep5_t_ctrl;

#[doc = "UEP5_R_CTRL register accessor: an alias for `Reg<UEP5_R_CTRL_SPEC>`"]
pub type UEP5_R_CTRL = crate::Reg<uep5_r_ctrl::UEP5_R_CTRL_SPEC>;
#[doc = "endpoint 5 send control"]
pub mod uep5_r_ctrl;

#[doc = "UEP6_T_LEN register accessor: an alias for `Reg<UEP6_T_LEN_SPEC>`"]
pub type UEP6_T_LEN = crate::Reg<uep6_t_len::UEP6_T_LEN_SPEC>;
#[doc = "endpoint 6 send the length"]
pub mod uep6_t_len;

#[doc = "UEP6_T_CTRL register accessor: an alias for `Reg<UEP6_T_CTRL_SPEC>`"]
pub type UEP6_T_CTRL = crate::Reg<uep6_t_ctrl::UEP6_T_CTRL_SPEC>;
#[doc = "endpoint 6 send control"]
pub mod uep6_t_ctrl;

#[doc = "UEP6_R_CTRL register accessor: an alias for `Reg<UEP6_R_CTRL_SPEC>`"]
pub type UEP6_R_CTRL = crate::Reg<uep6_r_ctrl::UEP6_R_CTRL_SPEC>;
#[doc = "endpoint 6 send control"]
pub mod uep6_r_ctrl;

#[doc = "UEP7_T_LEN register accessor: an alias for `Reg<UEP7_T_LEN_SPEC>`"]
pub type UEP7_T_LEN = crate::Reg<uep7_t_len::UEP7_T_LEN_SPEC>;
#[doc = "endpoint 7 send the length"]
pub mod uep7_t_len;

#[doc = "UEP7_T_CTRL register accessor: an alias for `Reg<UEP7_T_CTRL_SPEC>`"]
pub type UEP7_T_CTRL = crate::Reg<uep7_t_ctrl::UEP7_T_CTRL_SPEC>;
#[doc = "endpoint 7 send control"]
pub mod uep7_t_ctrl;

#[doc = "UEP7_R_CTRL register accessor: an alias for `Reg<UEP7_R_CTRL_SPEC>`"]
pub type UEP7_R_CTRL = crate::Reg<uep7_r_ctrl::UEP7_R_CTRL_SPEC>;
#[doc = "endpoint 7 send control"]
pub mod uep7_r_ctrl;

#[doc = "UEP8_T_LEN register accessor: an alias for `Reg<UEP8_T_LEN_SPEC>`"]
pub type UEP8_T_LEN = crate::Reg<uep8_t_len::UEP8_T_LEN_SPEC>;
#[doc = "endpoint 8 send the length"]
pub mod uep8_t_len;

#[doc = "UEP8_T_CTRL register accessor: an alias for `Reg<UEP8_T_CTRL_SPEC>`"]
pub type UEP8_T_CTRL = crate::Reg<uep8_t_ctrl::UEP8_T_CTRL_SPEC>;
#[doc = "endpoint 8 send control"]
pub mod uep8_t_ctrl;

#[doc = "UEP8_R_CTRL register accessor: an alias for `Reg<UEP8_R_CTRL_SPEC>`"]
pub type UEP8_R_CTRL = crate::Reg<uep8_r_ctrl::UEP8_R_CTRL_SPEC>;
#[doc = "endpoint 8 send control"]
pub mod uep8_r_ctrl;

#[doc = "UEP9_T_LEN register accessor: an alias for `Reg<UEP9_T_LEN_SPEC>`"]
pub type UEP9_T_LEN = crate::Reg<uep9_t_len::UEP9_T_LEN_SPEC>;
#[doc = "endpoint9 send the length"]
pub mod uep9_t_len;

#[doc = "UEP9_T_CTRL register accessor: an alias for `Reg<UEP9_T_CTRL_SPEC>`"]
pub type UEP9_T_CTRL = crate::Reg<uep9_t_ctrl::UEP9_T_CTRL_SPEC>;
#[doc = "endpoint 9 send control"]
pub mod uep9_t_ctrl;

#[doc = "UEP9_R_CTRL register accessor: an alias for `Reg<UEP9_R_CTRL_SPEC>`"]
pub type UEP9_R_CTRL = crate::Reg<uep9_r_ctrl::UEP9_R_CTRL_SPEC>;
#[doc = "endpoint 9 send control"]
pub mod uep9_r_ctrl;

#[doc = "UEP10_T_LEN register accessor: an alias for `Reg<UEP10_T_LEN_SPEC>`"]
pub type UEP10_T_LEN = crate::Reg<uep10_t_len::UEP10_T_LEN_SPEC>;
#[doc = "endpoint 10 send the length"]
pub mod uep10_t_len;

#[doc = "UEP10_T_CTRL register accessor: an alias for `Reg<UEP10_T_CTRL_SPEC>`"]
pub type UEP10_T_CTRL = crate::Reg<uep10_t_ctrl::UEP10_T_CTRL_SPEC>;
#[doc = "endpoint 10 send control"]
pub mod uep10_t_ctrl;

#[doc = "UEP10_R_CTRL register accessor: an alias for `Reg<UEP10_R_CTRL_SPEC>`"]
pub type UEP10_R_CTRL = crate::Reg<uep10_r_ctrl::UEP10_R_CTRL_SPEC>;
#[doc = "endpoint 10 send control"]
pub mod uep10_r_ctrl;

#[doc = "UEP11_T_LEN register accessor: an alias for `Reg<UEP11_T_LEN_SPEC>`"]
pub type UEP11_T_LEN = crate::Reg<uep11_t_len::UEP11_T_LEN_SPEC>;
#[doc = "endpoint 11 send the length"]
pub mod uep11_t_len;

#[doc = "UEP11_T_CTRL register accessor: an alias for `Reg<UEP11_T_CTRL_SPEC>`"]
pub type UEP11_T_CTRL = crate::Reg<uep11_t_ctrl::UEP11_T_CTRL_SPEC>;
#[doc = "endpoint 11 send control"]
pub mod uep11_t_ctrl;

#[doc = "UEP11_R_CTRL register accessor: an alias for `Reg<UEP11_R_CTRL_SPEC>`"]
pub type UEP11_R_CTRL = crate::Reg<uep11_r_ctrl::UEP11_R_CTRL_SPEC>;
#[doc = "endpoint 11 send control"]
pub mod uep11_r_ctrl;

#[doc = "UEP12_T_LEN register accessor: an alias for `Reg<UEP12_T_LEN_SPEC>`"]
pub type UEP12_T_LEN = crate::Reg<uep12_t_len::UEP12_T_LEN_SPEC>;
#[doc = "endpoint 12 send the length"]
pub mod uep12_t_len;

#[doc = "UEP12_T_CTRL register accessor: an alias for `Reg<UEP12_T_CTRL_SPEC>`"]
pub type UEP12_T_CTRL = crate::Reg<uep12_t_ctrl::UEP12_T_CTRL_SPEC>;
#[doc = "endpoint 12 send control"]
pub mod uep12_t_ctrl;

#[doc = "UEP12_R_CTRL register accessor: an alias for `Reg<UEP12_R_CTRL_SPEC>`"]
pub type UEP12_R_CTRL = crate::Reg<uep12_r_ctrl::UEP12_R_CTRL_SPEC>;
#[doc = "endpoint 12 send control"]
pub mod uep12_r_ctrl;

#[doc = "UEP13_T_LEN register accessor: an alias for `Reg<UEP13_T_LEN_SPEC>`"]
pub type UEP13_T_LEN = crate::Reg<uep13_t_len::UEP13_T_LEN_SPEC>;
#[doc = "endpoint 13 send the length"]
pub mod uep13_t_len;

#[doc = "UEP13_T_CTRL register accessor: an alias for `Reg<UEP13_T_CTRL_SPEC>`"]
pub type UEP13_T_CTRL = crate::Reg<uep13_t_ctrl::UEP13_T_CTRL_SPEC>;
#[doc = "endpoint 13 send control"]
pub mod uep13_t_ctrl;

#[doc = "UEP13_R_CTRL register accessor: an alias for `Reg<UEP13_R_CTRL_SPEC>`"]
pub type UEP13_R_CTRL = crate::Reg<uep13_r_ctrl::UEP13_R_CTRL_SPEC>;
#[doc = "endpoint 13 send control"]
pub mod uep13_r_ctrl;

#[doc = "UEP14_T_LEN register accessor: an alias for `Reg<UEP14_T_LEN_SPEC>`"]
pub type UEP14_T_LEN = crate::Reg<uep14_t_len::UEP14_T_LEN_SPEC>;
#[doc = "endpoint 14 send the length"]
pub mod uep14_t_len;

#[doc = "UEP14_T_CTRL register accessor: an alias for `Reg<UEP14_T_CTRL_SPEC>`"]
pub type UEP14_T_CTRL = crate::Reg<uep14_t_ctrl::UEP14_T_CTRL_SPEC>;
#[doc = "endpoint 14 send control"]
pub mod uep14_t_ctrl;

#[doc = "UEP14_R_CTRL register accessor: an alias for `Reg<UEP14_R_CTRL_SPEC>`"]
pub type UEP14_R_CTRL = crate::Reg<uep14_r_ctrl::UEP14_R_CTRL_SPEC>;
#[doc = "endpoint 14 send control"]
pub mod uep14_r_ctrl;

#[doc = "UEP15_T_LEN register accessor: an alias for `Reg<UEP15_T_LEN_SPEC>`"]
pub type UEP15_T_LEN = crate::Reg<uep15_t_len::UEP15_T_LEN_SPEC>;
#[doc = "endpoint 15 send the length"]
pub mod uep15_t_len;

#[doc = "UEP15_T_CTRL register accessor: an alias for `Reg<UEP15_T_CTRL_SPEC>`"]
pub type UEP15_T_CTRL = crate::Reg<uep15_t_ctrl::UEP15_T_CTRL_SPEC>;
#[doc = "endpoint 15 send control"]
pub mod uep15_t_ctrl;

#[doc = "UEP15_R_CTRL register accessor: an alias for `Reg<UEP15_R_CTRL_SPEC>`"]
pub type UEP15_R_CTRL = crate::Reg<uep15_r_ctrl::UEP15_R_CTRL_SPEC>;
#[doc = "endpoint 15 send control"]
pub mod uep15_r_ctrl;