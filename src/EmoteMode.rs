//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Debug, Clone)]
pub struct EmoteModeSheet {
    sheet: Sheet,
}
impl EmoteModeSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("EmoteMode")?;
        let sheet = resolver.read_excel_sheet(&exh, "EmoteMode", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<EmoteModeRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<EmoteModeRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for EmoteModeSheet {
    type Row = EmoteModeRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a EmoteModeSheet {
    type Item = (u32, Vec<(u16, EmoteModeRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, EmoteModeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, EmoteModeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct EmoteModeRow<'a> {
    row: &'a Row,
}
impl<'a> EmoteModeRow<'a> {
    pub fn StartEmote(&'a self) -> u16 {
        self.row.columns[0].into_u16().copied().unwrap()
    }
    pub fn EndEmote(&'a self) -> u16 {
        self.row.columns[1].into_u16().copied().unwrap()
    }
    pub fn ConditionMode(&'a self) -> u8 {
        self.row.columns[6].into_u8().copied().unwrap()
    }
    pub fn Move(&'a self) -> bool {
        self.row.columns[2].into_bool().copied().unwrap()
    }
    pub fn Camera(&'a self) -> bool {
        self.row.columns[3].into_bool().copied().unwrap()
    }
    pub fn EndOnRotate(&'a self) -> bool {
        self.row.columns[4].into_bool().copied().unwrap()
    }
    pub fn EndOnEmote(&'a self) -> bool {
        self.row.columns[5].into_bool().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> bool {
        self.row.columns[7].into_bool().copied().unwrap()
    }
}
