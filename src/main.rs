#[macro_use]
extern crate bmp;
use bmp::{open, Pixel, Image};


//fn neighbor_pixels(x: u32, y: u32) -> Vec<(u32, u32)> {
//    vec![(x-1, y-1), (x-1, y), (x-1, y+1),
//        (x, y-1), (x, y+1),
//        (x+1, y-1), (x+1, y), (x+1, y+1)]
//}
//
//fn interpolate_pixel(x: u32, y: u32, peers_to_split: u32, peer_number: u32, img: Image) -> Pixel {
//    let neighbors = neighbor_pixels(x, y);
////
//    let replicatedNeighbor = neighbors.iter().any(|&(n_x, n_y)| replicate_pixel(n_x, n_y, peers_to_split, peer_number));
//
//    if replicatedNeighbor {
//        let neighbor_to_interpolate = neighbors.retain(|&(n_x, n_y)| replicate_pixel(n_x, n_y, peers_to_split, peer_number));
//
//        let interpolated_r = neighbor_to_interpolate.map(|&(n_x, n_y)| img.get_pixel(n_x, n_y).r).mean();
//        let interpolated_g = neighbor_to_interpolate.map(|&(n_x, n_y)| img.get_pixel(n_x, n_y).g).mean();
//        let interpolated_b = neighbor_to_interpolate.map(|&(n_x, n_y)| img.get_pixel(n_x, n_y).b).mean();
//
//        px!(interpolated_r, interpolated_g, interpolated_b)
//    }
//    px!(0, 0, 0)
//}


#[allow(dead_code)]
struct HoloPixel {
    filled: bool,
    pixel: Option<Pixel>,
    x: u32,
    y: u32
}

#[allow(dead_code)]
fn replicate_pixel(x: u32, y: u32, peers_to_split: u32, peer_number: u32) -> bool {
    x%peers_to_split == peer_number || y%peers_to_split == peer_number
}

#[allow(dead_code)]
impl HoloPixel {
    fn new(x: u32, y: u32, px: Option<Pixel>) -> Self {
        HoloPixel { pixel: px, x: x, y: y, filled: px.is_some() }
    }

    fn interpolate(self) -> HoloPixel {
        HoloPixel { pixel: Some(px!(0, 0, 0)), .. self }
    }
}

#[allow(dead_code)]
struct HoloImage {
    image: Image,
    pixels: Vec<HoloPixel>
}

impl HoloImage {
    fn load_image(image: Image, peers_to_split: u32, peer_number: u32) -> HoloImage {
    let mut pixels = Vec::new();
    for (x, y) in image.coordinates() {
        let px = image.get_pixel(x, y);
        let mut hpx = HoloPixel::new(x, y, None);
        if replicate_pixel(x, y, peers_to_split, peer_number) {
            hpx = HoloPixel::new(x, y, Some(px));
        }
        pixels.push(hpx)
    }
        HoloImage { image: image, pixels: pixels }
    }

    fn modify_image(self) -> Image {
        let mut image = self.image;
        for h_pixel in self.pixels {
            match h_pixel.pixel {
                Some(p) => image.set_pixel(h_pixel.x, h_pixel.y, p),
                None => image.set_pixel(h_pixel.x, h_pixel.y, px!(0, 0, 0)),
            }
        }
        image
    }
}

fn main() {
    let file = "drake.bmp";
    let img = open(file).unwrap_or_else(|e| {
        panic!("Failed to open: {}", e);
    });
    let peers_to_split =25;
    let peer_number = 1;

    let hpeg = HoloImage::load_image(img, peers_to_split, peer_number);
    println!("Width -> {}", hpeg.pixels.capacity());
    let updated_image = hpeg.modify_image();


    let _ = updated_image.save("output.bmp");
}


