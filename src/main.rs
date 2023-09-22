use fuser::MountOption;

struct NullFS;

impl fuser::Filesystem for NullFS {}

fn main() -> Result<(), std::io::Error> {
    let mount_point = std::env::args().nth(1).unwrap();
    let mount_options = [
        MountOption::RO,
        MountOption::FSName("FizzBuzzFS".into()),
        MountOption::AllowOther,
        MountOption::AutoUnmount,
    ];
    return fuser::mount2(NullFS, mount_point, &mount_options);
}
