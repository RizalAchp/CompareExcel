use super::{
    deserializer::{deserialize_data_excel, validate},
    errors::{DpdError, DpdResult},
    SortVec,
};
use crate::dpdcmpexcel::CmpRslt;
use calamine::{open_workbook_auto, Reader, Sheets};
use similar::{capture_diff_slices, Algorithm, ChangeTag};
use std::path::Path;

#[allow(unused)]
#[derive(Debug)]
pub struct Comparison(pub Vec<CmpRslt>);

impl Default for Comparison {
    fn default() -> Self {
        Self(Vec::default())
    }
}

impl Comparison {
    pub fn get_data(self) -> (Vec<CmpRslt>, Vec<CmpRslt>) {
        let src = self
            .0
            .iter()
            .filter_map(|item| {
                if item.issrc {
                    Some(item.to_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let tgt = self
            .0
            .iter()
            .filter_map(|item| {
                if !item.issrc {
                    Some(item.to_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        (src, tgt)
    }
    #[allow(unused)]
    pub fn run(
        algortm: Algorithm,
        src: &Vec<Vec<String>>,
        target: &Vec<Vec<String>>,
        sheet: &str,
        src_file: &str,
        target_file: &str,
    ) -> DpdResult<Self> {
        let mut out = vec![];
        let old_data = src.into_iter().map(|x| &x[1..]).collect::<Vec<_>>();
        let new_data = target.into_iter().map(|x| &x[1..]).collect::<Vec<_>>();

        for (idx, op) in capture_diff_slices(algortm, &old_data, &new_data)
            .into_iter()
            .enumerate()
        {
            dbg!("idx capture_diff_slices: {:?}", idx);
            dbg!("op capture_diff_slices: {:?}", &op);
            for change in op.iter_changes(&old_data, &new_data) {
                match change.tag() {
                    ChangeTag::Equal => continue,
                    ChangeTag::Delete | ChangeTag::Insert => {
                        let (file, issrc, index) = if let Some(idx) = change.old_index() {
                            (src_file, true, idx)
                        } else {
                            (target_file, false, change.new_index().unwrap_or(0))
                        };
                        let data = change.value();
                        out.push(CmpRslt {
                            issrc,
                            index,
                            file: file.to_owned(),
                            sheet: sheet.to_owned(),
                            data: data.to_owned(),
                        });
                    }
                };
            }
        }
        Ok(Self(out))
    }
}

#[derive(Debug, Default)]
pub struct SizeTable {
    pub h: usize,
    pub w: usize,
}

impl From<(usize, usize)> for SizeTable {
    #[inline]
    fn from(d: (usize, usize)) -> Self {
        Self { h: d.0, w: d.1 }
    }
}

#[derive(Default)]
pub struct CmpData {
    pub file: String,
    pub exl: Option<Sheets>,
    pub sheets: Vec<String>,
    pub selected_data: Vec<Vec<String>>,
    pub size: SizeTable,
    pub has_header: bool,
    pub is_filtered: bool,
}
impl std::fmt::Debug for CmpData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CmpData")
            .field("file", &self.file)
            .field("exl", &"Option<Sheets>")
            .field("sheets", &self.sheets)
            .field("selected_data", &self.selected_data)
            .field("size", &self.size)
            .field("has_header", &self.has_header)
            .field("is_filtered", &self.is_filtered)
            .finish()
    }
}

impl CmpData {
    pub fn new<P: AsRef<Path>>(f: P) -> DpdResult<Self> {
        match validate(f.as_ref())? {
            super::deserializer::TypeTable::Excel(s) => match open_workbook_auto(&s) {
                Ok(exl) => {
                    let sheets = exl.sheet_names().to_owned();
                    Ok(Self {
                        file: s,
                        exl: Some(exl),
                        sheets,
                        ..Default::default()
                    })
                }
                Err(e) => Err(DpdError::Validation(format!(
                    "Error saat mencoba Membuka File `{}` | `{}`",
                    s, e
                ))),
            },
            super::deserializer::TypeTable::Csv(s) => {
                let mut reader = csv::Reader::from_path(&s)?;
                let data: Vec<Vec<String>> = reader.deserialize().filter_map(|f| f.ok()).collect();
                let size = SizeTable {
                    h: data.len(),
                    w: data
                        .iter()
                        .map(|item| item.len())
                        .max()
                        .unwrap_or(data[0].len()),
                };
                Ok(Self {
                    file: s,
                    selected_data: data,
                    has_header: reader.has_headers(),
                    size,
                    ..Default::default()
                })
            }
        }
    }
    pub(crate) fn set_selected_data(&mut self, sheet: &str) -> DpdResult<()> {
        if let Some(exl) = &mut self.exl {
            let data = match exl.worksheet_range(sheet) {
                Some(ws) => Ok(ws?),
                None => Err(DpdError::Validation(format!(
                    "Tidak ada nama sheet `{}` pada file `{}`",
                    &sheet, &self.file
                ))),
            }?;

            self.selected_data = deserialize_data_excel(&data);
            self.size = data.get_size().into();
            Ok(())
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn filter(&mut self, row: Option<usize>) -> DpdResult<()> {
        self.is_filtered = true;
        if let Some(r) = row {
            self.selected_data.filter_col(r)
        } else {
            self.selected_data.filter_col(self.size.w)
        }
    }
    
    #[inline]
    pub fn sort(&mut self, row: Option<usize>) -> DpdResult<()> {
        self.selected_data.sort_by_col(row.unwrap_or(self.size.w))
    }

    #[inline]
    pub fn close(&mut self) {
        self.file.clear();
        self.exl = None;
        self.sheets.clear();
        self.selected_data.clear();
        self.size = SizeTable::default();
        self.has_header = false;
        self.is_filtered = false;
    }
}
