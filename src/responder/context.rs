// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use url_serde::SerdeUrl;

use config::config::ConfigTrackerBanner;
use APP_CONF;

const LOGO_EXTENSION_SPLIT_SPAN: usize = 4;

lazy_static! {
    pub static ref CONFIG_CONTEXT: ConfigContext = ConfigContext {
        runtime_version: env!("CARGO_PKG_VERSION").to_string(),
        page_title: APP_CONF.branding.page_title.to_owned(),
        help_url: APP_CONF.branding.help_url.to_owned(),
        support_url: APP_CONF.branding.support_url.to_owned(),
        icon_color: APP_CONF.branding.icon_color.to_owned(),
        icon_url: APP_CONF.branding.icon_url.to_owned(),
        icon_mime: ImageMime::guess_from(APP_CONF.branding.icon_url.as_str()),
        logo_white_url: APP_CONF.branding.logo_white_url.to_owned(),
        logo_dark_url: APP_CONF.branding.logo_dark_url.to_owned(),
        custom_html: APP_CONF.branding.custom_html.to_owned(),
        payout_currency: APP_CONF.payout.currency.to_owned(),
        track_url: APP_CONF.tracker.track_url.to_owned(),
        track_parameter: APP_CONF.tracker.track_parameter.to_owned(),
        banners: ConfigContext::map_banners(&APP_CONF.tracker.banner)
    };
}

#[derive(Serialize)]
pub enum ImageMime {
    #[serde(rename = "image/png")]
    ImagePNG,

    #[serde(rename = "image/jpeg")]
    ImageJPEG,

    #[serde(rename = "image/gif")]
    ImageGIF,

    #[serde(rename = "image/svg")]
    ImageSVG,
}

impl ImageMime {
    fn guess_from(logo_url: &str) -> ImageMime {
        if logo_url.len() > LOGO_EXTENSION_SPLIT_SPAN {
            let (_, logo_url_extension) =
                logo_url.split_at(logo_url.len() - LOGO_EXTENSION_SPLIT_SPAN);

            match logo_url_extension {
                ".svg" => ImageMime::ImageSVG,
                ".jpg" => ImageMime::ImageJPEG,
                ".gif" => ImageMime::ImageGIF,
                _ => ImageMime::ImagePNG,
            }
        } else {
            ImageMime::ImagePNG
        }
    }
}

#[derive(Serialize)]
pub struct ConfigContext {
    pub runtime_version: String,
    pub page_title: String,
    pub help_url: SerdeUrl,
    pub support_url: SerdeUrl,
    pub icon_color: String,
    pub icon_url: SerdeUrl,
    pub icon_mime: ImageMime,
    pub logo_white_url: SerdeUrl,
    pub logo_dark_url: SerdeUrl,
    pub custom_html: Option<String>,
    pub payout_currency: String,
    pub track_url: String,
    pub track_parameter: String,
    pub banners: Vec<(SerdeUrl, u16, u16)>,
}

impl ConfigContext {
    fn map_banners(banners: &Vec<ConfigTrackerBanner>) -> Vec<(SerdeUrl, u16, u16)> {
        banners
            .into_iter()
            .map(|banner| {
                (
                    banner.banner_url.to_owned(),
                    banner.size_width,
                    banner.size_height,
                )
            })
            .collect::<Vec<(SerdeUrl, u16, u16)>>()
    }
}
