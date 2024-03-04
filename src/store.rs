use std::{collections::HashMap, fs::File};
use std::fs;
use std::io::{Error, BufReader, BufWriter};
use std::path::Path;
use oci_spec::{image::Descriptor, runtime::Mount};
use std::time::SystemTime;
use crate::common::string_to_systemtime;
type Digest=String;
pub struct Store{
    root:String,
}

pub struct Status{
    pub dref:String,
    pub offset:i64,
    pub total:i64,
    pub expected:Digest,
    pub start_at:SystemTime,
    pub update_at:SystemTime,
}

pub struct Info{
    pub digest:Digest,
	pub size   :u64,
	pub created_at:SystemTime,
	pub updated_at :SystemTime,
	pub labels   :HashMap<String,String>,
}

impl  Default for Info {
    fn default() -> Self {
        Self { digest: Default::default(), size: Default::default(), created_at: SystemTime::now(), updated_at: SystemTime::now(), labels: Default::default() }
    }
}
impl Store {
    pub fn new() -> Store {
        Store{ 
            root:"/".to_string(),
         }
    }
    pub fn info(&self, dgst:Digest)->Result<Info,Error> {
        let p = self.blobpath(dgst.clone()).unwrap();
        let metadata  = fs::metadata(p).unwrap();
        
        Ok(Info{
            digest: dgst.clone(),
            size: metadata.len(),
            created_at: metadata.created().unwrap(),
            updated_at: metadata.accessed().unwrap(),
            labels: HashMap::default(),
        })
    } 

    pub fn blobpath(&self, dgst:Digest) -> Result<String,Error>{
        let str_path = Path::new(&self.root.clone()).join("blobs").join(dgst).to_string_lossy().to_string();
        Ok(str_path)
        
    }
    pub fn read_at(&self, dgst:Digest) -> Result<BufReader<File>,Error> {
        let p = self.blobpath(dgst).unwrap();
        let file = File::open(p).unwrap();
        let reader = BufReader::new(file);
        Ok(reader)
    }
    pub fn delete(&self,dgst:Digest){
        let p = self.blobpath(dgst).unwrap();
        let _ = fs::remove_dir_all(p);

    }
    pub fn update(&self,dgst:Digest){
        let mut info = self.info(dgst).unwrap();
        info.updated_at = SystemTime::now();
        //todo ! modify file created time

        todo!()

    }
    // try walk all file
    pub fn walk(&self,filters:Vec<String>){

        let root = Path::new(&self.root.clone()).join("blobs").to_string_lossy().to_string();
        //hash validate
        todo!()
        
    }

    fn resumestatus(){
        todo!()
    }

    fn total(&self,ingestPath:String)->i64{
        let t = fs::read_to_string(Path::new(&ingestPath).join("total")).unwrap();
        let r = t.parse::<i64>().unwrap();
        r
    }

    pub fn writer(&self,mref:String, total:i64,expected:Digest)-> Result<BufWriter<File>,Error>{
        //writer should hold One should lock it
        if expected != ""{
            let p = self.blobpath(expected).unwrap();

        }
        let (path,refp,data) = self.ingest_paths(mref);
        // ensure that the ingest path has been created. todo()!
        let file = File::open(data).unwrap();
        let writer = BufWriter::new(file);

        Ok(writer)

    }

    fn ingest_paths(&self,mref:String)-> (String,String,String){
        let fp = self.ingest_root(mref);
        let rp = Path::new(&fp.clone()).join("ref").to_string_lossy().to_string();
        let dp = Path::new(&fp.clone()).join("data").to_string_lossy().to_string();
        return (fp,rp,dp)
    }

    pub fn status(&self,mref:String)->Option<Status>{
        let ingest_path = self.ingest_root(mref);
    
        let dp  = fs::metadata(Path::new(&ingest_path).join("data")).unwrap();
        let refdata = fs::read_to_string(Path::new(&ingest_path).join("ref")).unwrap();

        let startat_string =  fs::read_to_string(Path::new(&ingest_path).join("startedat")).unwrap();
        let updateat_string = fs::read_to_string(Path::new(&ingest_path).join("updatedat")).unwrap();
        
        let startat = string_to_systemtime(&startat_string).unwrap();
        let updateat = string_to_systemtime(&updateat_string).unwrap();


        Some(Status { dref: refdata, offset: dp.len() as i64, total: self.total(ingest_path), expected: String::default(), start_at: startat, update_at: updateat })
    }
    fn ingest_root(&self, mref:String)->String{
        Path::new(&self.root.clone()).join("ingest").join(mref).to_string_lossy().to_string()
    }

}