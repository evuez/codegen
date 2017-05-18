extern crate hyper;
extern crate qrcode;
extern crate image;

use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::server::{Server, Request, Response};
use hyper::header::{
    ContentDisposition,
    ContentLength,
    ContentType,
    DispositionType,
    DispositionParam,
    Charset
};

use image::{GrayImage, ImageLuma8};
use qrcode::QrCode;

use std::io::Cursor;
use std::io::Write;

fn main() {
    Server::http("0.0.0.0:4500").unwrap().handle(generator).unwrap();
}

fn generator(request: Request, mut response: Response) {
    println!("{}", request.uri);

    let code = QrCode::new(b"01234567").unwrap();
    let image: GrayImage = code.render().to_image();
    let ref mut image_buffer = Cursor::new(Vec::new());
    let _ = ImageLuma8(image).save(image_buffer, image::PNG);
    let image_data = image_buffer.get_ref();

    response.headers_mut().set(ContentType(Mime(TopLevel::Image, SubLevel::Png, vec![])));
    response.headers_mut().set(ContentDisposition {
        disposition: DispositionType::Inline,
        parameters: vec![DispositionParam::Filename(
            Charset::Us_Ascii,
            None,
            b"code.png".to_vec()
        )]
    });
    response.headers_mut().set(ContentLength(image_data.len() as u64));

    response.start().unwrap().write_all(image_data.as_slice()).unwrap();
}

