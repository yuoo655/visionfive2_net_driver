use crate::defs::*;Starfive2Hal
use crate::rings::*;

use core::marker::PhantomData;
use core::ptr::{read_volatile, write_volatile};

pub trait Starfive2Hal {
    fn phys_to_virt(pa: usize) -> usize {
        pa
    }
    fn virt_to_phys(va: usize) -> usize {
        va
    }
    fn dma_alloc_pages(pages: usize) -> (usize, usize);

    fn dma_free_pages(vaddr: usize, pages: usize);

    fn mdelay(m_times: usize);

    fn fence();
}

pub struct Starfive2NetDevice<A: Starfive2Hal> {
    pub rx_ring: RxRing<A>,
    pub tx_ring: TxRing<A>,
    phantom: PhantomData<A>,
}

impl<A: Starfive2Hal> StmmacDevice<A> {
    pub fn new() -> Self {
        let ioaddr = A::phys_to_virt(0x16040000);
        log::info!("---------init clk-------------");
        unsafe {
            for i in 97..112 {
                write_volatile(A::phys_to_virt(0x13020000 + i * 4) as *mut u32, 0x80000000);
            }
            for i in 221..228 {
                write_volatile(
                    A::phys_to_virt(0x17000000 + (i - 219) * 4) as *mut u32,
                    0x80000000,
                );
            }
        }

        mdio_write::<A>(ioaddr, 0xa001, 0x8020);
        mdio_write::<A>(ioaddr, 0xa010, 0xcbff);
        mdio_write::<A>(ioaddr, 0xa003, 0x850);

        log::info!("--------reset_trigger-----------");
        unsafe {
            write_volatile((A::phys_to_virt(0x13020300)) as *mut u32, 0xffe5afc4);
            write_volatile((A::phys_to_virt(0x13020300)) as *mut u32, 0xffe5afc0);

            write_volatile((A::phys_to_virt(0x17000038)) as *mut u32, 0xe1);
            write_volatile((A::phys_to_virt(0x17000038)) as *mut u32, 0xe0);
            write_volatile((A::phys_to_virt(0x17000038)) as *mut u32, 0xe2);
            write_volatile((A::phys_to_virt(0x17000038)) as *mut u32, 0xe3);

            write_volatile((A::phys_to_virt(0x13020190)) as *mut u32, 0x8);
            write_volatile((A::phys_to_virt(0x13020194)) as *mut u32, 0x1);
        }

        log::info!("-------------------phylink_start phylink_speed_up--------------");
        log::info!("-------------------phy_config_aneg--------------");
        mdio_write::<A>(ioaddr, 0x1de1, 0x300);

        let mut rx_ring = RxRing::<A>::new();
        let mut tx_ring = TxRing::<A>::new();
        log::info!("init_dma_rx_desc_rings");
        let rdes_base = rx_ring.rd.phy_addr as u32;
        let rdes_end = rdes_base + size as u32;
        let skb_start = 0x8201_0000 as usize;
        for i in 0..64 {
            let buff_addr = skb_start + 0x1000 * i;
            rx_ring.init_rx_desc(i, buff_addr);
            rx_ring.skbuf.push(A::phys_to_virt(buff_addr));
        }

        log::info!("init_dma_tx_desc_rings");
        let mut tx_ring = TxRing::<A>::new();
        let tdes_base = tx_ring.td.phy_addr as u32;
        let tskb_start = 0x8202_0000 as usize;
        for i in 0..64 {
            tx_ring.init_tx_desc(i, false);
        }

        unsafe {
            log::info!("-------------dwmac_dma_reset--------------------");
            let mut value = read_volatile((ioaddr + DMA_BUS_MODE) as *mut u32);

            value |= 1 as u32;

            write_volatile((ioaddr + DMA_BUS_MODE) as *mut u32, value);
        }

        unsafe {
            write_volatile((ioaddr + DMA_BUS_MODE) as *mut u32, 0x1);

            write_volatile((ioaddr + DMA_BUS_MODE) as *mut u32, 0xf0f08f1);

            write_volatile((ioaddr + DMA_CHAN_BASE_ADDR) as *mut u32, 0);

            write_volatile((ioaddr + DMA_CHAN_BASE_ADDR) as *mut u32, 0x100000);

            write_volatile((ioaddr + DMA_CHAN_RX_BASE_ADDR) as *mut u32, rdes_base);

            write_volatile((ioaddr + DMA_CHAN_RX_END_ADDR) as *mut u32, rdes_end);

            write_volatile((ioaddr + DMA_CHAN_TX_CONTROL) as *mut u32, 0x100010);

            write_volatile((ioaddr + DMA_CHAN_TX_BASE_ADDR) as *mut u32, tdes_base);
        }

        log::info!("set mac addr");
        let macid_lo = 0xddccbbaa;
        let macid_hi = 0x0605;
        unsafe {
            write_volatile((ioaddr + MAC_ADDR_HI) as *mut u32, macid_hi);
            write_volatile((ioaddr + MAC_ADDR_LO) as *mut u32, macid_lo);
        }

        unsafe {
            write_volatile((ioaddr) as *mut u32, 0x78200);

            write_volatile((ioaddr + MTL_RXQ_DMA_MAP0) as *mut u32, 0x0);

            write_volatile((ioaddr + GMAC_RXQ_CTRL0) as *mut u32, 0x2);

            write_volatile((ioaddr + MTL_CHAN_RX_OP_MODE) as *mut u32, 0x700000);

            write_volatile((ioaddr + MTL_CHAN_BASE_ADDR) as *mut u32, 0x70018);

            write_volatile((ioaddr + DMA_CHAN_TX_RING_LEN) as *mut u32, 64);

            write_volatile((ioaddr + DMA_CHAN_RX_RING_LEN) as *mut u32, 64);

            write_volatile((ioaddr + GMAC_QX_TX_FLOW_CTRL) as *mut u32, 0xffff0000);

            write_volatile((ioaddr + GMAC_QX_TX_FLOW_CTRL) as *mut u32, 1 << 1);
        }

        log::info!("---------start dma tx/rx----------------------------");
        unsafe {
            let mut value = read_volatile((ioaddr + DMA_CHAN_RX_CONTROL) as *mut u32);
            value |= DMA_CONTROL_SR;
            write_volatile((ioaddr + DMA_CHAN_RX_CONTROL) as *mut u32, value);

            let mut value = read_volatile((ioaddr) as *mut u32);
            value |= GMAC_CONFIG_RE;
            write_volatile((ioaddr) as *mut u32, value);

            let mut value = read_volatile((ioaddr + DMA_CHAN_TX_CONTROL) as *mut u32);
            value |= DMA_CONTROL_ST;
            write_volatile((ioaddr + DMA_CHAN_TX_CONTROL) as *mut u32, value);
            let mut value = read_volatile((ioaddr) as *mut u32);
            value |= GMAC_CONFIG_TE;
            write_volatile((ioaddr) as *mut u32, value);
        }

        log::info!("--------------stmmac_mac_link_up----------------------");
        unsafe {
            write_volatile((ioaddr) as *mut u32, 0x8072203);
        }

        stmmac_set_mac(ioaddr, true);

        unsafe {
            write_volatile((ioaddr + 0x70) as *mut u32, 0xffff0002);
        }








        let nic = Starfive2NetDevice::<A> {
            rx_ring: rx_ring,
            tx_ring: tx_ring,
            phantom: PhantomData,
        };

        nic
    }

