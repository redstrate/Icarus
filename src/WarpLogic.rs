//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WarpParamsElement<'a> {
    pub Function: &'a str,
    pub Argument: u32,
}
#[derive(Debug, Clone)]
pub struct WarpLogicSheet {
    sheet: Sheet,
}
impl WarpLogicSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WarpLogic")?;
        let sheet = resolver.read_excel_sheet(&exh, "WarpLogic", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<WarpLogicRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<WarpLogicRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for WarpLogicSheet {
    type Row = WarpLogicRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a WarpLogicSheet {
    type Item = (u32, Vec<(u16, WarpLogicRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, WarpLogicSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WarpLogicSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct WarpLogicRow<'a> {
    row: &'a Row,
}
impl<'a> WarpLogicRow<'a> {
    pub fn WarpParams(&'a self) -> [WarpParamsElement<'a>; 10] {
        [
            WarpParamsElement {
                Function: self.row.columns[3].into_string().unwrap(),
                Argument: self.row.columns[13].into_u32().copied().unwrap(),
            },
            WarpParamsElement {
                Function: self.row.columns[4].into_string().unwrap(),
                Argument: self.row.columns[14].into_u32().copied().unwrap(),
            },
            WarpParamsElement {
                Function: self.row.columns[5].into_string().unwrap(),
                Argument: self.row.columns[15].into_u32().copied().unwrap(),
            },
            WarpParamsElement {
                Function: self.row.columns[6].into_string().unwrap(),
                Argument: self.row.columns[16].into_u32().copied().unwrap(),
            },
            WarpParamsElement {
                Function: self.row.columns[7].into_string().unwrap(),
                Argument: self.row.columns[17].into_u32().copied().unwrap(),
            },
            WarpParamsElement {
                Function: self.row.columns[8].into_string().unwrap(),
                Argument: self.row.columns[18].into_u32().copied().unwrap(),
            },
            WarpParamsElement {
                Function: self.row.columns[9].into_string().unwrap(),
                Argument: self.row.columns[19].into_u32().copied().unwrap(),
            },
            WarpParamsElement {
                Function: self.row.columns[10].into_string().unwrap(),
                Argument: self.row.columns[20].into_u32().copied().unwrap(),
            },
            WarpParamsElement {
                Function: self.row.columns[11].into_string().unwrap(),
                Argument: self.row.columns[21].into_u32().copied().unwrap(),
            },
            WarpParamsElement {
                Function: self.row.columns[12].into_string().unwrap(),
                Argument: self.row.columns[22].into_u32().copied().unwrap(),
            },
        ]
    }
    pub fn Question(&'a self) -> &'a str {
        self.row.columns[23].into_string().unwrap()
    }
    pub fn ResponseYes(&'a self) -> &'a str {
        self.row.columns[24].into_string().unwrap()
    }
    pub fn ResponseNo(&'a self) -> &'a str {
        self.row.columns[25].into_string().unwrap()
    }
    pub fn WarpName(&'a self) -> &'a str {
        self.row.columns[1].into_string().unwrap()
    }
    pub fn Unknown0(&'a self) -> u32 {
        self.row.columns[0].into_u32().copied().unwrap()
    }
    pub fn CanSkipCutscene(&'a self) -> bool {
        self.row.columns[2].into_bool().copied().unwrap()
    }
}
