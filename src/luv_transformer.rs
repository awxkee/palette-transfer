/*
 * // Copyright 2024 (c) the Radzivon Bartoshyk. All rights reserved.
 * //
 * // Use of this source code is governed by a BSD-style
 * // license that can be found in the LICENSE file.
 */

use colorutils_rs::{
    luv_to_rgb, luv_with_alpha_to_rgba, rgb_to_luv, rgba_to_luv_with_alpha, TransferFunction,
    SRGB_TO_XYZ_D65, XYZ_TO_SRGB_D65,
};

pub(crate) fn image_to_luv_rgb<const CHANNELS: usize>(
    src: &[u8],
    dst: &mut [f32],
    width: u32,
    height: u32,
) {
    if CHANNELS == 3 {
        rgb_to_luv(
            src,
            width * CHANNELS as u32,
            dst,
            std::mem::size_of::<f32>() as u32 * width * CHANNELS as u32,
            width,
            height,
            &SRGB_TO_XYZ_D65,
            TransferFunction::Srgb,
        );
    } else if CHANNELS == 4 {
        rgba_to_luv_with_alpha(
            src,
            width * CHANNELS as u32,
            dst,
            std::mem::size_of::<f32>() as u32 * width * CHANNELS as u32,
            width,
            height,
            &SRGB_TO_XYZ_D65,
            TransferFunction::Srgb,
        );
    }
}

pub(crate) fn luv_to_image_rgb<const CHANNELS: usize>(
    src: &[f32],
    dst: &mut [u8],
    width: u32,
    height: u32,
) {
    if CHANNELS == 3 {
        luv_to_rgb(
            src,
            std::mem::size_of::<f32>() as u32 * width * CHANNELS as u32,
            dst,
            width * CHANNELS as u32,
            width,
            height,
            &XYZ_TO_SRGB_D65,
            TransferFunction::Srgb,
        );
    } else if CHANNELS == 4 {
        luv_with_alpha_to_rgba(
            src,
            std::mem::size_of::<f32>() as u32 * width * CHANNELS as u32,
            dst,
            width * CHANNELS as u32,
            width,
            height,
            &XYZ_TO_SRGB_D65,
            TransferFunction::Srgb,
        );
    }
}
