mod hash_functions;
use cursive::{
    backend::Dummy,
    traits::{Boxable, Nameable},
    views::{Dialog, DummyView, EditView, LinearLayout, ListView, SelectView, TextView},
    Cursive,
};
use hash_functions::*;
use std::{fs, io::Write, str::MatchIndices};
use std::{fs::OpenOptions, io::Read};

fn log_panic(msg: &str) -> ! {
    if !std::path::Path::new("logs.txt").exists() {
        fs::File::create("logs.txt");
    }
    OpenOptions::new()
        .append(true)
        .write(true)
        .open("logs.txt")
        .unwrap()
        .write_all(msg.as_bytes());
    panic!("{}", msg);
}
fn main() {
    let mut app = cursive::default();

    app.add_layer(
        Dialog::new()
            .title("Hash Checker")
            .content(
                ListView::new()
                    .child(
                        "File Location: ",
                        EditView::new().with_name("file").fixed_width(50),
                    )
                    .child("", DummyView)
                    .child("Hash: ", EditView::new().with_name("hash").fixed_width(128)),
            )
            .button("Check", |s: &mut Cursive| {
                let hash = match (s.call_on_name("hash", |d: &mut EditView| {
                    d.get_content().as_ref().clone().to_owned()
                })) {
                    Some(d) => d,
                    None => {
                        s.quit();
                        log_panic("ERROR getting HASH")
                    }
                };
                let file_location = s
                    .call_on_name("file", |d: &mut EditView| {
                        d.get_content().as_ref().clone().to_owned()
                    })
                    .unwrap();
                let mut file = match (fs::File::open(&file_location)) {
                    Ok(f) => f,
                    Err(e) => {
                        s.quit();
                        log_panic(format!("ERR: {:?}", e).as_str())
                    }
                };
                let mut buffer: Vec<u8> = vec![];
                match file.read_to_end(&mut buffer) {
                    Ok(d) => d,
                    Err(e) => {
                        log_panic(format!("ERR {:?}", e).as_str());
                    }
                };
                let final_hash = match get_hash_type(hash.clone()) {
                    HashType::MD5 => hash_md5(buffer.as_ref()),
                    HashType::SHA1 => hash_sha1(buffer.as_ref()),
                    HashType::SHA224 => hash_sha224(buffer.as_ref()),
                    HashType::SHA256 => hash_sha256(buffer.as_ref()),
                    HashType::SHA384 => hash_sha384(buffer.as_ref()),
                    HashType::SHA512 => hash_sha512(buffer.as_ref()),
                };
                popup_result(
                    s,
                    hash.clone(),
                    final_hash,
                    get_hash_type(hash.clone()),
                    file_location,
                );
            }),
    );
    app.add_global_callback('q', |a: &mut Cursive| {
        a.pop_layer();
    });
    app.run()
}

pub fn popup_result(
    app: &mut Cursive,
    hash: String,
    final_hash: String,
    hashtype: HashType,
    file_loc: String,
) {
    app.add_layer(
        Dialog::new().title("Result").content(
            ListView::new()
                .child("File: ", TextView::new(file_loc))
                .child("HashType: ", TextView::new(format!("{:?}", hashtype)))
                .child(
                    "valid hash: ",
                    TextView::new(format!("{}", hash == final_hash)),
                ),
        ),
    )
}
