pub const DMA_BUS_MODE: usize = 0x00001000;

/* SW Reset */
pub const DMA_BUS_MODE_SFT_RESET: usize = 0x1; /* Software Reset */

/* AXI Master Bus Mode */
pub const DMA_AXI_BUS_MODE: usize = 0x00001028;

pub const DMA_RCV_BASE_ADDR: usize = 0x0000100c; /* Receive List Base */

/* Ctrl Operational Mode */
pub const DMA_CONTROL: usize = 0x00001018;

pub const DMA_CONTROL_SR: usize =  1 << 0;

pub const DMA_CONTROL_ST: usize =  1 << 0;

pub const GMAC_CONFIG_RE:usize = 1 << 0;

pub const GMAC_CONFIG_TE:usize = 1 << 1;


pub const MAC_ENABLE_TX: u32 = 1 << 3; /* Transmitter Enable */
pub const MAC_ENABLE_RX: u32 = 1 << 2; /* Receiver Enable */

/* Received Poll Demand */
pub const DMA_XMT_POLL_DEMAND: u32 = 0x00001004;

/* Received Poll Demand */
pub const DMA_RCV_POLL_DEMAND: u32 = 0x00001008;

pub const DMA_CONTROL_ST: u32 = 0x00002000;

pub const SIFIVE_CCACHE_WAY_ENABLE: usize = 0x8;

pub const MAC_ADDR_HI: usize = 0x300;
pub const MAC_ADDR_LO: usize = 0x304;

pub const DMA_CHAN_BASE_ADDR: usize = 0x00001100;

pub const DMA_CHAN_CONTROL: usize =                 0x1100;

pub const DMA_CHAN_TX_CONTROL: usize =                 0x1100 + 0x4;
pub const DMA_CHAN_RX_CONTROL: usize =                 0x1100 + 0x8;
pub const DMA_CHAN_TX_BASE_ADDR_HI: usize =            0x1100 + 0x10;
pub const DMA_CHAN_TX_BASE_ADDR: usize =                 0x1100 + 0x14;
pub const DMA_CHAN_RX_BASE_ADDR_HI: usize =                 0x1100 + 0x18;
pub const DMA_CHAN_RX_BASE_ADDR: usize =                 0x1100 + 0x1c;
pub const DMA_CHAN_TX_END_ADDR: usize =                 0x1100 + 0x20;
pub const DMA_CHAN_RX_END_ADDR: usize =                 0x1100 + 0x28;
pub const DMA_CHAN_TX_RING_LEN: usize =                 0x1100 + 0x2c;
pub const DMA_CHAN_RX_RING_LEN: usize =                 0x1100 + 0x30;
pub const DMA_CHAN_INTR_ENA: usize =                 0x1100 + 0x34;
pub const DMA_CHAN_RX_WATCHDOG: usize =                 0x1100 + 0x38;
pub const DMA_CHAN_SLOT_CTRL_STATUS: usize =                 0x1100 + 0x3c;
pub const DMA_CHAN_CUR_TX_DESC: usize =                 0x1100 + 0x44;
pub const DMA_CHAN_CUR_RX_DESC: usize =                 0x1100 + 0x4c;
pub const DMA_CHAN_CUR_TX_BUF_ADDR: usize =                 0x1100 + 0x54;
pub const DMA_CHAN_CUR_RX_BUF_ADDR: usize =                 0x1100 + 0x5c;
pub const DMA_CHAN_STATUS: usize =                 0x1100 + 0x60;


pub const MTL_RXQ_DMA_MAP0: usize = 0xc30;

pub const GMAC_RXQ_CTRL0: usize = 0x000000a0;


pub const  MTL_CHAN_BASE_ADDR: usize = 		0x00000d00;

pub const MTL_CHAN_RX_OP_MODE: usize =     0x00000d00 + 0x30;

pub const GMAC_QX_TX_FLOW_CTRL: usize = 0x70;