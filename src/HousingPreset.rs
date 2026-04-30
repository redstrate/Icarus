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
pub struct HousingPresetSheet {
    sheet: Sheet,
}
impl HousingPresetSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("HousingPreset")?;
        let sheet = resolver.read_excel_sheet(&exh, "HousingPreset", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<HousingPresetRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<HousingPresetRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for HousingPresetSheet {
    type Row = HousingPresetRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Singular: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Plural: row
                .columns[2]
                .into_string()
                .cloned()
                .expect("Expected column 2 to be a string!"),
            Adjective: row
                .columns[1]
                .into_i8()
                .copied()
                .expect("Expected column 1 to be a int8!"),
            PossessivePronoun: row
                .columns[3]
                .into_i8()
                .copied()
                .expect("Expected column 3 to be a int8!"),
            StartsWithVowel: row
                .columns[4]
                .into_i8()
                .copied()
                .expect("Expected column 4 to be a int8!"),
            Unknown0: row
                .columns[5]
                .into_i8()
                .copied()
                .expect("Expected column 5 to be a int8!"),
            Pronoun: row
                .columns[6]
                .into_i8()
                .copied()
                .expect("Expected column 6 to be a int8!"),
            Article: row
                .columns[7]
                .into_i8()
                .copied()
                .expect("Expected column 7 to be a int8!"),
            ExteriorRoof: row
                .columns[10]
                .into_i32()
                .copied()
                .expect("Expected column 10 to be a int32!"),
            ExteriorWall: row
                .columns[11]
                .into_i32()
                .copied()
                .expect("Expected column 11 to be a int32!"),
            ExteriorWindow: row
                .columns[12]
                .into_i32()
                .copied()
                .expect("Expected column 12 to be a int32!"),
            ExteriorDoor: row
                .columns[13]
                .into_i32()
                .copied()
                .expect("Expected column 13 to be a int32!"),
            InteriorWall: row
                .columns[14]
                .into_i32()
                .copied()
                .expect("Expected column 14 to be a int32!"),
            InteriorFlooring: row
                .columns[15]
                .into_i32()
                .copied()
                .expect("Expected column 15 to be a int32!"),
            InteriorLighting: row
                .columns[16]
                .into_i32()
                .copied()
                .expect("Expected column 16 to be a int32!"),
            OtherFloorWall: row
                .columns[17]
                .into_i32()
                .copied()
                .expect("Expected column 17 to be a int32!"),
            OtherFloorFlooring: row
                .columns[18]
                .into_i32()
                .copied()
                .expect("Expected column 18 to be a int32!"),
            OtherFloorLighting: row
                .columns[19]
                .into_i32()
                .copied()
                .expect("Expected column 19 to be a int32!"),
            BasementWall: row
                .columns[20]
                .into_i32()
                .copied()
                .expect("Expected column 20 to be a int32!"),
            BasementFlooring: row
                .columns[21]
                .into_i32()
                .copied()
                .expect("Expected column 21 to be a int32!"),
            BasementLighting: row
                .columns[22]
                .into_i32()
                .copied()
                .expect("Expected column 22 to be a int32!"),
            MansionLighting: row
                .columns[23]
                .into_i32()
                .copied()
                .expect("Expected column 23 to be a int32!"),
            PlaceName: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            HousingSize: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a HousingPresetSheet {
    type Item = (u32, Vec<(u16, HousingPresetRow)>);
    type IntoIter = StructuredSheetIterator<'a, HousingPresetSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, HousingPresetSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct HousingPresetRow {
    ///""
    pub Singular: String,
    ///""
    pub Plural: String,
    ///""
    pub Adjective: i8,
    ///""
    pub PossessivePronoun: i8,
    ///""
    pub StartsWithVowel: i8,
    ///""
    pub Unknown0: i8,
    ///""
    pub Pronoun: i8,
    ///""
    pub Article: i8,
    ///""
    pub ExteriorRoof: i32,
    ///""
    pub ExteriorWall: i32,
    ///""
    pub ExteriorWindow: i32,
    ///""
    pub ExteriorDoor: i32,
    ///""
    pub InteriorWall: i32,
    ///""
    pub InteriorFlooring: i32,
    ///""
    pub InteriorLighting: i32,
    ///""
    pub OtherFloorWall: i32,
    ///""
    pub OtherFloorFlooring: i32,
    ///""
    pub OtherFloorLighting: i32,
    ///""
    pub BasementWall: i32,
    ///""
    pub BasementFlooring: i32,
    ///""
    pub BasementLighting: i32,
    ///""
    pub MansionLighting: i32,
    ///""
    pub PlaceName: u16,
    ///""
    pub HousingSize: u8,
}
