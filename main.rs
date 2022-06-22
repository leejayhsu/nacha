use std::io::BufRead;
mod types;

fn main() {
    let file = std::fs::File::open("example.ach").unwrap();
    let reader = std::io::BufReader::new(file);
    let mut file: types::NachaFile = Default::default();

    for line in reader.lines() {
        let line = line.unwrap();
        let record_type = &line[0..1];
        match record_type {
            "1" => {
                println!("file header found");
                file.file_header = types::FileHeader {
                    ..Default::default()
                };
                file.file_header.parse(line);
            }
            "5" => {
                println!("entry detail found");
                let mut batch: types::Batch = Default::default();
                batch.batch_header = types::BatchHeader {
                    ..Default::default()
                };
                batch.batch_header.parse(line);
                file.batches.push(batch);
            }
            "9" => {
                println!("file control found");
                file.file_control = types::FileControl {
                    ..Default::default()
                };
                file.file_control.parse(line);
                break;
            }
            _ => println!("unknown record found"),
        }
    }

    println!("Done parsing file");
    println!("{:?}", file);
}
