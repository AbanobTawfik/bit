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

    file.read_to_end(&mut buf);

    Ok(buf)
}

fn load_dictionaries<'a, 'b>(path: &'a str) -> Result<(CDict<'b>, DDict<'b>), Box<dyn Error>> {
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
pub struct Loader<'a> {
    base_path: String,
    cdict: CDict<'a>,
    ddict: DDict<'a>,
    cctx: CCtx<'a>,
    dctx: DCtx<'a>,
}

impl<'a> Loader<'a> {
    pub fn from(base_path: &str) -> Result<Loader, Box<dyn Error>> {
        let dict_path = construct_path(&vec![base_path, CONFIG_DICTIONARY]);

        let (cdict, ddict) = load_dictionaries(dict_path.as_str())?;

        let mut cctx = CCtx::create();
        let mut dctx = DCtx::create();


        cctx.ref_cdict(&cdict);
        dctx.ref_ddict(&ddict);

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

    pub fn check_data_exists(&self, key: &Key) -> bool {
        let path = construct_path(&vec![self.base_path.as_str(), DIR_OBJECTS, key.as_str()]);

        Path::new(&path).exists()
    }

    pub fn load_data(&mut self, key: &Key) -> Result<Object<u8>, Box<dyn Error>> {
        let path = construct_path(&vec![self.base_path.as_str(), DIR_OBJECTS, key.as_str()]);

        let encoded = load_file_bytes(path.as_str())?;
        let mut decoded = vec![0u8; encoded.len()];
        
        decompress_dctx(&mut self.dctx, decoded.as_mut_slice(), encoded.as_slice());
    
        Ok(rmp_serde::decode::from_slice(decoded.as_slice())?)
    }
}

#[test]
fn test_construct_path() {
    assert_eq!(construct_path(&vec!["test1"]), String::from("test1"));

    assert_eq!(construct_path(&vec!["test1", "test2"]), String::from("test1/test2"));

    assert_eq!(construct_path(&vec!["test1", "test2", "test3"]), String::from("test1/test2/test3"));
}

