// This list is based on https://github.com/Lynnesbian/new_mime_guess
const LIST: &[(&[&str], &[&str])] = &[
    (&["ecma"], &["application/ecmascript"]),
    (&["geojson"], &["application/geo+json"]),
    (&["gz"], &["application/gzip", "application/x-gzip"]),
    (&["hjson"], &["application/hjson"]),
    (&["jar"], &["application/java-archive"]),
    (&["js", "jsm"], &["application/javascript"]),
    (&["json"], &["application/json"]),
    (&["json5"], &["application/json5"]),
    (&["jsonml"], &["application/jsonml+json"]),
    (&["jsonld"], &["application/ld+json"]),
    (&["webmanifest"], &["application/manifest+json"]),
    (&["cjs"], &["application/node"]),
    (&["pdf"], &["application/pdf"]),
    (&["ps"], &["application/postscript"]),
    (&["ai"], &["application/postscript"]),
    (&["rss"], &["application/rss+xml"]),
    (&["toml"], &["application/toml", "text/toml", "text/x-toml"]),
    (&["7z"], &["application/x-7z-compressed"]),
    (&["bz"], &["application/x-bzip"]),
    (&["bz2"], &["application/x-bzip2"]),
    (&["tgz"], &["application/x-compressed"]),
    (&["iso"], &["application/x-iso9660-image"]),
    (&["php"], &["application/x-php", "application/x-httpd-php"]),
    (&["tar"], &["application/x-tar"]),
    (&["xml"], &["application/xml", "text/xml"]),
    (&["sitemap"], &["application/xml"]),
    (&["zip"], &["application/zip"]),
    (&["doc"], &["application/msword"]),
    (
        &["docm"],
        &["application/vnd.ms-word.document.macroenabled.12"],
    ),
    (
        &["docx"],
        &["application/vnd.openxmlformats-officedocument.wordprocessingml.document"],
    ),
    (&["dot"], &["application/msword"]),
    (
        &["dotm"],
        &["application/vnd.ms-word.template.macroenabled.12"],
    ),
    (
        &["dotx"],
        &["application/vnd.openxmlformats-officedocument.wordprocessingml.template"],
    ),
    (&["ppt"], &["application/vnd.ms-powerpoint"]),
    (
        &["pptm"],
        &["application/vnd.ms-powerpoint.presentation.macroenabled.12"],
    ),
    (
        &["pptx"],
        &["application/vnd.openxmlformats-officedocument.presentationml.presentation"],
    ),
    (&["pps"], &["application/vnd.ms-powerpoint"]),
    (
        &["ppsm"],
        &["application/vnd.ms-powerpoint.slideshow.macroenabled.12"],
    ),
    (
        &["ppsx"],
        &["application/vnd.openxmlformats-officedocument.presentationml.slideshow"],
    ),
    (&["pot"], &["application/vnd.ms-powerpoint"]),
    (
        &["potm"],
        &["application/vnd.ms-powerpoint.template.macroenabled.12"],
    ),
    (
        &["potx"],
        &["application/vnd.openxmlformats-officedocument.presentationml.template"],
    ),
    (&["ppa"], &["application/vnd.ms-powerpoint"]),
    (
        &["ppam"],
        &["application/vnd.ms-powerpoint.addin.macroenabled.12"],
    ),
    (&["xls"], &["application/vnd.ms-excel"]),
    (
        &["xlsb"],
        &["application/vnd.ms-excel.sheet.binary.macroenabled.12"],
    ),
    (
        &["xlsm"],
        &["application/vnd.ms-excel.sheet.macroenabled.12"],
    ),
    (
        &["xlsx"],
        &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"],
    ),
    (&["xla"], &["application/vnd.ms-excel"]),
    (
        &["xlam"],
        &["application/vnd.ms-excel.addin.macroenabled.12"],
    ),
    (&["xlc"], &["application/vnd.ms-excel"]),
    (&["xld"], &["application/vnd.ms-excel"]),
    (&["xlk"], &["application/vnd.ms-excel"]),
    (&["xll"], &["application/vnd.ms-excel"]),
    (&["xlm"], &["application/vnd.ms-excel"]),
    (&["xlt"], &["application/vnd.ms-excel"]),
    (
        &["xltm"],
        &["application/vnd.ms-excel.template.macroenabled.12"],
    ),
    (
        &["xltx"],
        &["application/vnd.openxmlformats-officedocument.spreadsheetml.template"],
    ),
    (&["xlw"], &["application/vnd.ms-excel"]),
    (&["odt"], &["application/vnd.oasis.opendocument.text"]),
    (
        &["ods"],
        &["application/vnd.oasis.opendocument.spreadsheet"],
    ),
    (
        &["odp"],
        &["application/vnd.oasis.opendocument.presentation"],
    ),
    (&["odg"], &["application/vnd.oasis.opendocument.graphics"]),
    (&["odc"], &["application/vnd.oasis.opendocument.chart"]),
    (&["odf"], &["application/vnd.oasis.opendocument.formula"]),
    (&["odi"], &["application/vnd.oasis.opendocument.image"]),
    (
        &["odm"],
        &["application/vnd.oasis.opendocument.text-master"],
    ),
    (&["odb"], &["application/vnd.oasis.opendocument.base"]),
    //
    (&["aac"], &["audio/aac", "audio/x-aac"]),
    (&["flac"], &["audio/flac", "audio/x-flac"]),
    (&["midi", "mid"], &["audio/midi", "audio/mid"]),
    (&["m4a"], &["audio/mp4", "audio/m4a"]),
    (&["mp4"], &["audio/mp4"]),
    (&["mp3"], &["audio/mpeg"]),
    (&["m2a"], &["audio/mpeg"]),
    (&["m3a"], &["audio/mpeg"]),
    (&["ogg"], &["audio/ogg", "audio/x-vorbis+ogg"]),
    (&["opus"], &["audio/ogg", "audio/x-opus+ogg"]),
    (&["wav", "wave"], &["audio/wav", "audio/w-wav"]),
    (&["weba"], &["audio/webm"]),
    (&["m3u8", "m3u"], &["audio/x-mpegurl"]),
    //
    (
        &["ttf"],
        &[
            "font/ttf",
            "application/x-font-ttf",
            "application/font-sfnt",
        ],
    ),
    (&["otf"], &["font/otf", "application/font-sfnt"]),
    (&["woff"], &["font/woff", "application/font-woff"]),
    (&["woff2"], &["font/woff2"]),
    //
    (&["apng"], &["image/apng"]),
    (&["bmp"], &["image/bmp"]),
    (&["gif"], &["image/gif"]),
    (&["jpeg", "jpg"], &["image/jpeg"]),
    (&["png"], &["image/png"]),
    (&["svg", "svgz"], &["image/svg+xml"]),
    (&["tiff", "tif"], &["image/tiff"]),
    (&["webp"], &["image/webp"]),
    (&["heic"], &["image/heic"]),
    (&["heics"], &["image/heic-sequence"]),
    (&["heif"], &["image/heif"]),
    (&["heifs"], &["image/heif-sequence"]),
    (&["ico"], &["image/vnd.microsoft.icon", "image/x-icon"]),
    //
    (&["coffee"], &["text/coffeescript"]),
    (&["css"], &["text/css"]),
    (&["csv"], &["text/csv"]),
    (&["html", "htm"], &["text/html", "application/xhtml+xml"]),
    (&["jsx"], &["text/jscript", "text/jsx"]),
    (&["less"], &["text/less"]),
    (&["md"], &["text/markdown", "text/x-markdown"]),
    (&["txt", "text"], &["text/plain"]),
    (&["ini"], &["text/plain"]),
    (&["py"], &["text/plain"]),
    (&["rtf"], &["text/rtf", "application/rtf"]),
    (&["sgml", "sgm"], &["text/sgml"]),
    (&["tsv"], &["text/tab-separated-values"]),
    (&["vcard"], &["text/vcard"]),
    (&["lua"], &["text/x-lua"]),
    (&["sql"], &["text/x-sql"]),
    (&["yaml", "yml"], &["text/yaml", "text/x-yaml"]),
    //
    (&["h261"], &["video/h261"]),
    (&["h263"], &["video/h263"]),
    (&["h264"], &["video/h264"]),
    (&["mp4"], &["video/mp4"]),
    (&["m1v"], &["video/mpeg"]),
    (&["m2v"], &["video/mpeg"]),
    (&["ogv"], &["video/ogg", "video/x-theora+ogg"]),
    (&["mov"], &["video/quicktime"]),
    (&["webm"], &["video/webm"]),
    (&["flv"], &["video/x-flv"]),
];
