use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use zstd::zstd_safe::{CDict, DDict, DCtx, CCtx, CLEVEL_DEFAULT, decompress_dctx};

use rmp_serde;

use crate::objects::{Object, Key};
use crate::repository::location::{CONFIG_DICTIONARY, DIR_OBJECTS};

fn load_file_bytes(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut buf = Vec::<u8>::new();

    file.read_to_end(&mut buf)?;

    Ok(buf)
}

fn load_dictionaries(path: &str) -> Result<(CDict<'static>, DDict<'static>), Box<dyn Error>> {
    let buf = load_file_bytes(path)?;

    Ok(
        (
            CDict::create(buf.as_slice(), CLEVEL_DEFAULT), 
            DDict::create(buf.as_slice())
        )
    )
}

fn construct_path(psuedo_path: &Vec<&str>) -> String {
    psuedo_path.into_iter().fold(String::from(""), |a, x| {
        if a.as_str() == "" {
            a + x
        } else {
            a + "/" + x
        }
    })
}

// Dictionary must outlive the context
pub struct Loader {
    base_path: String,
    cdict: CDict<'static>,
    ddict: DDict<'static>,
    cctx: CCtx<'static>,
    dctx: DCtx<'static>,
}

impl Loader {
    pub fn from(base_path: &str) -> Result<Loader, Box<dyn Error>> {
        let dict_path = construct_path(&vec![base_path, CONFIG_DICTIONARY]);

        let (cdict, ddict) = load_dictionaries(dict_path.as_str())?;

        let mut cctx = CCtx::create();
        let mut dctx = DCtx::create();


        cctx.ref_cdict(&cdict).map_err(|x| format!("Compression Dictionary Error: {}", x))?;
        dctx.ref_ddict(&ddict).map_err(|x| format!("Decompression Dictionary Error: {}", x))?;

        Ok(
            Loader {
                base_path: base_path.to_string(),
                cdict,
                ddict,
                cctx,
                dctx,
            }
        )
    }

    pub fn load_data(&mut self, key: &Key) -> Result<Object<u8>, Box<dyn Error>> {
        let path = construct_path(&vec![self.base_path.as_str(), DIR_OBJECTS, key.as_str()]);

        let encoded = load_file_bytes(path.as_str())?;
        let mut decoded = vec![0u8; encoded.len()];
        
        decompress_dctx(&mut self.dctx, decoded.as_mut_slice(), encoded.as_slice()).map_err(|x| format!("Decompression error: {}", x))?;
    
        Ok(rmp_serde::decode::from_slice(decoded.as_slice())?)
    }

    pub fn check_load_data(&mut self, key: &Key) -> Option<Object<u8>> {
        let path = construct_path(&vec![self.base_path.as_str(), DIR_OBJECTS, key.as_str()]);

        if !Path::new(path.as_str()).exists() {
            return None;
        }

        Some(self.load_data(key).unwrap())
    }
}

#[test]
fn test_construct_path() {
    assert_eq!(construct_path(&vec!["test1"]), String::from("test1"));

    assert_eq!(construct_path(&vec!["test1", "test2"]), String::from("test1/test2"));

    assert_eq!(construct_path(&vec!["test1", "test2", "test3"]), String::from("test1/test2/test3"));
}

