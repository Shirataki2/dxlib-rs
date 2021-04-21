use dxlib_sys::{
    dx_DrawBox, dx_DrawBoxAA, dx_DrawCircle, dx_DrawCircleAA, dx_DrawLine, dx_DrawLineAA,
    dx_DrawOval, dx_DrawOvalAA, dx_DrawPixel, dx_DrawTriangle, dx_DrawTriangleAA, FALSE, TRUE,
};

use crate::{
    color::Color,
    error::{I32CodeExt, Result},
    math::vector::Vector2,
};

pub enum Shape {
    Line(Line),
    LineAntiAlias(LineAntiAlias),
    Rect(Rect),
    RectAntiAlias(RectAntiAlias),
    Circle(Circle),
    CircleAntiAlias(CircleAntiAlias),
    Oval(Oval),
    OvalAntiAlias(OvalAntiAlias),
    Triangle(Triangle),
    TriangleAntiAlias(TriangleAntiAlias),
    Pixel(Pixel),
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub start: Vector2<i32>,
    pub end: Vector2<i32>,
    pub color: Color<u8>,
    pub thickness: i32,
}

impl Default for Line {
    fn default() -> Self {
        Self {
            start: Vector2::from([0, 0]),
            end: Vector2::from([64, 64]),
            color: Color::default(),
            thickness: 1,
        }
    }
}

impl Line {
    pub fn from_coords(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: Color<u8>,
        thickness: i32,
    ) -> Self {
        let start = Vector2::from([x1, y1]);
        let end = Vector2::from([x2, y2]);
        Self {
            start,
            end,
            color,
            thickness,
        }
    }

