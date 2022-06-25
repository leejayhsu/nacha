#![allow(unused)]
use chrono::{DateTime, NaiveDate, NaiveTime, ParseError, Utc};
use log::{debug, info};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct NachaFile {
    pub file_header: FileHeader,
    pub batches: Vec<Batch>,
    pub file_control: FileControl,
}
// todo: make all datetimes Option<> because they aren't guaranteed to exist in nacha file
impl NachaFile {
    pub fn new(content: String) -> NachaFile {
        let mut file = NachaFile {
            file_header: FileHeader::new(),
            batches: Vec::new(),
            file_control: FileControl::new(),
        };

        for linestr in content.lines() {
            let line = linestr.to_string();
            let record_type = &line[0..1];
            match record_type {
                "1" => {
                    debug!("file header found");
                    file.file_header.parse(line);
                }
                "5" => {
                    debug!("batch header found");
                    let batch = Batch {
                        batch_header: BatchHeader::parse(line),
                        detail_entries: Vec::new(),
                    };
                    file.batches.push(batch);
                }
                "6" => {
                    debug!("detail entry found");
                    file.last_batch().new_entry(line);
                }
                "7" => {
                    debug!("addendum entry found");
                    file.last_batch().last_entry().add_addenda(line);
                }
                "9" => {
                    debug!("file control found");
                    file.file_control.parse(line);
                    break;
                }
                _ => debug!("unknown record found"),
            }
        }
        info!("Done parsing file");
        return file;
    }

