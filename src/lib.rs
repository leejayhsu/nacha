use log::{debug, info};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]

pub struct NachaFile {
    pub file_header: FileHeader,
    pub batches: Vec<Batch>,
    pub file_control: FileControl,
}

impl NachaFile {
    pub fn new(content: String) -> NachaFile {
        let mut file = NachaFile {
            ..Default::default()
        };

        for linestr in content.lines() {
            let line = linestr.to_string();
            let record_type = &line[0..1];
            match record_type {
                "1" => {
                    debug!("file header found");
                    file.file_header = FileHeader {
                        ..Default::default()
                    };
                    file.file_header.parse(line);
                }
                "5" => {
                    debug!("batch header found");
                    let mut batch: Batch = Default::default();
                    batch.batch_header = BatchHeader {
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
                    file.file_control = FileControl {
                        ..Default::default()
                    };
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

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FileHeader {
    pub record_type_code: String,
    pub priority_code: String,
    pub immediate_destination: String,
    pub immediate_origin: String,
    pub file_creation_date: String,
    pub file_creation_time: String,
    pub file_id_modifier: String,
    pub record_size: String,
    pub blocking_factor: String,
    pub format_code: String,
    pub immediate_destination_name: String,
    pub immediate_origin_name: String,
    pub reference_code: String,
}

impl FileHeader {
    pub fn parse(&mut self, line: String) {
        self.record_type_code = line[0..1].trim().to_string();
        self.priority_code = line[1..3].trim().to_string();
        self.immediate_destination = line[3..13].trim().to_string();
        self.immediate_origin = line[13..23].trim().to_string();
        self.file_creation_date = line[23..29].trim().to_string();
        self.file_creation_time = line[29..33].trim().to_string();
        self.file_id_modifier = line[33..34].trim().to_string();
        self.record_size = line[34..37].trim().to_string();
        self.blocking_factor = line[37..39].trim().to_string();
        self.format_code = line[39..40].trim().to_string();
        self.immediate_destination_name = line[40..63].trim().to_string();
        self.immediate_origin_name = line[63..86].trim().to_string();
        self.reference_code = line[86..94].trim().to_string();
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Batch {
    pub batch_header: BatchHeader,
    pub detail_entries: Vec<DetailEntry>,
    // batch_control: BatchControl,
}

impl Batch {
    pub fn new_entry(&mut self) -> &mut DetailEntry {
        let detail: DetailEntry = Default::default();
        self.detail_entries.push(detail);
        return self.detail_entries.last_mut().unwrap();
    }

    pub fn last_entry(&mut self) -> &mut DetailEntry {
        return self.detail_entries.last_mut().unwrap();
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BatchHeader {
    pub record_type_code: String,
    pub service_class_code: String,
    pub company_name: String,
    pub company_discretionary_data: String,
    pub company_id: String,
    pub standard_entry_class_code: String,
    pub company_entry_description: String,
    pub company_descriptive_date: String,
    pub effective_entry_date: String,
    pub settlement_date: String,
    pub originator_status_code: String,
    pub originating_dfi_id: String,
    pub batch_number: String,
}

impl BatchHeader {
    pub fn parse(&mut self, line: String) {
        self.record_type_code = line[0..1].trim().to_string();
        self.service_class_code = line[1..4].trim().to_string();
        self.company_name = line[4..20].trim().to_string();
        self.company_discretionary_data = line[20..40].trim().to_string();
        self.company_id = line[40..50].trim().to_string();
        self.standard_entry_class_code = line[50..53].trim().to_string();
        self.company_entry_description = line[53..63].trim().to_string();
        self.company_descriptive_date = line[63..69].trim().to_string();
        self.effective_entry_date = line[69..75].trim().to_string();
        self.settlement_date = line[75..78].trim().to_string();
        self.originator_status_code = line[78..79].trim().to_string();
        self.originating_dfi_id = line[79..87].trim().to_string();
        self.batch_number = line[87..94].trim().to_string();
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DetailEntry {
    pub record_type_code: String,
    pub transaction_code: String,
    pub receiving_dfi_id: String,
    pub check_digit: String,
    pub dfi_account_number: String,
    pub amount: String,
    pub individual_id_number: String,
    pub individual_name: String,
    pub discretionary_data: String,
    pub addenda_record_indicator: String,
    pub trace_number: String,
    pub addenda: Vec<Addendum>,
}

impl DetailEntry {
    pub fn parse(&mut self, line: String) {
        self.record_type_code = line[0..1].trim().to_string();
        self.transaction_code = line[1..3].trim().to_string();
        self.receiving_dfi_id = line[3..11].trim().to_string();
        self.check_digit = line[11..12].trim().to_string();
        self.dfi_account_number = line[12..29].trim().to_string();
        self.amount = line[29..39].trim().to_string();
        self.individual_id_number = line[39..54].trim().to_string();
        self.individual_name = line[54..76].trim().to_string();
        self.discretionary_data = line[76..78].trim().to_string();
        self.addenda_record_indicator = line[78..79].trim().to_string();
        self.trace_number = line[79..94].trim().to_string();
    }

    pub fn new_addenda(&mut self) -> &mut Addendum {
        let addendum: Addendum = Default::default();
        self.addenda.push(addendum);
        return self.addenda.last_mut().unwrap();
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Addendum {
    record_type_code: String,
    addenda_type_code: String,
    payment_related_info: String,
    addenda_sequence_number: String,
    entry_detail_sequence_number: String,
}

impl Addendum {
    pub fn parse(&mut self, line: String) {
        self.record_type_code = line[0..1].trim().to_string();
        self.addenda_type_code = line[1..3].trim().to_string();
        self.payment_related_info = line[3..83].trim().to_string();
        self.addenda_sequence_number = line[83..87].trim().to_string();
        self.entry_detail_sequence_number = line[87..94].trim().to_string();
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FileControl {
    pub record_type_code: String,
    pub batch_count: String,
    pub block_count: String,
    pub entry_and_addenda_count: String,
    pub entry_hash: String,
    pub total_debit: String,
    pub total_credit: String,
    pub reserved: String,
}

impl FileControl {
    pub fn parse(&mut self, line: String) {
        self.record_type_code = line[0..1].trim().to_string();
        self.batch_count = line[1..7].trim().to_string();
        self.block_count = line[7..13].trim().to_string();
        self.entry_and_addenda_count = line[13..21].trim().to_string();
        self.entry_hash = line[21..31].trim().to_string();
        self.total_debit = line[31..43].trim().to_string();
        self.total_credit = line[43..55].trim().to_string();
        self.reserved = line[55..94].trim().to_string();
    }
}
