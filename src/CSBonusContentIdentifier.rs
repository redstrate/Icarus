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
pub struct CSBonusContentIdentifierSheet {
    sheet: Sheet,
}
impl CSBonusContentIdentifierSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("CSBonusContentIdentifier")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "CSBonusContentIdentifier", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<CSBonusContentIdentifierRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<CSBonusContentIdentifierRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for CSBonusContentIdentifierSheet {
    type Row = CSBonusContentIdentifierRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Content: row
                .columns[1]
                .into_u32()
                .copied()
                .expect("Expected column 1 to be a uint32!"),
            UnlockQuest0: row
                .columns[3]
                .into_u32()
                .copied()
                .expect("Expected column 3 to be a uint32!"),
            UnlockQuest1: row
                .columns[4]
                .into_u32()
                .copied()
                .expect("Expected column 4 to be a uint32!"),
            UnlockQuest2: row
                .columns[5]
                .into_u32()
                .copied()
                .expect("Expected column 5 to be a uint32!"),
            Unknown6: row
                .columns[6]
                .into_u32()
                .copied()
                .expect("Expected column 6 to be a uint32!"),
            Map: row
                .columns[7]
                .into_u32()
                .copied()
                .expect("Expected column 7 to be a uint32!"),
            Icon: row
                .columns[8]
                .into_i32()
                .copied()
                .expect("Expected column 8 to be a int32!"),
            ContentLinkType: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
            Unknown2: row
                .columns[2]
                .into_bool()
                .copied()
                .expect("Expected column 2 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a CSBonusContentIdentifierSheet {
    type Item = (u32, Vec<(u16, CSBonusContentIdentifierRow)>);
    type IntoIter = StructuredSheetIterator<'a, CSBonusContentIdentifierSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, CSBonusContentIdentifierSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CSBonusContentIdentifierRow {
    ///""
    pub Content: u32,
    ///""
    pub UnlockQuest0: u32,
    ///""
    pub UnlockQuest1: u32,
    ///""
    pub UnlockQuest2: u32,
    ///""
    pub Unknown6: u32,
    ///""
    pub Map: u32,
    ///""
    pub Icon: i32,
    ///""
    pub ContentLinkType: u8,
    ///""
    pub Unknown2: bool,
}
