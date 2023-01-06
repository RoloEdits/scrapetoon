use scraper::{Html, Selector};

///# Panics
///
/// Will panic if there are no images.
#[must_use]
pub fn from(html: &Html) -> u32 {

    let image_selector = Selector::parse("img._images").unwrap();

    let mut pixels_height = 0.0;

    for image in html.select(&image_selector) {
        pixels_height += image
            .value()
            .attr("height")
            .unwrap()
            .parse::<f32>()
            .unwrap();
    }

    pixels_height as u32
}

#[cfg(test)]
mod parse_chapter_length {
    use scraper::Html;

    use super::*;

    #[test]
    fn should_parse_chapter_length_in_pixels() {

        const IMAGE_LIST: &str = r##"<div class="viewer_img _img_viewer_area " id="_imageList">

	<img src="https://webtoon-phinf.pstatic.net/20200328_233/1585335417410FLscc_JPEG/15853354143659521.jpg?type=q90"
		width="700" height="1100.0" alt="image" class="_images"
		data-url="https://webtoon-phinf.pstatic.net/20200328_233/1585335417410FLscc_JPEG/15853354143659521.jpg?type=q90"
		rel="nofollow">

	<img src="https://webtoon-phinf.pstatic.net/20200328_75/15853354174135Hazs_JPEG/15853354150659524.jpg?type=q90"
		width="700" height="1100.0" alt="image" class="_images"
		data-url="https://webtoon-phinf.pstatic.net/20200328_75/15853354174135Hazs_JPEG/15853354150659524.jpg?type=q90"
		rel="nofollow">

	<img src="https://webtoon-phinf.pstatic.net/20200328_78/1585335418151rxvbz_JPEG/15853354150129524.jpg?type=q90"
		width="700" height="1100.0" alt="image" class="_images"
		data-url="https://webtoon-phinf.pstatic.net/20200328_78/1585335418151rxvbz_JPEG/15853354150129524.jpg?type=q90"
		rel="nofollow">

	<img src="https://webtoon-phinf.pstatic.net/20200328_246/1585335418152sJbR8_JPEG/15853354150329524.jpg?type=q90"
		width="700" height="1100.0" alt="image" class="_images"
		data-url="https://webtoon-phinf.pstatic.net/20200328_246/1585335418152sJbR8_JPEG/15853354150329524.jpg?type=q90"
		rel="nofollow">

	<img src="https://webtoon-phinf.pstatic.net/20200328_150/1585335418150bh3iI_JPEG/15853354150839522.jpg?type=q90"
		width="700" height="1100.0" alt="image" class="_images"
		data-url="https://webtoon-phinf.pstatic.net/20200328_150/1585335418150bh3iI_JPEG/15853354150839522.jpg?type=q90"
		rel="nofollow">

	<img src="https://webtoon-phinf.pstatic.net/20200328_230/1585335418216fU0kX_JPEG/15853354181949526.jpg?type=q90"
		width="700" height="1100.0" alt="image" class="_images"
		data-url="https://webtoon-phinf.pstatic.net/20200328_230/1585335418216fU0kX_JPEG/15853354181949526.jpg?type=q90"
		rel="nofollow">

	<img src="https://webtoon-phinf.pstatic.net/20200328_247/1585335418112vGp9Y_JPEG/15853354180899528.jpg?type=q90"
		width="700" height="1100.0" alt="image" class="_images"
		data-url="https://webtoon-phinf.pstatic.net/20200328_247/1585335418112vGp9Y_JPEG/15853354180899528.jpg?type=q90"
		rel="nofollow">

	<img src="https://webtoon-phinf.pstatic.net/20200328_93/1585335418732urXDx_JPEG/15853354187079521.jpg?type=q90"
		width="700" height="1000.0" alt="image" class="_images"
		data-url="https://webtoon-phinf.pstatic.net/20200328_93/1585335418732urXDx_JPEG/15853354187079521.jpg?type=q90"
		rel="nofollow">

	<img src="https://webtoon-phinf.pstatic.net/20200328_228/1585335418807auIen_JPEG/15853354187849529.jpg?type=q90"
		width="700" height="1170.0" alt="image" class="_images"
		data-url="https://webtoon-phinf.pstatic.net/20200328_228/1585335418807auIen_JPEG/15853354187849529.jpg?type=q90"
		rel="nofollow">

	<img src="https://webtoon-phinf.pstatic.net/20200328_154/15853354189123p6pE_JPEG/15853354188889520.jpg?type=q90"
		width="700" height="1170.0" alt="image" class="_images"
		data-url="https://webtoon-phinf.pstatic.net/20200328_154/15853354189123p6pE_JPEG/15853354188889520.jpg?type=q90"
		rel="nofollow">

	<img src="https://webtoon-phinf.pstatic.net/20200328_24/1585335418930bKIS5_JPEG/15853354189049526.jpg?type=q90"
		width="700" height="1170.0" alt="image" class="_images"
		data-url="https://webtoon-phinf.pstatic.net/20200328_24/1585335418930bKIS5_JPEG/15853354189049526.jpg?type=q90"
		rel="nofollow">
</div>"##;

        let html = Html::parse_document(IMAGE_LIST);

        let result = from(&html);

        assert_eq!(result, 12_210);
    }
}
