pub mod compares;
pub mod deserializer;
pub mod errors;

use std::{fmt, fmt::Display};

use console::{style, Style};

use self::errors::DpdResult;
pub use self::{
    compares::{CmpData, Comparison},
    errors::DpdError,
};

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

impl Display for CmpRslt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} | line: {} |  => {}",
            self.style.apply_to(&self.sign).bold(),
            style(Line(
                "file".to_owned(),
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

pub trait SortVec {
    type ReturnType;
    fn sort_by_col(&mut self, idx_col: usize, size: usize) -> Self::ReturnType;
}
impl SortVec for Vec<Vec<String>> {
    type ReturnType = Result<(), DpdError>;

    fn sort_by_col(&mut self, idx_col: usize, size: usize) -> DpdResult<()> {
        if idx_col >= self.len() {
            return Err(DpdError::Processing(
                "Error on Shorting Vector of data excel!".to_owned(),
            ));
        }
        let mut temp = self
            .iter()
            .filter_map(|item| {
                if item.len() == size {
                    Some(item.to_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        temp.sort_by(|a, b| a[idx_col].to_lowercase().cmp(&b[idx_col].to_lowercase()));
        self.clear();
        self.append(&mut temp);
        Ok(())
    }
    // add code here
}
pub trait Identic {
    type Type;
    fn get_identic(&self, rhs: &Self::Type) -> Self::Type;
}

impl Identic for Vec<String> {
    type Type = Vec<String>;
    fn get_identic(&self, rhs: &Self::Type) -> Self::Type {
        self.iter()
            .zip(rhs.iter())
            .filter_map(|(o, t)| if o == t { Some(o.to_owned()) } else { None })
            .collect::<Vec<_>>()
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub struct CmpRslt {
    pub oldindex: Option<usize>,
    pub newindex: Option<usize>,
    pub sheet: String,
    pub sign: String,
    pub style: Style,
    pub data: Vec<String>,
}
