use std::{collections::HashMap, error::Error, fs::{self, Permissions}, path::Path, os::{linux::fs::MetadataExt, unix::prelude::PermissionsExt}};

use crate::store::Digest;
pub trait  LabelstoreInterface {
    fn get(&self,digest:Digest)->Option<HashMap<String,String>>;
    fn set(&mut self,digest:Digest,maps:HashMap<String,String>)->Result<(),Box<dyn Error>>;
    fn update(&mut self,digest:Digest,maps:HashMap<String,String>)->Option<HashMap<String,String>>;
}


pub struct Labelstore{
    root:String,
    labels : HashMap<String,HashMap<String,String>>,
}

impl  LabelstoreInterface for Labelstore {
    fn get(&self,digest:Digest)->Option<HashMap<String,String>> {
        //todo lock
        Some(self.labels[&digest].clone())
    }

    fn set(&mut self,digest:Digest,maps:HashMap<String,String>)->Result<(),Box<dyn Error>> {
        //todo lock
        self.labels.insert(digest,maps);

        Ok(())
    }

    fn update(&mut self,digest:Digest,maps:HashMap<String,String>)->Option<HashMap<String,String>>  {
        if self.labels.get(&digest) == None{
            return None;
        }
        let mut mut_labels = self.labels.get(&digest).unwrap().clone();

        maps.iter().for_each(|(key,value)|{
            if value.is_empty(){
                mut_labels.remove(&key.clone());
            }else{
                mut_labels.insert(key.clone(), value.clone());
            }
        }
        );
        self.labels.insert(digest.clone(), mut_labels.clone());
        Some(mut_labels)
    }
}

impl  Labelstore {
    pub fn new(rt:String)->Self{
        let ingest_path = Path::new(&rt).join("ingest").to_string_lossy().to_string();
        
        let r =fs::create_dir_all(ingest_path.clone());
    

        if let Err(e) =  fs::set_permissions(ingest_path, Permissions::from_mode(0o777)){
            println!("create file error")
        }

        Labelstore { root:rt ,labels :HashMap::default()}
    }
}