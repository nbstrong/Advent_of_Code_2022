# Since rust is too hard lmao, let's do it in python

# Read in the file and create a list of the lines
with open("input.txt", "r") as f:
    lines = f.readlines()

class FileSystem :
    def __init__(self, root):
        self.root = root
        self.current_dir = root

class Folder:
    def __init__(self, name):
        self.name = name
        self.parent = None
        self.files = []
        self.folders = []
        self.size = 0

    def set_parent(self, parent):
        self.parent = parent

    def add_file(self, file):
        self.files.append(file)
        self.size += int(file.size)

    def add_folder(self, folder):
        self.folders.append(folder)
        self.size += int(folder.size)

class File:
    def __init__(self, name, size):
        self.name = name
        self.size = size

# Create the root folder
root = Folder("root")

# Create the file system
fs = FileSystem(root)

# Traverse through the command list and build the file system
for line in lines:
    # Print the line
    print(line, end="")

    # Split the line into the command and the path
    cmd = line.split(" ")

    # Strip newlines
    for i, c in enumerate(cmd):
        cmd[i] = c.strip()

    if cmd[1] == "ls":
        continue
    elif cmd[1] == "cd" and cmd [2] != "..":
        # Check if folder exists
        found = False
        for folder in fs.current_dir.folders:
            if folder.name == cmd[2]:
                fs.current_dir = folder
                found = True
                break
        if not found:
            # Create the folder
            new_folder = Folder(cmd[2])
            new_folder.set_parent(fs.current_dir)
            fs.current_dir.add_folder(new_folder)
            fs.current_dir = new_folder
    elif cmd[1] == "cd" and cmd[2] == "..":
        # Go up a directory
        fs.current_dir = fs.current_dir.parent
    elif cmd[0] == "dir":
        # Add the folder to the current dir
        new_folder = Folder(cmd[1])
        new_folder.set_parent(fs.current_dir)
        fs.current_dir.add_folder(new_folder)
    else:
        # Add the file to the current dir
        new_file = File(cmd[1], cmd[0])
        fs.current_dir.add_file(new_file)

# Traverse through the file system and updated the sizes to include the size of the subfolders
def update_sizes(folder):
    if folder.folders == []:
        return
    for subfolder in folder.folders:
        update_sizes(subfolder)
        folder.size += subfolder.size

update_sizes(root)

print("-" * 80)

# Print the file system
def print_fs(folder, indent):
    for files in folder.files:
        print((" " * indent)+ "F " + files.name + " " + str(files.size))
    if folder.folders == []:
        return
    for subfolder in folder.folders:
        print((" " * indent) + "D " + subfolder.name + " " + str(subfolder.size))
        print_fs(subfolder, indent + 1)

print_fs(root, 0)

# Return a list of all directories under a certain size
def get_dirs_under_size(folder, size):
    dirs = []
    if folder.folders == []:
        return dirs
    for subfolder in folder.folders:
        if subfolder.size <= size:
            dirs.append(subfolder)
        dirs += get_dirs_under_size(subfolder, size)
    return dirs

dirs_100k = get_dirs_under_size(root, 100000)

print("-" * 80)

# Print the directories under 100k and their sizes
for dir in dirs_100k:
    print("D " + dir.name + " " + str(dir.size))

# Sum the sizes of the directories
total_size = 0
for dir in dirs_100k:
    total_size += dir.size

print("Total size of directories under 100k: " + str(total_size))

# Part 2
max_fs_size = 70000000
free_space_needed = 30000000
target_space = max_fs_size - free_space_needed
amount_to_delete = fs.root.size - target_space

# Print the amount to delete
print(f"{fs.root.size} - {free_space_needed} = {amount_to_delete}")

# Get the directories over amount_to_delete
def get_dirs_over_size(folder, size):
    dirs = []
    if folder.folders == []:
        return dirs
    for subfolder in folder.folders:
        if subfolder.size >= size:
            dirs.append(subfolder)
        dirs += get_dirs_over_size(subfolder, size)
    return dirs

dirs_over = get_dirs_over_size(root, amount_to_delete)

# Print the directories over the limit
for dir in dirs_over:
    print("D " + dir.name + " " + str(dir.size))

# Sort the directories by size
dirs_over.sort(key=lambda x: x.size, reverse=True)

# Print the size of smallest directory
print("Smallest directory over the limit: " + str(dirs_over[-1].size))