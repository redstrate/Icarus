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
pub struct MandervilleWeaponEnhanceSheet {
    sheet: Sheet,
}
impl MandervilleWeaponEnhanceSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 39920u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MandervilleWeaponEnhance")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "MandervilleWeaponEnhance", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<MandervilleWeaponEnhanceRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<MandervilleWeaponEnhanceRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for MandervilleWeaponEnhanceSheet {
    type Row = MandervilleWeaponEnhanceRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Materia: [
                row
                    .columns[1]
                    .into_u32()
                    .copied()
                    .expect("Expected column 1 to be a uint32!"),
                row
                    .columns[4]
                    .into_u32()
                    .copied()
                    .expect("Expected column 4 to be a uint32!"),
                row
                    .columns[7]
                    .into_u32()
                    .copied()
                    .expect("Expected column 7 to be a uint32!"),
                row
                    .columns[10]
                    .into_u32()
                    .copied()
                    .expect("Expected column 10 to be a uint32!"),
            ],
            UpgradeItem: row
                .columns[13]
                .into_u32()
                .copied()
                .expect("Expected column 13 to be a uint32!"),
            Image: row
                .columns[15]
                .into_u32()
                .copied()
                .expect("Expected column 15 to be a uint32!"),
            UpgradeCost: row
                .columns[14]
                .into_u16()
                .copied()
                .expect("Expected column 14 to be a uint16!"),
            Unknown7: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
            MateriaBigValueIndex: [
                row
                    .columns[2]
                    .into_u8()
                    .copied()
                    .expect("Expected column 2 to be a uint8!"),
                row
                    .columns[5]
                    .into_u8()
                    .copied()
                    .expect("Expected column 5 to be a uint8!"),
                row
                    .columns[8]
                    .into_u8()
                    .copied()
                    .expect("Expected column 8 to be a uint8!"),
                row
                    .columns[11]
                    .into_u8()
                    .copied()
                    .expect("Expected column 11 to be a uint8!"),
            ],
            MateriaSmallValueIndex: [
                row
                    .columns[3]
                    .into_u8()
                    .copied()
                    .expect("Expected column 3 to be a uint8!"),
                row
                    .columns[6]
                    .into_u8()
                    .copied()
                    .expect("Expected column 6 to be a uint8!"),
                row
                    .columns[9]
                    .into_u8()
                    .copied()
                    .expect("Expected column 9 to be a uint8!"),
                row
                    .columns[12]
                    .into_u8()
                    .copied()
                    .expect("Expected column 12 to be a uint8!"),
            ],
        })
    }
}
impl<'a> IntoIterator for &'a MandervilleWeaponEnhanceSheet {
    type Item = (u32, Vec<(u16, MandervilleWeaponEnhanceRow)>);
    type IntoIter = StructuredSheetIterator<'a, MandervilleWeaponEnhanceSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MandervilleWeaponEnhanceSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MandervilleWeaponEnhanceRow {
    ///""
    pub Materia: [u32; 4],
    ///""
    pub UpgradeItem: u32,
    ///""
    pub Image: u32,
    ///""
    pub UpgradeCost: u16,
    ///""
    pub Unknown7: u8,
    ///""
    pub MateriaBigValueIndex: [u8; 4],
    ///""
    pub MateriaSmallValueIndex: [u8; 4],
}
