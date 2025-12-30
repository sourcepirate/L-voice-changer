
use std::process::Command;
use crate::temp::TempAudio;

const HP_FREQ : &str = "atempo=3/4,asetrate=44100*4/3";
const LP_FREQ : &str = "atempo=3/2,asetrate=44100*2/3";

#[derive(Copy, Debug, Clone)]
pub enum AudioType {
    Lowpass,
    Highpass
}

#[derive(Copy, Debug, Clone)]
pub enum FFMPEGError {
   CreationError,
   MergeError
}

impl AudioType {
    pub fn get_ext(self) -> &'static str {
        match self {
            AudioType::Lowpass => "lp.wav",
            AudioType::Highpass => "hp.wav"
        }
    }

    pub fn choose_freq(self) -> &'static str {
        match self {
            AudioType::Lowpass => LP_FREQ,
            AudioType::Highpass => HP_FREQ
        }
    }
}

pub fn convert_to(source: &str, path: &str, pass_type: AudioType) -> Result<(), FFMPEGError> {
    let mut command :Command = Command::new("ffmpeg");
    command.arg("-i")
           .arg(source)
           .arg("-filter:a")
           .arg(pass_type.choose_freq())
           .arg(path);
    let output = command.output();
    debug!("{:?}", output);
    match output {
        Ok(_) => Ok(()),
        Err(_) => Err(FFMPEGError::CreationError)
    }
}

pub fn merge_audio(t1: TempAudio, t2: TempAudio, outpath: &str) -> Result<(), FFMPEGError> {
   let mut command : Command = Command::new("ffmpeg");
   command
       .arg("-i").arg(t1.path())
       .arg("-i").arg(t2.path())
       .arg("-filter_complex").arg("amerge")
       .arg("-ac").arg("2")
       .arg("-c:a").arg("libmp3lame")
       .arg("-q:a").arg("4")
       .arg(outpath);
    let output = command.output();
    debug!("{:?}", output);
    match output {
        Ok(_) => Ok(()),
        Err(_) => Err(FFMPEGError::MergeError)
    }
}