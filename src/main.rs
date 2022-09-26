use crate::compares::compares_excel;
use calamine::open_workbook_auto;
use calamine::Reader;
use errors::DpdError;
use errors::DpdResult;
use std::ops::Not;
use std::path::PathBuf;

mod compares;
mod errors;

fn main() -> DpdResult<()> {
    // converts first argument into a csv (same name, silently overrides
    // if the file already exists

    let (file1, file2, isgui) = match (std::env::args().nth(1), std::env::args().nth(2)) {
        (Some(o), Some(b)) => (Some(o), Some(b), false),
        (Some(_), None) => (None, None, true),
        (None, Some(_)) => (None, None, true),
        (None, None) => (None, None, true),
    };
    if isgui.not() {
        let sce1 = PathBuf::from(file1.unwrap());
        let sce2 = PathBuf::from(file2.unwrap());
        match (
            sce1.extension().and_then(|s| s.to_str()),
            sce2.extension().and_then(|s| s.to_str()),
        ) {
            (Some("xlsx"), Some("xlsx"))
            | (Some("xlsm"), Some("xlsm"))
            | (Some("xlsb"), Some("xlsb"))
            | (Some("xls"), Some("xls")) => (),
            _ => return Err(DpdError::Validation("Expecting an excel file".to_owned())),
        }

        let mut xl1 = open_workbook_auto(&sce1).unwrap();
        let mut xl2 = open_workbook_auto(&sce2).unwrap();
        for (sheet, data) in xl1.worksheets().into_iter() {
            match xl2.worksheet_range(&sheet) {
                Some(d) => match d {
                    Ok(dt) => {
                        let dd = compares_excel(
                            data,
                            dt,
                            sce1.as_path().file_name().unwrap().to_str().unwrap(),
                            sce2.as_path().file_name().unwrap().to_str().unwrap(),
                            &sheet);
                        for data in dd.unwrap() {
                            println!("{data}");
                        }
                        } ,
                    Err(e) => {
                        eprintln!("error on parsing data in sheets {}: {}", sheet, e);
                        continue;
                    }
                },
                None => {
                    eprintln!("sheet {} is not exists on file {:?}", sheet, &sce2);
                    continue;
                }
            };
        };
    };
    Ok(())
}
