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
    #[allow(unused)]
    pub fn run(
        &mut self,
        src: &Vec<Vec<String>>,
        target: &Vec<Vec<String>>,
        sheet: &str,
    ) -> DpdResult<()> {
        self.0.clear();
        let src_cmp = src.iter().map(|x| x[1..].to_owned()).collect::<Vec<_>>();
        let target_cmp = target.iter().map(|x| x[1..].to_owned()).collect::<Vec<_>>();

        for (idx, op) in capture_diff_slices(Algorithm::Myers, &src_cmp, &target_cmp)
            .into_iter()
            .enumerate()
        {
            op.iter_changes(&src_cmp, &target_cmp).for_each(|change| {
                let data = change.value().to_owned();
                let (old, new) = (change.old_index(), change.new_index());
                let (sign, show) = match change.tag() {
                    ChangeTag::Delete => ("-".to_owned(), true),
                    ChangeTag::Insert => ("+".to_owned(), true),
                    ChangeTag::Equal => (String::default(), false),
                };
                if show {
                    self.0.push(CmpRslt {
                        oldindex: old,
                        newindex: new,
                        sheet: sheet.to_owned(),
                        sign,
                        data,
                    });
                }
            })
        }
        Ok(())
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
