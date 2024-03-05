use crate::common::{string_to_systemtime, compute_sha256};
use crate::labelstore::{Labelstore, LabelstoreInterface};
use std::fs::{self};
use std::io::{BufReader, BufWriter, Error};
use std::os::unix::prelude::OsStrExt;
use std::path::Path;
use std::time::SystemTime;
use std::{collections::HashMap, fs::File};
use walkdir::WalkDir;
use sha2::{Sha256, Digest};
pub(crate) type DigestString = String;
type StoreError = String;


pub struct Store {
    root: String,
    pub labe_interface: Box<dyn LabelstoreInterface>,
}

pub struct Status {
    pub dref: String,
    pub offset: i64,
    pub total: i64,
    pub expected: DigestString,
    pub start_at: SystemTime,
    pub update_at: SystemTime,
}

pub struct Info {
    pub digest: DigestString,
    pub size: u64,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub labels: HashMap<String, String>,
}

impl Default for Info {
    fn default() -> Self {
        Self {
            digest: Default::default(),
            size: Default::default(),
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
            labels: Default::default(),
        }
    }
}
impl Store {
    pub fn new(rt: String) -> Store {
        Store {
            root: rt.clone(),
            labe_interface: Box::new(Labelstore::new(rt)),
        }
    }
    pub fn info(&self, dgst: DigestString) -> Result<Info, StoreError> {
        let p = self.blobpath(dgst.clone());
        let metadata = fs::metadata(p).map_err(|e| e.to_string())?;
        let labels = self.labe_interface.get(dgst.clone()).unwrap_or_default();
        Ok(Info {
            digest: dgst.clone(),
            size: metadata.len(),
            created_at: metadata.created().map_err(|e| e.to_string())?,
            updated_at: metadata.accessed().map_err(|e| e.to_string())?,
            labels: labels,
        })
    }

    pub fn blobpath(&self, dgst: DigestString) -> String {
        let str_path = Path::new(&self.root.clone())
            .join("blobs")
            .join(dgst)
            .to_string_lossy()
            .to_string();
        str_path
    }
    pub fn read_at(&self, dgst: DigestString) -> Result<BufReader<File>, Error> {
        let p = self.blobpath(dgst);
        let file = File::open(p)?;
        let reader = BufReader::new(file);
        Ok(reader)
    }
    pub fn delete(&self, dgst: DigestString) -> Result<(), Error> {
        let p = self.blobpath(dgst);
        fs::remove_dir_all(p)?;
        Ok(())
    }
    pub fn update(&mut self, info: Info, filedpaths: Vec<String>) -> Option<Info> {
        let p = self.blobpath(info.digest.clone());
        if let Err(_) = fs::metadata(Path::new(&p)) {
            return None;
        }
        let mut labels: HashMap<String, String> = HashMap::new();
        let mut all = false;
        if filedpaths.len() == 0 {
            all = true;
            labels = info.labels.clone()
        } else {
            for x in filedpaths {
                if x.starts_with("labels.") {
                    let key = x.strip_prefix("labels.").unwrap().to_string();
                    labels.insert(key.clone(), info.labels.get(&key).unwrap().clone());
                    continue;
                }
                match x.as_str() {
                    "labels" => {
                        all = true;
                        labels = info.labels.clone();
                    }
                    _ => {
                        return None;
                    }
                }
            }
        }

        if all {
            let _ = self.labe_interface.set(info.digest.clone(), labels);
        } else {
            self.labe_interface.update(info.digest.clone(), labels);
        }

        let mut info = self.info(info.digest.clone()).unwrap();
        info.updated_at = SystemTime::now();
        //todo ! modify file created time
       
        Some(info)
    }
    // try walk all file

    pub fn walk(&self, func: impl Fn(Info) -> StoreError, filters: Vec<String>) {
        let root = Path::new(&self.root.clone())
            .join("blobs");
        
        WalkDir::new(root.clone()).into_iter().for_each(|entry|{
            let binding = entry.unwrap();
            let path = binding.path();
            if path == root{
                return;
            }
            if path.is_dir(){
                
            }
            let fi = fs::metadata(path).unwrap();

            let dgst = compute_sha256(path.file_name().unwrap().as_bytes());
  

            let labels = self.labe_interface.get(dgst.clone()).unwrap();
            let info  = Info { digest: dgst, size: fi.len(), created_at: fi.created().unwrap(), updated_at: fi.modified().unwrap(), labels: labels };
            //todo filter
            // if !filters.contains(info.){

            // }
            func(info);  
        }
    
        );
       

    }

   

    fn resumestatus() {
        todo!()
    }

    fn total(&self, ingest_path: String) -> i64 {
        let t =
            fs::read_to_string(Path::new(&ingest_path).join("total")).unwrap_or("-1".to_string());
        let r = t.parse::<i64>().unwrap();
        r
    }

    pub fn writer(
        &self,
        mref: String,
        total: i64,
        expected: DigestString,
    ) -> Result<BufWriter<File>, Error> {
        //writer should hold One should lock it
        if expected != "" {
            let p = self.blobpath(expected);
        }
        let (path, refp, data) = self.ingest_paths(mref);
        // ensure that the ingest path has been created. todo()!
        let file = File::open(data)?;
        let writer = BufWriter::new(file);

        Ok(writer)
    }

    fn ingest_paths(&self, mref: String) -> (String, String, String) {
        let fp = self.ingest_root(mref);
        let rp = Path::new(&fp.clone())
            .join("ref")
            .to_string_lossy()
            .to_string();
        let dp = Path::new(&fp.clone())
            .join("data")
            .to_string_lossy()
            .to_string();
        return (fp, rp, dp);
    }

    pub fn status(&self, mref: String) -> Option<Status> {
        let ingest_path = self.ingest_root(mref);

        let dp = fs::metadata(Path::new(&ingest_path).join("data")).unwrap();
        let refdata = fs::read_to_string(Path::new(&ingest_path).join("ref")).unwrap();

        let startat_string = fs::read_to_string(Path::new(&ingest_path).join("startedat")).unwrap();
        let updateat_string =
            fs::read_to_string(Path::new(&ingest_path).join("updatedat")).unwrap();

        let startat = string_to_systemtime(&startat_string).unwrap();
        let updateat = string_to_systemtime(&updateat_string).unwrap();

        Some(Status {
            dref: refdata,
            offset: dp.len() as i64,
            total: self.total(ingest_path),
            expected: String::default(),
            start_at: startat,
            update_at: updateat,
        })
    }
    fn ingest_root(&self, mref: String) -> String {
        Path::new(&self.root.clone())
            .join("ingest")
            .join(mref)
            .to_string_lossy()
            .to_string()
    }
}
