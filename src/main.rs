use std::vec::Vec;
use std::fs::File;
use std::io::Write;
use std::ops::Add;

struct Pixel {
    _r: u32,
    _g: u32,
    _b: u32
}

impl Pixel {
    pub fn to_string(&self) -> String {
        let mut str = String::new();
        str = str.add(self._r.to_string().as_str())
            .add(" ")
            .add(self._g.to_string().as_str())
            .add(" ")
            .add(self._b.to_string().as_str())
            .add(" ");
        return str;
    }
}

struct Image {
    width: u32,
    height: u32,
    image_name: String,
    img_header: String,
    pixels: Vec<Pixel>
}

impl Image {
    pub fn new(width: u32, height: u32, image_name: String) -> Image {
        let header = String::from("P3 ")
            + width.to_string().as_ref()
            + String::from(" ").as_ref()
            + height.to_string().as_ref()
            + String::from(" 255 ").as_ref();
        let mut pixels = Vec::new();
        pixels.reserve((height * width) as usize);
        Image {
            width,
            height,
            image_name,
            img_header: header,
            pixels
        }
    }
    pub fn create(&mut self) {
        let mut _a = 0.00f32;
        let mut _b = 0.00f32;
        let mut _d = 0.00f32;
        let mut _e = 0.00f32;

        _a = 255f32 / (self.width as f32);
        _b = 255f32 / (self.height as f32);

        for y in 0..self.height {
            for x in 0..self.width {
                let _d = _a * (x as f32);
                let _e = _b * (y as f32);

                let _r = 255 - (_e as u32);
                let _g = 255 - (_d as u32);
                let _b = ((_d as u32) * (_e as u32)) / 255;
                let pixel = Pixel { _r, _g, _b };

                self.pixels.push(pixel);
            }
        }
    }
    pub fn write(&self) {
        let mut file = File::create(&self.image_name).unwrap();
        file.write_all(self.img_header.as_bytes());
        for pixel in &self.pixels {
            file.write_all(pixel.to_string().as_bytes());
        }
        file.flush();
    }
}

fn main() {
    let mut image = Image::new(
        1024,
        1024,
        String::from("Image_1024.ppm")
    );
    image.create();
    image.write();
}
