use std::error::Error;

#[derive(Debug)]
struct File {
    _name: String,
    size: usize,
}

impl File {
    fn new(_name: String, size: usize) -> Self {
        File { _name, size }
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<File>,
    dirs: Vec<Directory>,
    parent: Option<Box<Directory>>,
}

impl Directory {
    fn new(name: String) -> Self {
        Directory {
            name,
            files: Vec::new(),
            dirs: Vec::new(),
            parent: None,
        }
    }

    fn cd(mut self, name: &str) -> Option<Self> {
        if name == ".." {
            let mut dir = *self.parent.take()?;
            dir.dirs.push(self);
            return Some(dir);
        }

        if name == "/" {
            let mut dir = self;
            while dir.parent.is_some() {
                dir = dir.cd("..")?;
            }
            return Some(dir);
        }

        let pos = self.dirs.iter().position(|dir| dir.name == name)?;
        let mut dir = self.dirs.remove(pos);
        dir.parent = Some(Box::new(self));
        Some(dir)
    }

    fn mkdir(&mut self, dir: Directory) {
        self.dirs.push(dir);
    }

    fn touch(&mut self, file: File) {
        self.files.push(file);
    }

    fn du(&self) -> usize {
        let mut size: usize = self.files.iter().map(|file| file.size).sum();
        size += self.dirs.iter().map(|dir| dir.du()).sum::<usize>();
        return size;
    }
}

fn small_du(dir: &Directory) -> usize {
    let cwd_size = dir.du();
    let subdir_size = dir.dirs.iter().map(|dir| small_du(dir)).sum::<usize>();
    if cwd_size > 100000 {
        return subdir_size;
    } else {
        return cwd_size + subdir_size;
    }
}

fn get_smallest(dir: &Directory, min: usize, last: usize) -> usize {
    let cwd_size = dir.du();
    if cwd_size < min || cwd_size > last {
        return dir
            .dirs
            .iter()
            .fold(last, |last, dir| get_smallest(dir, min, last));
    }

    return dir
        .dirs
        .iter()
        .fold(cwd_size, |last, dir| get_smallest(dir, min, last));
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cwd = Directory::new(String::from("/"));
    let input = include_str!("../inp_off.txt");

    for line in input.lines() {
        let (left, right) = line.split_once(" ").expect("uncorrect input");
        match left {
            "$" => {
                let cmd = right.split_once(" ");
                match cmd {
                    Some(("cd", dir)) => {
                        if dir.len() == 0 {
                            panic!("Invalid input");
                        }

                        cwd = cwd.cd(dir).expect(&format!("dir does not exist: {}", dir));
                    }
                    None if right == "ls" => (), // Do nothing wait for next lines
                    _ => panic!("Invalid input"),
                }
            }
            "dir" => {
                let new_dir = Directory::new(String::from(right));
                cwd.mkdir(new_dir);
            }
            _ => {
                let file = File::new(String::from(right), left.parse().expect("Invalid input"));
                cwd.touch(file);
            }
        }
    }
    cwd = cwd.cd("/").expect("Root must exist");

    println!("ex 1 {}", small_du(&cwd));

    let space_required = 30000000 - (70000000 - cwd.du());

    println!("ex 2 {}", get_smallest(&cwd, space_required, usize::MAX));

    Ok(())
}
