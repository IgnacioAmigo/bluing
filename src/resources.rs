use std::path::{Path,PathBuf};
use std::fs;
use std::io::{self,Read};
use std::ffi;

use sdl2::sys::Uint32;
use stb_image::{self, image};
use stb_image::image::LoadResult;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FileContainsNil,
    FailedToGetExePath,
}

pub struct Resources {
    root_path: PathBuf,
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

// used to load from ./assets

impl Resources {
    // TODO: investigate AsRef<>?
    pub fn from_relative_exe_path(rel_path: &Path) -> Result<Resources,Error> {
        let exe_file_name = std::env::current_exe().map_err(|_| Error::FailedToGetExePath)?;
        let exe_path = exe_file_name.parent().ok_or(Error::FailedToGetExePath)?;
        
        Ok(Resources {
            root_path: exe_path.join(rel_path)
        })       
    }

    pub fn load_cstring(&self, resource_name: &str) -> Result<ffi::CString, Error> {
        let mut file = fs::File::open(
            Resources::resource_name_to_path(&self.root_path,resource_name)
        )?;

        let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);
        file.read_to_end(&mut buffer)?;
        
        if buffer.iter().find(|x| **x == 0).is_some() {
            return Err(Error::FileContainsNil);
        }

        Ok(unsafe { ffi::CString::from_vec_unchecked(buffer) })
    }

    // todo: dumb return struct
    pub fn load_image(&self, resource_name: &Path) -> Result<(usize,usize,Vec<u8>), &str> {
        match stb_image::image::load(resource_name) {
            LoadResult::ImageU8(image_data) =>  Ok((image_data.width,image_data.height,image_data.data)),
            _ => Err("Error loading image; incorrect format?")
        }
    }

    fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
        let mut path: PathBuf = root_dir.into();
    
        for part in location.split("/") {
            path = path.join(part);
        }
    
        path
    }
    
}