    pub fn receive(&mut self) -> Option<(*mut u8, u32)> {
        let rx_ring = &mut self.rx_ring;
        let rd_dma = &mut rx_ring.rd;
        let idx = rx_ring.idx;
        let rd = rd_dma.read_volatile(idx).unwrap();

        let rdes0 = rd.rdes0;

        let status = rdes0 & (1 << 31);

        if status >> 31 == 1 {
            // log::info!("dma own");
            return None;
        }

        let len = (rdes0 & DESC_RXSTS_FRMLENMSK) >> DESC_RXSTS_FRMLENSHFT;

        // get data from skb
        let skb_va = rx_ring.skbuf[idx];
        let skb = skb_va as *mut u8;
        unsafe {
            let packet: &[u8] = core::slice::from_raw_parts(skb, len as usize);
            log::info!("idx {:?} packet {:x?} ", idx, packet);
        }

        Some((skb, len))
    }

    pub fn rx_clean(&mut self) {
        let rx_ring = &mut self.rx_ring;
        let rd_dma = &mut rx_ring.rd;
        let idx = rx_ring.idx;

        log::info!("clean idx {:?}", idx);
        let ioaddr = A::phys_to_virt(0x1002_0000);
        let value = unsafe { read_volatile((ioaddr + 0x104c) as *mut u32) };
        log::info!("Current Host rx descriptor -----{:#x?}", value);
        if idx == 15 {
            let skb_start = 0x1801_0000 as usize;
            for i in 0..16 {
                let buff_addr = skb_start + 0x1000 * i;
                rx_ring.init_rx_desc(i, buff_addr);
            }
            let rdes_base = rx_ring.rd.phy_addr as u32;
            sifive_ccache_flush_range::<A>(rdes_base as usize, rdes_base as usize + 0x1000);
            sifive_ccache_flush_range::<A>(0x1801_0000 as usize, 0x1802_0000);
        }

        rx_ring.idx = (idx + 1) % 16;
    }

