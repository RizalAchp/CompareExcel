use std::path::Path;

use calamine::{DataType, Range};

use super::errors::{DpdError, DpdResult};

#[allow(unused)]
pub(crate) fn convert_csv_to_excel(
    csv_data: Vec<Vec<String>>,
    excel_path: &str,
    sheets_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut wb = simple_excel_writer::Workbook::create(excel_path);
    let mut sheet = wb.create_sheet(sheets_name);

    wb.write_sheet(&mut sheet, |sw| {
        for csv in csv_data {
            let mut row = simple_excel_writer::Row::new();
            for field in csv {
                row.add_cell(field);
            }
            sw.append_row(row)?;
        }
        Ok(())
    })?;

    wb.close()?;
    Ok(())
}

#[allow(unused)]
pub(crate) fn deserialize_data_excel(range: &Range<DataType>) -> Vec<Vec<String>> {
    // let mut dest = String::new();
    let mut out = Vec::new();
    out.reserve(range.get_size().0);
    for r in range.rows() {
        let mut row = Vec::new();
        row.reserve(r.len());
        for c in r.iter() {
            match *c {
                DataType::Empty => (),
                DataType::String(ref s) => row.push(s.to_owned()),
                DataType::Float(ref f) | DataType::DateTime(ref f) => row.push(f.to_string()),
                DataType::Int(ref i) => row.push(i.to_string()),
                DataType::Error(ref e) => row.push(e.to_string()),
                DataType::Bool(ref b) => row.push(b.to_string()),
            };
        }
        if !row.is_empty() {
            out.push(row);
        }
    }
    out
}

pub enum TypeTable {
    Excel(String),
    Csv(String),
}

#[inline]
pub fn validate<P: AsRef<Path>>(f: P) -> DpdResult<TypeTable> {
    let file = f.as_ref();
    match file.extension().and_then(|s| s.to_str()) {
        Some("xlsx") | Some("xlsm") | Some("xlsb") | Some("xls") => Ok(TypeTable::Excel(
            file.to_str().unwrap_or_default().to_owned(),
        )),
        Some("csv") => Ok(TypeTable::Csv(file.to_str().unwrap_or_default().to_owned())),
        _ => Err(DpdError::Validation("Expecting an excel file".to_owned())),
    }
}
