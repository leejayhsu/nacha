#[derive(Debug, Default)]
pub struct NachaFile {
    pub file_header: FileHeader,
    pub batches: Vec<Batch>,
    pub file_control: FileControl,
}

#[derive(Debug, Default)]
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

#[derive(Debug, Default)]
pub struct Batch {
    pub batch_header: BatchHeader,
    pub detail_entries: Vec<DetailEntry>,
    // batch_control: BatchControl,
}

#[derive(Debug, Default)]
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
        self.record_type_code = line[0..1].to_string();
        self.service_class_code = line[1..4].to_string();
        self.company_name = line[4..20].to_string();
        self.company_discretionary_data = line[20..40].to_string();
        self.company_id = line[40..50].to_string();
        self.standard_entry_class_code = line[50..53].to_string();
        self.company_entry_description = line[53..63].to_string();
        self.company_descriptive_date = line[63..69].to_string();
        self.effective_entry_date = line[69..75].to_string();
        self.settlement_date = line[75..78].to_string();
        self.originator_status_code = line[78..79].to_string();
        self.originating_dfi_id = line[79..87].to_string();
        self.batch_number = line[87..94].to_string();
    }
}

#[derive(Debug, Default)]
pub struct DetailEntry {
    pub x: String,
}

#[derive(Debug, Default)]
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
        self.record_type_code = line[0..1].to_string();
        self.batch_count = line[1..7].to_string();
        self.block_count = line[7..13].to_string();
        self.entry_and_addenda_count = line[13..21].to_string();
        self.entry_hash = line[21..31].to_string();
        self.total_debit = line[31..43].to_string();
        self.total_credit = line[43..55].to_string();
        self.reserved = line[55..94].to_string();
    }
}
