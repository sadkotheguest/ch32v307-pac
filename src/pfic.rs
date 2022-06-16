#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Status Register"]
    pub isr1: crate::Reg<isr1::ISR1_SPEC>,
    #[doc = "0x04 - Interrupt Status Register"]
    pub isr2: crate::Reg<isr2::ISR2_SPEC>,
    _reserved2: [u8; 0x18],
    #[doc = "0x20 - Interrupt Pending Register"]
    pub ipr1: crate::Reg<ipr1::IPR1_SPEC>,
    #[doc = "0x24 - Interrupt Pending Register"]
    pub ipr2: crate::Reg<ipr2::IPR2_SPEC>,
    _reserved4: [u8; 0x18],
    #[doc = "0x40 - Interrupt Priority Register"]
    pub ithresdr: crate::Reg<ithresdr::ITHRESDR_SPEC>,
    #[doc = "0x44 - Interrupt Fast Address Register"]
    pub fibaddrr: crate::Reg<fibaddrr::FIBADDRR_SPEC>,
    #[doc = "0x48 - Interrupt Config Register"]
    pub cfgr: crate::Reg<cfgr::CFGR_SPEC>,
    _reserved_7_gisr: [u8; 0x04],
    _reserved8: [u8; 0x10],
    _reserved_8_fifoaddrr0: [u8; 0x04],
    _reserved9: [u8; 0x9c],
    #[doc = "0x100 - Interrupt Setting Register"]
    pub ienr1: crate::Reg<ienr1::IENR1_SPEC>,
    #[doc = "0x104 - Interrupt Setting Register"]
    pub ienr2: crate::Reg<ienr2::IENR2_SPEC>,
    _reserved11: [u8; 0x78],
    #[doc = "0x180 - Interrupt Clear Register"]
    pub irer1: crate::Reg<irer1::IRER1_SPEC>,
    #[doc = "0x184 - Interrupt Clear Register"]
    pub irer2: crate::Reg<irer2::IRER2_SPEC>,
    _reserved13: [u8; 0x78],
    #[doc = "0x200 - Interrupt Pending Register"]
    pub ipsr1: crate::Reg<ipsr1::IPSR1_SPEC>,
    #[doc = "0x204 - Interrupt Pending Register"]
    pub ipsr2: crate::Reg<ipsr2::IPSR2_SPEC>,
    _reserved15: [u8; 0x78],
    #[doc = "0x280 - Interrupt Pending Clear Register"]
    pub iprr1: crate::Reg<iprr1::IPRR1_SPEC>,
    #[doc = "0x284 - Interrupt Pending Clear Register"]
    pub iprr2: crate::Reg<iprr2::IPRR2_SPEC>,
    _reserved17: [u8; 0x78],
    #[doc = "0x300 - Interrupt ACTIVE Register"]
    pub iactr1: crate::Reg<iactr1::IACTR1_SPEC>,
    #[doc = "0x304 - Interrupt ACTIVE Register"]
    pub iactr2: crate::Reg<iactr2::IACTR2_SPEC>,
    _reserved19: [u8; 0xf8],
    #[doc = "0x400 - Interrupt Priority Register"]
    pub iprior0: crate::Reg<iprior0::IPRIOR0_SPEC>,
    #[doc = "0x401 - Interrupt Priority Register"]
    pub iprior1: crate::Reg<iprior1::IPRIOR1_SPEC>,
    #[doc = "0x402 - Interrupt Priority Register"]
    pub iprior2: crate::Reg<iprior2::IPRIOR2_SPEC>,
    #[doc = "0x403 - Interrupt Priority Register"]
    pub iprior3: crate::Reg<iprior3::IPRIOR3_SPEC>,
    #[doc = "0x404 - Interrupt Priority Register"]
    pub iprior4: crate::Reg<iprior4::IPRIOR4_SPEC>,
    #[doc = "0x405 - Interrupt Priority Register"]
    pub iprior5: crate::Reg<iprior5::IPRIOR5_SPEC>,
    #[doc = "0x406 - Interrupt Priority Register"]
    pub iprior6: crate::Reg<iprior6::IPRIOR6_SPEC>,
    #[doc = "0x407 - Interrupt Priority Register"]
    pub iprior7: crate::Reg<iprior7::IPRIOR7_SPEC>,
    #[doc = "0x408 - Interrupt Priority Register"]
    pub iprior8: crate::Reg<iprior8::IPRIOR8_SPEC>,
    #[doc = "0x409 - Interrupt Priority Register"]
    pub iprior9: crate::Reg<iprior9::IPRIOR9_SPEC>,
    #[doc = "0x40a - Interrupt Priority Register"]
    pub iprior10: crate::Reg<iprior10::IPRIOR10_SPEC>,
    #[doc = "0x40b - Interrupt Priority Register"]
    pub iprior11: crate::Reg<iprior11::IPRIOR11_SPEC>,
    #[doc = "0x40c - Interrupt Priority Register"]
    pub iprior12: crate::Reg<iprior12::IPRIOR12_SPEC>,
    #[doc = "0x40d - Interrupt Priority Register"]
    pub iprior13: crate::Reg<iprior13::IPRIOR13_SPEC>,
    #[doc = "0x40e - Interrupt Priority Register"]
    pub iprior14: crate::Reg<iprior14::IPRIOR14_SPEC>,
    #[doc = "0x40f - Interrupt Priority Register"]
    pub iprior15: crate::Reg<iprior15::IPRIOR15_SPEC>,
    #[doc = "0x410 - Interrupt Priority Register"]
    pub iprior16: crate::Reg<iprior16::IPRIOR16_SPEC>,
    #[doc = "0x411 - Interrupt Priority Register"]
    pub iprior17: crate::Reg<iprior17::IPRIOR17_SPEC>,
    #[doc = "0x412 - Interrupt Priority Register"]
    pub iprior18: crate::Reg<iprior18::IPRIOR18_SPEC>,
    #[doc = "0x413 - Interrupt Priority Register"]
    pub iprior19: crate::Reg<iprior19::IPRIOR19_SPEC>,
    #[doc = "0x414 - Interrupt Priority Register"]
    pub iprior20: crate::Reg<iprior20::IPRIOR20_SPEC>,
    #[doc = "0x415 - Interrupt Priority Register"]
    pub iprior21: crate::Reg<iprior21::IPRIOR21_SPEC>,
    #[doc = "0x416 - Interrupt Priority Register"]
    pub iprior22: crate::Reg<iprior22::IPRIOR22_SPEC>,
    #[doc = "0x417 - Interrupt Priority Register"]
    pub iprior23: crate::Reg<iprior23::IPRIOR23_SPEC>,
    #[doc = "0x418 - Interrupt Priority Register"]
    pub iprior24: crate::Reg<iprior24::IPRIOR24_SPEC>,
    #[doc = "0x419 - Interrupt Priority Register"]
    pub iprior25: crate::Reg<iprior25::IPRIOR25_SPEC>,
    #[doc = "0x41a - Interrupt Priority Register"]
    pub iprior26: crate::Reg<iprior26::IPRIOR26_SPEC>,
    #[doc = "0x41b - Interrupt Priority Register"]
    pub iprior27: crate::Reg<iprior27::IPRIOR27_SPEC>,
    #[doc = "0x41c - Interrupt Priority Register"]
    pub iprior28: crate::Reg<iprior28::IPRIOR28_SPEC>,
    #[doc = "0x41d - Interrupt Priority Register"]
    pub iprior29: crate::Reg<iprior29::IPRIOR29_SPEC>,
    #[doc = "0x41e - Interrupt Priority Register"]
    pub iprior30: crate::Reg<iprior30::IPRIOR30_SPEC>,
    #[doc = "0x41f - Interrupt Priority Register"]
    pub iprior31: crate::Reg<iprior31::IPRIOR31_SPEC>,
    #[doc = "0x420 - Interrupt Priority Register"]
    pub iprior32: crate::Reg<iprior32::IPRIOR32_SPEC>,
    #[doc = "0x421 - Interrupt Priority Register"]
    pub iprior33: crate::Reg<iprior33::IPRIOR33_SPEC>,
    #[doc = "0x422 - Interrupt Priority Register"]
    pub iprior34: crate::Reg<iprior34::IPRIOR34_SPEC>,
    #[doc = "0x423 - Interrupt Priority Register"]
    pub iprior35: crate::Reg<iprior35::IPRIOR35_SPEC>,
    #[doc = "0x424 - Interrupt Priority Register"]
    pub iprior36: crate::Reg<iprior36::IPRIOR36_SPEC>,
    #[doc = "0x425 - Interrupt Priority Register"]
    pub iprior37: crate::Reg<iprior37::IPRIOR37_SPEC>,
    #[doc = "0x426 - Interrupt Priority Register"]
    pub iprior38: crate::Reg<iprior38::IPRIOR38_SPEC>,
    #[doc = "0x427 - Interrupt Priority Register"]
    pub iprior39: crate::Reg<iprior39::IPRIOR39_SPEC>,
    #[doc = "0x428 - Interrupt Priority Register"]
    pub iprior40: crate::Reg<iprior40::IPRIOR40_SPEC>,
    #[doc = "0x429 - Interrupt Priority Register"]
    pub iprior41: crate::Reg<iprior41::IPRIOR41_SPEC>,
    #[doc = "0x42a - Interrupt Priority Register"]
    pub iprior42: crate::Reg<iprior42::IPRIOR42_SPEC>,
    #[doc = "0x42b - Interrupt Priority Register"]
    pub iprior43: crate::Reg<iprior43::IPRIOR43_SPEC>,
    #[doc = "0x42c - Interrupt Priority Register"]
    pub iprior44: crate::Reg<iprior44::IPRIOR44_SPEC>,
    #[doc = "0x42d - Interrupt Priority Register"]
    pub iprior45: crate::Reg<iprior45::IPRIOR45_SPEC>,
    #[doc = "0x42e - Interrupt Priority Register"]
    pub iprior46: crate::Reg<iprior46::IPRIOR46_SPEC>,
    #[doc = "0x42f - Interrupt Priority Register"]
    pub iprior47: crate::Reg<iprior47::IPRIOR47_SPEC>,
    #[doc = "0x430 - Interrupt Priority Register"]
    pub iprior48: crate::Reg<iprior48::IPRIOR48_SPEC>,
    #[doc = "0x431 - Interrupt Priority Register"]
    pub iprior49: crate::Reg<iprior49::IPRIOR49_SPEC>,
    #[doc = "0x432 - Interrupt Priority Register"]
    pub iprior50: crate::Reg<iprior50::IPRIOR50_SPEC>,
    #[doc = "0x433 - Interrupt Priority Register"]
    pub iprior51: crate::Reg<iprior51::IPRIOR51_SPEC>,
    #[doc = "0x434 - Interrupt Priority Register"]
    pub iprior52: crate::Reg<iprior52::IPRIOR52_SPEC>,
    #[doc = "0x435 - Interrupt Priority Register"]
    pub iprior53: crate::Reg<iprior53::IPRIOR53_SPEC>,
    #[doc = "0x436 - Interrupt Priority Register"]
    pub iprior54: crate::Reg<iprior54::IPRIOR54_SPEC>,
    #[doc = "0x437 - Interrupt Priority Register"]
    pub iprior55: crate::Reg<iprior55::IPRIOR55_SPEC>,
    #[doc = "0x438 - Interrupt Priority Register"]
    pub iprior56: crate::Reg<iprior56::IPRIOR56_SPEC>,
    #[doc = "0x439 - Interrupt Priority Register"]
    pub iprior57: crate::Reg<iprior57::IPRIOR57_SPEC>,
    #[doc = "0x43a - Interrupt Priority Register"]
    pub iprior58: crate::Reg<iprior58::IPRIOR58_SPEC>,
    #[doc = "0x43b - Interrupt Priority Register"]
    pub iprior59: crate::Reg<iprior59::IPRIOR59_SPEC>,
    #[doc = "0x43c - Interrupt Priority Register"]
    pub iprior60: crate::Reg<iprior60::IPRIOR60_SPEC>,
    #[doc = "0x43d - Interrupt Priority Register"]
    pub iprior61: crate::Reg<iprior61::IPRIOR61_SPEC>,
    #[doc = "0x43e - Interrupt Priority Register"]
    pub iprior62: crate::Reg<iprior62::IPRIOR62_SPEC>,
    #[doc = "0x43f - Interrupt Priority Register"]
    pub iprior63: crate::Reg<iprior63::IPRIOR63_SPEC>,
    #[doc = "0x440 - Interrupt Priority Register"]
    pub iprior64: crate::Reg<iprior64::IPRIOR64_SPEC>,
    #[doc = "0x441 - Interrupt Priority Register"]
    pub iprior65: crate::Reg<iprior65::IPRIOR65_SPEC>,
    #[doc = "0x442 - Interrupt Priority Register"]
    pub iprior66: crate::Reg<iprior66::IPRIOR66_SPEC>,
    #[doc = "0x443 - Interrupt Priority Register"]
    pub iprior67: crate::Reg<iprior67::IPRIOR67_SPEC>,
    #[doc = "0x444 - Interrupt Priority Register"]
    pub iprior68: crate::Reg<iprior68::IPRIOR68_SPEC>,
    #[doc = "0x445 - Interrupt Priority Register"]
    pub iprior69: crate::Reg<iprior69::IPRIOR69_SPEC>,
    #[doc = "0x446 - Interrupt Priority Register"]
    pub iprior70: crate::Reg<iprior70::IPRIOR70_SPEC>,
    #[doc = "0x447 - Interrupt Priority Register"]
    pub iprior71: crate::Reg<iprior71::IPRIOR71_SPEC>,
    #[doc = "0x448 - Interrupt Priority Register"]
    pub iprior72: crate::Reg<iprior72::IPRIOR72_SPEC>,
    #[doc = "0x449 - Interrupt Priority Register"]
    pub iprior73: crate::Reg<iprior73::IPRIOR73_SPEC>,
    #[doc = "0x44a - Interrupt Priority Register"]
    pub iprior74: crate::Reg<iprior74::IPRIOR74_SPEC>,
    #[doc = "0x44b - Interrupt Priority Register"]
    pub iprior75: crate::Reg<iprior75::IPRIOR75_SPEC>,
    #[doc = "0x44c - Interrupt Priority Register"]
    pub iprior76: crate::Reg<iprior76::IPRIOR76_SPEC>,
    #[doc = "0x44d - Interrupt Priority Register"]
    pub iprior77: crate::Reg<iprior77::IPRIOR77_SPEC>,
    #[doc = "0x44e - Interrupt Priority Register"]
    pub iprior78: crate::Reg<iprior78::IPRIOR78_SPEC>,
    #[doc = "0x44f - Interrupt Priority Register"]
    pub iprior79: crate::Reg<iprior79::IPRIOR79_SPEC>,
    #[doc = "0x450 - Interrupt Priority Register"]
    pub iprior80: crate::Reg<iprior80::IPRIOR80_SPEC>,
    #[doc = "0x451 - Interrupt Priority Register"]
    pub iprior81: crate::Reg<iprior81::IPRIOR81_SPEC>,
    #[doc = "0x452 - Interrupt Priority Register"]
    pub iprior82: crate::Reg<iprior82::IPRIOR82_SPEC>,
    #[doc = "0x453 - Interrupt Priority Register"]
    pub iprior83: crate::Reg<iprior83::IPRIOR83_SPEC>,
    #[doc = "0x454 - Interrupt Priority Register"]
    pub iprior84: crate::Reg<iprior84::IPRIOR84_SPEC>,
    #[doc = "0x455 - Interrupt Priority Register"]
    pub iprior85: crate::Reg<iprior85::IPRIOR85_SPEC>,
    #[doc = "0x456 - Interrupt Priority Register"]
    pub iprior86: crate::Reg<iprior86::IPRIOR86_SPEC>,
    #[doc = "0x457 - Interrupt Priority Register"]
    pub iprior87: crate::Reg<iprior87::IPRIOR87_SPEC>,
    #[doc = "0x458 - Interrupt Priority Register"]
    pub iprior88: crate::Reg<iprior88::IPRIOR88_SPEC>,
    #[doc = "0x459 - Interrupt Priority Register"]
    pub iprior89: crate::Reg<iprior89::IPRIOR89_SPEC>,
    #[doc = "0x45a - Interrupt Priority Register"]
    pub iprior90: crate::Reg<iprior90::IPRIOR90_SPEC>,
    #[doc = "0x45b - Interrupt Priority Register"]
    pub iprior91: crate::Reg<iprior91::IPRIOR91_SPEC>,
    #[doc = "0x45c - Interrupt Priority Register"]
    pub iprior92: crate::Reg<iprior92::IPRIOR92_SPEC>,
    #[doc = "0x45d - Interrupt Priority Register"]
    pub iprior93: crate::Reg<iprior93::IPRIOR93_SPEC>,
    #[doc = "0x45e - Interrupt Priority Register"]
    pub iprior94: crate::Reg<iprior94::IPRIOR94_SPEC>,
    #[doc = "0x45f - Interrupt Priority Register"]
    pub iprior95: crate::Reg<iprior95::IPRIOR95_SPEC>,
    #[doc = "0x460 - Interrupt Priority Register"]
    pub iprior96: crate::Reg<iprior96::IPRIOR96_SPEC>,
    #[doc = "0x461 - Interrupt Priority Register"]
    pub iprior97: crate::Reg<iprior97::IPRIOR97_SPEC>,
    #[doc = "0x462 - Interrupt Priority Register"]
    pub iprior98: crate::Reg<iprior98::IPRIOR98_SPEC>,
    #[doc = "0x463 - Interrupt Priority Register"]
    pub iprior99: crate::Reg<iprior99::IPRIOR99_SPEC>,
    #[doc = "0x464 - Interrupt Priority Register"]
    pub iprior100: crate::Reg<iprior100::IPRIOR100_SPEC>,
    #[doc = "0x465 - Interrupt Priority Register"]
    pub iprior101: crate::Reg<iprior101::IPRIOR101_SPEC>,
    #[doc = "0x466 - Interrupt Priority Register"]
    pub iprior102: crate::Reg<iprior102::IPRIOR102_SPEC>,
    #[doc = "0x467 - Interrupt Priority Register"]
    pub iprior103: crate::Reg<iprior103::IPRIOR103_SPEC>,
    #[doc = "0x468 - Interrupt Priority Register"]
    pub iprior104: crate::Reg<iprior104::IPRIOR104_SPEC>,
    #[doc = "0x469 - Interrupt Priority Register"]
    pub iprior105: crate::Reg<iprior105::IPRIOR105_SPEC>,
    #[doc = "0x46a - Interrupt Priority Register"]
    pub iprior106: crate::Reg<iprior106::IPRIOR106_SPEC>,
    #[doc = "0x46b - Interrupt Priority Register"]
    pub iprior107: crate::Reg<iprior107::IPRIOR107_SPEC>,
    #[doc = "0x46c - Interrupt Priority Register"]
    pub iprior108: crate::Reg<iprior108::IPRIOR108_SPEC>,
    #[doc = "0x46d - Interrupt Priority Register"]
    pub iprior109: crate::Reg<iprior109::IPRIOR109_SPEC>,
    #[doc = "0x46e - Interrupt Priority Register"]
    pub iprior110: crate::Reg<iprior110::IPRIOR110_SPEC>,
    #[doc = "0x46f - Interrupt Priority Register"]
    pub iprior111: crate::Reg<iprior111::IPRIOR111_SPEC>,
    #[doc = "0x470 - Interrupt Priority Register"]
    pub iprior112: crate::Reg<iprior112::IPRIOR112_SPEC>,
    #[doc = "0x471 - Interrupt Priority Register"]
    pub iprior113: crate::Reg<iprior113::IPRIOR113_SPEC>,
    #[doc = "0x472 - Interrupt Priority Register"]
    pub iprior114: crate::Reg<iprior114::IPRIOR114_SPEC>,
    #[doc = "0x473 - Interrupt Priority Register"]
    pub iprior115: crate::Reg<iprior115::IPRIOR115_SPEC>,
    #[doc = "0x474 - Interrupt Priority Register"]
    pub iprior116: crate::Reg<iprior116::IPRIOR116_SPEC>,
    #[doc = "0x475 - Interrupt Priority Register"]
    pub iprior117: crate::Reg<iprior117::IPRIOR117_SPEC>,
    #[doc = "0x476 - Interrupt Priority Register"]
    pub iprior118: crate::Reg<iprior118::IPRIOR118_SPEC>,
    #[doc = "0x477 - Interrupt Priority Register"]
    pub iprior119: crate::Reg<iprior119::IPRIOR119_SPEC>,
    #[doc = "0x478 - Interrupt Priority Register"]
    pub iprior120: crate::Reg<iprior120::IPRIOR120_SPEC>,
    #[doc = "0x479 - Interrupt Priority Register"]
    pub iprior121: crate::Reg<iprior121::IPRIOR121_SPEC>,
    #[doc = "0x47a - Interrupt Priority Register"]
    pub iprior122: crate::Reg<iprior122::IPRIOR122_SPEC>,
    #[doc = "0x47b - Interrupt Priority Register"]
    pub iprior123: crate::Reg<iprior123::IPRIOR123_SPEC>,
    #[doc = "0x47c - Interrupt Priority Register"]
    pub iprior124: crate::Reg<iprior124::IPRIOR124_SPEC>,
    #[doc = "0x47d - Interrupt Priority Register"]
    pub iprior125: crate::Reg<iprior125::IPRIOR125_SPEC>,
    #[doc = "0x47e - Interrupt Priority Register"]
    pub iprior126: crate::Reg<iprior126::IPRIOR126_SPEC>,
    #[doc = "0x47f - Interrupt Priority Register"]
    pub iprior127: crate::Reg<iprior127::IPRIOR127_SPEC>,
    #[doc = "0x480 - Interrupt Priority Register"]
    pub iprior128: crate::Reg<iprior128::IPRIOR128_SPEC>,
    #[doc = "0x481 - Interrupt Priority Register"]
    pub iprior129: crate::Reg<iprior129::IPRIOR129_SPEC>,
    #[doc = "0x482 - Interrupt Priority Register"]
    pub iprior130: crate::Reg<iprior130::IPRIOR130_SPEC>,
    #[doc = "0x483 - Interrupt Priority Register"]
    pub iprior131: crate::Reg<iprior131::IPRIOR131_SPEC>,
    #[doc = "0x484 - Interrupt Priority Register"]
    pub iprior132: crate::Reg<iprior132::IPRIOR132_SPEC>,
    #[doc = "0x485 - Interrupt Priority Register"]
    pub iprior133: crate::Reg<iprior133::IPRIOR133_SPEC>,
    #[doc = "0x486 - Interrupt Priority Register"]
    pub iprior134: crate::Reg<iprior134::IPRIOR134_SPEC>,
    #[doc = "0x487 - Interrupt Priority Register"]
    pub iprior135: crate::Reg<iprior135::IPRIOR135_SPEC>,
    #[doc = "0x488 - Interrupt Priority Register"]
    pub iprior136: crate::Reg<iprior136::IPRIOR136_SPEC>,
    #[doc = "0x489 - Interrupt Priority Register"]
    pub iprior137: crate::Reg<iprior137::IPRIOR137_SPEC>,
    #[doc = "0x48a - Interrupt Priority Register"]
    pub iprior138: crate::Reg<iprior138::IPRIOR138_SPEC>,
    #[doc = "0x48b - Interrupt Priority Register"]
    pub iprior139: crate::Reg<iprior139::IPRIOR139_SPEC>,
    #[doc = "0x48c - Interrupt Priority Register"]
    pub iprior140: crate::Reg<iprior140::IPRIOR140_SPEC>,
    #[doc = "0x48d - Interrupt Priority Register"]
    pub iprior141: crate::Reg<iprior141::IPRIOR141_SPEC>,
    #[doc = "0x48e - Interrupt Priority Register"]
    pub iprior142: crate::Reg<iprior142::IPRIOR142_SPEC>,
    #[doc = "0x48f - Interrupt Priority Register"]
    pub iprior143: crate::Reg<iprior143::IPRIOR143_SPEC>,
    #[doc = "0x490 - Interrupt Priority Register"]
    pub iprior144: crate::Reg<iprior144::IPRIOR144_SPEC>,
    #[doc = "0x491 - Interrupt Priority Register"]
    pub iprior145: crate::Reg<iprior145::IPRIOR145_SPEC>,
    #[doc = "0x492 - Interrupt Priority Register"]
    pub iprior146: crate::Reg<iprior146::IPRIOR146_SPEC>,
    #[doc = "0x493 - Interrupt Priority Register"]
    pub iprior147: crate::Reg<iprior147::IPRIOR147_SPEC>,
    #[doc = "0x494 - Interrupt Priority Register"]
    pub iprior148: crate::Reg<iprior148::IPRIOR148_SPEC>,
    #[doc = "0x495 - Interrupt Priority Register"]
    pub iprior149: crate::Reg<iprior149::IPRIOR149_SPEC>,
    #[doc = "0x496 - Interrupt Priority Register"]
    pub iprior150: crate::Reg<iprior150::IPRIOR150_SPEC>,
    #[doc = "0x497 - Interrupt Priority Register"]
    pub iprior151: crate::Reg<iprior151::IPRIOR151_SPEC>,
    #[doc = "0x498 - Interrupt Priority Register"]
    pub iprior152: crate::Reg<iprior152::IPRIOR152_SPEC>,
    #[doc = "0x499 - Interrupt Priority Register"]
    pub iprior153: crate::Reg<iprior153::IPRIOR153_SPEC>,
    #[doc = "0x49a - Interrupt Priority Register"]
    pub iprior154: crate::Reg<iprior154::IPRIOR154_SPEC>,
    #[doc = "0x49b - Interrupt Priority Register"]
    pub iprior155: crate::Reg<iprior155::IPRIOR155_SPEC>,
    #[doc = "0x49c - Interrupt Priority Register"]
    pub iprior156: crate::Reg<iprior156::IPRIOR156_SPEC>,
    #[doc = "0x49d - Interrupt Priority Register"]
    pub iprior157: crate::Reg<iprior157::IPRIOR157_SPEC>,
    #[doc = "0x49e - Interrupt Priority Register"]
    pub iprior158: crate::Reg<iprior158::IPRIOR158_SPEC>,
    #[doc = "0x49f - Interrupt Priority Register"]
    pub iprior159: crate::Reg<iprior159::IPRIOR159_SPEC>,
    #[doc = "0x4a0 - Interrupt Priority Register"]
    pub iprior160: crate::Reg<iprior160::IPRIOR160_SPEC>,
    #[doc = "0x4a1 - Interrupt Priority Register"]
    pub iprior161: crate::Reg<iprior161::IPRIOR161_SPEC>,
    #[doc = "0x4a2 - Interrupt Priority Register"]
    pub iprior162: crate::Reg<iprior162::IPRIOR162_SPEC>,
    #[doc = "0x4a3 - Interrupt Priority Register"]
    pub iprior163: crate::Reg<iprior163::IPRIOR163_SPEC>,
    #[doc = "0x4a4 - Interrupt Priority Register"]
    pub iprior164: crate::Reg<iprior164::IPRIOR164_SPEC>,
    #[doc = "0x4a5 - Interrupt Priority Register"]
    pub iprior165: crate::Reg<iprior165::IPRIOR165_SPEC>,
    #[doc = "0x4a6 - Interrupt Priority Register"]
    pub iprior166: crate::Reg<iprior166::IPRIOR166_SPEC>,
    #[doc = "0x4a7 - Interrupt Priority Register"]
    pub iprior167: crate::Reg<iprior167::IPRIOR167_SPEC>,
    #[doc = "0x4a8 - Interrupt Priority Register"]
    pub iprior168: crate::Reg<iprior168::IPRIOR168_SPEC>,
    #[doc = "0x4a9 - Interrupt Priority Register"]
    pub iprior169: crate::Reg<iprior169::IPRIOR169_SPEC>,
    #[doc = "0x4aa - Interrupt Priority Register"]
    pub iprior170: crate::Reg<iprior170::IPRIOR170_SPEC>,
    #[doc = "0x4ab - Interrupt Priority Register"]
    pub iprior171: crate::Reg<iprior171::IPRIOR171_SPEC>,
    #[doc = "0x4ac - Interrupt Priority Register"]
    pub iprior172: crate::Reg<iprior172::IPRIOR172_SPEC>,
    #[doc = "0x4ad - Interrupt Priority Register"]
    pub iprior173: crate::Reg<iprior173::IPRIOR173_SPEC>,
    #[doc = "0x4ae - Interrupt Priority Register"]
    pub iprior174: crate::Reg<iprior174::IPRIOR174_SPEC>,
    #[doc = "0x4af - Interrupt Priority Register"]
    pub iprior175: crate::Reg<iprior175::IPRIOR175_SPEC>,
    #[doc = "0x4b0 - Interrupt Priority Register"]
    pub iprior176: crate::Reg<iprior176::IPRIOR176_SPEC>,
    #[doc = "0x4b1 - Interrupt Priority Register"]
    pub iprior177: crate::Reg<iprior177::IPRIOR177_SPEC>,
    #[doc = "0x4b2 - Interrupt Priority Register"]
    pub iprior178: crate::Reg<iprior178::IPRIOR178_SPEC>,
    #[doc = "0x4b3 - Interrupt Priority Register"]
    pub iprior179: crate::Reg<iprior179::IPRIOR179_SPEC>,
    #[doc = "0x4b4 - Interrupt Priority Register"]
    pub iprior180: crate::Reg<iprior180::IPRIOR180_SPEC>,
    #[doc = "0x4b5 - Interrupt Priority Register"]
    pub iprior181: crate::Reg<iprior181::IPRIOR181_SPEC>,
    #[doc = "0x4b6 - Interrupt Priority Register"]
    pub iprior182: crate::Reg<iprior182::IPRIOR182_SPEC>,
    #[doc = "0x4b7 - Interrupt Priority Register"]
    pub iprior183: crate::Reg<iprior183::IPRIOR183_SPEC>,
    #[doc = "0x4b8 - Interrupt Priority Register"]
    pub iprior184: crate::Reg<iprior184::IPRIOR184_SPEC>,
    #[doc = "0x4b9 - Interrupt Priority Register"]
    pub iprior185: crate::Reg<iprior185::IPRIOR185_SPEC>,
    #[doc = "0x4ba - Interrupt Priority Register"]
    pub iprior186: crate::Reg<iprior186::IPRIOR186_SPEC>,
    #[doc = "0x4bb - Interrupt Priority Register"]
    pub iprior187: crate::Reg<iprior187::IPRIOR187_SPEC>,
    #[doc = "0x4bc - Interrupt Priority Register"]
    pub iprior188: crate::Reg<iprior188::IPRIOR188_SPEC>,
    #[doc = "0x4bd - Interrupt Priority Register"]
    pub iprior189: crate::Reg<iprior189::IPRIOR189_SPEC>,
    #[doc = "0x4be - Interrupt Priority Register"]
    pub iprior190: crate::Reg<iprior190::IPRIOR190_SPEC>,
    #[doc = "0x4bf - Interrupt Priority Register"]
    pub iprior191: crate::Reg<iprior191::IPRIOR191_SPEC>,
    #[doc = "0x4c0 - Interrupt Priority Register"]
    pub iprior192: crate::Reg<iprior192::IPRIOR192_SPEC>,
    #[doc = "0x4c1 - Interrupt Priority Register"]
    pub iprior193: crate::Reg<iprior193::IPRIOR193_SPEC>,
    #[doc = "0x4c2 - Interrupt Priority Register"]
    pub iprior194: crate::Reg<iprior194::IPRIOR194_SPEC>,
    #[doc = "0x4c3 - Interrupt Priority Register"]
    pub iprior195: crate::Reg<iprior195::IPRIOR195_SPEC>,
    #[doc = "0x4c4 - Interrupt Priority Register"]
    pub iprior196: crate::Reg<iprior196::IPRIOR196_SPEC>,
    #[doc = "0x4c5 - Interrupt Priority Register"]
    pub iprior197: crate::Reg<iprior197::IPRIOR197_SPEC>,
    #[doc = "0x4c6 - Interrupt Priority Register"]
    pub iprior198: crate::Reg<iprior198::IPRIOR198_SPEC>,
    #[doc = "0x4c7 - Interrupt Priority Register"]
    pub iprior199: crate::Reg<iprior199::IPRIOR199_SPEC>,
    #[doc = "0x4c8 - Interrupt Priority Register"]
    pub iprior200: crate::Reg<iprior200::IPRIOR200_SPEC>,
    #[doc = "0x4c9 - Interrupt Priority Register"]
    pub iprior201: crate::Reg<iprior201::IPRIOR201_SPEC>,
    #[doc = "0x4ca - Interrupt Priority Register"]
    pub iprior202: crate::Reg<iprior202::IPRIOR202_SPEC>,
    #[doc = "0x4cb - Interrupt Priority Register"]
    pub iprior203: crate::Reg<iprior203::IPRIOR203_SPEC>,
    #[doc = "0x4cc - Interrupt Priority Register"]
    pub iprior204: crate::Reg<iprior204::IPRIOR204_SPEC>,
    #[doc = "0x4cd - Interrupt Priority Register"]
    pub iprior205: crate::Reg<iprior205::IPRIOR205_SPEC>,
    #[doc = "0x4ce - Interrupt Priority Register"]
    pub iprior206: crate::Reg<iprior206::IPRIOR206_SPEC>,
    #[doc = "0x4cf - Interrupt Priority Register"]
    pub iprior207: crate::Reg<iprior207::IPRIOR207_SPEC>,
    #[doc = "0x4d0 - Interrupt Priority Register"]
    pub iprior208: crate::Reg<iprior208::IPRIOR208_SPEC>,
    #[doc = "0x4d1 - Interrupt Priority Register"]
    pub iprior209: crate::Reg<iprior209::IPRIOR209_SPEC>,
    #[doc = "0x4d2 - Interrupt Priority Register"]
    pub iprior210: crate::Reg<iprior210::IPRIOR210_SPEC>,
    #[doc = "0x4d3 - Interrupt Priority Register"]
    pub iprior211: crate::Reg<iprior211::IPRIOR211_SPEC>,
    #[doc = "0x4d4 - Interrupt Priority Register"]
    pub iprior212: crate::Reg<iprior212::IPRIOR212_SPEC>,
    #[doc = "0x4d5 - Interrupt Priority Register"]
    pub iprior213: crate::Reg<iprior213::IPRIOR213_SPEC>,
    #[doc = "0x4d6 - Interrupt Priority Register"]
    pub iprior214: crate::Reg<iprior214::IPRIOR214_SPEC>,
    #[doc = "0x4d7 - Interrupt Priority Register"]
    pub iprior215: crate::Reg<iprior215::IPRIOR215_SPEC>,
    #[doc = "0x4d8 - Interrupt Priority Register"]
    pub iprior216: crate::Reg<iprior216::IPRIOR216_SPEC>,
    #[doc = "0x4d9 - Interrupt Priority Register"]
    pub iprior217: crate::Reg<iprior217::IPRIOR217_SPEC>,
    #[doc = "0x4da - Interrupt Priority Register"]
    pub iprior218: crate::Reg<iprior218::IPRIOR218_SPEC>,
    #[doc = "0x4db - Interrupt Priority Register"]
    pub iprior219: crate::Reg<iprior219::IPRIOR219_SPEC>,
    #[doc = "0x4dc - Interrupt Priority Register"]
    pub iprior220: crate::Reg<iprior220::IPRIOR220_SPEC>,
    #[doc = "0x4dd - Interrupt Priority Register"]
    pub iprior221: crate::Reg<iprior221::IPRIOR221_SPEC>,
    #[doc = "0x4de - Interrupt Priority Register"]
    pub iprior222: crate::Reg<iprior222::IPRIOR222_SPEC>,
    #[doc = "0x4df - Interrupt Priority Register"]
    pub iprior223: crate::Reg<iprior223::IPRIOR223_SPEC>,
    #[doc = "0x4e0 - Interrupt Priority Register"]
    pub iprior224: crate::Reg<iprior224::IPRIOR224_SPEC>,
    #[doc = "0x4e1 - Interrupt Priority Register"]
    pub iprior225: crate::Reg<iprior225::IPRIOR225_SPEC>,
    #[doc = "0x4e2 - Interrupt Priority Register"]
    pub iprior226: crate::Reg<iprior226::IPRIOR226_SPEC>,
    #[doc = "0x4e3 - Interrupt Priority Register"]
    pub iprior227: crate::Reg<iprior227::IPRIOR227_SPEC>,
    #[doc = "0x4e4 - Interrupt Priority Register"]
    pub iprior228: crate::Reg<iprior228::IPRIOR228_SPEC>,
    #[doc = "0x4e5 - Interrupt Priority Register"]
    pub iprior229: crate::Reg<iprior229::IPRIOR229_SPEC>,
    #[doc = "0x4e6 - Interrupt Priority Register"]
    pub iprior230: crate::Reg<iprior230::IPRIOR230_SPEC>,
    #[doc = "0x4e7 - Interrupt Priority Register"]
    pub iprior231: crate::Reg<iprior231::IPRIOR231_SPEC>,
    #[doc = "0x4e8 - Interrupt Priority Register"]
    pub iprior232: crate::Reg<iprior232::IPRIOR232_SPEC>,
    #[doc = "0x4e9 - Interrupt Priority Register"]
    pub iprior233: crate::Reg<iprior233::IPRIOR233_SPEC>,
    #[doc = "0x4ea - Interrupt Priority Register"]
    pub iprior234: crate::Reg<iprior234::IPRIOR234_SPEC>,
    #[doc = "0x4eb - Interrupt Priority Register"]
    pub iprior235: crate::Reg<iprior235::IPRIOR235_SPEC>,
    #[doc = "0x4ec - Interrupt Priority Register"]
    pub iprior236: crate::Reg<iprior236::IPRIOR236_SPEC>,
    #[doc = "0x4ed - Interrupt Priority Register"]
    pub iprior237: crate::Reg<iprior237::IPRIOR237_SPEC>,
    #[doc = "0x4ee - Interrupt Priority Register"]
    pub iprior238: crate::Reg<iprior238::IPRIOR238_SPEC>,
    #[doc = "0x4ef - Interrupt Priority Register"]
    pub iprior239: crate::Reg<iprior239::IPRIOR239_SPEC>,
    #[doc = "0x4f0 - Interrupt Priority Register"]
    pub iprior240: crate::Reg<iprior240::IPRIOR240_SPEC>,
    #[doc = "0x4f1 - Interrupt Priority Register"]
    pub iprior241: crate::Reg<iprior241::IPRIOR241_SPEC>,
    #[doc = "0x4f2 - Interrupt Priority Register"]
    pub iprior242: crate::Reg<iprior242::IPRIOR242_SPEC>,
    #[doc = "0x4f3 - Interrupt Priority Register"]
    pub iprior243: crate::Reg<iprior243::IPRIOR243_SPEC>,
    #[doc = "0x4f4 - Interrupt Priority Register"]
    pub iprior244: crate::Reg<iprior244::IPRIOR244_SPEC>,
    #[doc = "0x4f5 - Interrupt Priority Register"]
    pub iprior245: crate::Reg<iprior245::IPRIOR245_SPEC>,
    #[doc = "0x4f6 - Interrupt Priority Register"]
    pub iprior246: crate::Reg<iprior246::IPRIOR246_SPEC>,
    #[doc = "0x4f7 - Interrupt Priority Register"]
    pub iprior247: crate::Reg<iprior247::IPRIOR247_SPEC>,
    _reserved267: [u8; 0x01],
    #[doc = "0x4f9 - Interrupt Priority Register"]
    pub iprior249: crate::Reg<iprior249::IPRIOR249_SPEC>,
    #[doc = "0x4fa - Interrupt Priority Register"]
    pub iprior250: crate::Reg<iprior250::IPRIOR250_SPEC>,
    #[doc = "0x4fb - Interrupt Priority Register"]
    pub iprior251: crate::Reg<iprior251::IPRIOR251_SPEC>,
    #[doc = "0x4fc - Interrupt Priority Register"]
    pub iprior252: crate::Reg<iprior252::IPRIOR252_SPEC>,
    #[doc = "0x4fd - Interrupt Priority Register"]
    pub iprior253: crate::Reg<iprior253::IPRIOR253_SPEC>,
    #[doc = "0x4fe - Interrupt Priority Register"]
    pub iprior254: crate::Reg<iprior254::IPRIOR254_SPEC>,
    #[doc = "0x4ff - Interrupt Priority Register"]
    pub iprior255: crate::Reg<iprior255::IPRIOR255_SPEC>,
    _reserved274: [u8; 0x0810],
    #[doc = "0xd10 - System Control Register"]
    pub sctlr: crate::Reg<sctlr::SCTLR_SPEC>,
    _reserved275: [u8; 0x02ec],
    #[doc = "0x1000 - System counter control register"]
    pub stk_ctlr: crate::Reg<stk_ctlr::STK_CTLR_SPEC>,
    _reserved_276_stk: [u8; 0x04],
    #[doc = "0x1008 - System counter high register"]
    pub stk_cnth: crate::Reg<stk_cnth::STK_CNTH_SPEC>,
    #[doc = "0x100c - System compare low register"]
    pub stk_cmplr: crate::Reg<stk_cmplr::STK_CMPLR_SPEC>,
    #[doc = "0x1010 - System compare high register"]
    pub stk_cmphr: crate::Reg<stk_cmphr::STK_CMPHR_SPEC>,
    _reserved280: [u8; 0x3ef4],
    #[doc = "0x4f08 - Interrupt Priority Register"]
    pub iprior248: crate::Reg<iprior248::IPRIOR248_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x4c - ID Config Register"]
    #[inline(always)]
    pub fn idcfgr(&self) -> &crate::Reg<idcfgr::IDCFGR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(76usize)
                as *const crate::Reg<idcfgr::IDCFGR_SPEC>)
        }
    }
    #[doc = "0x4c - Interrupt Global Register"]
    #[inline(always)]
    pub fn gisr(&self) -> &crate::Reg<gisr::GISR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(76usize)
                as *const crate::Reg<gisr::GISR_SPEC>)
        }
    }
    #[doc = "0x60 - Interrupt 3 address Register"]
    #[inline(always)]
    pub fn fifoaddrr3(&self) -> &crate::Reg<fifoaddrr3::FIFOADDRR3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(96usize)
                as *const crate::Reg<fifoaddrr3::FIFOADDRR3_SPEC>)
        }
    }
    #[doc = "0x60 - Interrupt 2 address Register"]
    #[inline(always)]
    pub fn fifoaddrr2(&self) -> &crate::Reg<fifoaddrr2::FIFOADDRR2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(96usize)
                as *const crate::Reg<fifoaddrr2::FIFOADDRR2_SPEC>)
        }
    }
    #[doc = "0x60 - Interrupt 1 address Register"]
    #[inline(always)]
    pub fn fifoaddrr1(&self) -> &crate::Reg<fifoaddrr1::FIFOADDRR1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(96usize)
                as *const crate::Reg<fifoaddrr1::FIFOADDRR1_SPEC>)
        }
    }
    #[doc = "0x60 - Interrupt 0 address Register"]
    #[inline(always)]
    pub fn fifoaddrr0(&self) -> &crate::Reg<fifoaddrr0::FIFOADDRR0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(96usize)
                as *const crate::Reg<fifoaddrr0::FIFOADDRR0_SPEC>)
        }
    }
    #[doc = "0x1004 - System counter low register"]
    #[inline(always)]
    pub fn stk_cntl(&self) -> &crate::Reg<stk_cntl::STK_CNTL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4100usize)
                as *const crate::Reg<stk_cntl::STK_CNTL_SPEC>)
        }
    }
    #[doc = "0x1004 - System START"]
    #[inline(always)]
    pub fn stk_sr(&self) -> &crate::Reg<stk_sr::STK_SR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4100usize)
                as *const crate::Reg<stk_sr::STK_SR_SPEC>)
        }
    }
}

