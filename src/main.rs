fn crc32(mut rdr: impl std::io::Read) -> std::io::Result<u32> {
    // https://eklitzke.org/efficient-file-copying-on-linux
    const BUF_LEN: usize = 1 << 17;

    let mut hasher = crc32fast::Hasher::new();
    let mut buf = [0; BUF_LEN];
    loop {
        let len = rdr.read(&mut buf)?;
        if len == 0 {
            break;
        }
        hasher.update(&buf[..len]);
    }

    Ok(hasher.finalize())
}

fn process_stream(path: &str, rdr: impl std::io::Read) -> bool {
    if let Ok(crc) = crc32(rdr) {
        println!("{:08x}\t{}", crc, path);
        true
    } else {
        eprintln!("Error: I/O error in '{}'", path);
        false
    }
}

fn process_file(path: &str) -> bool {
    if let Ok(rdr) = std::fs::File::open(path) {
        process_stream(path, rdr)
    } else {
        eprintln!("Error: cannot open '{}'", path);
        false
    }
}

fn main() {
    let mut ok = true;

    let args = std::env::args();
    if args.len() == 1 {
        ok &= process_stream("-", std::io::stdin().lock());
    } else {
        for path in args.skip(1) {
            ok &= process_file(&path);
        }
    }

    std::process::exit(if ok { 0 } else { 1 });
}
