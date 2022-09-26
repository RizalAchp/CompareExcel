use crate::errors::{DpdError, DpdResult};
use calamine::Sheets;
use calamine::{DataType, Range};
use console::style;
use console::Style;
use similar::{capture_diff_slices, Algorithm, ChangeTag};
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::{fmt, fmt::Write};

pub(crate) struct Line(String, Option<usize>);
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

#[allow(unused)]
pub(crate) struct CmpRslt {
    pub oldindex: Option<usize>,
    pub newindex: Option<usize>,
    pub sheet: String,
    pub sign: String,
    pub file: String,
    pub style: Style,
    pub data: Vec<String>,
}
#[allow(unused)]
#[derive(Default)]
pub(crate) struct Comparison {
    pub xl_src: Option<Sheets>,
    pub xl_cmp: Option<Sheets>,
}

impl Comparison {
    fn from_file(fsrc: &str, fcmp: &str) -> DpdResult<Self> {
        Ok(Self {
            xl_src: None,
            xl_cmp: None,
        })
    }

    fn from_drop_down(v: Vec<String>) -> DpdResult<Self> {
        unimplemented!()
    }

    fn validate(f: &str) -> bool {
        match PathBuf::from(f).extension().and_then(|s| s.to_str()) {
            Some("xlsx") | Some("xlsm") | Some("xlsb") | Some("xls") => true,
            _ => false,
        }
    }
}

impl CmpRslt {
    fn from_comp_algortm(
        sheet: String,
        file: String,
        oldindex: Option<usize>,
        newindex: Option<usize>,
        sign: String,
        style: Style,
        data: Vec<String>,
    ) -> Self {
        Self {
            oldindex,
            newindex,
            sheet,
            sign,
            file,
            style,
            data,
        }
    }
}
impl Display for CmpRslt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} | {}:{} |  => {}",
            self.style.apply_to(&self.sign).bold(),
            self.file,
            style(Line(
                self.file.to_string(),
                Some(self.newindex.unwrap_or(self.oldindex.unwrap_or(0)))
            ))
            .dim(),
            self.style
                .apply_to(LimitedVec {
                    0: self.data.to_owned()
                })
                .on_black()
        )
    }
}

pub(crate) fn deserialize_data(range: &Range<DataType>) -> DpdResult<Vec<Vec<String>>> {
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
            rslt?;

            if i != n && withdelim {
                write!(dest, ";")?;
            }
        }
        write!(dest, "\n")?
    }
    Ok(csv::ReaderBuilder::new()
        .has_headers(false)
        .trim(csv::Trim::All)
        .delimiter(b';')
        .from_reader(dest.as_bytes())
        .deserialize()
        .map(|d| d.unwrap_or(Vec::new()))
        .collect::<Vec<_>>())
}

pub(crate) fn compares_excel(
    data_src: Range<DataType>,
    data_cmp: Range<DataType>,
    file_src: &str,
    file_cmp: &str,
    sheet: &str,
) -> DpdResult<Vec<CmpRslt>> {
    let (src, cmp) = match (deserialize_data(&data_src), deserialize_data(&data_cmp)) {
        (Ok(s), Ok(c)) => Ok((s.sort_by_col(1).unwrap(), c.sort_by_col(1).unwrap())),
        (Ok(_), Err(_)) => Err(DpdError::Processing(format!(
            "Error on Processing Excel pada file: {}",
            file_src
        ))),
        (Err(_), Ok(_)) => Err(DpdError::Processing(format!(
            "Error on Processing Excel pada file: {}",
            file_cmp
        ))),
        (Err(_), Err(_)) => Err(DpdError::Processing(format!(
            "Error on Processing Excel pada file: {} dan file {}",
            file_src, file_cmp
        ))),
    }
    .unwrap();

    let src_cmp = src.iter().map(|x| x[1..].to_owned()).collect::<Vec<_>>();
    let cmp_cmp = cmp.iter().map(|x| x[1..].to_owned()).collect::<Vec<_>>();
    let mut out: Vec<CmpRslt> = Vec::new();
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
                    out.push(CmpRslt::from_comp_algortm(
                        sheet.to_owned(),
                        file.to_string(),
                        old,
                        new,
                        sign.to_owned(),
                        s,
                        src[i].to_owned(),
                    ));
                }
            })
        });
    Ok(out)
}

pub trait SortVec {
    type ReturnType;
    fn sort_by_col(&self, idx_col: usize) -> Self::ReturnType;
}

impl SortVec for Vec<Vec<String>> {
    type ReturnType = Result<Vec<Vec<String>>, DpdError>;

    fn sort_by_col(&self, idx_col: usize) -> Result<Vec<Vec<String>>, DpdError> {
        if idx_col >= self.len() {
            return Err(DpdError::Processing(
                "Error on Shorting Vector of data excel!".to_owned(),
            ));
        }
        let mut nw: Vec<_> = self
            .into_iter()
            .filter_map(|f| {
                if !f.is_empty() {
                    Some(f.to_owned())
                } else {
                    None
                }
            })
            .collect();
        nw.sort_by(|a, b| a[1].to_lowercase().cmp(&b[1].to_lowercase()));
        Ok(nw)
    }
    // add code here
}
