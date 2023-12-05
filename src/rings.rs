use alloc::vec::Vec;
use core::marker::PhantomData;
use core::ptr::NonNull;
use core::ptr::{read_volatile, write_volatile};

use crate::Starfive2Hal;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct RxDes {
    pub rdes0: u32,
    pub rdes1: u32,
    pub rdes2: u32,
    pub rdes3: u32,
}

pub struct RxRing<A> {
    pub rd: Dma<RxDes>,
    pub idx: usize,
    pub skbuf: Vec<usize>,
    phantom: PhantomData<A>,
}

impl<A: Starfive2Hal> RxRing<A> {
    pub fn new() -> Self {
        let count = 64;
        let pa = 0x8200_1000;
        let va = A::phys_to_virt(pa);

        let rd_dma = Dma::new(va as _, pa, count);
        let skbuf = Vec::new();

        Self {
            rd: rd_dma,
            idx: 0,
            skbuf: skbuf,
            phantom: PhantomData,
        }
    }

    pub fn init_rx_desc(&mut self, idx: usize, skb_phys_addr: usize) {
        let mut rd = RxDes {
            rdes0: 0,
            rdes1: 0,
            rdes2: 0,
            rdes3: 0,
        };
        rd.rdes0 = skb_phys_addr as u32;

        rd.rdes3 = 0x81000000;

        self.rd.write_volatile(idx, &rd);
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct TxDes {
    pub tdes0: u32,
    pub tdes1: u32,
    pub tdes2: u32,
    pub tdes3: u32,
}

pub struct TxRing<A> {
    pub td: Dma<TxDes>,
    pub idx: usize,
    pub skbuf: Vec<usize>,
    phantom: PhantomData<A>,
}

impl<A: Starfive2Hal> TxRing<A> {
    pub fn new() -> Self {
        let count = 64;
        let pa = 0x8200_2000;
        let va = A::phys_to_virt(pa);

        let td_dma = Dma::new(va as _, pa, count);
        let skbuf = Vec::new();

        Self {
            td: td_dma,
            idx: 0,
            skbuf: skbuf,
            phantom: PhantomData,
        }
    }

    pub fn init_tx_desc(&mut self, idx: usize, end: bool) {
        let mut td: TxDes = TxDes {
            tdes0: 0,
            tdes1: 0,
            tdes2: 0,
            tdes3: 0,
        };
        self.td.write_volatile(idx, &td);
    }

    pub fn set_transmit_des(&mut self, idx: usize, skb_addr: usize, len: usize) {
        let mut td = self.td.read_volatile(idx).unwrap();

        td.tdes0 = buff_addr as u32;
        td.tdes2 = len;
        td.tdes3 |= 1 << 29;
        td.tdes3 |= 1 << 28;
        td.tdes3 |= 1 << 31;
        self.td.write_volatile(idx, &td);
    }
}
