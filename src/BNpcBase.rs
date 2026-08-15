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
pub struct BNpcBaseSheet {
    sheet: Sheet,
}
impl BNpcBaseSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BNpcBase")?;
        let sheet = resolver.read_excel_sheet(&exh, "BNpcBase", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BNpcBaseRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BNpcBaseRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for BNpcBaseSheet {
    type Row = BNpcBaseRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Scale: row
                .columns[4]
                .into_f32()
                .copied()
                .expect("Expected column 4 to be a float32!"),
            Unknown11: row
                .columns[26]
                .into_u32()
                .copied()
                .expect("Expected column 26 to be a uint32!"),
            ArrayEventHandler: row
                .columns[11]
                .into_i32()
                .copied()
                .expect("Expected column 11 to be a int32!"),
            Behavior: row
                .columns[0]
                .into_u16()
                .copied()
                .expect("Expected column 0 to be a uint16!"),
            ModelChara: row
                .columns[5]
                .into_u16()
                .copied()
                .expect("Expected column 5 to be a uint16!"),
            BNpcCustomize: row
                .columns[6]
                .into_u16()
                .copied()
                .expect("Expected column 6 to be a uint16!"),
            NpcEquip: row
                .columns[7]
                .into_u16()
                .copied()
                .expect("Expected column 7 to be a uint16!"),
            Special: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            Unknown9: row
                .columns[22]
                .into_u16()
                .copied()
                .expect("Expected column 22 to be a uint16!"),
            Battalion: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            LinkRace: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            Rank: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
            SEPack: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
            Unknown0: row
                .columns[12]
                .into_u8()
                .copied()
                .expect("Expected column 12 to be a uint8!"),
            BNpcParts: row
                .columns[13]
                .into_u8()
                .copied()
                .expect("Expected column 13 to be a uint8!"),
            Unknown1: row
                .columns[14]
                .into_u8()
                .copied()
                .expect("Expected column 14 to be a uint8!"),
            Unknown2: row
                .columns[19]
                .into_u8()
                .copied()
                .expect("Expected column 19 to be a uint8!"),
            Unknown3: row
                .columns[20]
                .into_u8()
                .copied()
                .expect("Expected column 20 to be a uint8!"),
            Unknown10: row
                .columns[21]
                .into_u8()
                .copied()
                .expect("Expected column 21 to be a uint8!"),
            Unknown4: row
                .columns[23]
                .into_u8()
                .copied()
                .expect("Expected column 23 to be a uint8!"),
            IsOmnidirectional: row
                .columns[10]
                .into_bool()
                .copied()
                .expect("Expected column 10 to be a bool!"),
            Unknown6: row
                .columns[15]
                .into_bool()
                .copied()
                .expect("Expected column 15 to be a bool!"),
            IsTargetLine: row
                .columns[16]
                .into_bool()
                .copied()
                .expect("Expected column 16 to be a bool!"),
            IsDisplayLevel: row
                .columns[17]
                .into_bool()
                .copied()
                .expect("Expected column 17 to be a bool!"),
            Unknown7: row
                .columns[18]
                .into_bool()
                .copied()
                .expect("Expected column 18 to be a bool!"),
            Unknown_70: row
                .columns[24]
                .into_bool()
                .copied()
                .expect("Expected column 24 to be a bool!"),
            Unknown8: row
                .columns[25]
                .into_bool()
                .copied()
                .expect("Expected column 25 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a BNpcBaseSheet {
    type Item = (u32, Vec<(u16, BNpcBaseRow)>);
    type IntoIter = StructuredSheetIterator<'a, BNpcBaseSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BNpcBaseSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BNpcBaseRow {
    ///""
    pub Scale: f32,
    ///""
    pub Unknown11: u32,
    ///""
    pub ArrayEventHandler: i32,
    ///""
    pub Behavior: u16,
    ///""
    pub ModelChara: u16,
    ///""
    pub BNpcCustomize: u16,
    ///""
    pub NpcEquip: u16,
    ///""
    pub Special: u16,
    ///""
    pub Unknown9: u16,
    ///""
    pub Battalion: u8,
    ///""
    pub LinkRace: u8,
    ///""
    pub Rank: u8,
    ///""
    pub SEPack: u8,
    ///""
    pub Unknown0: u8,
    ///""
    pub BNpcParts: u8,
    ///""
    pub Unknown1: u8,
    ///""
    pub Unknown2: u8,
    ///""
    pub Unknown3: u8,
    ///""
    pub Unknown10: u8,
    ///""
    pub Unknown4: u8,
    ///""
    pub IsOmnidirectional: bool,
    ///""
    pub Unknown6: bool,
    ///""
    pub IsTargetLine: bool,
    ///""
    pub IsDisplayLevel: bool,
    ///""
    pub Unknown7: bool,
    ///""
    pub Unknown_70: bool,
    ///""
    pub Unknown8: bool,
}
