#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

extern crate alloc;

use core::cell::RefCell;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDevice;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::dma::{DmaRxBuf, DmaTxBuf};
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig};
use esp_hal::spi::Mode;
use esp_hal::spi::master::{Config, Spi, SpiDmaBus};
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{Blocking, dma_buffers};
use frr_shared::prelude::*;
use mipidsi::interface::SpiInterface;
use mipidsi::models::ST7789;
use mipidsi::{Display, NoResetPin};
use static_cell::StaticCell;
use {esp_backtrace as _, esp_println as _};

esp_bootloader_esp_idf::esp_app_desc!();

type AppType = App<
    UiStateTask<
        PushButtonInput<Input<'static>>,
        ColoredLcdDisplay<
            Display<
                SpiInterface<
                    'static,
                    SpiDevice<
                        'static,
                        CriticalSectionRawMutex,
                        SpiDmaBus<'static, Blocking>,
                        Output<'static>,
                    >,
                    Output<'static>,
                >,
                ST7789,
                NoResetPin,
            >,
        >,
    >,
    SsrDriver<Output<'static>>,
    Max31865Sampler<
        SpiDevice<'static, CriticalSectionRawMutex, SpiDmaBus<'static, Blocking>, Output<'static>>,
        Delay,
    >,
>;

type ConcreteDriverType = SsrDriver<Output<'static>>;

type ConcreteSamplerType = Max31865Sampler<
    SpiDevice<'static, CriticalSectionRawMutex, SpiDmaBus<'static, Blocking>, Output<'static>>,
    Delay,
>;

type ConcreteDisplayType = UiStateTask<
    PushButtonInput<Input<'static>>,
    ColoredLcdDisplay<
        Display<
            SpiInterface<
                'static,
                SpiDevice<
                    'static,
                    CriticalSectionRawMutex,
                    SpiDmaBus<'static, Blocking>,
                    Output<'static>,
                >,
                Output<'static>,
            >,
            ST7789,
            NoResetPin,
        >,
    >,
>;

static APP: StaticCell<AppType> = StaticCell::new();

static PROCESSOR: StaticCell<FrrProcessor<ConcreteDriverType>> = StaticCell::new();

static SAMPLER: StaticCell<ConcreteSamplerType> = StaticCell::new();

static UI: StaticCell<ConcreteDisplayType> = StaticCell::new();

static SPI_BUS: StaticCell<Mutex<CriticalSectionRawMutex, RefCell<SpiDmaBus<'static, Blocking>>>> =
    StaticCell::new();

static DI_BUFFER: StaticCell<[u8; 512]> = StaticCell::new();

/// NOTE: Implementation note
/// to use frr_shared with embassy_embedded_hal, or embedded_hal is quite verbose,
/// here is quick summary.
/// |
/// for Display inside UiStateTask
/// - Display is using mipidsi ST7789
/// - internally mipidsi is using display_interface (specially display_interface_spi)
/// - to create display_interface_spi you need create spi_device, based on current source code
///   mipidsi is using blocking spi from embedded_hal
/// - one way to create spi_device is using spi_bus with cs, we can use
///   embassy_embedded_hal::shared_bus::blocking::spi::SpiDevice,
/// - to create spi_bus we need to create static (singleton) SPI_BUS with static_cell, mutex and refcell, then
///   then initiate it with real spi_bus, there is several way to create spi bus as described in
///   the docs https://docs.espressif.com/projects/rust/esp-hal/1.0.0/esp32/esp_hal/spi/master/index.html,
///   one of it is to use embassy_embedded_hal

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 98768);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    let key_up = Input::new(peripherals.GPIO1, InputConfig::default());
    let key_right = Input::new(peripherals.GPIO2, InputConfig::default());
    let key_down = Input::new(peripherals.GPIO3, InputConfig::default());
    let key_left = Input::new(peripherals.GPIO4, InputConfig::default());
    let key_back = Input::new(peripherals.GPIO5, InputConfig::default());
    let key_enter = Input::new(peripherals.GPIO13, InputConfig::default());

    let inputs = PushButtonInput::new(key_up, key_right, key_down, key_left, key_back, key_enter);

    let sclk = peripherals.GPIO0;
    let miso = peripherals.GPIO12;
    let mosi = peripherals.GPIO14;
    let cs = Output::new(peripherals.GPIO16, Level::Low, OutputConfig::default());
    let dc = Output::new(peripherals.GPIO17, Level::Low, OutputConfig::default());

    let dma_channel = peripherals.DMA_SPI2;

    let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = dma_buffers!(32000);
    let dma_rx_buf = DmaRxBuf::new(rx_descriptors, rx_buffer).unwrap();
    let dma_tx_buf = DmaTxBuf::new(tx_descriptors, tx_buffer).unwrap();

    let spi_bus = Spi::new(
        peripherals.SPI2,
        Config::default()
            .with_frequency(Rate::from_khz(100))
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(sclk)
    .with_mosi(mosi)
    .with_miso(miso)
    .with_dma(dma_channel)
    .with_buffers(dma_rx_buf, dma_tx_buf);

    let di_buffer = DI_BUFFER.init([0u8; 512]);

    let static_bus = SPI_BUS.init(Mutex::new(RefCell::new(spi_bus)));

    let spi_dev = SpiDevice::new(static_bus, cs);

    let di = SpiInterface::new(spi_dev, dc, di_buffer);

    let mut delay = Delay::new();

    let st7789 = mipidsi::Builder::new(ST7789, di)
        .display_size(320u16, 172u16)
        .init(&mut delay)
        .unwrap();

    let color_display = ColoredLcdDisplay::new(st7789);

    let ssr_pin = Output::new(peripherals.GPIO21, Level::Low, OutputConfig::default());

    let cs_max31865 = Output::new(peripherals.GPIO22, Level::Low, OutputConfig::default());

    let max31865_spi = SpiDevice::new(static_bus, cs_max31865);

    let max31865_dev = Max31865::new(max31865_spi, delay, 1u8, 100.0, 400.0);

    let ui = UiStateTask::new(inputs, color_display);
    let driver = SsrDriver::new(ssr_pin);

    let sampler = Max31865Sampler::new(max31865_dev);

    let props = AppBuilderProps {
        ui,
        driver,
        sampler,
    };

    let app = build_app(props);
    let processor = app.processor_task;
    let sampler = app.sampler_task;
    let uitask = app.ui_task;

    let processor = PROCESSOR.init(processor);
    let sampler = SAMPLER.init(sampler);
    let ui = UI.init(uitask);

    spawner.spawn(ui_task(ui)).unwrap();
    spawner.spawn(sampler_task(sampler)).unwrap();
    spawner.spawn(processor_task(processor)).unwrap();
}

#[embassy_executor::task]
async fn processor_task(processor_task: &'static mut FrrProcessor<ConcreteDriverType>) {
    processor_task.run().await;
}

#[embassy_executor::task]
async fn ui_task(display_task: &'static mut ConcreteDisplayType) {
    display_task.run().await;
}

#[embassy_executor::task]
async fn sampler_task(sampler_task: &'static mut ConcreteSamplerType) {
    sampler_task.run().await;
}
