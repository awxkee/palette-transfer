/*
 * // Copyright 2024 (c) the Radzivon Bartoshyk. All rights reserved.
 * //
 * // Use of this source code is governed by a BSD-style
 * // license that can be found in the LICENSE file.
 */

use colorutils_rs::{oklab_to_rgb, oklab_to_rgba, rgb_to_oklab, rgba_to_oklab, TransferFunction};

pub(crate) fn image_to_oklab_rgb<const CHANNELS: usize>(
    src: &[u8],
    dst: &mut [f32],
    width: u32,
    height: u32,
) {
    if CHANNELS == 3 {
        rgb_to_oklab(
            src,
            width * CHANNELS as u32,
            dst,
            std::mem::size_of::<f32>() as u32 * width * CHANNELS as u32,
            width,
            height,
            TransferFunction::Srgb,
        );
    } else if CHANNELS == 4 {
        rgba_to_oklab(
            src,
            width * CHANNELS as u32,
            dst,
            std::mem::size_of::<f32>() as u32 * width * CHANNELS as u32,
            width,
            height,
            TransferFunction::Srgb,
        );
    }
}

pub(crate) fn oklab_to_image_rgb<const CHANNELS: usize>(
    src: &[f32],
    dst: &mut [u8],
    width: u32,
    height: u32,
) {
    if CHANNELS == 3 {
        oklab_to_rgb(
            src,
            std::mem::size_of::<f32>() as u32 * width * CHANNELS as u32,
            dst,
            width * CHANNELS as u32,
            width,
            height,
            TransferFunction::Srgb,
        );
    } else if CHANNELS == 4 {
        oklab_to_rgba(
            src,
            std::mem::size_of::<f32>() as u32 * width * CHANNELS as u32,
            dst,
            width * CHANNELS as u32,
            width,
            height,
            TransferFunction::Srgb,
        );
    }
}
