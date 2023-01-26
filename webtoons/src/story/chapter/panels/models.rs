use image::RgbImage;

pub struct WebtoonHtmlImageData {
    pub url: String,
    pub height: u32,
    pub width: u32,
    pub extension: String,
}

#[derive(Debug)]
pub struct IntermediateImageInfo<'a> {
    pub bytes: Vec<u8>,
    pub height: u32,
    pub width: u32,
    pub extension: &'a str,
    pub url: &'a str,
}

pub trait WebtoonImage {
    fn calculate_max_height(&self) -> u32;

    fn get_min_width(&self) -> u32;

    fn get_first_width(&self) -> u32;
}

impl<'a> WebtoonImage for Vec<IntermediateImageInfo<'a>> {
    fn calculate_max_height(&self) -> u32 {
        let mut accum: u32 = 0;
        for image in self {
            accum += image.height;
        }

        accum
    }

    fn get_min_width(&self) -> u32 {
        let mut min = 0;

        for image in self {
            if min == 0 || min > image.width {
                min = image.width;
            }
        }

        min
    }

    fn get_first_width(&self) -> u32 {
        self.iter().next().expect("Should not be empty").width
    }
}

pub struct BufferImage {
    pub buffer: RgbImage,
}
