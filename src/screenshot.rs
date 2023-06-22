use anyhow::Result;
use image::{ImageBuffer, Rgba};
use speedy2d::{image::ImageDataType, Graphics2D};
use std::fs;

const CLEAN_FOLDER: bool = true;
pub struct Screenshot {
    pub folder: String,
    counter: u32,
}

#[derive(Clone, Copy)]
pub enum Format {
    Jpeg,
    Png,
}

impl Screenshot {
    pub const fn new(folder: String) -> Self {
        Self { folder, counter: 0 }
    }

    pub fn capture(&mut self, graphics: &mut Graphics2D, format: Format) {
        if self.counter == 0 && CLEAN_FOLDER {
            let path = fs::canonicalize(&self.folder).unwrap();
            dbg!("Clearing screenshots folder: {}", path.display());
            let _ = fs::remove_dir_all(path);
        }
        if self.folder_path().is_ok() {
            let raw = graphics.capture(ImageDataType::RGBA);
            let image: ImageBuffer<Rgba<u8>, _> =
                ImageBuffer::from_raw(raw.size().x, raw.size().y, raw.data().as_slice()).unwrap();

            let extension = match format {
                Format::Jpeg => "jpeg",
                Format::Png => "png",
            };

            image
                .save(
                    std::path::Path::new(".")
                        .join(&self.folder)
                        .join(format!("{}.{}", self.counter, extension)),
                )
                .unwrap();

            self.counter += 1;
        } else {
            panic!("{:#?}", self.folder_path())
        }
    }

    fn folder_path(&self) -> Result<()> {
        if let Ok(entry) = fs::metadata(&self.folder) {
            if !entry.is_dir() {
                return Err(anyhow::format_err!(
                    "Couldn't create screenshots directory! Looks like path is occupied {}",
                    self.folder
                ));
            }
        } else {
            fs::create_dir(&self.folder)?;
        }
        Ok(())
    }
}
