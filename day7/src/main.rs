use std::path::PathBuf;

fn main() {
    let mut input_path = PathBuf::new();
    input_path.push(std::env::current_dir().unwrap());
    input_path.push("src/input.txt");

    // Take in the input.txt file and create a vector where each element is a string of the line
    let input_string = match std::fs::read_to_string(input_path) {
        Ok(s) => s,
        Err(e) => {
            println!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    let file_vec: Vec<&str> = input_string.lines().collect();

    // Print the file
    for line in &file_vec {
        println!("{}", line);
    }

    pub struct FileSystem {
        root: Box<Directory>,
        current_dir: Box<Directory>,
    }

    pub struct File {
        name: String,
        size: u32,
    }

    pub struct Directory {
        name: String,
        parent: Option<Box<Directory>>,
        files: Option<Box<Vec<File>>>,
        folders: Option<Box<Vec<Directory>>>,
        size: u32,
    }
    impl FileSystem {
        pub fn new(root: Box<Directory>) -> FileSystem {
            FileSystem {
                root: root,
                current_dir: root,
            }
        }
        pub fn get_root(&self) -> &Box<Directory> {
            &self.root
        }
        pub fn get_current_dir(&self) -> &Box<Directory> {
            &self.current_dir
        }
        pub fn set_root(&mut self, root: Directory) {
            self.root = Box::new(root);
        }
        pub fn set_current_dir(&mut self, current_dir: Directory) {
            self.current_dir = Box::new(current_dir);
        }
    }
    impl File {
        pub fn new(name: &str, size: u32) -> File {
            File {
                name: name.to_string(),
                size: size,
            }
        }
    }
    impl Directory {
        pub fn new(name: &str) -> Directory {
            Directory {
                name: name.to_string(),
                parent: None,
                files: None,
                folders: None,
                size: 0,
            }
        }
        pub fn get_name(&self) -> &str {
            &self.name
        }
        pub fn get_parent(&self) -> Option<&Box<Directory>> {
            self.parent.as_ref()
        }
        pub fn get_files(&self) -> Option<&Box<Vec<File>>> {
            self.files.as_ref()
        }
        pub fn get_folders(&self) -> Option<&Box<Vec<Directory>>> {
            self.folders.as_ref()
        }
        pub fn get_size(&self) -> u32 {
            self.size
        }
        pub fn is_parent_some(&self) -> bool {
            self.parent.is_some()
        }
        pub fn is_files_some(&self) -> bool {
            self.files.is_some()
        }
        pub fn is_folders_some(&self) -> bool {
            self.folders.is_some()
        }
        pub fn does_file_exist(&self, name: &str) -> bool {
            if self.files.is_some() {
                for file in self.files.as_ref().unwrap().iter() {
                    if file.name == name {
                        return true;
                    }
                }
            }
            false
        }
        pub fn does_folder_exist(&self, name: &str) -> bool {
            if self.folders.is_some() {
                for folder in self.folders.as_ref().unwrap().iter() {
                    if folder.get_name() == name {
                        return true;
                    }
                }
            }
            false
        }
        pub fn set_parent(&mut self, parent: Directory) {
            self.parent = Some(Box::new(parent));
        }
        pub fn add_file(&mut self, file: File) {
            if self.files.is_none() {
                self.files = Some(Box::new(Vec::new()));
            }
            self.files.as_mut().unwrap().push(file);
        }
        pub fn add_folder(&mut self, folder: Directory) {
            if self.folders.is_none() {
                self.folders = Some(Box::new(Vec::new()));
            }
            self.folders.as_mut().unwrap().push(folder);
        }
        pub fn update_size(&mut self) {
            let mut size = 0;
            if self.files.is_some() {
                for file in self.files.as_mut().unwrap().iter() {
                    size += file.size;
                }
            }
            if self.folders.is_some() {
                for folder in self.folders.as_mut().unwrap().iter() {
                    size += folder.size;
                }
            }
            self.size = size;
        }
    }

    // Go through the command file and create the file system
    let mut root = Box::new(Directory::new("root"));
    let mut file_system = FileSystem::new(root);

    // Go through the command file and create the directory structure
    for line in file_vec {
        let line_vec: Vec<&str> = line.split_whitespace().collect();
        if (line_vec[1] == "cd" && line_vec[2] != "..") {
            // Check if the current directory has any folders
            if file_system.current_dir.is_folders_some() {
                // TODO
            }
        }
    }





    // // Go through the command file and create the directory structure
    // let mut root = Directory::new("root");
    // let mut current_dir = root;
    // for line in file_vec {
    //     let line_vec: Vec<&str> = line.split_whitespace().collect();
    //     if (line_vec[1] == "cd" && line_vec[2] != "..") {
    //         // Check if the folder exists
    //         let mut found = false;
    //         if current_dir.get_folders().is_some() {
    //             for folder in current_dir.get_folders().as_mut().unwrap().iter() {
    //                 if folder.get_name() == &line_vec[2] {
    //                     current_dir = *folder;
    //                     found = true;
    //                     break;
    //                 }
    //             }
    //         }
    //         if !found {
    //             let mut new_dir = Directory::new(line_vec[2]);
    //             new_dir.set_parent(current_dir);
    //             current_dir.add_folder(new_dir);
    //             current_dir = *current_dir.get_folders().as_mut().unwrap().last().unwrap();
    //         }
    //     }
    //     else if (line_vec[1] == "cd" && line_vec[2] == "..") {
    //         current_dir = *current_dir.get_parent().unwrap();
    //     }
    //     else if (line_vec[0] == "ls") {
    //         continue;
    //     }
    //     else if (line_vec[0] == "dir") {
    //         let mut new_dir = Directory::new(line_vec[1]);
    //         new_dir.set_parent(current_dir);
    //         current_dir.add_folder(new_dir);
    //     }
    //     else {
    //         let mut new_file = File {
    //             name: line_vec[1].to_string(),
    //             size: line_vec[2].parse::<u32>().unwrap(),
    //         };
    //         current_dir.add_file(new_file);
    //     }
    // }

    // // Update the size of each directory
    // let mut stack = Vec::new();
    // stack.push(root);
    // while !stack.is_empty() {
    //     let mut current_dir = stack.pop().unwrap();
    //     current_dir.update_size();
    //     if current_dir.get_folders().is_some() {
    //         for folder in current_dir.get_folders().as_mut().unwrap().iter() {
    //             stack.push(*folder);
    //         }
    //     }
    // }

    // // Print the size of each directory
    // let mut stack = Vec::new();
    // stack.push(root);
    // while !stack.is_empty() {
    //     let mut current_dir = stack.pop().unwrap();
    //     println!("{} {}", current_dir.get_name(), current_dir.get_size());
    //     if current_dir.get_folders().is_some() {
    //         for folder in current_dir.get_folders().as_mut().unwrap().iter() {
    //             stack.push(*folder);
    //         }
    //     }
    // }
}
