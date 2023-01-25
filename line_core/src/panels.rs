use crate::comments::parse_chapter_number;
use anyhow::{anyhow, bail, Context, Result};
use core::time;
use image::{GenericImage, ImageBuffer, RgbImage};
use indicatif::ParallelProgressIterator;
use project_core::BlockingResponseFactory;
use rand::prelude::*;
use rayon::prelude::*;
use reqwest::StatusCode;
use scraper::{Html, Selector};
use std::{collections::VecDeque, fs, path::Path, thread};

/// # Errors
///
/// # Panics
pub fn get(url: &str, path: &str, start: u16, end: u16) -> Result<()> {
    let path = Path::new(path);

    // 8 Threads is around the line at which problems start to occur when pinging out too many times at once as all getting blocked
    rayon::ThreadPoolBuilder::new()
        .num_threads(6)
        .build_global()
        .context("Failed to build thread pool")?;

    let range: Vec<_> = (start..=end).collect();

    let total = range.len() as u64;

    range
        .into_par_iter()
        .progress_count(total)
        .try_for_each(|chapter| {
            if get_chapter_panels(url, path, chapter).is_err() {
                // TODO: Log
                bail!("Failed to parse Chapter {chapter}")
            }
            Ok(())
        })?;

    Ok(())
}

fn get_chapter_panels(url: &str, path: &Path, chapter: u16) -> Result<()> {
    let url = url_builder(url, chapter);

    let response = BlockingResponseFactory::get(&url)?;

    if response.status() != StatusCode::OK {
        return Ok(());
    }

    let body = response
        .text()
        .context("Failed to get text body from response")?;

    let html = Html::parse_document(&body);

    let links = get_image_links(&html)?;

    let chapter_number = parse_chapter_number(&html)?;

    let downloaded_images = download_links_async(&links, &url, chapter_number)?;

    let image = stitch_images(&downloaded_images)?;

    write_images(&image, path, chapter_number)?;

    Ok(())
}

fn get_image_links(html: &Html) -> Result<VecDeque<WebtoonHtmlImageData>> {
    let link_selector = Selector::parse(r#"img._images"#).unwrap();
    let mut links: VecDeque<WebtoonHtmlImageData> = VecDeque::new();

    for link in html.select(&link_selector) {
        let url = link.value().attr("data-url").unwrap().to_string();

        let width = link
            .value()
            .attr("width")
            .ok_or_else(|| anyhow!("Failed to locate width value in element"))?
            .to_string()
            .parse::<f64>()
            .context("Failed to parse image width as f64")? as u32;

        let height = link
            .value()
            .attr("height")
            .ok_or_else(|| anyhow!("Failed to locate height value in element"))?
            .to_string()
            .parse::<f64>()
            .context("Failed to parse image height as f64")? as u32;

        let extension = parse_extension(&url)?;

        links.push_back(WebtoonHtmlImageData {
            url,
            height,
            width,
            extension,
        });
    }

    Ok(links)
}

fn download_links_async<'a>(
    webtoon_image_data: &'a VecDeque<WebtoonHtmlImageData>,
    url: &'a str,
    chapter_number: u16,
) -> Result<VecDeque<IntermediateImageInfo<'a>>> {
    let mut rng = thread_rng();
    // 1-5 seconds
    let random_wait = rng.gen_range(1..5);

    // So all the requests aren't sent at the same time
    thread::sleep(time::Duration::from_secs(random_wait));

    // TODO: Move to builder
    let client = reqwest::blocking::Client::new();

    let mut images: VecDeque<IntermediateImageInfo> = VecDeque::new();

    for image in webtoon_image_data {
        let mut retries = 5;
        let mut wait = 1;
        let bytes = loop {
            match client
                .get(&image.url)
                .header("referer", "https://www.webtoons.com/")
                .send()
            {
                Err(_) => {
                    if retries > 0 {
                        retries -= 1;
                        thread::sleep(time::Duration::from_secs(wait));
                        wait *= 2;
                    } else {
                        bail!(
                            "Cannot connect. Check URL: {}\nChapter: {chapter_number}",
                            image.url
                        );
                    }
                }
                Ok(ok) => break ok,
            }
        }
        .bytes()?
        .to_vec();

        let height = image.height;
        let width = image.width;
        let extension = &image.extension;

        images.push_back(IntermediateImageInfo {
            bytes,
            height,
            width,
            extension,
            url,
        });
    }

    Ok(images)
}

