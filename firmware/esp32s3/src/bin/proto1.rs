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
use embassy_time::Timer;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::dma::{DmaRxBuf, DmaTxBuf};
use esp_hal::gpio::{DriveStrength, Input, InputConfig, Level, Output, OutputConfig, Pull};
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

    // NOTE: if not work, look at esp-rs example
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 73744);

    let scl_mosi = peripherals.GPIO11;
    let scl_sck = peripherals.GPIO12;
    let scl_miso = peripherals.GPIO13;

    let max31865_ddry = peripherals.GPIO42;
    let max31865_cs = peripherals.GPIO5;

    let ssr1_pin = peripherals.GPIO1;
    let ssr2_pin = peripherals.GPIO2;

    let zero_crossing = peripherals.GPIO3;

    let sw1_pin = peripherals.GPIO15;
    let sw2_pin = peripherals.GPIO16;
    let sw3_pin = peripherals.GPIO46;
    let sw4_pin = peripherals.GPIO17;
    let sw5_pin = peripherals.GPIO18;
    let sw6_pin = peripherals.GPIO21;

    let lcd_dc = peripherals.GPIO8;
    let lcd_cs = peripherals.GPIO10;

    let led_indicator_pin = peripherals.GPIO47;

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    let led_indicator_out_config = OutputConfig::default()
        .with_pull(Pull::Up)
        .with_drive_strength(DriveStrength::_40mA);

    let mut led_indicator = Output::new(led_indicator_pin, Level::Low, led_indicator_out_config);

    let key_up = Input::new(sw1_pin, InputConfig::default());
    let key_right = Input::new(sw2_pin, InputConfig::default());
    let key_down = Input::new(sw3_pin, InputConfig::default());
    let key_left = Input::new(sw4_pin, InputConfig::default());
    let key_back = Input::new(sw5_pin, InputConfig::default());
    let key_enter = Input::new(sw6_pin, InputConfig::default());

    let inputs = PushButtonInput::new(key_up, key_right, key_down, key_left, key_back, key_enter);

    let lcd_cs = Output::new(lcd_cs, Level::Low, OutputConfig::default());
    let lcd_dc = Output::new(lcd_dc, Level::Low, OutputConfig::default());

    // for esp32s3
    let dma_channel = peripherals.DMA_CH0;

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
    .with_sck(scl_sck)
    .with_mosi(scl_mosi)
    .with_miso(scl_miso)
    .with_dma(dma_channel)
    .with_buffers(dma_rx_buf, dma_tx_buf);

    let di_buffer = DI_BUFFER.init([0u8; 512]);

    let static_bus = SPI_BUS.init(Mutex::new(RefCell::new(spi_bus)));

    let spi_dev = SpiDevice::new(static_bus, lcd_cs);

    let di = SpiInterface::new(spi_dev, lcd_dc, di_buffer);

    let mut delay = Delay::new();

    let st7789 = mipidsi::Builder::new(ST7789, di)
        .display_size(320u16, 172u16)
        .init(&mut delay)
        .unwrap();

    let color_display = ColoredLcdDisplay::new(st7789);

    let ssr_pin = Output::new(ssr1_pin, Level::Low, OutputConfig::default());

    let cs_max31865 = Output::new(max31865_cs, Level::Low, OutputConfig::default());

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

    loop {
        led_indicator.toggle();
        Timer::after_millis(1000).await
    }
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
