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
pub struct PetSheet {
    sheet: Sheet,
}
impl PetSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Pet")?;
        let sheet = resolver.read_excel_sheet(&exh, "Pet", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<PetRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<PetRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for PetSheet {
    type Row = PetRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Abilities: [
                row
                    .columns[1]
                    .into_u16()
                    .copied()
                    .expect("Expected column 1 to be a uint16!"),
                row
                    .columns[2]
                    .into_u16()
                    .copied()
                    .expect("Expected column 2 to be a uint16!"),
                row
                    .columns[3]
                    .into_u16()
                    .copied()
                    .expect("Expected column 3 to be a uint16!"),
                row
                    .columns[4]
                    .into_u16()
                    .copied()
                    .expect("Expected column 4 to be a uint16!"),
            ],
            AutoAction: row
                .columns[5]
                .into_u16()
                .copied()
                .expect("Expected column 5 to be a uint16!"),
            SmallScalePercentage: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
            MediumScalePercentage: row
                .columns[10]
                .into_u8()
                .copied()
                .expect("Expected column 10 to be a uint8!"),
            LargeScalePercentage: row
                .columns[11]
                .into_u8()
                .copied()
                .expect("Expected column 11 to be a uint8!"),
            AllowedPetMirage: [
                row
                    .columns[13]
                    .into_u8()
                    .copied()
                    .expect("Expected column 13 to be a uint8!"),
                row
                    .columns[14]
                    .into_u8()
                    .copied()
                    .expect("Expected column 14 to be a uint8!"),
                row
                    .columns[15]
                    .into_u8()
                    .copied()
                    .expect("Expected column 15 to be a uint8!"),
                row
                    .columns[16]
                    .into_u8()
                    .copied()
                    .expect("Expected column 16 to be a uint8!"),
                row
                    .columns[17]
                    .into_u8()
                    .copied()
                    .expect("Expected column 17 to be a uint8!"),
                row
                    .columns[18]
                    .into_u8()
                    .copied()
                    .expect("Expected column 18 to be a uint8!"),
            ],
            Unknown18: row
                .columns[19]
                .into_u8()
                .copied()
                .expect("Expected column 19 to be a uint8!"),
            Unknown14: row
                .columns[12]
                .into_i8()
                .copied()
                .expect("Expected column 12 to be a int8!"),
            AutoDespawn: row
                .columns[6]
                .into_bool()
                .copied()
                .expect("Expected column 6 to be a bool!"),
            NonCombatSummon: row
                .columns[7]
                .into_bool()
                .copied()
                .expect("Expected column 7 to be a bool!"),
            ShowPetActionBar: row
                .columns[8]
                .into_bool()
                .copied()
                .expect("Expected column 8 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a PetSheet {
    type Item = (u32, Vec<(u16, PetRow)>);
    type IntoIter = StructuredSheetIterator<'a, PetSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, PetSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PetRow {
    ///""
    pub Name: String,
    ///""
    pub Abilities: [u16; 4],
    ///""
    pub AutoAction: u16,
    ///""
    pub SmallScalePercentage: u8,
    ///""
    pub MediumScalePercentage: u8,
    ///""
    pub LargeScalePercentage: u8,
    ///""
    pub AllowedPetMirage: [u8; 6],
    ///""
    pub Unknown18: u8,
    ///""
    pub Unknown14: i8,
    ///""
    pub AutoDespawn: bool,
    ///""
    pub NonCombatSummon: bool,
    ///""
    pub ShowPetActionBar: bool,
}
