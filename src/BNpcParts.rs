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
pub struct BNpcPartsSheet {
    sheet: Sheet,
}
impl BNpcPartsSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BNpcParts")?;
        let sheet = resolver.read_excel_sheet(&exh, "BNpcParts", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BNpcPartsRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BNpcPartsRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for BNpcPartsSheet {
    type Row = BNpcPartsRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            X1: row
                .columns[7]
                .into_f32()
                .copied()
                .expect("Expected column 7 to be a float32!"),
            X2: row
                .columns[18]
                .into_f32()
                .copied()
                .expect("Expected column 18 to be a float32!"),
            X3: row
                .columns[29]
                .into_f32()
                .copied()
                .expect("Expected column 29 to be a float32!"),
            X4: row
                .columns[40]
                .into_f32()
                .copied()
                .expect("Expected column 40 to be a float32!"),
            X5: row
                .columns[51]
                .into_f32()
                .copied()
                .expect("Expected column 51 to be a float32!"),
            Unknown0: row
                .columns[62]
                .into_f32()
                .copied()
                .expect("Expected column 62 to be a float32!"),
            Y1: row
                .columns[8]
                .into_f32()
                .copied()
                .expect("Expected column 8 to be a float32!"),
            Y2: row
                .columns[19]
                .into_f32()
                .copied()
                .expect("Expected column 19 to be a float32!"),
            Y3: row
                .columns[30]
                .into_f32()
                .copied()
                .expect("Expected column 30 to be a float32!"),
            Y4: row
                .columns[41]
                .into_f32()
                .copied()
                .expect("Expected column 41 to be a float32!"),
            Y5: row
                .columns[52]
                .into_f32()
                .copied()
                .expect("Expected column 52 to be a float32!"),
            Unknown1: row
                .columns[63]
                .into_f32()
                .copied()
                .expect("Expected column 63 to be a float32!"),
            Z1: row
                .columns[9]
                .into_f32()
                .copied()
                .expect("Expected column 9 to be a float32!"),
            Z2: row
                .columns[20]
                .into_f32()
                .copied()
                .expect("Expected column 20 to be a float32!"),
            Z3: row
                .columns[31]
                .into_f32()
                .copied()
                .expect("Expected column 31 to be a float32!"),
            Z4: row
                .columns[42]
                .into_f32()
                .copied()
                .expect("Expected column 42 to be a float32!"),
            Z5: row
                .columns[53]
                .into_f32()
                .copied()
                .expect("Expected column 53 to be a float32!"),
            Unknown2: row
                .columns[64]
                .into_f32()
                .copied()
                .expect("Expected column 64 to be a float32!"),
            Scale1: row
                .columns[11]
                .into_f32()
                .copied()
                .expect("Expected column 11 to be a float32!"),
            Scale2: row
                .columns[22]
                .into_f32()
                .copied()
                .expect("Expected column 22 to be a float32!"),
            Unknown3: row
                .columns[33]
                .into_f32()
                .copied()
                .expect("Expected column 33 to be a float32!"),
            Scale4: row
                .columns[44]
                .into_f32()
                .copied()
                .expect("Expected column 44 to be a float32!"),
            Scale5: row
                .columns[55]
                .into_f32()
                .copied()
                .expect("Expected column 55 to be a float32!"),
            Unknown4: row
                .columns[66]
                .into_f32()
                .copied()
                .expect("Expected column 66 to be a float32!"),
            BNpcBase1: row
                .columns[1]
                .into_u16()
                .copied()
                .expect("Expected column 1 to be a uint16!"),
            BNpcBase2: row
                .columns[12]
                .into_u16()
                .copied()
                .expect("Expected column 12 to be a uint16!"),
            BNpcBase3: row
                .columns[23]
                .into_u16()
                .copied()
                .expect("Expected column 23 to be a uint16!"),
            BNpcBase4: row
                .columns[34]
                .into_u16()
                .copied()
                .expect("Expected column 34 to be a uint16!"),
            BNpcBase5: row
                .columns[45]
                .into_u16()
                .copied()
                .expect("Expected column 45 to be a uint16!"),
            Unknown5: row
                .columns[56]
                .into_u16()
                .copied()
                .expect("Expected column 56 to be a uint16!"),
            Unknown6: row
                .columns[10]
                .into_i16()
                .copied()
                .expect("Expected column 10 to be a int16!"),
            Unknown7: row
                .columns[21]
                .into_i16()
                .copied()
                .expect("Expected column 21 to be a int16!"),
            Scale3: row
                .columns[32]
                .into_i16()
                .copied()
                .expect("Expected column 32 to be a int16!"),
            Unknown8: row
                .columns[43]
                .into_i16()
                .copied()
                .expect("Expected column 43 to be a int16!"),
            Unknown9: row
                .columns[54]
                .into_i16()
                .copied()
                .expect("Expected column 54 to be a int16!"),
            Unknown10: row
                .columns[65]
                .into_i16()
                .copied()
                .expect("Expected column 65 to be a int16!"),
            PartSlot1: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            PartSlot2: row
                .columns[13]
                .into_u8()
                .copied()
                .expect("Expected column 13 to be a uint8!"),
            PartSlot3: row
                .columns[24]
                .into_u8()
                .copied()
                .expect("Expected column 24 to be a uint8!"),
            PartSlot4: row
                .columns[35]
                .into_u8()
                .copied()
                .expect("Expected column 35 to be a uint8!"),
            PartSlot5: row
                .columns[46]
                .into_u8()
                .copied()
                .expect("Expected column 46 to be a uint8!"),
            Unknown11: row
                .columns[57]
                .into_u8()
                .copied()
                .expect("Expected column 57 to be a uint8!"),
            Unknown12: row
                .columns[3]
                .into_bool()
                .copied()
                .expect("Expected column 3 to be a bool!"),
            Unknown13: row
                .columns[14]
                .into_bool()
                .copied()
                .expect("Expected column 14 to be a bool!"),
            Unknown14: row
                .columns[25]
                .into_bool()
                .copied()
                .expect("Expected column 25 to be a bool!"),
            Unknown15: row
                .columns[36]
                .into_bool()
                .copied()
                .expect("Expected column 36 to be a bool!"),
            Unknown16: row
                .columns[47]
                .into_bool()
                .copied()
                .expect("Expected column 47 to be a bool!"),
            Unknown17: row
                .columns[58]
                .into_bool()
                .copied()
                .expect("Expected column 58 to be a bool!"),
            Unknown18: row
                .columns[4]
                .into_bool()
                .copied()
                .expect("Expected column 4 to be a bool!"),
            Unknown19: row
                .columns[15]
                .into_bool()
                .copied()
                .expect("Expected column 15 to be a bool!"),
            Unknown20: row
                .columns[26]
                .into_bool()
                .copied()
                .expect("Expected column 26 to be a bool!"),
            Unknown21: row
                .columns[37]
                .into_bool()
                .copied()
                .expect("Expected column 37 to be a bool!"),
            Unknown22: row
                .columns[48]
                .into_bool()
                .copied()
                .expect("Expected column 48 to be a bool!"),
            Unknown23: row
                .columns[59]
                .into_bool()
                .copied()
                .expect("Expected column 59 to be a bool!"),
            Unknown24: row
                .columns[5]
                .into_bool()
                .copied()
                .expect("Expected column 5 to be a bool!"),
            Unknown25: row
                .columns[16]
                .into_bool()
                .copied()
                .expect("Expected column 16 to be a bool!"),
            Unknown26: row
                .columns[27]
                .into_bool()
                .copied()
                .expect("Expected column 27 to be a bool!"),
            Unknown27: row
                .columns[38]
                .into_bool()
                .copied()
                .expect("Expected column 38 to be a bool!"),
            Unknown28: row
                .columns[49]
                .into_bool()
                .copied()
                .expect("Expected column 49 to be a bool!"),
            Unknown29: row
                .columns[60]
                .into_bool()
                .copied()
                .expect("Expected column 60 to be a bool!"),
            Unknown30: row
                .columns[6]
                .into_bool()
                .copied()
                .expect("Expected column 6 to be a bool!"),
            Unknown31: row
                .columns[17]
                .into_bool()
                .copied()
                .expect("Expected column 17 to be a bool!"),
            Unknown32: row
                .columns[28]
                .into_bool()
                .copied()
                .expect("Expected column 28 to be a bool!"),
            Unknown33: row
                .columns[39]
                .into_bool()
                .copied()
                .expect("Expected column 39 to be a bool!"),
            Unknown34: row
                .columns[50]
                .into_bool()
                .copied()
                .expect("Expected column 50 to be a bool!"),
            Unknown35: row
                .columns[61]
                .into_bool()
                .copied()
                .expect("Expected column 61 to be a bool!"),
            Unknown36: row
                .columns[0]
                .into_bool()
                .copied()
                .expect("Expected column 0 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a BNpcPartsSheet {
    type Item = (u32, Vec<(u16, BNpcPartsRow)>);
    type IntoIter = StructuredSheetIterator<'a, BNpcPartsSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BNpcPartsSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BNpcPartsRow {
    ///""
    pub X1: f32,
    ///""
    pub X2: f32,
    ///""
    pub X3: f32,
    ///""
    pub X4: f32,
    ///""
    pub X5: f32,
    ///""
    pub Unknown0: f32,
    ///""
    pub Y1: f32,
    ///""
    pub Y2: f32,
    ///""
    pub Y3: f32,
    ///""
    pub Y4: f32,
    ///""
    pub Y5: f32,
    ///""
    pub Unknown1: f32,
    ///""
    pub Z1: f32,
    ///""
    pub Z2: f32,
    ///""
    pub Z3: f32,
    ///""
    pub Z4: f32,
    ///""
    pub Z5: f32,
    ///""
    pub Unknown2: f32,
    ///""
    pub Scale1: f32,
    ///""
    pub Scale2: f32,
    ///""
    pub Unknown3: f32,
    ///""
    pub Scale4: f32,
    ///""
    pub Scale5: f32,
    ///""
    pub Unknown4: f32,
    ///""
    pub BNpcBase1: u16,
    ///""
    pub BNpcBase2: u16,
    ///""
    pub BNpcBase3: u16,
    ///""
    pub BNpcBase4: u16,
    ///""
    pub BNpcBase5: u16,
    ///""
    pub Unknown5: u16,
    ///""
    pub Unknown6: i16,
    ///""
    pub Unknown7: i16,
    ///""
    pub Scale3: i16,
    ///""
    pub Unknown8: i16,
    ///""
    pub Unknown9: i16,
    ///""
    pub Unknown10: i16,
    ///""
    pub PartSlot1: u8,
    ///""
    pub PartSlot2: u8,
    ///""
    pub PartSlot3: u8,
    ///""
    pub PartSlot4: u8,
    ///""
    pub PartSlot5: u8,
    ///""
    pub Unknown11: u8,
    ///""
    pub Unknown12: bool,
    ///""
    pub Unknown13: bool,
    ///""
    pub Unknown14: bool,
    ///""
    pub Unknown15: bool,
    ///""
    pub Unknown16: bool,
    ///""
    pub Unknown17: bool,
    ///""
    pub Unknown18: bool,
    ///""
    pub Unknown19: bool,
    ///""
    pub Unknown20: bool,
    ///""
    pub Unknown21: bool,
    ///""
    pub Unknown22: bool,
    ///""
    pub Unknown23: bool,
    ///""
    pub Unknown24: bool,
    ///""
    pub Unknown25: bool,
    ///""
    pub Unknown26: bool,
    ///""
    pub Unknown27: bool,
    ///""
    pub Unknown28: bool,
    ///""
    pub Unknown29: bool,
    ///""
    pub Unknown30: bool,
    ///""
    pub Unknown31: bool,
    ///""
    pub Unknown32: bool,
    ///""
    pub Unknown33: bool,
    ///""
    pub Unknown34: bool,
    ///""
    pub Unknown35: bool,
    ///""
    pub Unknown36: bool,
}
