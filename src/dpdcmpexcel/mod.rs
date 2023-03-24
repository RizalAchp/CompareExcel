pub mod compares;
pub mod deserializer;
pub mod errors;

use std::{fmt, fmt::Display};

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
            "{} |  => {}",
            Line("file".to_owned(), Some(self.index)),
            LimitedVec {
                0: self.data.to_owned()
            }
        )
    }
}

pub trait SortVec {
    type ReturnType;
    fn filter_col(&mut self, size: usize) -> Self::ReturnType;
    fn sort_by_col(&mut self, idx_col: usize) -> Self::ReturnType;
}
impl SortVec for Vec<Vec<String>> {
    type ReturnType = DpdResult<()>;
    #[inline(always)]
    fn filter_col(&mut self, size_row: usize) -> Self::ReturnType {
        self.retain(|f| f.len() == size_row);
        Ok(())
    }

    #[inline(always)]
    fn sort_by_col(&mut self, idx_col: usize) -> Self::ReturnType {
        if idx_col >= self.len() {
            return Err(DpdError::Processing(
                "Error on Shorting Vector of data excel!".to_owned(),
            ));
        }
        self.sort_by(|a, b| a[idx_col].to_lowercase().cmp(&b[idx_col].to_lowercase()));
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
#[derive(Debug, Clone)]
pub struct CmpRslt {
    pub issrc: bool,
    pub tag: similar::ChangeTag,
    pub index: usize,
    pub file: String,
    pub sheet: String,
    pub data: Vec<String>,
}

impl Default for CmpRslt {
    fn default() -> Self {
        Self {
            issrc: Default::default(),
            tag: similar::ChangeTag::Delete,
            index: Default::default(),
            file: Default::default(),
            sheet: Default::default(),
            data: Default::default(),
        }
    }
}
