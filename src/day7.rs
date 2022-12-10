use core::borrow;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use regex::Regex;

struct File {
    name: String,
    size: usize,
}

type DirectoryC = RefCell<Directory>;

struct Directory {
    name: String,
    parent: Weak<DirectoryC>,
    files: Vec<File>,
    sub_dirs: Vec<Rc<DirectoryC>>,
}

impl Directory {
    fn root() -> Rc<DirectoryC> {
        Rc::new_cyclic(|we| {
            Directory {
                name: "/".to_string(),
                parent: we.clone(),
                files: Default::default(),
                sub_dirs: Default::default(),
            }
            .into()
        })
    }

    fn new(name: &str, parent: &Rc<DirectoryC>) -> Rc<DirectoryC> {
        Rc::new(
            Directory {
                name: name.to_string(),
                parent: Rc::<DirectoryC>::downgrade(parent),
                files: Default::default(),
                sub_dirs: Default::default(),
            }
            .into(),
        )
    }

    fn sub_dirs_rec(&self) -> Vec<Rc<DirectoryC>> {
        let mut sub_dirs: Vec<Rc<DirectoryC>> = self
            .sub_dirs
            .iter()
            .map(|d| d.borrow().sub_dirs_rec())
            .flatten()
            .collect();
        sub_dirs.append(&mut self.sub_dirs.clone());
        sub_dirs
    }

    fn size(&self) -> usize {
        let sub_size: usize = self.sub_dirs.iter().map(|d| d.borrow().size()).sum();
        let own_size: usize = self.files.iter().map(|f| f.size).sum();
        own_size + sub_size
    }
}

struct FileSystem {
    root: Rc<DirectoryC>,
    current_dir: Rc<DirectoryC>,
}

impl FileSystem {
    const CAPACITY: usize = 70_000_000;

    fn new() -> FileSystem {
        let root = Directory::root();
        FileSystem {
            root: root.clone(),
            current_dir: root,
        }
    }

    fn cd(&mut self, dir: &str) {
        match dir {
            "/" => self.current_dir = self.root.clone(),
            ".." => {
                let parent = self.current_dir.borrow().parent.upgrade().unwrap();
                self.current_dir = parent;
            }
            dir => {
                let sub_dir = self
                    .current_dir
                    .borrow()
                    .sub_dirs
                    .iter()
                    .find(|d| d.borrow().name == dir)
                    .unwrap()
                    .clone();

                self.current_dir = sub_dir;
            }
        }
    }

    fn add_dir(&mut self, name: &str) {
        let new_dir = Directory::new(name, &self.current_dir);
        self.current_dir.borrow_mut().sub_dirs.push(new_dir);
    }

    fn add_file(&mut self, name: &str, size: usize) {
        let file = File {
            name: name.to_string(),
            size,
        };
        self.current_dir.borrow_mut().files.push(file);
    }

    fn free_space(&self) -> usize {
        return FileSystem::CAPACITY - self.root.borrow().size();
    }
}

fn parse_commands(cmd_data: &str, fs: &mut FileSystem) {
    let cd_cmd = Regex::new(r"^\$ cd (..|/|\w+)$").unwrap();
    let ls_cmd = Regex::new(r"^\$ ls$").unwrap();
    let add_dir_cmd = Regex::new(r"^dir (\w+)$").unwrap();
    let add_file_cmd = Regex::new(r"^(\d+) (\w+.\w+)$").unwrap();

    for l in cmd_data.lines() {
        if let Some(cap) = cd_cmd.captures(l) {
            fs.cd(cap.get(1).unwrap().as_str());
            continue;
        } else if ls_cmd.is_match(l) {
        } else if let Some(cap) = add_dir_cmd.captures(l) {
            fs.add_dir(cap.get(1).unwrap().as_str())
        } else if let Some(cap) = add_file_cmd.captures(l) {
            let name = cap.get(2).unwrap().as_str();
            let size = cap.get(1).unwrap().as_str().parse().unwrap();

            fs.add_file(name, size);
        } else {
            panic!("Unknow console output {}", l)
        }
    }
}

pub fn main_day7() {
    println!("----- DAY 7 --------");

    let test_data = std::fs::read_to_string("./data/day7.txt").unwrap();

    let mut fs = FileSystem::new();
    parse_commands(&test_data, &mut fs);

    let all_dirs = fs.root.borrow().sub_dirs_rec();
    let sizes = all_dirs.iter().map(|d| d.borrow().size());

    let size_sum: usize = sizes.filter(|s| s <= &1_00_000).sum();
    println!("Sum of sizes <= 100000: {}", size_sum);

    let to_be_freed = 30_000_000 - fs.free_space();
    println!("To be freed {}", to_be_freed);

    let dir_to_delete = all_dirs
        .iter()
        .filter(|d| d.borrow().size() >= to_be_freed)
        .min_by_key(|d| d.borrow().size())
        .cloned()
        .unwrap();

    println!("To be deleted {}, size {}", dir_to_delete.borrow().name, dir_to_delete.borrow().size());
}