fn url_builder(base_url: &str, chapter: u16) -> String {
    const BASE_URL: &str = r"https://www.webtoons.com/*/*/*/*/viewer?";

    const EP_NO: &str = "&episode_no=";

    // The 'title_no=<NUM>' portion
    let title = base_url.split('?').collect::<Vec<&str>>()[1];

    let fully_formed = format!("{BASE_URL}{title}{EP_NO}{chapter}");

    fully_formed
}

fn parse_extension(url: &str) -> Result<String> {
    let path = Path::new(url);

    if let Some(ext) = path.extension() {
        let result = ext
            .to_owned()
            .into_string()
            .expect("Failed to cast OsString to String")
            .split('?')
            .collect::<Vec<_>>()[0]
            .to_string();

        return Ok(result);
    }

    bail!("Failed to parse image file extension")
}

fn write_images(image: &BufferImage, path: &Path, chapter_number: u16) -> Result<()> {
    if !path
        .try_exists()
        .context("Failed to check if chapter folder exists")?
    {
        fs::create_dir(path).context("failed to create chapter folder")?;
    }

    let name = path.join(chapter_number.to_string()).with_extension("png");

    image
        .buffer
        .save_with_format(name, image::ImageFormat::Png)
        .context("Failed top write out final, large PNG")?;

    Ok(())
}

fn stitch_images(images: &VecDeque<IntermediateImageInfo<'_>>) -> Result<BufferImage> {
    let min_width = images.get_min_width();
    let first_width = images.get_first_width();
    let max_height = images.calculate_max_height();

    let mut offset: u32 = 0;

    let mut buffer: RgbImage = ImageBuffer::new(first_width, max_height);

    for image in images {
        // Range of 50 pixels from the smallest width.
        if image.width > min_width + 50 {
            continue;
        }

        let ext = match image.extension {
            "jpg" => image::ImageFormat::Jpeg,
            "png" => image::ImageFormat::Png,
            "gif" => image::ImageFormat::Gif,
            "webp" => image::ImageFormat::WebP,
            _ => bail!("Unhandled File Type, got {}", image.extension),
        };

        let holder = image::load_from_memory_with_format(&image.bytes, ext)
            .with_context(|| format!("Failed to load image from memory. URL: `{}`", image.url))?;

        if holder.width() > first_width {
            let resized = holder.resize(
                first_width,
                max_height,
                image::imageops::FilterType::Lanczos3,
            );

            buffer
                .copy_from(&resized.to_rgb8(), 0, offset)
                .with_context(|| format!("Failed to build image from: '{}'", image.url))?;

            offset += resized.height();
        } else {
            buffer
                .copy_from(&holder.to_rgb8(), 0, offset)
                .with_context(|| format!("Failed to build image from: '{}'", image.url))?;
            offset += image.height;
        }
    }

    Ok(BufferImage { buffer })
}

#[derive(Debug)]
struct IntermediateImageInfo<'a> {
    bytes: Vec<u8>,
    height: u32,
    width: u32,
    extension: &'a str,
    url: &'a str,
}

trait WebtoonImage {
    fn calculate_max_height(&self) -> u32;

    fn get_min_width(&self) -> u32;

    fn get_first_width(&self) -> u32;
}

impl<'a> WebtoonImage for VecDeque<IntermediateImageInfo<'a>> {
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

struct BufferImage {
    buffer: RgbImage,
}

struct WebtoonHtmlImageData {
    url: String,
    height: u32,
    width: u32,
    extension: String,
}
