use super::{
    deserializer::{deserialize_data_excel, validate},
    errors::{DpdError, DpdResult},
};
use crate::dpdcmpexcel::CmpRslt;
use calamine::{open_workbook_auto, Reader, Sheets};
use console::Style;
use similar::{capture_diff_slices, Algorithm, ChangeTag};
use std::path::Path;

#[allow(unused)]
pub struct Comparison(Vec<CmpRslt>);

impl Comparison {
    #[allow(unused)]
    #[inline]
    pub fn default() -> Self {
        Self(Vec::default())
    }
    #[allow(unused)]
    pub fn run(
        src: &Vec<Vec<String>>,
        target: &Vec<Vec<String>>,
        sheet: &str,
    ) -> DpdResult<Self> {
        let src_cmp = src.iter().map(|x| x[1..].to_owned()).collect::<Vec<_>>();
        let target_cmp = target.iter().map(|x| x[1..].to_owned()).collect::<Vec<_>>();

        let mut out: Vec<CmpRslt> = Vec::new();
        for (idx, op) in capture_diff_slices(Algorithm::Myers, &src_cmp, &target_cmp)
            .into_iter()
            .enumerate()
        {
            op.iter_changes(&src_cmp, &target_cmp).for_each(|change| {
                let data = change.value().to_owned();
                let (old, new) = (change.old_index(), change.new_index());
                let (sign, style, show) = match change.tag() {
                    ChangeTag::Delete => ("-".to_owned(), Style::new().red().bold(), true),
                    ChangeTag::Insert => ("+".to_owned(), Style::new().green().bold(), true),
                    ChangeTag::Equal => (String::default(), Style::new().dim(), false),
                };
                if show {
                    out.push(CmpRslt {
                        oldindex: old,
                        newindex: new,
                        sheet: sheet.to_owned(),
                        sign,
                        style,
                        data,
                    });
                }
            })
        }
        Ok(Self(out))
    }
}

pub struct CmpData {
    pub file: String,
    pub exl: Sheets,
    pub sheets: Vec<String>,
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
                Ok(Self { file, exl, sheets })
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
        let data = match self.exl.worksheet_range(sheet) {
            Some(ws) => Ok(ws?),
            None => Err(DpdError::Validation(format!(
                "Tidak ada nama sheet `{}` pada file `{}`",
                &sheet, &self.file
            ))),
        }?;

        Ok(deserialize_data_excel(&data))
    }
}