#[doc = "ISR1 register accessor: an alias for `Reg<ISR1_SPEC>`"]
pub type ISR1 = crate::Reg<isr1::ISR1_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr1;

#[doc = "ISR2 register accessor: an alias for `Reg<ISR2_SPEC>`"]
pub type ISR2 = crate::Reg<isr2::ISR2_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr2;

#[doc = "IPR1 register accessor: an alias for `Reg<IPR1_SPEC>`"]
pub type IPR1 = crate::Reg<ipr1::IPR1_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod ipr1;

#[doc = "IPR2 register accessor: an alias for `Reg<IPR2_SPEC>`"]
pub type IPR2 = crate::Reg<ipr2::IPR2_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod ipr2;

#[doc = "ITHRESDR register accessor: an alias for `Reg<ITHRESDR_SPEC>`"]
pub type ITHRESDR = crate::Reg<ithresdr::ITHRESDR_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod ithresdr;

#[doc = "FIBADDRR register accessor: an alias for `Reg<FIBADDRR_SPEC>`"]
pub type FIBADDRR = crate::Reg<fibaddrr::FIBADDRR_SPEC>;
#[doc = "Interrupt Fast Address Register"]
pub mod fibaddrr;

#[doc = "CFGR register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "Interrupt Config Register"]
pub mod cfgr;