    pub fn last_batch(&mut self) -> &mut Batch {
        return self.batches.last_mut().unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileHeader {
    pub record_type_code: String,
    pub priority_code: String,
    pub immediate_destination: String,
    pub immediate_origin: String,
    pub file_creation_date: Option<NaiveDate>,
    pub file_creation_time: Option<NaiveTime>,
    pub file_id_modifier: String,
    pub record_size: String,
    pub blocking_factor: String,
    pub format_code: String,
    pub immediate_destination_name: String,
    pub immediate_origin_name: String,
    pub reference_code: String,
}

impl FileHeader {
    pub fn new() -> FileHeader {
        FileHeader {
            record_type_code: "".to_string(),
            priority_code: "".to_string(),
            immediate_destination: "".to_string(),
            immediate_origin: "".to_string(),
            file_creation_date: None,
            file_creation_time: None,
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
        let maybe_date = NaiveDate::parse_from_str(line[23..29].trim(), "%y%m%d");
        let date = match maybe_date {
            Ok(d) => Some(d),
            Err(e) => None,
        };
        let maybe_time = NaiveTime::parse_from_str(line[29..33].trim(), "%H%M");
        let time = match maybe_time {
            Ok(t) => Some(t),
            Err(e) => None,
        };

        self.record_type_code = line[0..1].trim().to_string();
        self.priority_code = line[1..3].trim().to_string();
        self.immediate_destination = line[3..13].trim().to_string();
        self.immediate_origin = line[13..23].trim().to_string();
        self.file_creation_date = date;
        self.file_creation_time = time;
        self.file_id_modifier = line[33..34].trim().to_string();
        self.record_size = line[34..37].trim().to_string();
        self.blocking_factor = line[37..39].trim().to_string();
        self.format_code = line[39..40].trim().to_string();
        self.immediate_destination_name = line[40..63].trim().to_string();
        self.immediate_origin_name = line[63..86].trim().to_string();
        self.reference_code = line[86..94].trim().to_string();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Batch {
    pub batch_header: BatchHeader,
    pub detail_entries: Vec<DetailEntry>,
    // batch_control: BatchControl,
}

impl Batch {
    pub fn new_entry(&mut self, line: String) {
        let detail = DetailEntry::parse(line);
        self.detail_entries.push(detail);
    }

    pub fn last_entry(&mut self) -> &mut DetailEntry {
        return self.detail_entries.last_mut().unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchHeader {
    pub record_type_code: String,
    pub service_class_code: String,
    pub company_name: String,
    pub company_discretionary_data: String,
    pub company_id: String,
    pub standard_entry_class_code: String,
    pub company_entry_description: String,
    pub company_descriptive_date: String,
    pub effective_entry_date: Option<NaiveDate>,
    pub settlement_date: Option<NaiveDate>,
    pub originator_status_code: String,
    pub originating_dfi_id: String,
    pub batch_number: String,
}

impl BatchHeader {
    pub fn parse(line: String) -> BatchHeader {
        let maybe_effective_date = NaiveDate::parse_from_str(line[69..75].trim(), "%y%m%d");
        let edate = match maybe_effective_date {
            Ok(d) => Some(d),
            Err(e) => None,
        };
        let maybe_settlement_date = NaiveDate::parse_from_str(line[75..78].trim(), "%y%m%d");
        let sdate = match maybe_settlement_date {
            Ok(d) => Some(d),
            Err(e) => None,
        };

        let bh = BatchHeader {
            record_type_code: line[0..1].trim().to_string(),
            service_class_code: line[1..4].trim().to_string(),
            company_name: line[4..20].trim().to_string(),
            company_discretionary_data: line[20..40].trim().to_string(),
            company_id: line[40..50].trim().to_string(),
            standard_entry_class_code: line[50..53].trim().to_string(),
            company_entry_description: line[53..63].trim().to_string(),
            company_descriptive_date: line[63..69].trim().to_string(),
            effective_entry_date: edate,
            settlement_date: sdate,
            originator_status_code: line[78..79].trim().to_string(),
            originating_dfi_id: line[79..87].trim().to_string(),
            batch_number: line[87..94].trim().to_string(),
        };
        return bh;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetailEntry {
    pub record_type_code: String,
    pub transaction_code: String,
    pub receiving_dfi_id: String,
    pub check_digit: String,
    pub dfi_account_number: String,
    pub amount: i32,
    pub individual_id_number: String,
    pub individual_name: String,
    pub discretionary_data: String,
    pub addenda_record_indicator: String,
    pub trace_number: String,
    pub addenda: Vec<Addendum>,
}

impl DetailEntry {
    pub fn parse(line: String) -> DetailEntry {
        let entry = DetailEntry {
            record_type_code: line[0..1].trim().to_string(),
            transaction_code: line[1..3].trim().to_string(),
            receiving_dfi_id: line[3..11].trim().to_string(),
            check_digit: line[11..12].trim().to_string(),
            dfi_account_number: line[12..29].trim().to_string(),
            amount: line[29..39].trim().parse().unwrap(),
            individual_id_number: line[39..54].trim().to_string(),
            individual_name: line[54..76].trim().to_string(),
            discretionary_data: line[76..78].trim().to_string(),
            addenda_record_indicator: line[78..79].trim().to_string(),
            trace_number: line[79..94].trim().to_string(),
            addenda: Vec::new(),
        };
        return entry;
    }

    pub fn add_addenda(&mut self, line: String) {
        let new_addendum = Addendum::parse(line);
        self.addenda.push(new_addendum);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Addendum {
    record_type_code: String,
    addenda_type_code: String,
    payment_related_info: String,
    addenda_sequence_number: String,
    entry_detail_sequence_number: String,
}

impl Addendum {
    pub fn parse(line: String) -> Addendum {
        let a = Addendum {
            record_type_code: line[0..1].trim().to_string(),
            addenda_type_code: line[1..3].trim().to_string(),
            payment_related_info: line[3..83].trim().to_string(),
            addenda_sequence_number: line[83..87].trim().to_string(),
            entry_detail_sequence_number: line[87..94].trim().to_string(),
        };
        return a;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileControl {
    pub record_type_code: String,
    pub batch_count: i32,
    pub block_count: i32,
    pub entry_and_addenda_count: i32,
    pub entry_hash: String,
    pub total_debit: i32,
    pub total_credit: i32,
    pub reserved: String,
}

impl FileControl {
    pub fn new() -> FileControl {
        FileControl {
            record_type_code: "".to_string(),
            batch_count: 0,
            block_count: 0,
            entry_and_addenda_count: 0,
            entry_hash: "".to_string(),
            total_debit: 0,
            total_credit: 0,
            reserved: "".to_string(),
        }
    }
    pub fn parse(&mut self, line: String) {
        self.record_type_code = line[0..1].trim().to_string();
        self.batch_count = line[1..7].trim().parse().unwrap();
        self.block_count = line[7..13].trim().parse().unwrap();
        self.entry_and_addenda_count = line[13..21].trim().parse().unwrap();
        self.entry_hash = line[21..31].trim().to_string();
        self.total_debit = line[31..43].trim().parse().unwrap();
        self.total_credit = line[43..55].trim().parse().unwrap();
        self.reserved = line[55..94].trim().to_string();
    }
}
