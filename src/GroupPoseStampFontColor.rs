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
pub struct GroupPoseStampFontColorSheet {
    sheet: Sheet,
}
impl GroupPoseStampFontColorSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GroupPoseStampFontColor")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "GroupPoseStampFontColor", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<GroupPoseStampFontColorRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<GroupPoseStampFontColorRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for GroupPoseStampFontColorSheet {
    type Row = GroupPoseStampFontColorRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown0: row
                .columns[3]
                .into_string()
                .cloned()
                .expect("Expected column 3 to be a string!"),
            Unknown1: row
                .columns[0]
                .into_i32()
                .copied()
                .expect("Expected column 0 to be a int32!"),
            Unknown2: row
                .columns[1]
                .into_i32()
                .copied()
                .expect("Expected column 1 to be a int32!"),
            Unknown3: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a GroupPoseStampFontColorSheet {
    type Item = (u32, Vec<(u16, GroupPoseStampFontColorRow)>);
    type IntoIter = StructuredSheetIterator<'a, GroupPoseStampFontColorSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GroupPoseStampFontColorSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GroupPoseStampFontColorRow {
    ///""
    pub Unknown0: String,
    ///""
    pub Unknown1: i32,
    ///""
    pub Unknown2: i32,
    ///""
    pub Unknown3: u8,
}