#[doc = "GISR register accessor: an alias for `Reg<GISR_SPEC>`"]
pub type GISR = crate::Reg<gisr::GISR_SPEC>;
#[doc = "Interrupt Global Register"]
pub mod gisr;

#[doc = "IDCFGR register accessor: an alias for `Reg<IDCFGR_SPEC>`"]
pub type IDCFGR = crate::Reg<idcfgr::IDCFGR_SPEC>;
#[doc = "ID Config Register"]
pub mod idcfgr;

#[doc = "FIFOADDRR0 register accessor: an alias for `Reg<FIFOADDRR0_SPEC>`"]
pub type FIFOADDRR0 = crate::Reg<fifoaddrr0::FIFOADDRR0_SPEC>;
#[doc = "Interrupt 0 address Register"]
pub mod fifoaddrr0;

#[doc = "FIFOADDRR1 register accessor: an alias for `Reg<FIFOADDRR1_SPEC>`"]
pub type FIFOADDRR1 = crate::Reg<fifoaddrr1::FIFOADDRR1_SPEC>;
#[doc = "Interrupt 1 address Register"]
pub mod fifoaddrr1;

#[doc = "FIFOADDRR2 register accessor: an alias for `Reg<FIFOADDRR2_SPEC>`"]
pub type FIFOADDRR2 = crate::Reg<fifoaddrr2::FIFOADDRR2_SPEC>;
#[doc = "Interrupt 2 address Register"]
pub mod fifoaddrr2;