    pub fn draw(&self) -> Result<()> {
        unsafe {
            dx_DrawLine(
                self.start[0],
                self.start[1],
                self.end[0],
                self.end[1],
                self.color.as_u32(),
                self.thickness,
            )
            .ensure_zero()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LineAntiAlias {
    pub start: Vector2<f32>,
    pub end: Vector2<f32>,
    pub color: Color<u8>,
    pub thickness: f32,
}

impl Default for LineAntiAlias {
    fn default() -> Self {
        Self {
            start: Vector2::from([0.0, 0.0]),
            end: Vector2::from([64.0, 64.0]),
            color: Color::default(),
            thickness: 1.0,
        }
    }
}

impl LineAntiAlias {
    pub fn from_coords(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        color: Color<u8>,
        thickness: f32,
    ) -> Self {
        let start = Vector2::from([x1, y1]);
        let end = Vector2::from([x2, y2]);
        Self {
            start,
            end,
            color,
            thickness,
        }
    }

    pub fn draw(&self) -> Result<()> {
        unsafe {
            dx_DrawLineAA(
                self.start[0],
                self.start[1],
                self.end[0],
                self.end[1],
                self.color.as_u32(),
                self.thickness,
            )
            .ensure_zero()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub left_top: Vector2<i32>,
    pub right_bottom: Vector2<i32>,
    pub color: Color<u8>,
    pub filled: bool,
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            left_top: Vector2::default(),
            right_bottom: Vector2::from([64, 64]),
            color: Color::default(),
            filled: false,
        }
    }
}

impl Rect {
    pub fn from_coords(x1: i32, y1: i32, x2: i32, y2: i32, color: Color<u8>, filled: bool) -> Self {
        let left_top = Vector2::from([x1, y1]);
        let right_bottom = Vector2::from([x2, y2]);
        Self {
            left_top,
            right_bottom,
            color,
            filled,
        }
    }

    pub fn draw(&self) -> Result<()> {
        unsafe {
            dx_DrawBox(
                self.left_top[0],
                self.left_top[1],
                self.right_bottom[0],
                self.right_bottom[1],
                self.color.as_u32(),
                if self.filled { TRUE } else { FALSE },
            )
            .ensure_zero()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RectAntiAlias {
    pub left_top: Vector2<f32>,
    pub right_bottom: Vector2<f32>,
    pub color: Color<u8>,
    pub filled: bool,
    pub thickness: f32,
}

impl Default for RectAntiAlias {
    fn default() -> Self {
        Self {
            left_top: Vector2::default(),
            right_bottom: Vector2::from([64.0, 64.0]),
            color: Color::default(),
            filled: false,
            thickness: 1.0,
        }
    }
}

impl RectAntiAlias {
    pub fn from_coords(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        color: Color<u8>,
        filled: bool,
        thickness: f32,
    ) -> Self {
        let left_top = Vector2::from([x1, y1]);
        let right_bottom = Vector2::from([x2, y2]);
        Self {
            left_top,
            right_bottom,
            color,
            filled,
            thickness,
        }
    }

    pub fn draw(&self) -> Result<()> {
        unsafe {
            dx_DrawBoxAA(
                self.left_top[0],
                self.left_top[1],
                self.right_bottom[0],
                self.right_bottom[1],
                self.color.as_u32(),
                self.filled as i32,
                self.thickness,
            )
            .ensure_zero()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Vector2<i32>,
    pub radius: i32,
    pub color: Color<u8>,
    pub filled: bool,
    pub thickness: i32,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            center: Vector2::from([64, 64]),
            radius: 64,
            color: Color::default(),
            filled: false,
            thickness: 1,
        }
    }
}

impl Circle {
    pub fn from_coords(
        x: i32,
        y: i32,
        radius: i32,
        color: Color<u8>,
        filled: bool,
        thickness: i32,
    ) -> Self {
        let center = Vector2::from([x, y]);
        Self {
            center,
            radius,
            color,
            filled,
            thickness,
        }
    }

    pub fn draw(&self) -> Result<()> {
        unsafe {
            dx_DrawCircle(
                self.center[0],
                self.center[1],
                self.radius,
                self.color.as_u32(),
                self.filled as i32,
                self.thickness,
            )
            .ensure_zero()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CircleAntiAlias {
    pub center: Vector2<f32>,
    pub radius: f32,
    pub resolution: i32,
    pub color: Color<u8>,
    pub filled: bool,
    pub thickness: f32,
}

impl Default for CircleAntiAlias {
    fn default() -> Self {
        Self {
            center: Vector2::from([64.0, 64.0]),
            radius: 64.0,
            resolution: 32,
            color: Color::default(),
            filled: false,
            thickness: 1.0,
        }
    }
}

impl CircleAntiAlias {
    pub fn from_coords(
        x: f32,
        y: f32,
        radius: f32,
        resolution: i32,
        color: Color<u8>,
        filled: bool,
        thickness: f32,
    ) -> Self {
        let center = Vector2::from([x, y]);
        Self {
            center,
            radius,
            resolution,
            color,
            filled,
            thickness,
        }
    }

    pub fn draw(&self) -> Result<()> {
        unsafe {
            dx_DrawCircleAA(
                self.center[0],
                self.center[1],
                self.radius,
                self.resolution,
                self.color.as_u32(),
                self.filled as i32,
                self.thickness,
            )
            .ensure_zero()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Oval {
    pub center: Vector2<i32>,
    pub radius: Vector2<i32>,
    pub color: Color<u8>,
    pub filled: bool,
    pub thickness: i32,
}

impl Default for Oval {
    fn default() -> Self {
        Self {
            center: Vector2::from([64, 64]),
            radius: Vector2::from([64, 32]),
            color: Color::default(),
            filled: false,
            thickness: 1,
        }
    }
}

impl Oval {
    pub fn from_coords(
        x: i32,
        y: i32,
        rx: i32,
        ry: i32,
        color: Color<u8>,
        filled: bool,
        thickness: i32,
    ) -> Self {
        let center = Vector2::from([x, y]);
        let radius = Vector2::from([rx, ry]);
        Self {
            center,
            radius,
            color,
            filled,
            thickness,
        }
    }

    pub fn draw(&self) -> Result<()> {
        unsafe {
            dx_DrawOval(
                self.center[0],
                self.center[1],
                self.radius[0],
                self.radius[1],
                self.color.as_u32(),
                self.filled as i32,
                self.thickness,
            )
            .ensure_zero()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OvalAntiAlias {
    pub center: Vector2<f32>,
    pub radius: Vector2<f32>,
    pub resolution: i32,
    pub color: Color<u8>,
    pub filled: bool,
    pub thickness: f32,
}

impl Default for OvalAntiAlias {
    fn default() -> Self {
        Self {
            center: Vector2::from([64.0, 64.0]),
            radius: Vector2::from([64.0, 32.0]),
            resolution: 32,
            color: Color::default(),
            filled: false,
            thickness: 1.0,
        }
    }
}

impl OvalAntiAlias {
    #[allow(clippy::too_many_arguments)]
    pub fn from_coords(
        x: f32,
        y: f32,
        rx: f32,
        ry: f32,
        resolution: i32,
        color: Color<u8>,
        filled: bool,
        thickness: f32,
    ) -> Self {
        let center = Vector2::from([x, y]);
        let radius = Vector2::from([rx, ry]);
        Self {
            center,
            radius,
            resolution,
            color,
            filled,
            thickness,
        }
    }

    pub fn draw(&self) -> Result<()> {
        unsafe {
            dx_DrawOvalAA(
                self.center[0],
                self.center[1],
                self.radius[0],
                self.radius[1],
                self.resolution,
                self.color.as_u32(),
                self.filled as i32,
                self.thickness,
            )
            .ensure_zero()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub v1: Vector2<i32>,
    pub v2: Vector2<i32>,
    pub v3: Vector2<i32>,
    pub color: Color<u8>,
    pub filled: bool,
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            v1: Vector2::from([0, 0]),
            v2: Vector2::from([0, 60]),
            v3: Vector2::from([40, 30]),
            color: Color::default(),
            filled: false,
        }
    }
}

impl Triangle {
    pub fn draw(&self) -> Result<()> {
        unsafe {
            dx_DrawTriangle(
                self.v1[0],
                self.v1[1],
                self.v2[0],
                self.v2[1],
                self.v3[0],
                self.v3[1],
                self.color.as_u32(),
                self.filled as i32,
            )
            .ensure_zero()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TriangleAntiAlias {
    pub v1: Vector2<f32>,
    pub v2: Vector2<f32>,
    pub v3: Vector2<f32>,
    pub color: Color<u8>,
    pub filled: bool,
    pub thickness: f32,
}

impl Default for TriangleAntiAlias {
    fn default() -> Self {
        Self {
            v1: Vector2::from([0.0, 0.0]),
            v2: Vector2::from([0.0, 60.0]),
            v3: Vector2::from([40.0, 30.0]),
            color: Color::default(),
            filled: false,
            thickness: 1.0,
        }
    }
}

impl TriangleAntiAlias {
    pub fn draw(&self) -> Result<()> {
        unsafe {
            dx_DrawTriangleAA(
                self.v1[0],
                self.v1[1],
                self.v2[0],
                self.v2[1],
                self.v3[0],
                self.v3[1],
                self.color.as_u32(),
                self.filled as i32,
                self.thickness,
            )
            .ensure_zero()
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Pixel {
    coord: Vector2<i32>,
    color: Color<u8>,
}

impl Pixel {
    pub fn draw(&self) -> Result<()> {
        unsafe { dx_DrawPixel(self.coord[0], self.coord[1], self.color.as_u32()).ensure_zero() }
    }
}
