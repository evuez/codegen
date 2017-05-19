extern crate image;
extern crate iron;
extern crate qrcode;
extern crate router;

use iron::prelude::Iron;
use router::Router;


mod generators {
    use image::{PNG, GrayImage, ImageLuma8};
    use iron::prelude::{IronResult, Request, Response};
    use iron::headers::ContentType;
    use iron::status;
    use qrcode::QrCode;

    use std::io::Cursor;

    pub fn qrcode(req: &mut Request) -> IronResult<Response> {
        let ref code = req.extensions.get::<Router>().unwrap().find("code").unwrap();

        let code = QrCode::new(*code).unwrap();
        let image: GrayImage = code.render().to_image();
        let ref mut image_buffer = Cursor::new(Vec::new());
        let _ = ImageLuma8(image).save(image_buffer, PNG);
        let image_data = image_buffer.get_ref();

        Ok(Response::with((ContentType::png().0, status::Created, image_data.as_slice())))
    }
}

fn main() {
    let mut router = Router::new();
    router.get("/qrcodes/:code", generators::qrcode, "code");

    Iron::new(router).http("localhost:4500").unwrap();
}
