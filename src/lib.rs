use std::env;
use std::fs;
use std::io::{self};
use std::process::{Command, ExitStatus};
use surf;
use semver::{Version};

pub async fn check_version_text(now_version: &semver::Version, url: &String) -> anyhow::Result<(), String> {    
  let mut response = surf::get(url).await.unwrap();
  let latest_version = response.body_string().await.unwrap();
  // println!("latest_version is {}", &latest_version);
  let latest_version = Version::parse(&latest_version.trim()).unwrap();
  if now_version.lt(&latest_version) {
      Ok(())
  } else {
      Err("Already latest version".to_owned())
  }
}


pub async fn self_update(url: &String, binary_name: &String) -> io::Result<ExitStatus> {
  println!("get {}", url);
  let mut response = surf::get(url).await.unwrap();
  println!("get finish {}", response.status());
  let buf: Vec<u8> = response.body_bytes().await.unwrap();
  let filename = env::current_exe().unwrap().with_file_name(binary_name);
  let filename_new = env::current_exe().unwrap().with_file_name(format!("{}_new", binary_name));
  println!("write to {}", filename_new.clone().display());
  fs::write(&filename_new, &buf)?;
  
  Command::new("chmod").arg("+x").arg(&filename_new).status()?;
  let status = Command::new(&filename_new).status()?;

  if status.success() {
      println!("rewrite {} to {}", filename_new.display(), filename.display());
      fs::rename(&filename_new, &filename)?;
  } else {
      fs::remove_file(&filename_new)?;
  }

  Ok(status)
}