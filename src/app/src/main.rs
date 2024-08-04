/*
 * // Copyright 2024 (c) the Radzivon Bartoshyk. All rights reserved.
 * //
 * // Use of this source code is governed by a BSD-style
 * // license that can be found in the LICENSE file.
 */
use image::{EncodableLayout, GenericImageView, ImageReader};
use palette_transfer::{copy_palette_rgb, TransferColorspace};

fn main() {
    let source = ImageReader::open("./assets/dwl.jpeg")
        .unwrap()
        .decode()
        .unwrap();
    let source_dimensions = source.dimensions();
    let destination = ImageReader::open("./assets/twl.jpeg")
        .unwrap()
        .decode()
        .unwrap();
    let destination = destination.to_rgb8();
    let destination_dimension = destination.dimensions();
    let src = source.as_bytes();
    let target = destination.as_bytes();
    let mut dst = Vec::from(target);
    copy_palette_rgb(
        src,
        source_dimensions.0,
        source_dimensions.1,
        &mut dst,
        destination_dimension.0,
        destination_dimension.1,
        0.7,
        TransferColorspace::LAB,
    )
    .unwrap();

    image::save_buffer(
        "converted_lalphabeta.jpg",
        &dst,
        destination_dimension.0,
        destination_dimension.1,
        image::ExtendedColorType::Rgb8,
    )
    .unwrap();
}
