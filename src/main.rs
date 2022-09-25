use calamine::{open_workbook_auto, DataType, Range, Reader};
use similar::{capture_diff_slices, Algorithm, ChangeTag};
mod compares;
use std::fmt::Write;
use std::ops::Not;

use console::{style, Style};
use std::fmt;
struct Line(String, Option<usize>);
impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sz = self.0.len();
        let st = if sz > 30 { &self.0[0..30] } else { &self.0 };
        match self.1 {
            None => write!(f, "    "),
            Some(idx) => {
                for _ in 0..31 - st.len() {
                    write!(f, " ")?
                }
                write!(f, "{:>4}", idx + 1)
            }
        }
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LimitedVec<T>(pub Vec<T>);
impl fmt::Display for LimitedVec<String> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in self.0.iter() {
            write!(f, "|{:.<10}", if i.len() <= 10 { &i } else { &i[..8] })?;
        }
        Ok(())
    }
}
use std::env;

use std::path::PathBuf;

fn main() {
    // converts first argument into a csv (same name, silently overrides
    // if the file already exists

    let (file1, file2, isgui) = match (env::args().nth(1), env::args().nth(2)) {
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
            _ => panic!("Expecting an excel file"),
        }

        let mut xl1 = open_workbook_auto(&sce1).unwrap();
        let mut xl2 = open_workbook_auto(&sce2).unwrap();
        for (sheet, data) in xl1.worksheets().into_iter() {
            match xl2.worksheet_range(&sheet) {
                Some(d) => match d {
                    Ok(dt) => compares_excel(
                        data,
                        dt,
                        sce1.as_path().file_name().unwrap().to_str().unwrap(),
                        sce2.as_path().file_name().unwrap().to_str().unwrap(),
                    ),
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
        }
    }
}
fn deserialize_data(range: &Range<DataType>) -> Vec<Vec<String>> {
    // let mut dest = String::new();
    let n = range.get_size().1 - 1;
    let mut dest = String::new();
    for r in range.rows() {
        for (i, c) in r.iter().enumerate() {
            let (rslt, withdelim) = match *c {
                DataType::Empty => (Ok(()), false),
                DataType::String(ref s) => (write!(dest, "{}", s), true),
                DataType::Float(ref f) | DataType::DateTime(ref f) => (write!(dest, "{}", f), true),
                DataType::Int(ref i) => (write!(dest, "{}", i), true),
                DataType::Error(ref e) => (write!(dest, "{:?}", e), true),
                DataType::Bool(ref b) => (write!(dest, "{}", b), true),
            };
            rslt.unwrap();

            if i != n && withdelim {
                write!(dest, ";").unwrap();
            }
        }
        write!(dest, "\n").unwrap();
    }
    csv::ReaderBuilder::new()
        .has_headers(false)
        .trim(csv::Trim::All)
        .delimiter(b';')
        .from_reader(dest.as_bytes())
        .deserialize()
        .map(|d| d.unwrap_or(Vec::new()))
        .collect()
}

fn compares_excel(
    data_src: Range<DataType>,
    data_cmp: Range<DataType>,
    file_src: &str,
    file_cmp: &str,
) {
    let mut src = deserialize_data(&data_src)
        .into_iter()
        .filter_map(|f| if !f.is_empty() { Some(f) } else { None })
        .collect::<Vec<_>>();
    let mut cmp = deserialize_data(&data_cmp)
        .into_iter()
        .filter_map(|f| if !f.is_empty() { Some(f) } else { None })
        .collect::<Vec<_>>();
    src.sort_by(|a, b| a[1].to_lowercase().cmp(&b[1].to_lowercase()));
    cmp.sort_by(|a, b| a[1].to_lowercase().cmp(&b[1].to_lowercase()));

    let src_cmp = src.iter().map(|x| x[1..].to_vec()).collect::<Vec<_>>();
    let cmp_cmp = cmp.iter().map(|x| x[1..].to_vec()).collect::<Vec<_>>();
    // for (idx, (itemsrc, itemcmp)) in src.into_iter().zip(cmp).enumerate() {
    capture_diff_slices(Algorithm::Myers, &src_cmp, &cmp_cmp)
        .into_iter()
        .enumerate()
        .for_each(|(i, op)| {
            op.iter_changes(&src_cmp, &cmp_cmp).for_each(|change| {
                let val = change.value();
                let file = if src_cmp.contains(&val) {
                    &file_src
                } else {
                    &file_cmp
                };
                let (old, new) = (change.old_index(), change.new_index());
                let (sign, s, show) = match change.tag() {
                    ChangeTag::Delete => ("-", Style::new().red().bold(), true),
                    ChangeTag::Insert => ("+", Style::new().green().bold(), true),
                    ChangeTag::Equal => (" ", Style::new().dim(), false),
                };
                if show {
                    println!(
                        "{} | {}:{} |  => {}",
                        s.apply_to(sign).bold(),
                        file,
                        style(Line(
                            file.to_string(),
                            Some(new.unwrap_or(old.unwrap_or(0)))
                        ))
                        .dim(),
                        s.apply_to(LimitedVec {
                            0: src[i].to_owned()
                        })
                        .on_black()
                    );
                }
            })
        });
    // }
}
