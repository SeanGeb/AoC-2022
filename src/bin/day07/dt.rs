use std::cmp::min;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt;

/// A file entry with size only (name provided by parent DirEntry map).
#[derive(Debug)]
pub struct File {
    size: u64,
}

impl File {
    pub fn new(size: u64) -> File {
        File { size }
    }
}

/// An item in a directory.
#[derive(Debug)]
pub enum DirEntry {
    File(File),
    Dir(Dir),
}

impl From<Dir> for DirEntry {
    fn from(value: Dir) -> Self {
        DirEntry::Dir(value)
    }
}

impl From<File> for DirEntry {
    fn from(value: File) -> Self {
        DirEntry::File(value)
    }
}

/// A directory entry with name and contents, which may be files or dirs.
#[derive(Debug)]
pub struct Dir {
    contents: HashMap<String, DirEntry>,
    contents_size: Option<u64>,
}

impl Dir {
    /// Create a new empty Dir.
    pub fn new() -> Dir {
        Dir {
            contents: HashMap::new(),
            contents_size: None,
        }
    }

    /// Add a new file to the Dir. Panics if the file is already present with a
    /// different size, or if that name was already used for a directory.
    pub fn add_file(self: &mut Dir, name: String, size: u64) {
        let f = File::new(size);
        if let Some(old) = self.contents.insert(name, DirEntry::File(f)) {
            match old {
                DirEntry::Dir(_) => panic!("file name given was already a dir"),
                DirEntry::File(old_f) => assert_eq!(size, old_f.size),
            }
        }
    }

    /// Attempts to add a dir, but returns an existing dir if one exists.
    pub fn add_or_get_dir(self: &mut Dir, name: String) -> &mut Dir {
        match self.contents.entry(name) {
            Entry::Occupied(e) => match e.into_mut() {
                DirEntry::Dir(ref mut d) => d,
                _ => panic!("wanted dir, found file"),
            },
            Entry::Vacant(e) => match e.insert(Dir::new().into()) {
                DirEntry::Dir(ref mut d) => d,
                _ => unreachable!(),
            },
        }
    }

    /// Recursively calculate the size of the Dir, filling out the contents_size
    /// values, and returning the total size.
    pub fn update_contents_size(self: &mut Dir) -> u64 {
        let sum = self
            .contents
            .iter_mut()
            .map(|(_, e)| match e {
                DirEntry::File(f) => f.size,
                DirEntry::Dir(ref mut d) => d.update_contents_size(),
            })
            .sum();
        self.contents_size = Some(sum);
        sum
    }

    /// Finds the smallest dir with total size >= at_least, returning its size.
    pub fn smallest_dir_size_gte(self: &Dir, at_least: u64) -> u64 {
        let mut best_size_so_far = u64::MAX;
        let mut update_best_size = |s| {
            if s >= at_least {
                best_size_so_far = min(best_size_so_far, s);
            }
        };

        update_best_size(
            self.contents_size.expect("run update_contents_size first"),
        );
        if let Some(best_contents_size) = self
            .contents
            .iter()
            .filter_map(|(_, e)| match e {
                DirEntry::Dir(ref d) => Some(d.smallest_dir_size_gte(at_least)),
                _ => None,
            })
            .min()
        {
            update_best_size(best_contents_size);
        }
        best_size_so_far
    }

    /// Finds all dirs with total size <= at_most, returning the sum of their
    /// total sizes.
    pub fn sum_dir_size_lte(self: &Dir, at_most: u64) -> u64 {
        self.contents
            .iter()
            .filter_map(|(_, e)| match e {
                DirEntry::Dir(ref d) => Some(d.sum_dir_size_lte(at_most)),
                _ => None,
            })
            .sum::<u64>()
            + if self.contents_size.expect("call update_contents_size first")
                <= at_most
            {
                self.contents_size.unwrap()
            } else {
                0
            }
    }

    /// Print this directory in tree form.
    fn print(self: &Dir, indent: u16, f: &mut fmt::Formatter) -> fmt::Result {
        for (name, e) in self.contents.iter() {
            f.write_str("  ".repeat(indent.into()).as_str())?;
            f.write_str(
                match e {
                    DirEntry::File(f) => {
                        format!("- {} (file, size={})\n", name, f.size)
                    },
                    DirEntry::Dir(_) => format!("- {} (dir)\n", name),
                }
                .as_str(),
            )?;
            if let DirEntry::Dir(subdir) = e {
                subdir.print(indent + 1, f)?;
            }
        }

        Ok(())
    }

    /// Traverses the directory tree rooted here with the provided path.
    /// If the path is empty, this directory is returned.
    /// Otherwise, the leftmost element of the path is popped and the Dir it
    /// refers to is traversed. Adds new dirs as required or panics if the
    /// name refers to a file instead.
    pub fn cd(self: &mut Dir, path: &[String]) -> &mut Dir {
        let mut x = self;
        for seg in path {
            x = match seg.as_str() {
                "." => x,
                ".." => panic!("unable to cd to parent directory"),
                s => x.add_or_get_dir(s.to_string()),
            };
        }
        x
    }
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.print(0, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_size() {
        use aoc2022::utils::test::catch_unwind_silent;

        let mut root = Dir::new();
        root.add_file("baz".into(), 10);
        assert_eq!(root.update_contents_size(), 10);

        let dir_a = root.add_or_get_dir("a".into());
        dir_a.add_file("foo".into(), 123);
        dir_a.add_file("bar".into(), 100);
        assert_eq!(dir_a.update_contents_size(), 223);
        assert_eq!(root.update_contents_size(), 233);

        let dir_b = root.add_or_get_dir("dir_b".into());
        let dir_b_sub = dir_b.add_or_get_dir("dir_b_sub".into());
        dir_b_sub.add_file("thonk".into(), 500);
        assert_eq!(root.update_contents_size(), 733);

        let dir_b = root.cd(&["dir_b".into(), "dir_b_sub".into()]);
        assert_eq!(dir_b.update_contents_size(), 500);

        println!("{root}");

        assert!(catch_unwind_silent(move || {
            root.cd(&["baz".into()]);
        })
        .is_err());
    }
}
