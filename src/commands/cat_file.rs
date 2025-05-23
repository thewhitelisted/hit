// to my dismay, this file is not a photo of a cat.

pub fn cat_file(hash: &str, print: bool) {
    // look up file at .hit/objects/ab/cdef...
    let object_dir = format!(".hit/objects/{}", &hash[..2]);
    let object_file = (&hash[2..]).to_string();
    let object_path = std::path::PathBuf::from(&object_dir).join(object_file);
    // check if file exists
    if !object_path.exists() {
        eprintln!("Error: Object {} not found", hash);
        std::process::exit(1);
    }
    // read file
    let compressed_data = std::fs::read(&object_path).expect("Failed to read object file");
    // decompress file
    let mut decoder = flate2::read::ZlibDecoder::new(&compressed_data[..]);
    let mut decompressed_data = Vec::new();
    std::io::copy(&mut decoder, &mut decompressed_data).expect("Failed to decompress object");
    // parse header
    let header_end = decompressed_data
        .iter()
        .position(|&b| b == 0)
        .expect("Failed to find header end");
    let header = String::from_utf8_lossy(&decompressed_data[..header_end]);
    let header_parts: Vec<&str> = header.split_whitespace().collect();
    if header_parts.len() < 2 {
        eprintln!("Error: Invalid object format");
        std::process::exit(1);
    }
    let object_length: usize = header_parts[1]
        .parse()
        .expect("Failed to parse object length");
    if object_length != decompressed_data.len() - header_end - 1 {
        eprintln!("Error: Object length mismatch");
        std::process::exit(1);
    }
    // print object
    if print {
        // print object content
        let content = &decompressed_data[header_end + 1..];
        let content_str = String::from_utf8_lossy(content);
        println!("{}", content_str);
    } else {
        // print hash
        println!("{}", hash);
    }
}
