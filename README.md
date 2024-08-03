# Copy palette of one image to another

Copies palette from source image to destination using different color spaces and statistics approach.

### Example

```rust
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
    & mut dst,
    destination_dimension.0,
    destination_dimension.1,
    TransferColorspace::OKLAB,
)
.unwrap();

image::save_buffer(
    "converted_oklab.jpg",
    & dst,
    destination_dimension.0,
    destination_dimension.1,
    image::ExtendedColorType::Rgb8,
)
.unwrap();
```

#### Results example

<p float="left">
    <img src="https://github.com/awxkee/palette-transfer/blob/4580425a17a3e551f22b6c53b15bb0e9343d911c/assets/dwl.jpeg?raw=true" width="250" height="170">
    <img src="https://github.com/awxkee/palette-transfer/blob/master/assets/twl.jpeg?raw=true" width="250" height="170">
    <img src="https://github.com/awxkee/palette-transfer/blob/master/assets/target_display.jpg?raw=true" width="250" height="170">
</p>