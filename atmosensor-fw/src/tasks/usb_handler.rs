use core::mem::MaybeUninit;
use stm32f1xx_hal::pac::{interrupt, Interrupt, NVIC};
use stm32f1xx_hal::{
    usb::Peripheral,
    usb::{UsbBus, UsbBusType},
};
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use crate::tasks::{push_new_cmd, Command, CommandQueue};
use crate::utils::CobsBuffer;

static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBusType>> = None;
static mut USB_DEVICE: Option<UsbDevice<UsbBusType>> = None;
static mut USB_TX_RAW_BUFFER: [u8; 1024] = [0u8; 1024];
static mut USB_TX_BUFFER: MaybeUninit<CobsBuffer<1024>> = MaybeUninit::uninit();
static mut USB_RX_RAW_BUFFER: [u8; 1024] = [0u8; 1024];
static mut USB_RX_BUFFER: MaybeUninit<CobsBuffer<1024>> = MaybeUninit::uninit();
static mut USB_RAW_BUFFER: [u8; 1024] = [0u8; 1024];
static mut NOT_ENCODED_YET_BUFFER: [u8; 256] = [0u8; 256];

static mut USB_RESPONSE_QUEUE: CommandQueue<12> = CommandQueue::new();

pub fn send_usb_msg(cmd: &Command) {
    critical_section::with(|_cs| {
        let _ = unsafe { USB_RESPONSE_QUEUE.push(*cmd) };
    })
}

fn get_next_outbound_usb_msg() -> Option<Command> {
    critical_section::with(|_cs| unsafe { USB_RESPONSE_QUEUE.pop() })
}

pub struct UsbHandler {}

impl UsbHandler {
    pub fn new(peripheral: Peripheral) -> Self {
        unsafe {
            let usb_bus = UsbBus::new(peripheral);
            USB_BUS = Some(usb_bus);
            USB_SERIAL = Some(SerialPort::new(USB_BUS.as_ref().unwrap()));
            let usb_dev =
                UsbDeviceBuilder::new(USB_BUS.as_ref().unwrap(), UsbVidPid(0x16c0, 0x27dd))
                    .manufacturer("Snostorm Labs")
                    .product("Atmosensor")
                    .serial_number("FAKE")
                    .device_class(USB_CLASS_CDC)
                    .build();
            USB_DEVICE = Some(usb_dev);

            USB_TX_BUFFER = MaybeUninit::new(CobsBuffer::new(&mut USB_TX_RAW_BUFFER));
            USB_RX_BUFFER = MaybeUninit::new(CobsBuffer::new(&mut USB_RX_RAW_BUFFER));
        }

        let cmd_rcvr = UsbHandler {};

        unsafe {
            NVIC::unmask(Interrupt::USB_HP_CAN_TX);
            NVIC::unmask(Interrupt::USB_LP_CAN_RX0);
        }

        cmd_rcvr
    }

    pub fn run(&self) {
        let rx_buffer = unsafe { USB_RX_BUFFER.assume_init_mut() };
        let mut cmd_buf = [0u8; 1024];
        if rx_buffer.data[0] == 0x00 && rx_buffer.data[1] == 0x00 {
            return;
        }

        let mut encoded_buf = [0u8; 4];
        let serial = unsafe { USB_SERIAL.as_mut().unwrap() };
        let packet = critical_section::with(|cs| rx_buffer.read_packet(&cs, &mut cmd_buf));

        match packet {
            Ok(cmd_bytes) => {
                if cmd_bytes >= 2 {
                    if cmd_buf[0] == 0xde && cmd_buf[1] == 0x00 {
                        let encoded_bytes = cobs::encode(&[0xde, 0x01], &mut encoded_buf);
                        let _ = serial.write(&encoded_buf[..encoded_bytes]);
                        let _ = serial.flush();
                    } else if let Some(cmd) = crate::tasks::Command::from_bytes(&cmd_buf[..]) {
                        push_new_cmd(&cmd);
                    }
                }
            }
            _ => {}
        }

        let usb_msg = get_next_outbound_usb_msg();
        let tx_buffer = unsafe { &mut USB_TX_RAW_BUFFER };
        let serial = unsafe { USB_SERIAL.as_mut().unwrap() };
        if let Some(cmd) = usb_msg {
            unsafe {
                if let Ok(cmd_bytes) = cmd.to_bytes(&mut NOT_ENCODED_YET_BUFFER) {
                    let encoded_bytes =
                        cobs::encode(&NOT_ENCODED_YET_BUFFER[..cmd_bytes], tx_buffer);
                    let _ = serial.write(&tx_buffer[..encoded_bytes]);
                    let _ = serial.flush();
                }
            }
        }
    }
}

#[interrupt]
fn USB_HP_CAN_TX() {
    usb_interrupt();
}

#[interrupt]
fn USB_LP_CAN_RX0() {
    usb_interrupt();
}

fn usb_interrupt() {
    let usb_dev = unsafe { USB_DEVICE.as_mut().unwrap() };
    let serial = unsafe { USB_SERIAL.as_mut().unwrap() };

    if !usb_dev.poll(&mut [serial]) {
        return;
    }

    critical_section::with(|cs| unsafe {
        let rx_buffer = USB_RX_BUFFER.assume_init_mut();
        if let Ok(bytes_read) = serial.read(&mut USB_RAW_BUFFER) {
            rx_buffer.write_bytes(&cs, &USB_RAW_BUFFER[..bytes_read]);
        }
    });
}
