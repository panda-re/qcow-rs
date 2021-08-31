use super::*;

#[derive(Debug, Clone, Copy)]
pub struct TreeLimits {
    pub files: usize,
    pub depth: usize,
}

pub fn output_tree<R: ReadAt>(superblock: &ext4::SuperBlock<R>, path: &str, limits: TreeLimits) {
    let inode = superblock.resolve_path(path).unwrap().inode;
    let inode = superblock.load_inode(inode).unwrap();
    
    println!("{}", path);
    recurse_print_tree(&superblock, &inode, 1, &mut Vec::new(), limits);
}

fn recurse_print_tree<R: ReadAt>(
    superblock: &ext4::SuperBlock<R>,
    inode: &Inode,
    depth: usize,
    last_stack: &mut Vec<bool>,
    limits: TreeLimits,
) {
    use Enhanced::*;
    match superblock.enhance(inode) {
        Ok(Directory(dir)) => {
            if depth > limits.depth {
                if !dir.is_empty() {
                    let indents: String = last_stack
                        .iter()
                        .map(|&x| if x { "   " } else { "│  " })
                        .collect();
                    println!(" {}└── …", indents);
                }
                return;
            }

            // TODO: sort dir to top
            let mut file_count = 0;
            let mut any_filtered = false;
            let items: Vec<_> = dir
                .into_iter()
                .filter(|item| {
                    if item.name == "." || item.name == ".." {
                        false
                    } else if item.file_type == ext4::FileType::Directory {
                        true
                    } else {
                        file_count += 1;

                        let file_cap_exceeded = file_count > limits.files;
                        any_filtered |= file_cap_exceeded;

                        !file_cap_exceeded
                    }
                })
                .collect();
            let len = items.len();

            for (i, item) in items.into_iter().enumerate() {
                let is_end = i == len - 1;
                let indents: String = last_stack
                    .iter()
                    .map(|&x| if x { "   " } else { "│  " })
                    .collect();

                println!(
                    " {}{} {}",
                    indents,
                    if is_end && !any_filtered {
                        "└──"
                    } else {
                        "├──"
                    },
                    item.name,
                );

                if let DirEntry {
                    file_type: ext4::FileType::Directory,
                    inode,
                    ..
                } = item
                {
                    if let Ok(inode) = superblock.load_inode(inode) {
                        last_stack.push(is_end && !any_filtered);
                        recurse_print_tree(superblock, &inode, depth + 1, last_stack, limits);
                        last_stack.pop();
                    }
                }
            }

            if any_filtered {
                let indents: String = last_stack
                    .iter()
                    .map(|&x| if x { "   " } else { "│  " })
                    .collect();
                println!(" {}└── …", indents);
            }
        }
        _ => (),
    }
}
