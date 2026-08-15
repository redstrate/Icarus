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
pub struct RelicSheet {
    sheet: Sheet,
}
impl RelicSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Relic")?;
        let sheet = resolver.read_excel_sheet(&exh, "Relic", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<RelicRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<RelicRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for RelicSheet {
    type Row = RelicRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            ItemAtma: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
            ItemAnimus: row
                .columns[1]
                .into_u32()
                .copied()
                .expect("Expected column 1 to be a uint32!"),
            Icon: row
                .columns[2]
                .into_i32()
                .copied()
                .expect("Expected column 2 to be a int32!"),
            Materia0: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            Materia1: row
                .columns[7]
                .into_u16()
                .copied()
                .expect("Expected column 7 to be a uint16!"),
            Materia2: row
                .columns[11]
                .into_u16()
                .copied()
                .expect("Expected column 11 to be a uint16!"),
            Materia3: row
                .columns[14]
                .into_u16()
                .copied()
                .expect("Expected column 14 to be a uint16!"),
            NoteMain0: row
                .columns[4]
                .into_u8()
                .copied()
                .expect("Expected column 4 to be a uint8!"),
            NoteSub0: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            NoteSelection10: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
            NoteMain1: row
                .columns[8]
                .into_u8()
                .copied()
                .expect("Expected column 8 to be a uint8!"),
            NoteSub1: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
            NoteSelection1: row
                .columns[10]
                .into_u8()
                .copied()
                .expect("Expected column 10 to be a uint8!"),
            NoteMain2: row
                .columns[12]
                .into_u8()
                .copied()
                .expect("Expected column 12 to be a uint8!"),
            NoteSub2: row
                .columns[13]
                .into_u8()
                .copied()
                .expect("Expected column 13 to be a uint8!"),
            NoteSelection3: row
                .columns[15]
                .into_u8()
                .copied()
                .expect("Expected column 15 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a RelicSheet {
    type Item = (u32, Vec<(u16, RelicRow)>);
    type IntoIter = StructuredSheetIterator<'a, RelicSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, RelicSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RelicRow {
    ///""
    pub ItemAtma: u32,
    ///""
    pub ItemAnimus: u32,
    ///""
    pub Icon: i32,
    ///""
    pub Materia0: u16,
    ///""
    pub Materia1: u16,
    ///""
    pub Materia2: u16,
    ///""
    pub Materia3: u16,
    ///""
    pub NoteMain0: u8,
    ///""
    pub NoteSub0: u8,
    ///""
    pub NoteSelection10: u8,
    ///""
    pub NoteMain1: u8,
    ///""
    pub NoteSub1: u8,
    ///""
    pub NoteSelection1: u8,
    ///""
    pub NoteMain2: u8,
    ///""
    pub NoteSub2: u8,
    ///""
    pub NoteSelection3: u8,
}
