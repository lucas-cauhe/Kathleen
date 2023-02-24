
use std::sync::mpsc::{Receiver};

pub struct Context2Thread {
    pub rx_ch1: Receiver<i32>,
    pub rx_ch2: Receiver<i32>,
    thread_is_out: bool,
}

impl Context2Thread {
    pub fn new(rx1: Receiver<i32>, rx2: Receiver<i32>) -> Self {
        Context2Thread { 
            rx_ch1: rx1, 
            rx_ch2: rx2, 
            thread_is_out: false,
        }
    }
}