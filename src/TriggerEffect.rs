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
pub struct TriggerEffectSheet {
    sheet: Sheet,
}
impl TriggerEffectSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("TriggerEffect")?;
        let sheet = resolver.read_excel_sheet(&exh, "TriggerEffect", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<TriggerEffectRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<TriggerEffectRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for TriggerEffectSheet {
    type Row = TriggerEffectRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown0: row
                .columns[0]
                .into_i32()
                .copied()
                .expect("Expected column 0 to be a int32!"),
            Unknown1: row
                .columns[1]
                .into_i32()
                .copied()
                .expect("Expected column 1 to be a int32!"),
            Unknown2: row
                .columns[2]
                .into_i32()
                .copied()
                .expect("Expected column 2 to be a int32!"),
            Unknown3: row
                .columns[3]
                .into_i32()
                .copied()
                .expect("Expected column 3 to be a int32!"),
            Unknown4: row
                .columns[4]
                .into_i32()
                .copied()
                .expect("Expected column 4 to be a int32!"),
            Unknown5: row
                .columns[5]
                .into_i32()
                .copied()
                .expect("Expected column 5 to be a int32!"),
            Unknown6: row
                .columns[6]
                .into_i32()
                .copied()
                .expect("Expected column 6 to be a int32!"),
            Unknown7: row
                .columns[7]
                .into_i32()
                .copied()
                .expect("Expected column 7 to be a int32!"),
        })
    }
}
impl<'a> IntoIterator for &'a TriggerEffectSheet {
    type Item = (u32, Vec<(u16, TriggerEffectRow)>);
    type IntoIter = StructuredSheetIterator<'a, TriggerEffectSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, TriggerEffectSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TriggerEffectRow {
    ///""
    pub Unknown0: i32,
    ///""
    pub Unknown1: i32,
    ///""
    pub Unknown2: i32,
    ///""
    pub Unknown3: i32,
    ///""
    pub Unknown4: i32,
    ///""
    pub Unknown5: i32,
    ///""
    pub Unknown6: i32,
    ///""
    pub Unknown7: i32,
}
