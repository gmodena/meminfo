use procfs::process::Process;

fn main() {
    let this_pid: u32;
    let this_proc: Process;

    this_pid = std::process::id();
    this_proc = procfs::process::Process::myself().unwrap();
    // SmapsRollup contains aggregate memory mapped information.
    let proc_info = this_proc.smaps_rollup().unwrap();
    println!("{:?} @ {:?}", this_pid, this_proc);
    println!("{:?}", proc_info);
    println!("min {:?}, max {:?}", proc_info.memory_map.address.0, proc_info.memory_map.address.1);

    // A vector of (MemoryMap, MemoryMapData) objects, representing currently mapped
    // regions and verbose info about them.
    // https://docs.rs/procfs/latest/procfs/process/struct.MemoryMap.html
    let this_proc_mmaps = this_proc.smaps().unwrap();
    for (mmap, mmap_data) in this_proc_mmaps {
        println!("min {:?}, max {:?}", mmap.address.0, mmap.address.1);
        println!("{:#?}", mmap_data)
    };
}
