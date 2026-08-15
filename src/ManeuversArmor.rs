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
pub struct ManeuversArmorSheet {
    sheet: Sheet,
}
impl ManeuversArmorSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ManeuversArmor")?;
        let sheet = resolver.read_excel_sheet(&exh, "ManeuversArmor", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ManeuversArmorRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ManeuversArmorRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for ManeuversArmorSheet {
    type Row = ManeuversArmorRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[10]
                .into_string()
                .cloned()
                .expect("Expected column 10 to be a string!"),
            Description: row
                .columns[11]
                .into_string()
                .cloned()
                .expect("Expected column 11 to be a string!"),
            FalconName: row
                .columns[1]
                .into_u32()
                .copied()
                .expect("Expected column 1 to be a uint32!"),
            RavenName: row
                .columns[2]
                .into_u32()
                .copied()
                .expect("Expected column 2 to be a uint32!"),
            NeutralMapIcon: row
                .columns[5]
                .into_u32()
                .copied()
                .expect("Expected column 5 to be a uint32!"),
            FalconImage: row
                .columns[6]
                .into_u32()
                .copied()
                .expect("Expected column 6 to be a uint32!"),
            RavenImage: row
                .columns[7]
                .into_u32()
                .copied()
                .expect("Expected column 7 to be a uint32!"),
            FalconMapImage: row
                .columns[8]
                .into_u32()
                .copied()
                .expect("Expected column 8 to be a uint32!"),
            RavenMapImage: row
                .columns[9]
                .into_u32()
                .copied()
                .expect("Expected column 9 to be a uint32!"),
            Unknown0: row
                .columns[0]
                .into_u16()
                .copied()
                .expect("Expected column 0 to be a uint16!"),
            Unknown1: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
            Unknown2: row
                .columns[4]
                .into_bool()
                .copied()
                .expect("Expected column 4 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a ManeuversArmorSheet {
    type Item = (u32, Vec<(u16, ManeuversArmorRow)>);
    type IntoIter = StructuredSheetIterator<'a, ManeuversArmorSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ManeuversArmorSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ManeuversArmorRow {
    ///""
    pub Name: String,
    ///""
    pub Description: String,
    ///""
    pub FalconName: u32,
    ///""
    pub RavenName: u32,
    ///""
    pub NeutralMapIcon: u32,
    ///""
    pub FalconImage: u32,
    ///""
    pub RavenImage: u32,
    ///""
    pub FalconMapImage: u32,
    ///""
    pub RavenMapImage: u32,
    ///""
    pub Unknown0: u16,
    ///""
    pub Unknown1: u8,
    ///""
    pub Unknown2: bool,
}