#[doc = "FIFOADDRR3 register accessor: an alias for `Reg<FIFOADDRR3_SPEC>`"]
pub type FIFOADDRR3 = crate::Reg<fifoaddrr3::FIFOADDRR3_SPEC>;
#[doc = "Interrupt 3 address Register"]
pub mod fifoaddrr3;

#[doc = "IENR1 register accessor: an alias for `Reg<IENR1_SPEC>`"]
pub type IENR1 = crate::Reg<ienr1::IENR1_SPEC>;
#[doc = "Interrupt Setting Register"]
pub mod ienr1;

#[doc = "IENR2 register accessor: an alias for `Reg<IENR2_SPEC>`"]
pub type IENR2 = crate::Reg<ienr2::IENR2_SPEC>;
#[doc = "Interrupt Setting Register"]
pub mod ienr2;

#[doc = "IRER1 register accessor: an alias for `Reg<IRER1_SPEC>`"]
pub type IRER1 = crate::Reg<irer1::IRER1_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod irer1;

#[doc = "IRER2 register accessor: an alias for `Reg<IRER2_SPEC>`"]
pub type IRER2 = crate::Reg<irer2::IRER2_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod irer2;

#[doc = "IPSR1 register accessor: an alias for `Reg<IPSR1_SPEC>`"]
pub type IPSR1 = crate::Reg<ipsr1::IPSR1_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod ipsr1;

#[doc = "IPSR2 register accessor: an alias for `Reg<IPSR2_SPEC>`"]
pub type IPSR2 = crate::Reg<ipsr2::IPSR2_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod ipsr2;

#[doc = "IPRR1 register accessor: an alias for `Reg<IPRR1_SPEC>`"]
pub type IPRR1 = crate::Reg<iprr1::IPRR1_SPEC>;
#[doc = "Interrupt Pending Clear Register"]
pub mod iprr1;

#[doc = "IPRR2 register accessor: an alias for `Reg<IPRR2_SPEC>`"]
pub type IPRR2 = crate::Reg<iprr2::IPRR2_SPEC>;
#[doc = "Interrupt Pending Clear Register"]
pub mod iprr2;

#[doc = "IACTR1 register accessor: an alias for `Reg<IACTR1_SPEC>`"]
pub type IACTR1 = crate::Reg<iactr1::IACTR1_SPEC>;
#[doc = "Interrupt ACTIVE Register"]
pub mod iactr1;

#[doc = "IACTR2 register accessor: an alias for `Reg<IACTR2_SPEC>`"]
pub type IACTR2 = crate::Reg<iactr2::IACTR2_SPEC>;
#[doc = "Interrupt ACTIVE Register"]
pub mod iactr2;

#[doc = "IPRIOR0 register accessor: an alias for `Reg<IPRIOR0_SPEC>`"]
pub type IPRIOR0 = crate::Reg<iprior0::IPRIOR0_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior0;

#[doc = "IPRIOR1 register accessor: an alias for `Reg<IPRIOR1_SPEC>`"]
pub type IPRIOR1 = crate::Reg<iprior1::IPRIOR1_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior1;

#[doc = "IPRIOR2 register accessor: an alias for `Reg<IPRIOR2_SPEC>`"]
pub type IPRIOR2 = crate::Reg<iprior2::IPRIOR2_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior2;

#[doc = "IPRIOR3 register accessor: an alias for `Reg<IPRIOR3_SPEC>`"]
pub type IPRIOR3 = crate::Reg<iprior3::IPRIOR3_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior3;

#[doc = "IPRIOR4 register accessor: an alias for `Reg<IPRIOR4_SPEC>`"]
pub type IPRIOR4 = crate::Reg<iprior4::IPRIOR4_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior4;

#[doc = "IPRIOR5 register accessor: an alias for `Reg<IPRIOR5_SPEC>`"]
pub type IPRIOR5 = crate::Reg<iprior5::IPRIOR5_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior5;

#[doc = "IPRIOR6 register accessor: an alias for `Reg<IPRIOR6_SPEC>`"]
pub type IPRIOR6 = crate::Reg<iprior6::IPRIOR6_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior6;

#[doc = "IPRIOR7 register accessor: an alias for `Reg<IPRIOR7_SPEC>`"]
pub type IPRIOR7 = crate::Reg<iprior7::IPRIOR7_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior7;

#[doc = "IPRIOR8 register accessor: an alias for `Reg<IPRIOR8_SPEC>`"]
pub type IPRIOR8 = crate::Reg<iprior8::IPRIOR8_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior8;

#[doc = "IPRIOR9 register accessor: an alias for `Reg<IPRIOR9_SPEC>`"]
pub type IPRIOR9 = crate::Reg<iprior9::IPRIOR9_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior9;

#[doc = "IPRIOR10 register accessor: an alias for `Reg<IPRIOR10_SPEC>`"]
pub type IPRIOR10 = crate::Reg<iprior10::IPRIOR10_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior10;

