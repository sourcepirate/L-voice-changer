use std::fs::remove_file;
use std::path::PathBuf;
use crate::command::{convert_to, FFMPEGError, AudioType};

pub struct TempAudio {
   path: String,
   atype: AudioType
}

impl TempAudio {
   pub fn new(path: String, atype: AudioType) -> Self {
     TempAudio {
         path,
         atype
     }
   }

   pub fn from_source(source: &str, pass_type: AudioType) -> Self {
       let mut temp_path = PathBuf::from(source);
       let newfilename = pass_type.get_ext();
       temp_path.set_file_name(newfilename);
       let path_new = temp_path.into_os_string().into_string();
       TempAudio::new(path_new.unwrap(), pass_type)
   }

   pub fn create(&self, source: &str) -> Result<(), FFMPEGError> {
       convert_to(source, self.path.as_str() , self.atype)
   }

   pub fn path(&self) -> &str {
       self.path.as_str()
   }
}

impl Drop for TempAudio {
    fn drop(&mut self) {
        let _ = remove_file(self.path.as_str());
    }
}

