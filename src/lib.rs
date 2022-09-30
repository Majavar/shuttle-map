#[macro_use]
extern crate rocket;

mod fractal;

use crate::fractal::Fractal;
use ::image::{DynamicImage, ImageBuffer, ImageFormat, ImageOutputFormat};
use ::noise::{NoiseFn, Perlin, Seedable};
use ::rand;
use ::rocket::{http::ContentType, response::content};
use ::std::io::Cursor;

#[get("/")]
fn index() -> content::RawHtml<&'static str> {
    content::RawHtml(include_str!("index.html"))
}

#[get("/")]
fn image() -> (ContentType, Vec<u8>) {
    let noise = Fractal::<Perlin>::new().set_seed(rand::random());

    let image: DynamicImage = ImageBuffer::from_fn(512, 512, |x, y| {
        let value = noise.get([(x as f64) / 128.0, (y as f64) / 128.0]);
        ::image::Luma([((value + 1.0) * 128.0) as u8])
    })
    .into();
    let mut bytes = Cursor::new(Vec::new());
    image
        .write_to(&mut bytes, ImageOutputFormat::from(ImageFormat::Png))
        .expect("");

    (ContentType::PNG, bytes.into_inner())
}

#[get("/")]
fn style() -> content::RawCss<&'static str> {
    content::RawCss(include_str!("style.css"))
}

#[get("/")]
fn favicon() -> content::RawHtml<&'static str> {
    content::RawHtml("")
}

#[shuttle_service::main]
async fn rocket() -> shuttle_service::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/image", routes![image])
        .mount("/style.css", routes![style])
        .mount("/favicon.png", routes![favicon]);

    Ok(rocket)
}
