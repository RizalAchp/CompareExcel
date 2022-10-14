use super::{
    deserializer::{deserialize_data_excel, validate},
    errors::{DpdError, DpdResult},
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
        let src = self.0.iter()
            .filter_map(|item| if item.issrc { Some(item.to_owned()) } else { None })
            .collect::<Vec<_>>();
        let tgt = self.0.iter()
            .filter_map(|item| if !item.issrc { Some(item.to_owned()) } else { None })
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
            for change in op.iter_changes(&old_data, &new_data) {
                let (old, new) = (change.old_index(), change.new_index());
                let (file, issrc) = if old.is_some() {
                    (src_file, true)
                } else {
                    (target_file, false)
                };
                let data = change.value();
                let (sign, show) = match change.tag() {
                    ChangeTag::Delete => ("-", true),
                    ChangeTag::Insert => ("+", true),
                    ChangeTag::Equal => ("", false),
                };
                if show {
                    out.push(CmpRslt {
                        issrc,
                        oldindex: old,
                        newindex: new,
                        file: file.to_owned(),
                        sheet: sheet.to_owned(),
                        sign: sign.to_owned(),
                        data: data.to_owned(),
                    });
                }
            }
        }
        Ok(Self(out))
    }
}

pub struct CmpData {
    pub file: String,
    pub exl: Option<Sheets>,
    pub sheets: Vec<String>,
}
impl std::fmt::Debug for CmpData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CmpData (file: {:?}, ", self.file)?;
        if self.exl.is_some() {
            write!(f, "exl: Sheets, ")
        } else {
            write!(f, "exl: None, ")
        }?;
        write!(f, "sheets: {:?})", self.sheets)
    }
}

impl Default for CmpData {
    fn default() -> Self {
        Self {
            file: Default::default(),
            exl: None,
            sheets: Default::default(),
        }
    }
}

impl CmpData {
    pub fn new<P: AsRef<Path>>(f: P) -> DpdResult<Self> {
        validate(f.as_ref())?;
        let file = match f.as_ref().to_str() {
            Some(p) => p.to_owned(),
            None => String::new(),
        };
        match open_workbook_auto(f.as_ref()) {
            Ok(exl) => {
                let sheets = exl.sheet_names().to_owned();
                Ok(Self {
                    file,
                    exl: Some(exl),
                    sheets,
                })
            }
            Err(e) => Err(DpdError::Validation(format!(
                "Error saat mencoba Membuka File `{}` | `{}`",
                file, e
            ))),
        }
    }
    pub(crate) fn get_deserialized_data(
        &mut self,
        sheet: &str,
    ) -> DpdResult<(Vec<Vec<String>>, (usize, usize))> {
        if let Some(exl) = &mut self.exl {
            let data = match exl.worksheet_range(sheet) {
                Some(ws) => Ok(ws?),
                None => Err(DpdError::Validation(format!(
                    "Tidak ada nama sheet `{}` pada file `{}`",
                    &sheet, &self.file
                ))),
            }?;

            Ok(deserialize_data_excel(&data))
        } else {
            return Err(DpdError::Validation(
                "There is no Excel file Loaded on the application".to_owned(),
            ));
        }
    }
}
