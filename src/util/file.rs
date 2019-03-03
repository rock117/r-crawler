use std::fs;
use std::path::*;
use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::io::Write;
//
// 获取目录下的文件，不递归
pub fn get_files<P: AsRef<Path>>(path: P)  -> io::Result<Vec<PathBuf>> {
    let dir = fs::read_dir(path)?;
    let mut files = Vec::new();
    for entry in dir {
        let e = entry?;
        let p = e.path();
        files.push(p);
    }
    Result::Ok(files)
}

//
//递归获取目录下所有文件
pub fn get_files_recurvly<P: AsRef<Path>>(path:P) -> io::Result<Vec<PathBuf>>{
    let files = get_files(path)?;
    let mut all_files:Vec<PathBuf> = Vec::new();
    for f in files.iter() {
        if f.is_dir(){
            all_files.extend(get_files_recurvly(f)?);
        }
    }
    all_files.extend(files);
    Result::Ok(all_files)
}

pub fn read_lines<P: AsRef<Path>> (path:P) -> io::Result<Vec<String>>{
    let reader = BufReader::new(std::fs::File::open(path)?);
    let mut all_lines = Vec::new();
    for line in reader.lines() {
        all_lines.push(line?);
    }
    Result::Ok(all_lines)
}

pub fn write<P: AsRef<Path>> (file_name: P, content: &str) -> std::io::Result<()>  {
    let mut file = File::create(file_name)?;
    file.write_all(content.as_bytes())
}