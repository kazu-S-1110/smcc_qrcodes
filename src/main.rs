use qrcode_generator::QrCodeEcc;



fn main() {
    qrcode_generator::to_png_to_file("Hello world!", QrCodeEcc::Low, 1024, "./output.png").unwrap();
}
