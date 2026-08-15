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
pub struct ReplaceActionSheet {
    sheet: Sheet,
}
impl ReplaceActionSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ReplaceAction")?;
        let sheet = resolver.read_excel_sheet(&exh, "ReplaceAction", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ReplaceActionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ReplaceActionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for ReplaceActionSheet {
    type Row = ReplaceActionRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Action: row
                .columns[0]
                .into_i32()
                .copied()
                .expect("Expected column 0 to be a int32!"),
            ReplaceActions: [
                row
                    .columns[3]
                    .into_i32()
                    .copied()
                    .expect("Expected column 3 to be a int32!"),
                row
                    .columns[6]
                    .into_i32()
                    .copied()
                    .expect("Expected column 6 to be a int32!"),
                row
                    .columns[9]
                    .into_i32()
                    .copied()
                    .expect("Expected column 9 to be a int32!"),
                row
                    .columns[12]
                    .into_i32()
                    .copied()
                    .expect("Expected column 12 to be a int32!"),
            ],
            Param1: row
                .columns[2]
                .into_i16()
                .copied()
                .expect("Expected column 2 to be a int16!"),
            Param2: row
                .columns[5]
                .into_i16()
                .copied()
                .expect("Expected column 5 to be a int16!"),
            Param3: row
                .columns[8]
                .into_i16()
                .copied()
                .expect("Expected column 8 to be a int16!"),
            Param4: row
                .columns[11]
                .into_i16()
                .copied()
                .expect("Expected column 11 to be a int16!"),
            Type1: row
                .columns[1]
                .into_i8()
                .copied()
                .expect("Expected column 1 to be a int8!"),
            Type2: row
                .columns[4]
                .into_i8()
                .copied()
                .expect("Expected column 4 to be a int8!"),
            Type3: row
                .columns[7]
                .into_i8()
                .copied()
                .expect("Expected column 7 to be a int8!"),
            Type4: row
                .columns[10]
                .into_i8()
                .copied()
                .expect("Expected column 10 to be a int8!"),
            ReplaceSettable: row
                .columns[13]
                .into_i8()
                .copied()
                .expect("Expected column 13 to be a int8!"),
            Unknown_70: row
                .columns[14]
                .into_bool()
                .copied()
                .expect("Expected column 14 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a ReplaceActionSheet {
    type Item = (u32, Vec<(u16, ReplaceActionRow)>);
    type IntoIter = StructuredSheetIterator<'a, ReplaceActionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ReplaceActionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReplaceActionRow {
    ///""
    pub Action: i32,
    ///""
    pub ReplaceActions: [i32; 4],
    ///""
    pub Param1: i16,
    ///""
    pub Param2: i16,
    ///""
    pub Param3: i16,
    ///""
    pub Param4: i16,
    ///""
    pub Type1: i8,
    ///""
    pub Type2: i8,
    ///""
    pub Type3: i8,
    ///""
    pub Type4: i8,
    ///""
    pub ReplaceSettable: i8,
    ///""
    pub Unknown_70: bool,
}
