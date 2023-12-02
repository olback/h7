use crate::Led;
use core::{cell::RefCell, mem};
use critical_section::Mutex;
use embedded_display_controller::{DisplayControllerLayer, PixelFormat};
use h7_display::{FrameBuffer, H7Display};
use stm32h7xx_hal::{interrupt, ltdc::LtdcLayer1};

type Pixel = embedded_graphics::pixelcolor::Rgb565;

pub const SCREEN_WIDTH: usize = 1024;
pub const SCREEN_HEIGHT: usize = 768;
// pub const PIXEL_CLOCK: fugit::Hertz<u32> = 57800u32.kHz();
pub const PIXEL_CLOCK: fugit::HertzU32 = fugit::Rate::<u32, _, _>::kHz(57800u32);
pub const H_BACK_PORCH: u16 = 80;
pub const H_FRONT_PORCH: u16 = 24;
pub const H_SYNC_LEN: u16 = 68;
pub const H_SYNC_POL: bool = false;
pub const V_BACK_PORCH: u16 = 29;
pub const V_FRONT_PORCH: u16 = 3;
pub const V_SYNC_LEN: u16 = 6;
pub const V_SYNC_POL: bool = false;

pub const FRAME_BUFFER_SIZE: usize = mem::size_of::<FrameBuffer<Pixel, SCREEN_WIDTH, SCREEN_HEIGHT>>();
pub const FRAME_BUFFER_ALLOC_SIZE: usize = FRAME_BUFFER_SIZE * 2;
pub const FRAME_RATE: u32 = 30;

pub static GPU: Mutex<RefCell<Option<Gpu>>> = Mutex::new(RefCell::new(None));

pub struct Gpu {
    display: H7Display<'static, Pixel, SCREEN_HEIGHT, SCREEN_HEIGHT>,
    layer: LtdcLayer1,
}

impl Gpu {
    pub fn new(display: H7Display<'static, Pixel, SCREEN_HEIGHT, SCREEN_HEIGHT>, mut layer: LtdcLayer1) -> Self {
        unsafe { layer.enable(display.front_buffer().as_ptr() as *const u16, PixelFormat::RGB565) };
        Self { display, layer }
    }

    pub fn swap(&mut self) {
        if !self.layer.is_swap_pending() {
            let (front, _) = self.display.swap_buffers();
            unsafe { self.layer.swap_framebuffer(front.as_ptr() as *const u16) };
            unsafe { Led::Blue.toggle() };
            core::hint::black_box(front);
            cortex_m::asm::dmb();
            cortex_m::asm::dsb();
        }

        // while self.layer.is_swap_pending() {
        //     cortex_m::asm::nop();
        // }
    }
}

// impl core::ops::Deref for Gpu {
//     type Target = H7Display<'static, Pixel, SCREEN_HEIGHT, SCREEN_HEIGHT>;

//     fn deref(&self) -> &Self::Target {
//         &self.display
//     }
// }

// impl core::ops::DerefMut for Gpu {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.display
//     }
// }

// Interrupt to swap framebuffers
#[interrupt]
fn TIM2() {
    unsafe {
        crate::utils::interrupt_free(|cs| {
            if let Some(disp) = GPU.borrow(cs).borrow_mut().as_mut() {
                disp.swap();
            }
        });
        (*stm32h7xx_hal::pac::TIM2::PTR).sr.write(|w| w.uif().clear_bit());
    };
}

#[interrupt]
fn LTDC() {
    unsafe { Led::Red.on() };
}

#[interrupt]
fn DMA2D() {
    unsafe { Led::Red.on() };
}

#[interrupt]
fn DSI() {
    unsafe { Led::Red.on() };
}
