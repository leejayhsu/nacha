use log::{debug, info};
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
                debug!("file header found");
                file.file_header = types::FileHeader {
                    ..Default::default()
                };
                file.file_header.parse(line);
            }
            "5" => {
                debug!("batch header found");
                let mut batch: types::Batch = Default::default();
                batch.batch_header = types::BatchHeader {
                    ..Default::default()
                };
                batch.batch_header.parse(line);
                file.batches.push(batch);
            }
            "6" => {
                debug!("detail entry found");
                file.last_batch().new_entry().parse(line);
            }
            "7" => {
                debug!("addendum entry found");
                file.last_batch().last_entry().new_addenda().parse(line);
            }
            "9" => {
                debug!("file control found");
                file.file_control = types::FileControl {
                    ..Default::default()
                };
                file.file_control.parse(line);
                break;
            }
            _ => debug!("unknown record found"),
        }
    }

    info!("Done parsing file");
    info!("{:#?}", file);
}
