/// Example given [here](https://github.com/bodil/bitmaps/issues/35).
use bitmaps::Bitmap;

fn main() {
    // This compiles and triggers UB at runtime
    let _ = Bitmap::<1>::try_from(&[2u8][..]).unwrap();
    // UB occurs during assume_init() due to invalid bool value
}
