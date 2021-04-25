use std::{ffi::CStr, ptr};

use crate::{
    color::Color,
    error::{I32CodeExt, Result},
    math::vector::Vector2,
    utils::to_sjis_bytes,
};
use dxlib_sys::{
    consts::*, dx_CreateFontToHandle, dx_DeleteFontToHandle, dx_DrawString, dx_DrawStringToHandle,
};
use smart_default::SmartDefault;

#[derive(Debug, Clone, SmartDefault)]
pub struct TextStyle<'f> {
    pub coord: Vector2<i32>,
    #[default(_code = "Color::white()")]
    pub color: Color<u8>,
    #[default(_code = "Color::transparent()")]
    pub edge_color: Color<u8>,
    pub font: Option<&'f Font>,
}

impl<'f> TextStyle<'f> {
    pub fn builder() -> TextStyleBuilder<'f> {
        TextStyleBuilder::default()
    }

    pub fn draw(&self, string: &str) -> Result<()> {
        let s = to_sjis_bytes(string);
        let s = CStr::from_bytes_with_nul(&s)?;
        unsafe {
            match &self.font {
                Some(font) => {
                    dx_DrawStringToHandle(
                        self.coord[0],
                        self.coord[1],
                        s.as_ptr(),
                        self.color.as_u32(),
                        font.handle,
                        self.edge_color.as_u32(),
                        0,
                    )
                    .ensure_zero()?;
                }
                None => {
                    dx_DrawString(
                        self.coord[0],
                        self.coord[1],
                        s.as_ptr(),
                        self.color.as_u32(),
                        self.edge_color.as_u32(),
                    )
                    .ensure_zero()?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct TextStyleBuilder<'f> {
    coord: Option<Vector2<i32>>,
    color: Option<Color<u8>>,
    edge_color: Option<Color<u8>>,
    font: Option<&'f Font>,
}

impl<'f> TextStyleBuilder<'f> {
    pub fn coord(&mut self, v: Vector2<i32>) -> &mut Self {
        self.coord = Some(v);
        self
    }

    pub fn color(&mut self, c: Color<u8>) -> &mut Self {
        self.color = Some(c);
        self
    }

    pub fn edge_color(&mut self, c: Color<u8>) -> &mut Self {
        self.edge_color = Some(c);
        self
    }

    pub fn font(&mut self, font: &'f Font) -> &mut Self {
        self.font = Some(font);
        self
    }

    pub fn build(&mut self) -> TextStyle {
        let mut text = TextStyle::default();
        if let Some(coord) = self.coord {
            text.coord = coord;
        }
        if let Some(color) = self.color {
            text.color = color;
        }
        if let Some(edge_color) = self.edge_color {
            text.edge_color = edge_color;
        }
        text.font = self.font;
        text
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum FontType {
    Normal = DX_FONTTYPE_NORMAL,
    Edge = DX_FONTTYPE_EDGE,
    AntiAliasing = DX_FONTTYPE_ANTIALIASING,
    AntiAliasing4x4 = DX_FONTTYPE_ANTIALIASING_4X4,
    AntiAliasing8x8 = DX_FONTTYPE_ANTIALIASING_8X8,
    AntiAliasingEdge4x4 = DX_FONTTYPE_ANTIALIASING_EDGE_4X4,
    AntiAliasingEdge8x8 = DX_FONTTYPE_ANTIALIASING_EDGE_8X8,
}

#[derive(Debug, Clone, SmartDefault)]
pub struct Font {
    handle: i32,
    name: Option<String>,
    #[default(_code = "-1")]
    size: i32,
    #[default(_code = "-1")]
    thickness: i32,
    #[default(_code = "FontType::Normal")]
    font_type: FontType,
}

impl Font {
    pub fn builder() -> FontBuilder {
        FontBuilder::default()
    }

    fn close(&self) -> Result<()> {
        unsafe { dx_DeleteFontToHandle(self.handle).ensure_zero() }
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

#[derive(Debug, Clone, Default)]
pub struct FontBuilder {
    name: Option<String>,
    size: Option<i32>,
    thickness: Option<i32>,
    font_type: Option<FontType>,
}

impl FontBuilder {
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(String::from(name));
        self
    }

    pub fn size(&mut self, size: i32) -> &mut Self {
        self.size = Some(size);
        self
    }

    pub fn thickness(&mut self, thickness: i32) -> &mut Self {
        self.thickness = Some(thickness);
        self
    }

    pub fn font_type(&mut self, font_type: FontType) -> &mut Self {
        self.font_type = Some(font_type);
        self
    }

    pub fn build(&mut self) -> Result<Font> {
        let mut font = Font::default();
        if let Some(ref name) = self.name {
            font.name = Some(name.clone());
        }
        if let Some(size) = self.size {
            font.size = size;
        }
        if let Some(thickness) = self.thickness {
            font.thickness = thickness;
        }
        if let Some(font_type) = self.font_type {
            font.font_type = font_type;
        }
        let name = match self.name.clone() {
            Some(name) => to_sjis_bytes(&name).as_ptr() as *const i8,
            None => ptr::null::<i8>(),
        };
        unsafe {
            let handle = dx_CreateFontToHandle(
                name,
                font.size,
                font.thickness,
                font.font_type as i32,
                -1,
                -1,
                0,
                -1,
            )
            .ensure_not_minus1()?;
            font.handle = handle;
        }
        Ok(font)
    }
}
