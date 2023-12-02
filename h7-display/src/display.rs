use {crate::FrameBuffer, core::ops::Range, embedded_graphics_core::prelude::*};

#[derive(Debug)]
pub struct H7Display<'b, COLOR: PixelColor, const WIDTH: usize, const HEIGHT: usize>
where
    [(); WIDTH * HEIGHT]:,
{
    swapped: bool,
    buffers: [&'b mut FrameBuffer<COLOR, WIDTH, HEIGHT>; 2],
}

impl<'b, COLOR: PixelColor, const WIDTH: usize, const HEIGHT: usize> H7Display<'b, COLOR, WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
{
    pub const fn new(fb0: &'b mut FrameBuffer<COLOR, WIDTH, HEIGHT>, fb1: &'b mut FrameBuffer<COLOR, WIDTH, HEIGHT>) -> Self {
        Self {
            swapped: false,
            buffers: [fb0, fb1],
        }
    }

    #[inline(always)]
    pub fn front_buffer(&self) -> &FrameBuffer<COLOR, WIDTH, HEIGHT> {
        // SAFETY: self.swapped is always either 0 or 1 and always in range of the self.buffers array
        unsafe { self.buffers.get_unchecked(self.swapped as usize) }
    }

    #[inline(always)]
    unsafe fn front_buffer_mut(&mut self) -> &FrameBuffer<COLOR, WIDTH, HEIGHT> {
        // SAFETY: self.swapped is always either 0 or 1 and always in range of the self.buffers array.
        // SAFETY: This is still unsafe since the front buffer is probably being drawn to the display.
        unsafe { self.buffers.get_unchecked_mut(self.swapped as usize) }
    }

    #[inline(always)]
    pub fn back_buffer(&self) -> &FrameBuffer<COLOR, WIDTH, HEIGHT> {
        // SAFETY: self.swapped is always either 0 or 1 and always in range of the self.buffers array
        unsafe { self.buffers.get_unchecked((!self.swapped) as usize) }
    }

    #[inline(always)]
    pub fn back_buffer_mut(&mut self) -> &mut FrameBuffer<COLOR, WIDTH, HEIGHT> {
        // SAFETY: self.swapped is always either 0 or 1 and always in range of the self.buffers array
        unsafe { self.buffers.get_unchecked_mut((!self.swapped) as usize) }
    }

    #[inline(always)]
    fn get_buffers(&mut self) -> (&FrameBuffer<COLOR, WIDTH, HEIGHT>, &mut FrameBuffer<COLOR, WIDTH, HEIGHT>) {
        (self.front_buffer(), unsafe { core::mem::transmute(self.back_buffer()) })
    }

    #[inline(always)]
    pub fn swap_buffers(&mut self) -> (&FrameBuffer<COLOR, WIDTH, HEIGHT>, &mut FrameBuffer<COLOR, WIDTH, HEIGHT>) {
        self.swapped = !self.swapped;

        let (front, back) = self.get_buffers();

        // after swap, copy the new front to the new back
        // back.copy_from_slice(&**front);

        (front, back)
    }

    pub fn scroll(&mut self, px: i32, fill: COLOR) {
        match px {
            0 => { /* nop */ }
            i32::MIN..=-1 => {
                let n_rows = px.unsigned_abs() as usize;
                let back_buffer_ptr = self.back_buffer_mut().as_mut_ptr();
                unsafe {
                    core::ptr::copy(back_buffer_ptr, back_buffer_ptr.add(n_rows * WIDTH), (self.height() - n_rows) * WIDTH);

                    let r1 = Self::row_range(0);
                    let r2 = Self::row_range(n_rows);
                    self.back_buffer_mut().get_unchecked_mut(r1.start..r2.start).fill(fill);
                }
            }
            1..=i32::MAX => {
                let n_rows = px.unsigned_abs() as usize;
                let back_buffer_ptr = self.back_buffer_mut().as_mut_ptr();
                unsafe {
                    core::ptr::copy(back_buffer_ptr.add(n_rows * WIDTH), back_buffer_ptr, (self.height() - n_rows) * WIDTH);

                    let r1 = Self::row_range(HEIGHT - n_rows);
                    let r2 = Self::row_range(HEIGHT);
                    self.back_buffer_mut().get_unchecked_mut(r1.start..r2.start).fill(fill);
                }
            }
        }
    }

    #[inline(always)]
    pub const fn width(&self) -> usize {
        WIDTH
    }

    #[inline(always)]
    pub const fn height(&self) -> usize {
        HEIGHT
    }

    #[inline(always)]
    const fn row_range(row: usize) -> Range<usize> {
        // assert!(row < HEIGHT);
        let start = row * WIDTH;
        start..(start + WIDTH)
    }

    #[inline]
    fn bounded_x(x: i32) -> usize {
        x.clamp(0, WIDTH as i32) as usize
    }

    #[inline]
    fn bounded_y(y: i32) -> usize {
        y.clamp(0, HEIGHT as i32) as usize
    }
}

impl<'b, COLOR: PixelColor, const WIDTH: usize, const HEIGHT: usize> DrawTarget for H7Display<'b, COLOR, WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
{
    type Color = COLOR;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics_core::Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels {
            if point.x >= 0 && point.x < WIDTH as i32 && point.y >= 0 && point.y < HEIGHT as i32 {
                *self.back_buffer_mut().at_mut(point.x as usize, point.y as usize) = color;
            }
        }

        Ok(())
    }

    fn fill_contiguous<I>(&mut self, area: &embedded_graphics_core::primitives::Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        self.draw_iter(area.points().zip(colors).map(|(pos, color)| embedded_graphics_core::Pixel(pos, color)))
    }

    fn fill_solid(&mut self, area: &embedded_graphics_core::primitives::Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        // This impl is ~1000x faster than `self.fill_contiguous(area, core::iter::repeat(color))`
        let x_start = Self::bounded_x(area.top_left.x);
        let x_end = Self::bounded_x(area.top_left.x + area.size.width as i32);
        let x_len = x_end - x_start;

        let y_start = Self::bounded_y(area.top_left.y);
        let y_end = Self::bounded_y(area.top_left.y + area.size.height as i32);

        let back_buffer = self.back_buffer_mut();

        for y in y_start..y_end {
            let idx_start = FrameBuffer::<COLOR, WIDTH, HEIGHT>::xy_to_index(x_start, y);
            // back_buffer[idx_start..(idx_start + x_len)].fill(color);
            unsafe {
                back_buffer.get_unchecked_mut(idx_start..(idx_start + x_len)).fill(color);
            };
        }

        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.back_buffer_mut().fill(color);
        Ok(())
    }
}

impl<'b, COLOR: PixelColor, const WIDTH: usize, const HEIGHT: usize> OriginDimensions for H7Display<'b, COLOR, WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
{
    fn size(&self) -> Size {
        Size::new(WIDTH as u32, HEIGHT as u32)
    }
}
