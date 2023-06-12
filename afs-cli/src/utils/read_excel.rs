use calamine::{open_workbook, DataType, Reader, Xlsx};
use std::collections::HashSet;

pub fn read_excel(file: String) -> HashSet<String> {
    let mut tuples = HashSet::new();

    let mut workbook: Xlsx<_> = open_workbook(file).unwrap();

    let sheet_name = workbook.sheet_names().first().unwrap();

    let r = workbook
        .worksheet_range(sheet_name.clone().as_str())
        .unwrap()
        .unwrap();

    for row_number in 0..=r.rows().len() {
        // considering only the 2nd column will be the key messages
        let first_column = r.get((row_number, 1));
        let first_column_value: String = match first_column {
            Some(DataType::String(first_column)) => {
                if first_column.to_lowercase().contains("key") {
                    continue;
                }
                first_column.to_string()
            }
            _ => continue,
        };

        tuples.insert(first_column_value);
    }
    tuples
}
