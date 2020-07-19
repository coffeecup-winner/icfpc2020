rust-bmp
========
[![Build Status](https://travis-ci.org/sondrele/rust-bmp.svg?branch=master)](https://travis-ci.org/sondrele/rust-bmp)

[Full documentation](https://docs.rs/bmp/)

Small module for reading and writing bitmap images.
See the documentation for the current status of BMP encoding and decoding support.

Usage
-----
An updated version of the library should be available on [crates.io](https://crates.io/crates/bmp).
Add the following to your `Cargo.toml` to get is a dependency.

```toml
[dependencies]
bmp = "*"
```
### Initializing
Initialize a new image with the `new` function, by specifying `width` and `height`.
```rust
extern crate bmp;
use bmp::Image;

let mut img = Image::new(100, 100);
```
### Editing
Edit image data using the `get_pixel` and `set_pixel` functions.
Save an image with the `save` function, by specifying the `path`. The function returns
an `IoResult` which indicates whether the save was successful or not.
```rust
let pixel = img.get_pixel(0, 0);
img.set_pixel(50, 50, Pixel::new(255, 255, 255));
let _ = img.save("path/to/img.bmp");
```
### Opening
Open an existing image with the `open` function, by specifying the `path`. The function
returns a `BmpResult`, that contains either a `Image` or a `BmpError`.
```rust
extern crate bmp;

let img = bmp::open("path/to/img.bmp").unwrap_or_else(|e| {
    panic!("Failed to open: {}", e);
});
```
Coordinate convention
---------------------
The BMP images are accessed in row-major order, where point (0, 0) is defined to  be in the
upper left corner of the image.
Example:

```rust
#[macro_use]
extern crate bmp;
use bmp::{Image, Pixel};

fn main() {
    let mut img = Image::new(256, 256);

    for (x, y) in img.coordinates() {
        img.set_pixel(x, y, px!(x, y, 200));
    }
    let _ = img.save("img.bmp");
}
```
