use std::io;

use anyhow::Context;
use image::io::Reader;
use image::GenericImageView;
use parry2d_f64::bounding_volume::AABB;
use serde::{Deserialize, Serialize};

use crate::rnotev0_4::strokes::bitmapimage::BitmapImageFormat;
use crate::rnotev0_5::base64;
use crate::rnotev0_5::shapes::Rectangle;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename = "bitmapimage")]
pub struct BitmapImage {
    /// The bounds field of the image should not be used to determine the stroke bounds. Use rectangle.bounds() instead.
    #[serde(rename = "image")]
    pub image: Image,
    #[serde(rename = "rectangle")]
    pub rectangle: Rectangle,
}

impl Default for BitmapImage {
    fn default() -> Self {
        Self {
            image: Image::default(),
            rectangle: Rectangle::default(),
        }
    }
}

impl BitmapImage {
    /// The default offset in surface coords when importing a bitmap image
    pub const IMPORT_OFFSET_DEFAULT: na::Vector2<f64> = na::vector![32.0, 32.0];
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ImageMemoryFormat {
    R8g8b8a8Premultiplied,
    B8g8r8a8Premultiplied,
}

impl Default for ImageMemoryFormat {
    fn default() -> Self {
        Self::R8g8b8a8Premultiplied
    }
}

/// A pixel image
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename = "image")]
pub struct Image {
    /// The image data. is (de) serialized in base64 encoding
    #[serde(rename = "data", with = "base64")]
    pub data: Vec<u8>,
    /// the target rect in the coordinate space of the doc
    #[serde(rename = "rectangle")]
    pub rect: Rectangle,
    /// width of the data
    #[serde(rename = "pixel_width")]
    pub pixel_width: u32,
    /// height of the data
    #[serde(rename = "pixel_height")]
    pub pixel_height: u32,
    /// the memory format
    #[serde(rename = "memory_format")]
    pub memory_format: ImageMemoryFormat,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            data: Default::default(),
            rect: Rectangle::default(),
            pixel_width: Default::default(),
            pixel_height: Default::default(),
            memory_format: Default::default(),
        }
    }
}

impl From<image::DynamicImage> for Image {
    fn from(dynamic_image: image::DynamicImage) -> Self {
        let pixel_width = dynamic_image.width();
        let pixel_height = dynamic_image.height();
        let memory_format = ImageMemoryFormat::R8g8b8a8Premultiplied;
        let data = dynamic_image.into_rgba8().to_vec();

        let bounds = AABB::new(
            na::point![0.0, 0.0],
            na::point![f64::from(pixel_width), f64::from(pixel_height)],
        );

        Self {
            data,
            rect: Rectangle::from_p2d_aabb(bounds),
            pixel_width,
            pixel_height,
            memory_format,
        }
    }
}

impl Image {
    pub fn assert_valid(&self) -> anyhow::Result<()> {
        if self.pixel_width == 0
            || self.pixel_width == 0
            || self.data.len() as u32 != 4 * self.pixel_width * self.pixel_height
        {
            Err(anyhow::anyhow!(
                "assert_image() failed, invalid size or data"
            ))
        } else {
            Ok(())
        }
    }

    pub fn try_from_encoded_bytes(
        bytes: &[u8],
        format: image::ImageFormat,
    ) -> Result<Self, anyhow::Error> {
        let mut reader = Reader::new(io::Cursor::new(bytes));
        reader.set_format(format);

        Ok(Image::from(reader.decode()?))
    }

    pub fn convert_to_rgba8pre(&mut self) -> anyhow::Result<()> {
        self.assert_valid()?;

        match self.memory_format {
            ImageMemoryFormat::R8g8b8a8Premultiplied => {
                // Already in the correct format
                return Ok(());
            }
            ImageMemoryFormat::B8g8r8a8Premultiplied => {
                let imgbuf_bgra8 = image::ImageBuffer::<image::Bgra<u8>, Vec<u8>>::from_vec(
                    self.pixel_width,
                    self.pixel_height,
                    self.data.clone(),
                )
                .ok_or(anyhow::anyhow!(
                    "RgbaImage::from_vec() failed in Image to_imgbuf() for image with Format {:?}",
                    self.memory_format
                ))?;

                let dynamic_image = image::DynamicImage::ImageBgra8(imgbuf_bgra8).into_rgba8();

                *self = Self {
                    pixel_width: self.pixel_width,
                    pixel_height: self.pixel_height,
                    data: dynamic_image.into_vec(),
                    rect: self.rect.clone(),
                    memory_format: ImageMemoryFormat::R8g8b8a8Premultiplied,
                };
            }
        }

        Ok(())
    }

    pub fn to_imgbuf(self) -> Result<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, anyhow::Error> {
        self.assert_valid()?;

        match self.memory_format {
            ImageMemoryFormat::R8g8b8a8Premultiplied => {
                image::RgbaImage::from_vec(self.pixel_width, self.pixel_height, self.data).ok_or(
                    anyhow::anyhow!(
                    "RgbaImage::from_vec() failed in Image to_imgbuf() for image with Format {:?}",
                    self.memory_format
                ),
                )
            }
            ImageMemoryFormat::B8g8r8a8Premultiplied => {
                let imgbuf_bgra8 = image::ImageBuffer::<image::Bgra<u8>, Vec<u8>>::from_vec(
                    self.pixel_width,
                    self.pixel_height,
                    self.data,
                )
                .ok_or(anyhow::anyhow!(
                    "RgbaImage::from_vec() failed in Image to_imgbuf() for image with Format {:?}",
                    self.memory_format
                ))?;

                Ok(image::DynamicImage::ImageBgra8(imgbuf_bgra8).into_rgba8())
            }
        }
    }

    pub fn into_encoded_bytes(
        self,
        format: image::ImageOutputFormat,
    ) -> Result<Vec<u8>, anyhow::Error> {
        self.assert_valid()?;
        let mut bytes_buf: Vec<u8> = vec![];

        let dynamic_image = image::DynamicImage::ImageRgba8(
            self.to_imgbuf()
                .context("image.to_imgbuf() failed in image_to_bytes()")?,
        );
        dynamic_image
            .write_to(&mut bytes_buf, format)
            .context("dynamic_image.write_to() failed in image_to_bytes()")?;

        Ok(bytes_buf)
    }
}

impl TryFrom<crate::rnotev0_4::strokes::bitmapimage::BitmapImage> for BitmapImage {
    type Error = anyhow::Error;

    fn try_from(
        bi: crate::rnotev0_4::strokes::bitmapimage::BitmapImage,
    ) -> anyhow::Result<BitmapImage> {
        let bytes = ::base64::decode(bi.data_base64)?;

        let image = Image::try_from_encoded_bytes(
            &bytes,
            match bi.format {
                BitmapImageFormat::Jpeg => image::ImageFormat::Jpeg,
                BitmapImageFormat::Png => image::ImageFormat::Png,
            },
        )?;

        Ok(BitmapImage {
            image,
            rectangle: bi.rectangle.into(),
        })
    }
}
