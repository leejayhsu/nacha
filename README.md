<br />

<div align="center">
    <h1>nacha</h1>
    <p><h3 align="center">A terminal tool for exploring and parsing NACHA files</h3></p>
</div>

<hr>

### Usage
```sh
cargo install nacha
```
To parse a nacha file, just provide the filename!
```sh
nacha my_nacha_file.ach
```
![GIF showing tui usage](https://raw.githubusercontent.com/leejayhsu/nacha/main/examples/demo.gif)
### Examples
Given a nacha file `my_nacha.ach` like below:
```
101 23138010401210428821906240000A094101Federal Reserve Bank   My Bank Name                   
5225Name on Account                     121042882 PPDREG.SALARY      190625   1121042880000001
62723138010412345678         0100000000               Receiver Account Name   0121042880000001
82250000010023138010000100000000000000000000121042882                          121042880000001
9000001000001000000010023138010000100000000000000000000                                       
9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999
9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999
9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999
9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999
9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999
```
#### Explore file in terminal
```sh
nacha my_nacha.ach
```
#### Export as json
```sh
nacha my_nacha.ach my_nacha.json
```
```json
{
  "file_header": {
    "record_type_code": "1",
    "priority_code": "01",
    "immediate_destination": "231380104",
    "immediate_origin": "0121042882",
    "file_creation_date": "2019-06-24",
    "file_creation_time": "00:00",
    "file_id_modifier": "A",
    "record_size": "094",
    "blocking_factor": "10",
    "format_code": "1",
    "immediate_destination_name": "Federal Reserve Bank",
    "immediate_origin_name": "My Bank Name",
    "reference_code": ""
  },
  "batches": [
    {
      "batch_header": {
        "record_type_code": "5",
        "service_class_code": "225",
        "company_name": "Name on Account",
        "company_discretionary_data": "",
        "company_id": "121042882",
        "standard_entry_class_code": "PPD",
        "company_entry_description": "REG.SALARY",
        "company_descriptive_date": "",
        "effective_entry_date": "2019-06-25",
        "settlement_date": null,
        "originator_status_code": "1",
        "originating_dfi_id": "12104288",
        "batch_number": "0000001"
      },
      "detail_entries": [
        {
          "record_type_code": "6",
          "transaction_code": "27",
          "receiving_dfi_id": "23138010",
          "check_digit": "4",
          "dfi_account_number": "12345678",
          "amount": 100000000,
          "individual_id_number": "",
          "individual_name": "Receiver Account Name",
          "discretionary_data": "",
          "addenda_record_indicator": "0",
          "trace_number": "121042880000001",
          "addenda": []
        }
      ],
      "batch_control": {
        "record_type_code": "8",
        "service_class_code": "225",
        "entry_addenda_count": "000001",
        "entry_hash": "0023138010",
        "total_debit": 100000000,
        "total_credit": 0,
        "company_id": "0121042882",
        "message_authentication_code": "",
        "reserved": "",
        "originating_dfi_id": "12104288",
        "batch_number": "0000001"
      }
    }
  ],
  "file_control": {
    "record_type_code": "9",
    "batch_count": 1,
    "block_count": 1,
    "entry_and_addenda_count": 1,
    "entry_hash": "0023138010",
    "total_debit": 100000000,
    "total_credit": 0,
    "reserved": ""
  }
}
```
#### Export as yaml
```sh
nacha my_nacha.ach my_nacha.yaml
```
```yaml
file_header:
  record_type_code: "1"
  priority_code: "01"
  immediate_destination: "231380104"
  immediate_origin: "0121042882"
  file_creation_date: 2019-06-24
  file_creation_time: "00:00"
  file_id_modifier: A
  record_size: "094"
  blocking_factor: "10"
  format_code: "1"
  immediate_destination_name: Federal Reserve Bank
  immediate_origin_name: My Bank Name
  reference_code: ""
batches:
  - batch_header:
      record_type_code: "5"
      service_class_code: "225"
      company_name: Name on Account
      company_discretionary_data: ""
      company_id: "121042882"
      standard_entry_class_code: PPD
      company_entry_description: REG.SALARY
      company_descriptive_date: ""
      effective_entry_date: 2019-06-25
      settlement_date: ~
      originator_status_code: "1"
      originating_dfi_id: "12104288"
      batch_number: "0000001"
    detail_entries:
      - record_type_code: "6"
        transaction_code: "27"
        receiving_dfi_id: "23138010"
        check_digit: "4"
        dfi_account_number: "12345678"
        amount: 100000000
        individual_id_number: ""
        individual_name: Receiver Account Name
        discretionary_data: ""
        addenda_record_indicator: "0"
        trace_number: "121042880000001"
        addenda: []
    batch_control:
      record_type_code: "8"
      service_class_code: "225"
      entry_addenda_count: "000001"
      entry_hash: "0023138010"
      total_debit: 100000000
      total_credit: 0
      company_id: "0121042882"
      message_authentication_code: ""
      reserved: ""
      originating_dfi_id: "12104288"
      batch_number: "0000001"
file_control:
  record_type_code: "9"
  batch_count: 1
  block_count: 1
  entry_and_addenda_count: 1
  entry_hash: "0023138010"
  total_debit: 100000000
  total_credit: 0
  reserved: ""
```