    pub fn transmit(&mut self, skb_pa: usize, len: usize) {
        let tx_ring: &mut TxRing<A> = &mut self.tx_ring;
        let idx: usize = tx_ring.idx;

        tx_ring.set_transmit_des(idx, skb_pa, len);

        let tdes_base = self.tx_ring.td.phy_addr as u32;
        sifive_ccache_flush_range::<A>(tdes_base as usize, tdes_base as usize + 0x1000);
        sifive_ccache_flush_range::<A>(skb_pa as usize, skb_pa as usize + 0x1000);

        let tail_ptr = tdes_base + (mem::size_of::<TxDes>() * (i + 1)) as u32;
        unsafe {
            write_volatile((ioaddr + 0x1120) as *mut u32, tail_ptr);
        }

        // wait until transmit finish
        loop {
            let td = self.tx_ring.td.read_volatile(idx).unwrap();
            if td.tdes0 & (1 << 31) == 0 {
                break;
            }
        }

        self.tx_ring.idx = (idx + 1) % 64;
    }
}

pub fn mdio_write<A: Starfive2Hal>(ioaddr: usize, data: u32, value: u32) {
    loop {
        let value = unsafe { read_volatile((ioaddr + 0x10) as *mut u32) };

        if value & MII_BUSY != 1 {
            break;
        }
        A::mdelay(10);
    }

    unsafe {
        write_volatile((ioaddr + 0x14) as *mut u32, data);
        write_volatile((ioaddr + 0x10) as *mut u32, value);
    }

    loop {
        let value = unsafe { read_volatile((ioaddr + 0x10) as *mut u32) };

        if value & MII_BUSY != 1 {
            break;
        }
        A::mdelay(10);
    }
}

pub fn stmmac_set_mac(ioaddr: usize, enable: bool) {
    let old_val: u32;
    let mut value: u32;

    log::info!("stmmac_set_mac--------------------enable={:?}", enable);

    old_val = unsafe { read_volatile(ioaddr as *mut u32) };
    value = old_val;

    if enable {
        value |= MAC_ENABLE_RX | MAC_ENABLE_TX;
    } else {
        value &= !(MAC_ENABLE_TX | MAC_ENABLE_RX);
    }

    if value != old_val {
        unsafe { write_volatile(ioaddr as *mut u32, value) }
    }
}


pub trait Starfive2Hal {
    fn phys_to_virt(pa: usize) -> usize {
        pa
    }
    fn virt_to_phys(va: usize) -> usize {
        va
    }

    fn dma_alloc_pages(pages: usize) -> (usize, usize);

    fn dma_free_pages(vaddr: usize, pages: usize);

    fn mdelay(m_times: usize);

    fn fence();
}