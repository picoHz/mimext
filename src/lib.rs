#![no_std]
#![forbid(unsafe_code)]
//! Zero-dependency & no-std media type <-> file extension conversion.

/// Gets correspondent file extensions from a media type.
///
/// The media type must be a lower-case string.
///
/// ```
/// # use mimext::mime_to_ext;
/// assert_eq!(mime_to_ext("image/jpeg"), &["jpeg", "jpg"]);
/// assert_eq!(mime_to_ext("text/x-yaml"), &["yaml", "yml"]);
/// assert_eq!(mime_to_ext("application/msword"), &["doc"]);
/// ```
pub fn mime_to_ext<T>(mime: T) -> &'static [&'static str]
where
    T: AsRef<str>,
{
    let mime = mime.as_ref();
    include!(concat!(env!("OUT_DIR"), "/mime_to_ext.rs"))
}

/// Gets correspondent media types from a file extension.
///
/// The file extension must be a lower-case string.
///
/// ```
/// # use mimext::ext_to_mime;
/// assert_eq!(ext_to_mime("yml"), &["text/yaml", "text/x-yaml"]);
/// assert_eq!(ext_to_mime("geojson"), &["application/geo+json"]);
/// assert_eq!(ext_to_mime("odt"), &["application/vnd.oasis.opendocument.text"]);
/// ```
pub fn ext_to_mime<T>(ext: T) -> &'static [&'static str]
where
    T: AsRef<str>,
{
    let ext = ext.as_ref();
    include!(concat!(env!("OUT_DIR"), "/ext_to_mime.rs"))
}