#[doc = "IPRIOR11 register accessor: an alias for `Reg<IPRIOR11_SPEC>`"]
pub type IPRIOR11 = crate::Reg<iprior11::IPRIOR11_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior11;

#[doc = "IPRIOR12 register accessor: an alias for `Reg<IPRIOR12_SPEC>`"]
pub type IPRIOR12 = crate::Reg<iprior12::IPRIOR12_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior12;

#[doc = "IPRIOR13 register accessor: an alias for `Reg<IPRIOR13_SPEC>`"]
pub type IPRIOR13 = crate::Reg<iprior13::IPRIOR13_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior13;

#[doc = "IPRIOR14 register accessor: an alias for `Reg<IPRIOR14_SPEC>`"]
pub type IPRIOR14 = crate::Reg<iprior14::IPRIOR14_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior14;

#[doc = "IPRIOR15 register accessor: an alias for `Reg<IPRIOR15_SPEC>`"]
pub type IPRIOR15 = crate::Reg<iprior15::IPRIOR15_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior15;

#[doc = "IPRIOR16 register accessor: an alias for `Reg<IPRIOR16_SPEC>`"]
pub type IPRIOR16 = crate::Reg<iprior16::IPRIOR16_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior16;

#[doc = "IPRIOR17 register accessor: an alias for `Reg<IPRIOR17_SPEC>`"]
pub type IPRIOR17 = crate::Reg<iprior17::IPRIOR17_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior17;

#[doc = "IPRIOR18 register accessor: an alias for `Reg<IPRIOR18_SPEC>`"]
pub type IPRIOR18 = crate::Reg<iprior18::IPRIOR18_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior18;

#[doc = "IPRIOR19 register accessor: an alias for `Reg<IPRIOR19_SPEC>`"]
pub type IPRIOR19 = crate::Reg<iprior19::IPRIOR19_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior19;

#[doc = "IPRIOR20 register accessor: an alias for `Reg<IPRIOR20_SPEC>`"]
pub type IPRIOR20 = crate::Reg<iprior20::IPRIOR20_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior20;

#[doc = "IPRIOR21 register accessor: an alias for `Reg<IPRIOR21_SPEC>`"]
pub type IPRIOR21 = crate::Reg<iprior21::IPRIOR21_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior21;

#[doc = "IPRIOR22 register accessor: an alias for `Reg<IPRIOR22_SPEC>`"]
pub type IPRIOR22 = crate::Reg<iprior22::IPRIOR22_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior22;

#[doc = "IPRIOR23 register accessor: an alias for `Reg<IPRIOR23_SPEC>`"]
pub type IPRIOR23 = crate::Reg<iprior23::IPRIOR23_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior23;

#[doc = "IPRIOR24 register accessor: an alias for `Reg<IPRIOR24_SPEC>`"]
pub type IPRIOR24 = crate::Reg<iprior24::IPRIOR24_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior24;

#[doc = "IPRIOR25 register accessor: an alias for `Reg<IPRIOR25_SPEC>`"]
pub type IPRIOR25 = crate::Reg<iprior25::IPRIOR25_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior25;

#[doc = "IPRIOR26 register accessor: an alias for `Reg<IPRIOR26_SPEC>`"]
pub type IPRIOR26 = crate::Reg<iprior26::IPRIOR26_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior26;

#[doc = "IPRIOR27 register accessor: an alias for `Reg<IPRIOR27_SPEC>`"]
pub type IPRIOR27 = crate::Reg<iprior27::IPRIOR27_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior27;

#[doc = "IPRIOR28 register accessor: an alias for `Reg<IPRIOR28_SPEC>`"]
pub type IPRIOR28 = crate::Reg<iprior28::IPRIOR28_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior28;

#[doc = "IPRIOR29 register accessor: an alias for `Reg<IPRIOR29_SPEC>`"]
pub type IPRIOR29 = crate::Reg<iprior29::IPRIOR29_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior29;

#[doc = "IPRIOR30 register accessor: an alias for `Reg<IPRIOR30_SPEC>`"]
pub type IPRIOR30 = crate::Reg<iprior30::IPRIOR30_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior30;

#[doc = "IPRIOR31 register accessor: an alias for `Reg<IPRIOR31_SPEC>`"]
pub type IPRIOR31 = crate::Reg<iprior31::IPRIOR31_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior31;

#[doc = "IPRIOR32 register accessor: an alias for `Reg<IPRIOR32_SPEC>`"]
pub type IPRIOR32 = crate::Reg<iprior32::IPRIOR32_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior32;

#[doc = "IPRIOR33 register accessor: an alias for `Reg<IPRIOR33_SPEC>`"]
pub type IPRIOR33 = crate::Reg<iprior33::IPRIOR33_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior33;

#[doc = "IPRIOR34 register accessor: an alias for `Reg<IPRIOR34_SPEC>`"]
pub type IPRIOR34 = crate::Reg<iprior34::IPRIOR34_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior34;

#[doc = "IPRIOR35 register accessor: an alias for `Reg<IPRIOR35_SPEC>`"]
pub type IPRIOR35 = crate::Reg<iprior35::IPRIOR35_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior35;

#[doc = "IPRIOR36 register accessor: an alias for `Reg<IPRIOR36_SPEC>`"]
pub type IPRIOR36 = crate::Reg<iprior36::IPRIOR36_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior36;

#[doc = "IPRIOR37 register accessor: an alias for `Reg<IPRIOR37_SPEC>`"]
pub type IPRIOR37 = crate::Reg<iprior37::IPRIOR37_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior37;

#[doc = "IPRIOR38 register accessor: an alias for `Reg<IPRIOR38_SPEC>`"]
pub type IPRIOR38 = crate::Reg<iprior38::IPRIOR38_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior38;

#[doc = "IPRIOR39 register accessor: an alias for `Reg<IPRIOR39_SPEC>`"]
pub type IPRIOR39 = crate::Reg<iprior39::IPRIOR39_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior39;

#[doc = "IPRIOR40 register accessor: an alias for `Reg<IPRIOR40_SPEC>`"]
pub type IPRIOR40 = crate::Reg<iprior40::IPRIOR40_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior40;

#[doc = "IPRIOR41 register accessor: an alias for `Reg<IPRIOR41_SPEC>`"]
pub type IPRIOR41 = crate::Reg<iprior41::IPRIOR41_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior41;

#[doc = "IPRIOR42 register accessor: an alias for `Reg<IPRIOR42_SPEC>`"]
pub type IPRIOR42 = crate::Reg<iprior42::IPRIOR42_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior42;

#[doc = "IPRIOR43 register accessor: an alias for `Reg<IPRIOR43_SPEC>`"]
pub type IPRIOR43 = crate::Reg<iprior43::IPRIOR43_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior43;

#[doc = "IPRIOR44 register accessor: an alias for `Reg<IPRIOR44_SPEC>`"]
pub type IPRIOR44 = crate::Reg<iprior44::IPRIOR44_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior44;

#[doc = "IPRIOR45 register accessor: an alias for `Reg<IPRIOR45_SPEC>`"]
pub type IPRIOR45 = crate::Reg<iprior45::IPRIOR45_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior45;

#[doc = "IPRIOR46 register accessor: an alias for `Reg<IPRIOR46_SPEC>`"]
pub type IPRIOR46 = crate::Reg<iprior46::IPRIOR46_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior46;

#[doc = "IPRIOR47 register accessor: an alias for `Reg<IPRIOR47_SPEC>`"]
pub type IPRIOR47 = crate::Reg<iprior47::IPRIOR47_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior47;

#[doc = "IPRIOR48 register accessor: an alias for `Reg<IPRIOR48_SPEC>`"]
pub type IPRIOR48 = crate::Reg<iprior48::IPRIOR48_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior48;

#[doc = "IPRIOR49 register accessor: an alias for `Reg<IPRIOR49_SPEC>`"]
pub type IPRIOR49 = crate::Reg<iprior49::IPRIOR49_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior49;

#[doc = "IPRIOR50 register accessor: an alias for `Reg<IPRIOR50_SPEC>`"]
pub type IPRIOR50 = crate::Reg<iprior50::IPRIOR50_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior50;

#[doc = "IPRIOR51 register accessor: an alias for `Reg<IPRIOR51_SPEC>`"]
pub type IPRIOR51 = crate::Reg<iprior51::IPRIOR51_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior51;

#[doc = "IPRIOR52 register accessor: an alias for `Reg<IPRIOR52_SPEC>`"]
pub type IPRIOR52 = crate::Reg<iprior52::IPRIOR52_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior52;

#[doc = "IPRIOR53 register accessor: an alias for `Reg<IPRIOR53_SPEC>`"]
pub type IPRIOR53 = crate::Reg<iprior53::IPRIOR53_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior53;

#[doc = "IPRIOR54 register accessor: an alias for `Reg<IPRIOR54_SPEC>`"]
pub type IPRIOR54 = crate::Reg<iprior54::IPRIOR54_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior54;

#[doc = "IPRIOR55 register accessor: an alias for `Reg<IPRIOR55_SPEC>`"]
pub type IPRIOR55 = crate::Reg<iprior55::IPRIOR55_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior55;

#[doc = "IPRIOR56 register accessor: an alias for `Reg<IPRIOR56_SPEC>`"]
pub type IPRIOR56 = crate::Reg<iprior56::IPRIOR56_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior56;

#[doc = "IPRIOR57 register accessor: an alias for `Reg<IPRIOR57_SPEC>`"]
pub type IPRIOR57 = crate::Reg<iprior57::IPRIOR57_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior57;

#[doc = "IPRIOR58 register accessor: an alias for `Reg<IPRIOR58_SPEC>`"]
pub type IPRIOR58 = crate::Reg<iprior58::IPRIOR58_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior58;

#[doc = "IPRIOR59 register accessor: an alias for `Reg<IPRIOR59_SPEC>`"]
pub type IPRIOR59 = crate::Reg<iprior59::IPRIOR59_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior59;

#[doc = "IPRIOR60 register accessor: an alias for `Reg<IPRIOR60_SPEC>`"]
pub type IPRIOR60 = crate::Reg<iprior60::IPRIOR60_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior60;

#[doc = "IPRIOR61 register accessor: an alias for `Reg<IPRIOR61_SPEC>`"]
pub type IPRIOR61 = crate::Reg<iprior61::IPRIOR61_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior61;

#[doc = "IPRIOR62 register accessor: an alias for `Reg<IPRIOR62_SPEC>`"]
pub type IPRIOR62 = crate::Reg<iprior62::IPRIOR62_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior62;

#[doc = "IPRIOR63 register accessor: an alias for `Reg<IPRIOR63_SPEC>`"]
pub type IPRIOR63 = crate::Reg<iprior63::IPRIOR63_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior63;

#[doc = "IPRIOR64 register accessor: an alias for `Reg<IPRIOR64_SPEC>`"]
pub type IPRIOR64 = crate::Reg<iprior64::IPRIOR64_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior64;

#[doc = "IPRIOR65 register accessor: an alias for `Reg<IPRIOR65_SPEC>`"]
pub type IPRIOR65 = crate::Reg<iprior65::IPRIOR65_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior65;

#[doc = "IPRIOR66 register accessor: an alias for `Reg<IPRIOR66_SPEC>`"]
pub type IPRIOR66 = crate::Reg<iprior66::IPRIOR66_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior66;

#[doc = "IPRIOR67 register accessor: an alias for `Reg<IPRIOR67_SPEC>`"]
pub type IPRIOR67 = crate::Reg<iprior67::IPRIOR67_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior67;

#[doc = "IPRIOR68 register accessor: an alias for `Reg<IPRIOR68_SPEC>`"]
pub type IPRIOR68 = crate::Reg<iprior68::IPRIOR68_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior68;

#[doc = "IPRIOR69 register accessor: an alias for `Reg<IPRIOR69_SPEC>`"]
pub type IPRIOR69 = crate::Reg<iprior69::IPRIOR69_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior69;

#[doc = "IPRIOR70 register accessor: an alias for `Reg<IPRIOR70_SPEC>`"]
pub type IPRIOR70 = crate::Reg<iprior70::IPRIOR70_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior70;

#[doc = "IPRIOR71 register accessor: an alias for `Reg<IPRIOR71_SPEC>`"]
pub type IPRIOR71 = crate::Reg<iprior71::IPRIOR71_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior71;

#[doc = "IPRIOR72 register accessor: an alias for `Reg<IPRIOR72_SPEC>`"]
pub type IPRIOR72 = crate::Reg<iprior72::IPRIOR72_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior72;

#[doc = "IPRIOR73 register accessor: an alias for `Reg<IPRIOR73_SPEC>`"]
pub type IPRIOR73 = crate::Reg<iprior73::IPRIOR73_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior73;

