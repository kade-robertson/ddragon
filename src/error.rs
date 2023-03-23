#![cfg_attr(docsrs, doc(cfg(any(feature = "sync", feature = "async"))))]

use thiserror::Error;

#[derive(Error, Debug)]
/// Any potential error the client may run into during operation.
pub enum ClientError {
    #[error("Could not parse URL.")]
    /// Indicates the operation failed because parsing a URL via the `url` crate
    /// failed.
    UrlParseError(#[from] url::ParseError),
    #[cfg(feature = "sync")]
    #[error("Could not complete request.")]
    /// Indicates a request failed, for the same reasons any `ureq` request may
    /// fail.
    Request(#[from] Box<ureq::Error>),
    #[cfg(feature = "async-base")]
    #[error("Could not complete request.")]
    /// Indicates a request failed, for the same reasons any `reqwest` request
    /// may fail.
    AsyncRequest(#[from] reqwest::Error),
    #[cfg(feature = "async-base")]
    #[error("Could not complete request.")]
    /// Indicates a request failed, for the same reasons any `reqwest-middlware`
    /// request may fail.
    AsyncMiddlewareRequest(#[from] reqwest_middleware::Error),
    #[error("Could not parse JSON data.")]
    /// Indicates a failed attempt at parsing JSON data.
    Parse(#[from] std::io::Error),
    #[cfg(feature = "image")]
    #[error("Could not parse image data.")]
    /// Indicates that attempting to convert bytes to a [image::DynamicImage]
    /// failed.
    Image(#[from] image::ImageError),
    #[error("Could not find the latest API version.")]
    /// Indicates during instantiation that the version lists provided by the
    /// ddragon API was empty.
    NoLatestVersion,
    #[error("Specific champion data could not be parsed.")]
    /// Indicates data for the requested champion couldn't be found in the
    /// parsed document.
    NoChampionData,
}
