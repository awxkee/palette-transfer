/*
 * // Copyright 2024 (c) the Radzivon Bartoshyk. All rights reserved.
 * //
 * // Use of this source code is governed by a BSD-style
 * // license that can be found in the LICENSE file.
 */
use crate::alphabeta_transformer::{image_to_lalphabeta_rgb, lalpha_beta_to_image_rgb};
use crate::image_configuration::ImageConfiguration;
use crate::lab_transformer::{image_to_lab_rgb, lab_to_image_rgb};
use crate::luv_transformer::{image_to_luv_rgb, luv_to_image_rgb};
use crate::oklab_transformer::{image_to_oklab_rgb, oklab_to_image_rgb};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum TransferColorspace {
    LALPHABETA,
    OKLAB,
    LAB,
    LUV,
}

impl TransferColorspace {
    pub(crate) fn get_perceptual_transformer(
        &self,
        image_configuration: ImageConfiguration,
    ) -> fn(&[u8], &mut [f32], u32, u32) {
        if image_configuration == ImageConfiguration::Rgb {
            match self {
                TransferColorspace::LALPHABETA => {
                    image_to_lalphabeta_rgb::<{ ImageConfiguration::Rgb as u8 }>
                }
                TransferColorspace::OKLAB => image_to_oklab_rgb::<3>,
                TransferColorspace::LAB => image_to_lab_rgb::<3>,
                TransferColorspace::LUV => image_to_luv_rgb::<3>,
            }
        } else {
            match self {
                TransferColorspace::LALPHABETA => {
                    image_to_lalphabeta_rgb::<{ ImageConfiguration::Rgba as u8 }>
                }
                TransferColorspace::OKLAB => image_to_oklab_rgb::<4>,
                TransferColorspace::LAB => image_to_lab_rgb::<4>,
                TransferColorspace::LUV => image_to_luv_rgb::<4>,
            }
        }
    }

    pub(crate) fn get_display_transformer(
        &self,
        image_configuration: ImageConfiguration,
    ) -> fn(&[f32], &mut [u8], u32, u32) {
        if image_configuration == ImageConfiguration::Rgb {
            match self {
                TransferColorspace::LALPHABETA => {
                    lalpha_beta_to_image_rgb::<{ ImageConfiguration::Rgb as u8 }>
                }
                TransferColorspace::OKLAB => oklab_to_image_rgb::<3>,
                TransferColorspace::LAB => lab_to_image_rgb::<3>,
                TransferColorspace::LUV => luv_to_image_rgb::<3>,
            }
        } else {
            match self {
                TransferColorspace::LALPHABETA => {
                    lalpha_beta_to_image_rgb::<{ ImageConfiguration::Rgba as u8 }>
                }
                TransferColorspace::OKLAB => oklab_to_image_rgb::<4>,
                TransferColorspace::LAB => lab_to_image_rgb::<4>,
                TransferColorspace::LUV => luv_to_image_rgb::<4>,
            }
        }
    }
}
