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
pub struct BNpcStateSheet {
    sheet: Sheet,
}
impl BNpcStateSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BNpcState")?;
        let sheet = resolver.read_excel_sheet(&exh, "BNpcState", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BNpcStateRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BNpcStateRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for BNpcStateSheet {
    type Row = BNpcStateRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a BNpcStateSheet {
    type Item = (u32, Vec<(u16, BNpcStateRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, BNpcStateSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BNpcStateSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BNpcStateRow<'a> {
    row: &'a Row,
}
impl<'a> BNpcStateRow<'a> {
    pub fn Scale(&'a self) -> f32 {
        self.row.columns[11].into_f32().copied().unwrap()
    }
    pub fn LoopTimeline(&'a self) -> i32 {
        self.row.columns[13].into_i32().copied().unwrap()
    }
    pub fn Idle(&'a self) -> u16 {
        self.row.columns[4].into_u16().copied().unwrap()
    }
    pub fn Slot(&'a self) -> u8 {
        self.row.columns[0].into_u8().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> u8 {
        self.row.columns[3].into_u8().copied().unwrap()
    }
    pub fn Attribute0(&'a self) -> u8 {
        self.row.columns[5].into_u8().copied().unwrap()
    }
    pub fn Attribute1(&'a self) -> u8 {
        self.row.columns[7].into_u8().copied().unwrap()
    }
    pub fn Attribute2(&'a self) -> u8 {
        self.row.columns[9].into_u8().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> u8 {
        self.row.columns[12].into_u8().copied().unwrap()
    }
    pub fn OverRay(&'a self) -> i8 {
        self.row.columns[1].into_i8().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> i8 {
        self.row.columns[2].into_i8().copied().unwrap()
    }
    pub fn AttributeFlag0(&'a self) -> bool {
        self.row.columns[6].into_bool().copied().unwrap()
    }
    pub fn AttributeFlag1(&'a self) -> bool {
        self.row.columns[8].into_bool().copied().unwrap()
    }
    pub fn AttributeFlag2(&'a self) -> bool {
        self.row.columns[10].into_bool().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> bool {
        self.row.columns[14].into_bool().copied().unwrap()
    }
}
