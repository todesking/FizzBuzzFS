use libc::ENOENT;
use std::time::{Duration, UNIX_EPOCH};

use fuser::{FileAttr, FileType, MountOption};

const TTL: Duration = Duration::from_secs(1);

const BLOCK_SIZE: u32 = 512;
const FILE_SIZE: u64 = u64::MAX / 2;
const FILE_BLOCKS: u64 = FILE_SIZE / BLOCK_SIZE as u64
    + if FILE_SIZE % BLOCK_SIZE as u64 == 0 {
        0
    } else {
        1
    };

const ROOT_DIR_ATTR: FileAttr = FileAttr {
    ino: 1,
    size: 0,
    blocks: 0,
    atime: UNIX_EPOCH,
    mtime: UNIX_EPOCH,
    ctime: UNIX_EPOCH,
    crtime: UNIX_EPOCH,
    kind: FileType::Directory,
    perm: 0o555,
    nlink: 2,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
    blksize: BLOCK_SIZE,
};

const FIZZBUZZ_TXT_ATTR: FileAttr = FileAttr {
    ino: 2,
    size: FILE_SIZE,
    blocks: FILE_BLOCKS,
    atime: UNIX_EPOCH,
    mtime: UNIX_EPOCH,
    ctime: UNIX_EPOCH,
    crtime: UNIX_EPOCH,
    kind: FileType::RegularFile,
    perm: 0o644,
    nlink: 1,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
    blksize: BLOCK_SIZE,
};

struct FizzBuzzFS;

impl fuser::Filesystem for FizzBuzzFS {
    fn lookup(
        &mut self,
        _req: &fuser::Request<'_>,
        parent: u64,
        name: &std::ffi::OsStr,
        reply: fuser::ReplyEntry,
    ) {
        if parent == 1 && name.to_str() == Some("FizzBuzz.txt") {
            reply.entry(&TTL, &FIZZBUZZ_TXT_ATTR, 0);
        } else {
            reply.error(ENOENT);
        }
    }
    fn getattr(&mut self, _req: &fuser::Request<'_>, ino: u64, reply: fuser::ReplyAttr) {
        match ino {
            1 => reply.attr(&TTL, &ROOT_DIR_ATTR),
            2 => reply.attr(&TTL, &FIZZBUZZ_TXT_ATTR),
            _ => reply.error(ENOENT),
        }
    }
    fn readdir(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        _fh: u64,
        offset: i64,
        mut reply: fuser::ReplyDirectory,
    ) {
        if ino != 1 {
            reply.error(ENOENT);
            return;
        }
        let entries = vec![
            (1, FileType::Directory, "."),
            (1, FileType::Directory, ".."),
            (2, FileType::RegularFile, "FizzBuzz.txt"),
        ];
        for (i, (ino, ft, name)) in entries.into_iter().enumerate().skip(offset as usize) {
            if reply.add(ino, (i + 1) as i64, ft, name) {
                break;
            }
        }
        reply.ok();
    }
}

fn main() -> Result<(), std::io::Error> {
    let mount_point = std::env::args().nth(1).unwrap();
    let mount_options = [
        MountOption::RO,
        MountOption::FSName("FizzBuzzFS".into()),
        MountOption::AllowOther,
        MountOption::AutoUnmount,
    ];
    return fuser::mount2(FizzBuzzFS, mount_point, &mount_options);
}
