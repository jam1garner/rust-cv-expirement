use opencv::core::Mat;
use opencv::imgcodecs::imread;
use opencv::types::VectorOfPoint2d;
use opencv::objdetect::{QRCodeDetector, QRCodeDetectorTrait};

use show_image::*;
use imageproc::rect::Rect;
use imageproc::drawing::{draw_filled_rect_mut, draw_hollow_rect_mut, draw_text_mut};

const FILENAME: &str = "qr-test.png";

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prop = font_loader::system_fonts::FontPropertyBuilder::new().family("Arial").bold().build();
    let (font, _) = font_loader::system_fonts::get(&prop).unwrap();
    let font = rusttype::Font::try_from_vec(font).unwrap();

    let mut decoder = QRCodeDetector::default().unwrap();
    let mut points = VectorOfPoint2d::new();
    let mut img = imread(FILENAME, 0).unwrap();
    let mut straight_qr_code = Mat::default();
    let _ = decoder.detect_and_decode(&mut img, &mut points, &mut straight_qr_code).unwrap();

    let mut image = image::open(FILENAME).unwrap();

    let (a, b, c, d) = (
        points.get(0).unwrap(),
        points.get(1).unwrap(),
        points.get(2).unwrap(),
        points.get(3).unwrap()
    );

    let top = a.y.min(b.y).min(c.y).min(d.y) as i32;
    let bottom = a.y.max(b.y).max(c.y).max(d.y) as i32;
    let left = a.x.min(b.x).min(c.x).min(d.x) as i32;
    let right = a.x.max(b.x).max(c.x).max(d.x) as i32;

    let width = (right - left) as u32;
    let height = (bottom - top) as u32;

    for i in 0..=5 {
        draw_hollow_rect_mut(
            &mut image,
            Rect::at(left - i, top - i).of_size(width + (i * 2) as u32, height + (i * 2) as u32),
            [255, 0, 255, 255].into()
        );

        draw_hollow_rect_mut(
            &mut image,
            Rect::at(left + i, top + i).of_size(width - (i * 2) as u32, height - (i * 2) as u32),
            [255, 0, 255, 255].into()
        );
    }

    draw_filled_rect_mut(
        &mut image,
        Rect::at(left - 5, top - 110).of_size(300, 110),
        [255, 0, 255, 255].into()
    );

    draw_text_mut(
        &mut image,
        [0, 0, 0, 255].into(),
        left as u32,
        (top - 100) as u32,
        rusttype::Scale::uniform(64.0),
        &font,
        "QR Code"
    );

    let image = image.as_image_view().unwrap();

    let window = create_window(FILENAME, Default::default())?;
    window.set_image("image-001", image)?;

    for event in window.event_channel()? {
        if let event::WindowEvent::CloseRequested(_) = event {
            break
        }
    }

    Ok(())
}
