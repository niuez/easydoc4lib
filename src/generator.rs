use std::fs;
use std::io::{ BufWriter, Write };
use std::path::Path;
use regex::Regex;
use crate::FileLines;

pub fn generate_document_for_md(file_re: &Regex, code_path: &Path, out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if code_path.is_file() {
        if file_re.is_match(code_path.as_os_str().to_str().unwrap()) {
            let doc_name = format!("{}.{}", code_path.file_stem().unwrap().to_str().unwrap(), "md");
            let mut doc_pathbuf = out_dir.join(code_path.parent().unwrap());
            doc_pathbuf.push(doc_name);

            fs::create_dir_all(doc_pathbuf.parent().unwrap())?;

            println!("[generate] {:?} -> {:?}", code_path, doc_pathbuf);

            let code_lines = FileLines::read(code_path).expect(&format!("can't open code_path({:?})", code_path));

            let mut doc_file = BufWriter::new(fs::OpenOptions::new().create(true).write(true).open(doc_pathbuf)?);
            for com in code_lines.get_comments()? {
                doc_file.write(com.as_bytes())?;
                doc_file.write(b"\n")?;
            }
            doc_file.write(b"# Code")?;
            doc_file.write(b"\n")?;
            doc_file.write(b"")?;
            doc_file.write(b"\n")?;
            doc_file.write(b"```cpp")?;
            doc_file.write(b"\n")?;
            for codes in code_lines.get_code_lines() {
                doc_file.write(codes.as_bytes())?;
                doc_file.write(b"\n")?;
            }
            doc_file.write(b"```")?;
            doc_file.write(b"\n")?;
        }
        Ok(())
    }
    else {
        for entry in code_path.read_dir()? {
            if let Ok(entry) = entry {
                generate_document_for_md(file_re, entry.path().as_path(), out_dir)?;
            }
        }
        Ok(())
    }
}
