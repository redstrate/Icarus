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
pub struct UIColorSheet {
    sheet: Sheet,
}
impl UIColorSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("UIColor")?;
        let sheet = resolver.read_excel_sheet(&exh, "UIColor", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<UIColorRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<UIColorRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for UIColorSheet {
    type Row = UIColorRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Dark: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
            Light: row
                .columns[1]
                .into_u32()
                .copied()
                .expect("Expected column 1 to be a uint32!"),
            ClassicFF: row
                .columns[2]
                .into_u32()
                .copied()
                .expect("Expected column 2 to be a uint32!"),
            ClearBlue: row
                .columns[3]
                .into_u32()
                .copied()
                .expect("Expected column 3 to be a uint32!"),
            ClearWhite: row
                .columns[4]
                .into_u32()
                .copied()
                .expect("Expected column 4 to be a uint32!"),
            ClearGreen: row
                .columns[5]
                .into_u32()
                .copied()
                .expect("Expected column 5 to be a uint32!"),
            ClearGrey: row
                .columns[6]
                .into_u32()
                .copied()
                .expect("Expected column 6 to be a uint32!"),
            ClearPink: row
                .columns[7]
                .into_u32()
                .copied()
                .expect("Expected column 7 to be a uint32!"),
        })
    }
}
impl<'a> IntoIterator for &'a UIColorSheet {
    type Item = (u32, Vec<(u16, UIColorRow)>);
    type IntoIter = StructuredSheetIterator<'a, UIColorSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, UIColorSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UIColorRow {
    ///""
    pub Dark: u32,
    ///""
    pub Light: u32,
    ///""
    pub ClassicFF: u32,
    ///""
    pub ClearBlue: u32,
    ///""
    pub ClearWhite: u32,
    ///""
    pub ClearGreen: u32,
    ///""
    pub ClearGrey: u32,
    ///""
    pub ClearPink: u32,
}