#[doc = "IPRIOR74 register accessor: an alias for `Reg<IPRIOR74_SPEC>`"]
pub type IPRIOR74 = crate::Reg<iprior74::IPRIOR74_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior74;

#[doc = "IPRIOR75 register accessor: an alias for `Reg<IPRIOR75_SPEC>`"]
pub type IPRIOR75 = crate::Reg<iprior75::IPRIOR75_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior75;

#[doc = "IPRIOR76 register accessor: an alias for `Reg<IPRIOR76_SPEC>`"]
pub type IPRIOR76 = crate::Reg<iprior76::IPRIOR76_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior76;

#[doc = "IPRIOR77 register accessor: an alias for `Reg<IPRIOR77_SPEC>`"]
pub type IPRIOR77 = crate::Reg<iprior77::IPRIOR77_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior77;

#[doc = "IPRIOR78 register accessor: an alias for `Reg<IPRIOR78_SPEC>`"]
pub type IPRIOR78 = crate::Reg<iprior78::IPRIOR78_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior78;

#[doc = "IPRIOR79 register accessor: an alias for `Reg<IPRIOR79_SPEC>`"]
pub type IPRIOR79 = crate::Reg<iprior79::IPRIOR79_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior79;

#[doc = "IPRIOR80 register accessor: an alias for `Reg<IPRIOR80_SPEC>`"]
pub type IPRIOR80 = crate::Reg<iprior80::IPRIOR80_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior80;

#[doc = "IPRIOR81 register accessor: an alias for `Reg<IPRIOR81_SPEC>`"]
pub type IPRIOR81 = crate::Reg<iprior81::IPRIOR81_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior81;

#[doc = "IPRIOR82 register accessor: an alias for `Reg<IPRIOR82_SPEC>`"]
pub type IPRIOR82 = crate::Reg<iprior82::IPRIOR82_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior82;

#[doc = "IPRIOR83 register accessor: an alias for `Reg<IPRIOR83_SPEC>`"]
pub type IPRIOR83 = crate::Reg<iprior83::IPRIOR83_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior83;

#[doc = "IPRIOR84 register accessor: an alias for `Reg<IPRIOR84_SPEC>`"]
pub type IPRIOR84 = crate::Reg<iprior84::IPRIOR84_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior84;

#[doc = "IPRIOR85 register accessor: an alias for `Reg<IPRIOR85_SPEC>`"]
pub type IPRIOR85 = crate::Reg<iprior85::IPRIOR85_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior85;

#[doc = "IPRIOR86 register accessor: an alias for `Reg<IPRIOR86_SPEC>`"]
pub type IPRIOR86 = crate::Reg<iprior86::IPRIOR86_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior86;

#[doc = "IPRIOR87 register accessor: an alias for `Reg<IPRIOR87_SPEC>`"]
pub type IPRIOR87 = crate::Reg<iprior87::IPRIOR87_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior87;

#[doc = "IPRIOR88 register accessor: an alias for `Reg<IPRIOR88_SPEC>`"]
pub type IPRIOR88 = crate::Reg<iprior88::IPRIOR88_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior88;

#[doc = "IPRIOR89 register accessor: an alias for `Reg<IPRIOR89_SPEC>`"]
pub type IPRIOR89 = crate::Reg<iprior89::IPRIOR89_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior89;

#[doc = "IPRIOR90 register accessor: an alias for `Reg<IPRIOR90_SPEC>`"]
pub type IPRIOR90 = crate::Reg<iprior90::IPRIOR90_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior90;

#[doc = "IPRIOR91 register accessor: an alias for `Reg<IPRIOR91_SPEC>`"]
pub type IPRIOR91 = crate::Reg<iprior91::IPRIOR91_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior91;

#[doc = "IPRIOR92 register accessor: an alias for `Reg<IPRIOR92_SPEC>`"]
pub type IPRIOR92 = crate::Reg<iprior92::IPRIOR92_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior92;

#[doc = "IPRIOR93 register accessor: an alias for `Reg<IPRIOR93_SPEC>`"]
pub type IPRIOR93 = crate::Reg<iprior93::IPRIOR93_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior93;

#[doc = "IPRIOR94 register accessor: an alias for `Reg<IPRIOR94_SPEC>`"]
pub type IPRIOR94 = crate::Reg<iprior94::IPRIOR94_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior94;

#[doc = "IPRIOR95 register accessor: an alias for `Reg<IPRIOR95_SPEC>`"]
pub type IPRIOR95 = crate::Reg<iprior95::IPRIOR95_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior95;

#[doc = "IPRIOR96 register accessor: an alias for `Reg<IPRIOR96_SPEC>`"]
pub type IPRIOR96 = crate::Reg<iprior96::IPRIOR96_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior96;

#[doc = "IPRIOR97 register accessor: an alias for `Reg<IPRIOR97_SPEC>`"]
pub type IPRIOR97 = crate::Reg<iprior97::IPRIOR97_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior97;

#[doc = "IPRIOR98 register accessor: an alias for `Reg<IPRIOR98_SPEC>`"]
pub type IPRIOR98 = crate::Reg<iprior98::IPRIOR98_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior98;

#[doc = "IPRIOR99 register accessor: an alias for `Reg<IPRIOR99_SPEC>`"]
pub type IPRIOR99 = crate::Reg<iprior99::IPRIOR99_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior99;

#[doc = "IPRIOR100 register accessor: an alias for `Reg<IPRIOR100_SPEC>`"]
pub type IPRIOR100 = crate::Reg<iprior100::IPRIOR100_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior100;

#[doc = "IPRIOR101 register accessor: an alias for `Reg<IPRIOR101_SPEC>`"]
pub type IPRIOR101 = crate::Reg<iprior101::IPRIOR101_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior101;

#[doc = "IPRIOR102 register accessor: an alias for `Reg<IPRIOR102_SPEC>`"]
pub type IPRIOR102 = crate::Reg<iprior102::IPRIOR102_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior102;

#[doc = "IPRIOR103 register accessor: an alias for `Reg<IPRIOR103_SPEC>`"]
pub type IPRIOR103 = crate::Reg<iprior103::IPRIOR103_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior103;

#[doc = "IPRIOR104 register accessor: an alias for `Reg<IPRIOR104_SPEC>`"]
pub type IPRIOR104 = crate::Reg<iprior104::IPRIOR104_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior104;

#[doc = "IPRIOR105 register accessor: an alias for `Reg<IPRIOR105_SPEC>`"]
pub type IPRIOR105 = crate::Reg<iprior105::IPRIOR105_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior105;

#[doc = "IPRIOR106 register accessor: an alias for `Reg<IPRIOR106_SPEC>`"]
pub type IPRIOR106 = crate::Reg<iprior106::IPRIOR106_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior106;

#[doc = "IPRIOR107 register accessor: an alias for `Reg<IPRIOR107_SPEC>`"]
pub type IPRIOR107 = crate::Reg<iprior107::IPRIOR107_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior107;

#[doc = "IPRIOR108 register accessor: an alias for `Reg<IPRIOR108_SPEC>`"]
pub type IPRIOR108 = crate::Reg<iprior108::IPRIOR108_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior108;

#[doc = "IPRIOR109 register accessor: an alias for `Reg<IPRIOR109_SPEC>`"]
pub type IPRIOR109 = crate::Reg<iprior109::IPRIOR109_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior109;

#[doc = "IPRIOR110 register accessor: an alias for `Reg<IPRIOR110_SPEC>`"]
pub type IPRIOR110 = crate::Reg<iprior110::IPRIOR110_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior110;

#[doc = "IPRIOR111 register accessor: an alias for `Reg<IPRIOR111_SPEC>`"]
pub type IPRIOR111 = crate::Reg<iprior111::IPRIOR111_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior111;

#[doc = "IPRIOR112 register accessor: an alias for `Reg<IPRIOR112_SPEC>`"]
pub type IPRIOR112 = crate::Reg<iprior112::IPRIOR112_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior112;

#[doc = "IPRIOR113 register accessor: an alias for `Reg<IPRIOR113_SPEC>`"]
pub type IPRIOR113 = crate::Reg<iprior113::IPRIOR113_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior113;

#[doc = "IPRIOR114 register accessor: an alias for `Reg<IPRIOR114_SPEC>`"]
pub type IPRIOR114 = crate::Reg<iprior114::IPRIOR114_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior114;

#[doc = "IPRIOR115 register accessor: an alias for `Reg<IPRIOR115_SPEC>`"]
pub type IPRIOR115 = crate::Reg<iprior115::IPRIOR115_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior115;

#[doc = "IPRIOR116 register accessor: an alias for `Reg<IPRIOR116_SPEC>`"]
pub type IPRIOR116 = crate::Reg<iprior116::IPRIOR116_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior116;

#[doc = "IPRIOR117 register accessor: an alias for `Reg<IPRIOR117_SPEC>`"]
pub type IPRIOR117 = crate::Reg<iprior117::IPRIOR117_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior117;

#[doc = "IPRIOR118 register accessor: an alias for `Reg<IPRIOR118_SPEC>`"]
pub type IPRIOR118 = crate::Reg<iprior118::IPRIOR118_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior118;

#[doc = "IPRIOR119 register accessor: an alias for `Reg<IPRIOR119_SPEC>`"]
pub type IPRIOR119 = crate::Reg<iprior119::IPRIOR119_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior119;

#[doc = "IPRIOR120 register accessor: an alias for `Reg<IPRIOR120_SPEC>`"]
pub type IPRIOR120 = crate::Reg<iprior120::IPRIOR120_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior120;

#[doc = "IPRIOR121 register accessor: an alias for `Reg<IPRIOR121_SPEC>`"]
pub type IPRIOR121 = crate::Reg<iprior121::IPRIOR121_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior121;

#[doc = "IPRIOR122 register accessor: an alias for `Reg<IPRIOR122_SPEC>`"]
pub type IPRIOR122 = crate::Reg<iprior122::IPRIOR122_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior122;

#[doc = "IPRIOR123 register accessor: an alias for `Reg<IPRIOR123_SPEC>`"]
pub type IPRIOR123 = crate::Reg<iprior123::IPRIOR123_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior123;

#[doc = "IPRIOR124 register accessor: an alias for `Reg<IPRIOR124_SPEC>`"]
pub type IPRIOR124 = crate::Reg<iprior124::IPRIOR124_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior124;

#[doc = "IPRIOR125 register accessor: an alias for `Reg<IPRIOR125_SPEC>`"]
pub type IPRIOR125 = crate::Reg<iprior125::IPRIOR125_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior125;

#[doc = "IPRIOR126 register accessor: an alias for `Reg<IPRIOR126_SPEC>`"]
pub type IPRIOR126 = crate::Reg<iprior126::IPRIOR126_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior126;

#[doc = "IPRIOR127 register accessor: an alias for `Reg<IPRIOR127_SPEC>`"]
pub type IPRIOR127 = crate::Reg<iprior127::IPRIOR127_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior127;

#[doc = "IPRIOR128 register accessor: an alias for `Reg<IPRIOR128_SPEC>`"]
pub type IPRIOR128 = crate::Reg<iprior128::IPRIOR128_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior128;

#[doc = "IPRIOR129 register accessor: an alias for `Reg<IPRIOR129_SPEC>`"]
pub type IPRIOR129 = crate::Reg<iprior129::IPRIOR129_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior129;

#[doc = "IPRIOR130 register accessor: an alias for `Reg<IPRIOR130_SPEC>`"]
pub type IPRIOR130 = crate::Reg<iprior130::IPRIOR130_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior130;

#[doc = "IPRIOR131 register accessor: an alias for `Reg<IPRIOR131_SPEC>`"]
pub type IPRIOR131 = crate::Reg<iprior131::IPRIOR131_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior131;

#[doc = "IPRIOR132 register accessor: an alias for `Reg<IPRIOR132_SPEC>`"]
pub type IPRIOR132 = crate::Reg<iprior132::IPRIOR132_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior132;

#[doc = "IPRIOR133 register accessor: an alias for `Reg<IPRIOR133_SPEC>`"]
pub type IPRIOR133 = crate::Reg<iprior133::IPRIOR133_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior133;

#[doc = "IPRIOR134 register accessor: an alias for `Reg<IPRIOR134_SPEC>`"]
pub type IPRIOR134 = crate::Reg<iprior134::IPRIOR134_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior134;

#[doc = "IPRIOR135 register accessor: an alias for `Reg<IPRIOR135_SPEC>`"]
pub type IPRIOR135 = crate::Reg<iprior135::IPRIOR135_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior135;

#[doc = "IPRIOR136 register accessor: an alias for `Reg<IPRIOR136_SPEC>`"]
pub type IPRIOR136 = crate::Reg<iprior136::IPRIOR136_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior136;

#[doc = "IPRIOR137 register accessor: an alias for `Reg<IPRIOR137_SPEC>`"]
pub type IPRIOR137 = crate::Reg<iprior137::IPRIOR137_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior137;

