extern crate image;
extern crate reqwest;
extern crate tempdir;

use std::io::copy;
use std::fs::File;
use std::io;
use tempdir::TempDir;

pub fn get_image(url: &str) -> io::Result<image::DynamicImage> {
    let tmp_dir = TempDir::new("example")?;
    let target = url;
    let mut response = reqwest::get(target).expect("request failed");

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        let fname = tmp_dir.path().join(fname);
        File::create(fname)?
    };

    copy(&mut response, &mut dest)?;

    let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

    let fname = tmp_dir.path().join(fname);
    Ok(image::open(fname).expect("Unable to open image"))
}
