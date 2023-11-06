use sdl2::{
  mixer::{Chunk, LoaderRWops},
  rwops::RWops,
};
use std::{collections::HashMap, fs};

pub fn load_sounds(dir_path: &str) -> HashMap<String, Chunk> {
  load_sounds_with_base_dir(dir_path, dir_path)
}

fn load_sounds_with_base_dir(base_dir_path: &str, dir_path: &str) -> HashMap<String, Chunk> {
  let mut sounds = HashMap::new();

  for entry in fs::read_dir(dir_path).unwrap() {
    let entry = entry.unwrap();
    let path = entry.path();
    let path = path.to_str().unwrap();

    if entry.file_type().unwrap().is_dir() {
      sounds.extend(load_sounds_with_base_dir(base_dir_path, path));
    } else {
      let relative_path = path.strip_prefix(base_dir_path).unwrap();

      sounds.insert(
        relative_path[..relative_path.rfind('.').unwrap_or(relative_path.len())].to_owned(),
        RWops::from_file(path, "rb").unwrap().load_wav().unwrap(),
      );
    }
  }

  sounds
}