#[doc = "IPRIOR138 register accessor: an alias for `Reg<IPRIOR138_SPEC>`"]
pub type IPRIOR138 = crate::Reg<iprior138::IPRIOR138_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior138;

#[doc = "IPRIOR139 register accessor: an alias for `Reg<IPRIOR139_SPEC>`"]
pub type IPRIOR139 = crate::Reg<iprior139::IPRIOR139_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior139;

#[doc = "IPRIOR140 register accessor: an alias for `Reg<IPRIOR140_SPEC>`"]
pub type IPRIOR140 = crate::Reg<iprior140::IPRIOR140_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior140;

#[doc = "IPRIOR141 register accessor: an alias for `Reg<IPRIOR141_SPEC>`"]
pub type IPRIOR141 = crate::Reg<iprior141::IPRIOR141_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior141;

#[doc = "IPRIOR142 register accessor: an alias for `Reg<IPRIOR142_SPEC>`"]
pub type IPRIOR142 = crate::Reg<iprior142::IPRIOR142_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior142;

#[doc = "IPRIOR143 register accessor: an alias for `Reg<IPRIOR143_SPEC>`"]
pub type IPRIOR143 = crate::Reg<iprior143::IPRIOR143_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior143;

#[doc = "IPRIOR144 register accessor: an alias for `Reg<IPRIOR144_SPEC>`"]
pub type IPRIOR144 = crate::Reg<iprior144::IPRIOR144_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior144;

#[doc = "IPRIOR145 register accessor: an alias for `Reg<IPRIOR145_SPEC>`"]
pub type IPRIOR145 = crate::Reg<iprior145::IPRIOR145_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior145;

#[doc = "IPRIOR146 register accessor: an alias for `Reg<IPRIOR146_SPEC>`"]
pub type IPRIOR146 = crate::Reg<iprior146::IPRIOR146_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior146;

#[doc = "IPRIOR147 register accessor: an alias for `Reg<IPRIOR147_SPEC>`"]
pub type IPRIOR147 = crate::Reg<iprior147::IPRIOR147_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior147;

#[doc = "IPRIOR148 register accessor: an alias for `Reg<IPRIOR148_SPEC>`"]
pub type IPRIOR148 = crate::Reg<iprior148::IPRIOR148_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior148;

#[doc = "IPRIOR149 register accessor: an alias for `Reg<IPRIOR149_SPEC>`"]
pub type IPRIOR149 = crate::Reg<iprior149::IPRIOR149_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior149;

#[doc = "IPRIOR150 register accessor: an alias for `Reg<IPRIOR150_SPEC>`"]
pub type IPRIOR150 = crate::Reg<iprior150::IPRIOR150_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior150;

#[doc = "IPRIOR151 register accessor: an alias for `Reg<IPRIOR151_SPEC>`"]
pub type IPRIOR151 = crate::Reg<iprior151::IPRIOR151_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior151;

#[doc = "IPRIOR152 register accessor: an alias for `Reg<IPRIOR152_SPEC>`"]
pub type IPRIOR152 = crate::Reg<iprior152::IPRIOR152_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior152;

#[doc = "IPRIOR153 register accessor: an alias for `Reg<IPRIOR153_SPEC>`"]
pub type IPRIOR153 = crate::Reg<iprior153::IPRIOR153_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior153;

#[doc = "IPRIOR154 register accessor: an alias for `Reg<IPRIOR154_SPEC>`"]
pub type IPRIOR154 = crate::Reg<iprior154::IPRIOR154_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior154;

#[doc = "IPRIOR155 register accessor: an alias for `Reg<IPRIOR155_SPEC>`"]
pub type IPRIOR155 = crate::Reg<iprior155::IPRIOR155_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior155;

#[doc = "IPRIOR156 register accessor: an alias for `Reg<IPRIOR156_SPEC>`"]
pub type IPRIOR156 = crate::Reg<iprior156::IPRIOR156_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior156;

#[doc = "IPRIOR157 register accessor: an alias for `Reg<IPRIOR157_SPEC>`"]
pub type IPRIOR157 = crate::Reg<iprior157::IPRIOR157_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior157;

#[doc = "IPRIOR158 register accessor: an alias for `Reg<IPRIOR158_SPEC>`"]
pub type IPRIOR158 = crate::Reg<iprior158::IPRIOR158_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior158;

#[doc = "IPRIOR159 register accessor: an alias for `Reg<IPRIOR159_SPEC>`"]
pub type IPRIOR159 = crate::Reg<iprior159::IPRIOR159_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior159;

#[doc = "IPRIOR160 register accessor: an alias for `Reg<IPRIOR160_SPEC>`"]
pub type IPRIOR160 = crate::Reg<iprior160::IPRIOR160_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior160;

#[doc = "IPRIOR161 register accessor: an alias for `Reg<IPRIOR161_SPEC>`"]
pub type IPRIOR161 = crate::Reg<iprior161::IPRIOR161_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior161;

#[doc = "IPRIOR162 register accessor: an alias for `Reg<IPRIOR162_SPEC>`"]
pub type IPRIOR162 = crate::Reg<iprior162::IPRIOR162_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior162;

#[doc = "IPRIOR163 register accessor: an alias for `Reg<IPRIOR163_SPEC>`"]
pub type IPRIOR163 = crate::Reg<iprior163::IPRIOR163_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior163;

#[doc = "IPRIOR164 register accessor: an alias for `Reg<IPRIOR164_SPEC>`"]
pub type IPRIOR164 = crate::Reg<iprior164::IPRIOR164_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior164;

#[doc = "IPRIOR165 register accessor: an alias for `Reg<IPRIOR165_SPEC>`"]
pub type IPRIOR165 = crate::Reg<iprior165::IPRIOR165_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior165;

#[doc = "IPRIOR166 register accessor: an alias for `Reg<IPRIOR166_SPEC>`"]
pub type IPRIOR166 = crate::Reg<iprior166::IPRIOR166_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior166;

#[doc = "IPRIOR167 register accessor: an alias for `Reg<IPRIOR167_SPEC>`"]
pub type IPRIOR167 = crate::Reg<iprior167::IPRIOR167_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior167;

#[doc = "IPRIOR168 register accessor: an alias for `Reg<IPRIOR168_SPEC>`"]
pub type IPRIOR168 = crate::Reg<iprior168::IPRIOR168_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior168;

#[doc = "IPRIOR169 register accessor: an alias for `Reg<IPRIOR169_SPEC>`"]
pub type IPRIOR169 = crate::Reg<iprior169::IPRIOR169_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior169;

#[doc = "IPRIOR170 register accessor: an alias for `Reg<IPRIOR170_SPEC>`"]
pub type IPRIOR170 = crate::Reg<iprior170::IPRIOR170_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior170;

#[doc = "IPRIOR171 register accessor: an alias for `Reg<IPRIOR171_SPEC>`"]
pub type IPRIOR171 = crate::Reg<iprior171::IPRIOR171_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior171;

#[doc = "IPRIOR172 register accessor: an alias for `Reg<IPRIOR172_SPEC>`"]
pub type IPRIOR172 = crate::Reg<iprior172::IPRIOR172_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior172;

#[doc = "IPRIOR173 register accessor: an alias for `Reg<IPRIOR173_SPEC>`"]
pub type IPRIOR173 = crate::Reg<iprior173::IPRIOR173_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior173;

#[doc = "IPRIOR174 register accessor: an alias for `Reg<IPRIOR174_SPEC>`"]
pub type IPRIOR174 = crate::Reg<iprior174::IPRIOR174_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior174;

#[doc = "IPRIOR175 register accessor: an alias for `Reg<IPRIOR175_SPEC>`"]
pub type IPRIOR175 = crate::Reg<iprior175::IPRIOR175_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior175;

#[doc = "IPRIOR176 register accessor: an alias for `Reg<IPRIOR176_SPEC>`"]
pub type IPRIOR176 = crate::Reg<iprior176::IPRIOR176_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior176;

#[doc = "IPRIOR177 register accessor: an alias for `Reg<IPRIOR177_SPEC>`"]
pub type IPRIOR177 = crate::Reg<iprior177::IPRIOR177_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior177;

#[doc = "IPRIOR178 register accessor: an alias for `Reg<IPRIOR178_SPEC>`"]
pub type IPRIOR178 = crate::Reg<iprior178::IPRIOR178_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior178;

#[doc = "IPRIOR179 register accessor: an alias for `Reg<IPRIOR179_SPEC>`"]
pub type IPRIOR179 = crate::Reg<iprior179::IPRIOR179_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior179;

#[doc = "IPRIOR180 register accessor: an alias for `Reg<IPRIOR180_SPEC>`"]
pub type IPRIOR180 = crate::Reg<iprior180::IPRIOR180_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior180;

#[doc = "IPRIOR181 register accessor: an alias for `Reg<IPRIOR181_SPEC>`"]
pub type IPRIOR181 = crate::Reg<iprior181::IPRIOR181_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior181;

#[doc = "IPRIOR182 register accessor: an alias for `Reg<IPRIOR182_SPEC>`"]
pub type IPRIOR182 = crate::Reg<iprior182::IPRIOR182_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior182;

#[doc = "IPRIOR183 register accessor: an alias for `Reg<IPRIOR183_SPEC>`"]
pub type IPRIOR183 = crate::Reg<iprior183::IPRIOR183_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior183;

#[doc = "IPRIOR184 register accessor: an alias for `Reg<IPRIOR184_SPEC>`"]
pub type IPRIOR184 = crate::Reg<iprior184::IPRIOR184_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior184;

#[doc = "IPRIOR185 register accessor: an alias for `Reg<IPRIOR185_SPEC>`"]
pub type IPRIOR185 = crate::Reg<iprior185::IPRIOR185_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior185;

#[doc = "IPRIOR186 register accessor: an alias for `Reg<IPRIOR186_SPEC>`"]
pub type IPRIOR186 = crate::Reg<iprior186::IPRIOR186_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior186;

#[doc = "IPRIOR187 register accessor: an alias for `Reg<IPRIOR187_SPEC>`"]
pub type IPRIOR187 = crate::Reg<iprior187::IPRIOR187_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior187;

#[doc = "IPRIOR188 register accessor: an alias for `Reg<IPRIOR188_SPEC>`"]
pub type IPRIOR188 = crate::Reg<iprior188::IPRIOR188_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior188;

#[doc = "IPRIOR189 register accessor: an alias for `Reg<IPRIOR189_SPEC>`"]
pub type IPRIOR189 = crate::Reg<iprior189::IPRIOR189_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior189;

#[doc = "IPRIOR190 register accessor: an alias for `Reg<IPRIOR190_SPEC>`"]
pub type IPRIOR190 = crate::Reg<iprior190::IPRIOR190_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior190;

#[doc = "IPRIOR191 register accessor: an alias for `Reg<IPRIOR191_SPEC>`"]
pub type IPRIOR191 = crate::Reg<iprior191::IPRIOR191_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior191;

#[doc = "IPRIOR192 register accessor: an alias for `Reg<IPRIOR192_SPEC>`"]
pub type IPRIOR192 = crate::Reg<iprior192::IPRIOR192_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior192;

#[doc = "IPRIOR193 register accessor: an alias for `Reg<IPRIOR193_SPEC>`"]
pub type IPRIOR193 = crate::Reg<iprior193::IPRIOR193_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior193;

#[doc = "IPRIOR194 register accessor: an alias for `Reg<IPRIOR194_SPEC>`"]
pub type IPRIOR194 = crate::Reg<iprior194::IPRIOR194_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior194;

#[doc = "IPRIOR195 register accessor: an alias for `Reg<IPRIOR195_SPEC>`"]
pub type IPRIOR195 = crate::Reg<iprior195::IPRIOR195_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior195;

#[doc = "IPRIOR196 register accessor: an alias for `Reg<IPRIOR196_SPEC>`"]
pub type IPRIOR196 = crate::Reg<iprior196::IPRIOR196_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior196;

#[doc = "IPRIOR197 register accessor: an alias for `Reg<IPRIOR197_SPEC>`"]
pub type IPRIOR197 = crate::Reg<iprior197::IPRIOR197_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior197;

#[doc = "IPRIOR198 register accessor: an alias for `Reg<IPRIOR198_SPEC>`"]
pub type IPRIOR198 = crate::Reg<iprior198::IPRIOR198_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior198;

#[doc = "IPRIOR199 register accessor: an alias for `Reg<IPRIOR199_SPEC>`"]
pub type IPRIOR199 = crate::Reg<iprior199::IPRIOR199_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior199;

#[doc = "IPRIOR200 register accessor: an alias for `Reg<IPRIOR200_SPEC>`"]
pub type IPRIOR200 = crate::Reg<iprior200::IPRIOR200_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior200;

#[doc = "IPRIOR201 register accessor: an alias for `Reg<IPRIOR201_SPEC>`"]
pub type IPRIOR201 = crate::Reg<iprior201::IPRIOR201_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior201;

