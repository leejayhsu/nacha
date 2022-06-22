use std::io::BufRead;

#[derive(Debug)]
struct FileHeader {
    record_type_code: String,
    priority_code: String,
    immediate_destination: String,
    immediate_origin: String,
    file_creation_date: String,
    file_creation_time: String,
    file_id_modifier: String,
    record_size: String,
    blocking_factor: String,
    format_code: String,
    immediate_destination_name: String,
    immediate_origin_name: String,
    reference_code: String,
}

impl FileHeader {
    pub fn new() -> Self {
        Self {
            record_type_code: "".to_string(),
            priority_code: "".to_string(),
            immediate_destination: "".to_string(),
            immediate_origin: "".to_string(),
            file_creation_date: "".to_string(),
            file_creation_time: "".to_string(),
            file_id_modifier: "".to_string(),
            record_size: "".to_string(),
            blocking_factor: "".to_string(),
            format_code: "".to_string(),
            immediate_destination_name: "".to_string(),
            immediate_origin_name: "".to_string(),
            reference_code: "".to_string(),
        }
    }

    pub fn parse(&mut self, line: String) {
        self.record_type_code = line[0..1].to_string();
        self.priority_code = line[1..3].to_string();
        self.immediate_destination = line[3..13].to_string();
        self.immediate_origin = line[13..23].to_string();
        self.file_creation_date = line[23..29].to_string();
        self.file_creation_time = line[29..33].to_string();
        self.file_id_modifier = line[33..34].to_string();
        self.record_size = line[34..37].to_string();
        self.blocking_factor = line[37..39].to_string();
        self.format_code = line[39..40].to_string();
        self.immediate_destination_name = line[40..63].to_string();
        self.immediate_origin_name = line[63..86].to_string();
        self.reference_code = line[86..94].to_string();
    }
}

fn main() {
    // https://users.rust-lang.org/t/parse-a-file-line-by-line/68610/3
    let file = std::fs::File::open("example.ach").unwrap();
    let reader = std::io::BufReader::new(file);
    let mut file_header = FileHeader::new();
    for line in reader.lines() {
        let line = line.unwrap();
        file_header.parse(line)
    }

    println!("{:?}", file_header);
    println!("Done parsing file");
}
