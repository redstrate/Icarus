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
pub struct GcArmyExpeditionTraitSheet {
    sheet: Sheet,
}
impl GcArmyExpeditionTraitSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GcArmyExpeditionTrait")?;
        let sheet = resolver.read_excel_sheet(&exh, "GcArmyExpeditionTrait", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<GcArmyExpeditionTraitRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<GcArmyExpeditionTraitRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for GcArmyExpeditionTraitSheet {
    type Row = GcArmyExpeditionTraitRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown0: row
                .columns[10]
                .into_string()
                .cloned()
                .expect("Expected column 10 to be a string!"),
            Unknown1: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
            Unknown2: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            Unknown3: row
                .columns[4]
                .into_u8()
                .copied()
                .expect("Expected column 4 to be a uint8!"),
            Unknown4: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
            Unknown5: row
                .columns[8]
                .into_u8()
                .copied()
                .expect("Expected column 8 to be a uint8!"),
            Unknown6: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            Unknown7: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
            Unknown8: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            Unknown9: row
                .columns[7]
                .into_u8()
                .copied()
                .expect("Expected column 7 to be a uint8!"),
            Unknown10: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a GcArmyExpeditionTraitSheet {
    type Item = (u32, Vec<(u16, GcArmyExpeditionTraitRow)>);
    type IntoIter = StructuredSheetIterator<'a, GcArmyExpeditionTraitSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GcArmyExpeditionTraitSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GcArmyExpeditionTraitRow {
    ///""
    pub Unknown0: String,
    ///""
    pub Unknown1: u8,
    ///""
    pub Unknown2: u8,
    ///""
    pub Unknown3: u8,
    ///""
    pub Unknown4: u8,
    ///""
    pub Unknown5: u8,
    ///""
    pub Unknown6: u8,
    ///""
    pub Unknown7: u8,
    ///""
    pub Unknown8: u8,
    ///""
    pub Unknown9: u8,
    ///""
    pub Unknown10: u8,
}