#[doc = "IPRIOR202 register accessor: an alias for `Reg<IPRIOR202_SPEC>`"]
pub type IPRIOR202 = crate::Reg<iprior202::IPRIOR202_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior202;

#[doc = "IPRIOR203 register accessor: an alias for `Reg<IPRIOR203_SPEC>`"]
pub type IPRIOR203 = crate::Reg<iprior203::IPRIOR203_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior203;

#[doc = "IPRIOR204 register accessor: an alias for `Reg<IPRIOR204_SPEC>`"]
pub type IPRIOR204 = crate::Reg<iprior204::IPRIOR204_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior204;

#[doc = "IPRIOR205 register accessor: an alias for `Reg<IPRIOR205_SPEC>`"]
pub type IPRIOR205 = crate::Reg<iprior205::IPRIOR205_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior205;

#[doc = "IPRIOR206 register accessor: an alias for `Reg<IPRIOR206_SPEC>`"]
pub type IPRIOR206 = crate::Reg<iprior206::IPRIOR206_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior206;

#[doc = "IPRIOR207 register accessor: an alias for `Reg<IPRIOR207_SPEC>`"]
pub type IPRIOR207 = crate::Reg<iprior207::IPRIOR207_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior207;

#[doc = "IPRIOR208 register accessor: an alias for `Reg<IPRIOR208_SPEC>`"]
pub type IPRIOR208 = crate::Reg<iprior208::IPRIOR208_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior208;

#[doc = "IPRIOR209 register accessor: an alias for `Reg<IPRIOR209_SPEC>`"]
pub type IPRIOR209 = crate::Reg<iprior209::IPRIOR209_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior209;

#[doc = "IPRIOR210 register accessor: an alias for `Reg<IPRIOR210_SPEC>`"]
pub type IPRIOR210 = crate::Reg<iprior210::IPRIOR210_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior210;

#[doc = "IPRIOR211 register accessor: an alias for `Reg<IPRIOR211_SPEC>`"]
pub type IPRIOR211 = crate::Reg<iprior211::IPRIOR211_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior211;

#[doc = "IPRIOR212 register accessor: an alias for `Reg<IPRIOR212_SPEC>`"]
pub type IPRIOR212 = crate::Reg<iprior212::IPRIOR212_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior212;

#[doc = "IPRIOR213 register accessor: an alias for `Reg<IPRIOR213_SPEC>`"]
pub type IPRIOR213 = crate::Reg<iprior213::IPRIOR213_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior213;

#[doc = "IPRIOR214 register accessor: an alias for `Reg<IPRIOR214_SPEC>`"]
pub type IPRIOR214 = crate::Reg<iprior214::IPRIOR214_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior214;

#[doc = "IPRIOR215 register accessor: an alias for `Reg<IPRIOR215_SPEC>`"]
pub type IPRIOR215 = crate::Reg<iprior215::IPRIOR215_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior215;

#[doc = "IPRIOR216 register accessor: an alias for `Reg<IPRIOR216_SPEC>`"]
pub type IPRIOR216 = crate::Reg<iprior216::IPRIOR216_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior216;

#[doc = "IPRIOR217 register accessor: an alias for `Reg<IPRIOR217_SPEC>`"]
pub type IPRIOR217 = crate::Reg<iprior217::IPRIOR217_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior217;

#[doc = "IPRIOR218 register accessor: an alias for `Reg<IPRIOR218_SPEC>`"]
pub type IPRIOR218 = crate::Reg<iprior218::IPRIOR218_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior218;

#[doc = "IPRIOR219 register accessor: an alias for `Reg<IPRIOR219_SPEC>`"]
pub type IPRIOR219 = crate::Reg<iprior219::IPRIOR219_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior219;

#[doc = "IPRIOR220 register accessor: an alias for `Reg<IPRIOR220_SPEC>`"]
pub type IPRIOR220 = crate::Reg<iprior220::IPRIOR220_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior220;

#[doc = "IPRIOR221 register accessor: an alias for `Reg<IPRIOR221_SPEC>`"]
pub type IPRIOR221 = crate::Reg<iprior221::IPRIOR221_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior221;

#[doc = "IPRIOR222 register accessor: an alias for `Reg<IPRIOR222_SPEC>`"]
pub type IPRIOR222 = crate::Reg<iprior222::IPRIOR222_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior222;

#[doc = "IPRIOR223 register accessor: an alias for `Reg<IPRIOR223_SPEC>`"]
pub type IPRIOR223 = crate::Reg<iprior223::IPRIOR223_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior223;

#[doc = "IPRIOR224 register accessor: an alias for `Reg<IPRIOR224_SPEC>`"]
pub type IPRIOR224 = crate::Reg<iprior224::IPRIOR224_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior224;

#[doc = "IPRIOR225 register accessor: an alias for `Reg<IPRIOR225_SPEC>`"]
pub type IPRIOR225 = crate::Reg<iprior225::IPRIOR225_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior225;

#[doc = "IPRIOR226 register accessor: an alias for `Reg<IPRIOR226_SPEC>`"]
pub type IPRIOR226 = crate::Reg<iprior226::IPRIOR226_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior226;

#[doc = "IPRIOR227 register accessor: an alias for `Reg<IPRIOR227_SPEC>`"]
pub type IPRIOR227 = crate::Reg<iprior227::IPRIOR227_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior227;

#[doc = "IPRIOR228 register accessor: an alias for `Reg<IPRIOR228_SPEC>`"]
pub type IPRIOR228 = crate::Reg<iprior228::IPRIOR228_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior228;

#[doc = "IPRIOR229 register accessor: an alias for `Reg<IPRIOR229_SPEC>`"]
pub type IPRIOR229 = crate::Reg<iprior229::IPRIOR229_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior229;

#[doc = "IPRIOR230 register accessor: an alias for `Reg<IPRIOR230_SPEC>`"]
pub type IPRIOR230 = crate::Reg<iprior230::IPRIOR230_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior230;

#[doc = "IPRIOR231 register accessor: an alias for `Reg<IPRIOR231_SPEC>`"]
pub type IPRIOR231 = crate::Reg<iprior231::IPRIOR231_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior231;

#[doc = "IPRIOR232 register accessor: an alias for `Reg<IPRIOR232_SPEC>`"]
pub type IPRIOR232 = crate::Reg<iprior232::IPRIOR232_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior232;

#[doc = "IPRIOR233 register accessor: an alias for `Reg<IPRIOR233_SPEC>`"]
pub type IPRIOR233 = crate::Reg<iprior233::IPRIOR233_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior233;

#[doc = "IPRIOR234 register accessor: an alias for `Reg<IPRIOR234_SPEC>`"]
pub type IPRIOR234 = crate::Reg<iprior234::IPRIOR234_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior234;

#[doc = "IPRIOR235 register accessor: an alias for `Reg<IPRIOR235_SPEC>`"]
pub type IPRIOR235 = crate::Reg<iprior235::IPRIOR235_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior235;

#[doc = "IPRIOR236 register accessor: an alias for `Reg<IPRIOR236_SPEC>`"]
pub type IPRIOR236 = crate::Reg<iprior236::IPRIOR236_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior236;

#[doc = "IPRIOR237 register accessor: an alias for `Reg<IPRIOR237_SPEC>`"]
pub type IPRIOR237 = crate::Reg<iprior237::IPRIOR237_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior237;

#[doc = "IPRIOR238 register accessor: an alias for `Reg<IPRIOR238_SPEC>`"]
pub type IPRIOR238 = crate::Reg<iprior238::IPRIOR238_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior238;

#[doc = "IPRIOR239 register accessor: an alias for `Reg<IPRIOR239_SPEC>`"]
pub type IPRIOR239 = crate::Reg<iprior239::IPRIOR239_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior239;

#[doc = "IPRIOR240 register accessor: an alias for `Reg<IPRIOR240_SPEC>`"]
pub type IPRIOR240 = crate::Reg<iprior240::IPRIOR240_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior240;

#[doc = "IPRIOR241 register accessor: an alias for `Reg<IPRIOR241_SPEC>`"]
pub type IPRIOR241 = crate::Reg<iprior241::IPRIOR241_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior241;

#[doc = "IPRIOR242 register accessor: an alias for `Reg<IPRIOR242_SPEC>`"]
pub type IPRIOR242 = crate::Reg<iprior242::IPRIOR242_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior242;

#[doc = "IPRIOR243 register accessor: an alias for `Reg<IPRIOR243_SPEC>`"]
pub type IPRIOR243 = crate::Reg<iprior243::IPRIOR243_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior243;

#[doc = "IPRIOR244 register accessor: an alias for `Reg<IPRIOR244_SPEC>`"]
pub type IPRIOR244 = crate::Reg<iprior244::IPRIOR244_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior244;

#[doc = "IPRIOR245 register accessor: an alias for `Reg<IPRIOR245_SPEC>`"]
pub type IPRIOR245 = crate::Reg<iprior245::IPRIOR245_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior245;

#[doc = "IPRIOR246 register accessor: an alias for `Reg<IPRIOR246_SPEC>`"]
pub type IPRIOR246 = crate::Reg<iprior246::IPRIOR246_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior246;

#[doc = "IPRIOR247 register accessor: an alias for `Reg<IPRIOR247_SPEC>`"]
pub type IPRIOR247 = crate::Reg<iprior247::IPRIOR247_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior247;

#[doc = "IPRIOR248 register accessor: an alias for `Reg<IPRIOR248_SPEC>`"]
pub type IPRIOR248 = crate::Reg<iprior248::IPRIOR248_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior248;

#[doc = "IPRIOR249 register accessor: an alias for `Reg<IPRIOR249_SPEC>`"]
pub type IPRIOR249 = crate::Reg<iprior249::IPRIOR249_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior249;

#[doc = "IPRIOR250 register accessor: an alias for `Reg<IPRIOR250_SPEC>`"]
pub type IPRIOR250 = crate::Reg<iprior250::IPRIOR250_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior250;

#[doc = "IPRIOR251 register accessor: an alias for `Reg<IPRIOR251_SPEC>`"]
pub type IPRIOR251 = crate::Reg<iprior251::IPRIOR251_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior251;

#[doc = "IPRIOR252 register accessor: an alias for `Reg<IPRIOR252_SPEC>`"]
pub type IPRIOR252 = crate::Reg<iprior252::IPRIOR252_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior252;

#[doc = "IPRIOR253 register accessor: an alias for `Reg<IPRIOR253_SPEC>`"]
pub type IPRIOR253 = crate::Reg<iprior253::IPRIOR253_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior253;

#[doc = "IPRIOR254 register accessor: an alias for `Reg<IPRIOR254_SPEC>`"]
pub type IPRIOR254 = crate::Reg<iprior254::IPRIOR254_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior254;

#[doc = "IPRIOR255 register accessor: an alias for `Reg<IPRIOR255_SPEC>`"]
pub type IPRIOR255 = crate::Reg<iprior255::IPRIOR255_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod iprior255;

#[doc = "SCTLR register accessor: an alias for `Reg<SCTLR_SPEC>`"]
pub type SCTLR = crate::Reg<sctlr::SCTLR_SPEC>;
#[doc = "System Control Register"]
pub mod sctlr;

#[doc = "STK_CTLR register accessor: an alias for `Reg<STK_CTLR_SPEC>`"]
pub type STK_CTLR = crate::Reg<stk_ctlr::STK_CTLR_SPEC>;
#[doc = "System counter control register"]
pub mod stk_ctlr;

#[doc = "STK_SR register accessor: an alias for `Reg<STK_SR_SPEC>`"]
pub type STK_SR = crate::Reg<stk_sr::STK_SR_SPEC>;
#[doc = "System START"]
pub mod stk_sr;

#[doc = "STK_CNTL register accessor: an alias for `Reg<STK_CNTL_SPEC>`"]
pub type STK_CNTL = crate::Reg<stk_cntl::STK_CNTL_SPEC>;
#[doc = "System counter low register"]
pub mod stk_cntl;

#[doc = "STK_CNTH register accessor: an alias for `Reg<STK_CNTH_SPEC>`"]
pub type STK_CNTH = crate::Reg<stk_cnth::STK_CNTH_SPEC>;
#[doc = "System counter high register"]
pub mod stk_cnth;

#[doc = "STK_CMPLR register accessor: an alias for `Reg<STK_CMPLR_SPEC>`"]
pub type STK_CMPLR = crate::Reg<stk_cmplr::STK_CMPLR_SPEC>;
#[doc = "System compare low register"]
pub mod stk_cmplr;

#[doc = "STK_CMPHR register accessor: an alias for `Reg<STK_CMPHR_SPEC>`"]
pub type STK_CMPHR = crate::Reg<stk_cmphr::STK_CMPHR_SPEC>;
#[doc = "System compare high register"]
pub mod stk_cmphr;