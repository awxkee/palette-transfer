/*
 * // Copyright 2024 (c) the Radzivon Bartoshyk. All rights reserved.
 * //
 * // Use of this source code is governed by a BSD-style
 * // license that can be found in the LICENSE file.
 */

use colorutils_rs::{LAlphaBeta, Rgb, TransferFunction};

use crate::image_configuration::ImageConfiguration;

pub(crate) fn image_to_lalphabeta_rgb<const CHANNELS: u8>(
    src: &[u8],
    dst: &mut [f32],
    _: u32,
    _: u32,
) {
    let channels: ImageConfiguration = CHANNELS.into();
    for (source_chunk, dst_chunk) in src
        .chunks_exact(channels.get_channels_count())
        .zip(dst.chunks_exact_mut(channels.get_channels_count()))
    {
        unsafe {
            let r = *source_chunk.get_unchecked(channels.get_r_channel_offset());
            let g = *source_chunk.get_unchecked(channels.get_g_channel_offset());
            let b = *source_chunk.get_unchecked(channels.get_b_channel_offset());
            let rgb = Rgb::<u8>::new(r, g, b);
            let lalphabeta = LAlphaBeta::from_rgb(rgb, TransferFunction::Srgb);
            *dst_chunk.get_unchecked_mut(0) = lalphabeta.l;
            *dst_chunk.get_unchecked_mut(1) = lalphabeta.alpha;
            *dst_chunk.get_unchecked_mut(2) = lalphabeta.beta;
            if channels.has_alpha() {
                let a = *source_chunk.get_unchecked(channels.get_a_channel_offset());
                let dst_a = a as f32 * (1. / 255.);
                *dst_chunk.get_unchecked_mut(3) = dst_a;
            }
        }
    }
}

pub(crate) fn lalpha_beta_to_image_rgb<const CHANNELS: u8>(
    src: &[f32],
    dst: &mut [u8],
    _: u32,
    _: u32,
) {
    let channels: ImageConfiguration = CHANNELS.into();
    for (src_chunk, dst_chunk) in src
        .chunks_exact(channels.get_channels_count())
        .zip(dst.chunks_exact_mut(channels.get_channels_count()))
    {
        unsafe {
            let l = *src_chunk.get_unchecked(0);
            let alpha = *src_chunk.get_unchecked(1);
            let beta = *src_chunk.get_unchecked(2);
            let lalphabeta = LAlphaBeta::new(l, alpha, beta);
            let rgb = lalphabeta.to_rgb(TransferFunction::Srgb);
            *dst_chunk.get_unchecked_mut(channels.get_r_channel_offset()) = rgb.r;
            *dst_chunk.get_unchecked_mut(channels.get_g_channel_offset()) = rgb.g;
            *dst_chunk.get_unchecked_mut(channels.get_b_channel_offset()) = rgb.b;
            if channels.has_alpha() {
                let a = *src_chunk.get_unchecked(3);
                let dst_a = (a * 255.).min(255.).max(1.) as u8;
                *dst_chunk.get_unchecked_mut(channels.get_a_channel_offset()) = dst_a;
            }
        }
    }
}
