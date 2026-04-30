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
pub struct ENpcDressUpDressSheet {
    sheet: Sheet,
}
impl ENpcDressUpDressSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ENpcDressUpDress")?;
        let sheet = resolver.read_excel_sheet(&exh, "ENpcDressUpDress", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ENpcDressUpDressRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ENpcDressUpDressRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for ENpcDressUpDressSheet {
    type Row = ENpcDressUpDressRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            ModelMainHand: row
                .columns[37]
                .into_u64()
                .copied()
                .expect("Expected column 37 to be a uint64!"),
            ModelOffHand: row
                .columns[40]
                .into_u64()
                .copied()
                .expect("Expected column 40 to be a uint64!"),
            Unknown0: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
            ENpc: row
                .columns[7]
                .into_u32()
                .copied()
                .expect("Expected column 7 to be a uint32!"),
            ModelHead: row
                .columns[43]
                .into_u32()
                .copied()
                .expect("Expected column 43 to be a uint32!"),
            ModelBody: row
                .columns[46]
                .into_u32()
                .copied()
                .expect("Expected column 46 to be a uint32!"),
            ModelHands: row
                .columns[49]
                .into_u32()
                .copied()
                .expect("Expected column 49 to be a uint32!"),
            ModelLegs: row
                .columns[52]
                .into_u32()
                .copied()
                .expect("Expected column 52 to be a uint32!"),
            ModelFeet: row
                .columns[55]
                .into_u32()
                .copied()
                .expect("Expected column 55 to be a uint32!"),
            Unknown1: row
                .columns[58]
                .into_u32()
                .copied()
                .expect("Expected column 58 to be a uint32!"),
            Unknown2: row
                .columns[61]
                .into_u32()
                .copied()
                .expect("Expected column 61 to be a uint32!"),
            Unknown3: row
                .columns[64]
                .into_u32()
                .copied()
                .expect("Expected column 64 to be a uint32!"),
            Unknown4: row
                .columns[67]
                .into_u32()
                .copied()
                .expect("Expected column 67 to be a uint32!"),
            Unknown5: row
                .columns[70]
                .into_u32()
                .copied()
                .expect("Expected column 70 to be a uint32!"),
            Unknown6: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            Behavior: row
                .columns[9]
                .into_u16()
                .copied()
                .expect("Expected column 9 to be a uint16!"),
            Unknown7: row
                .columns[10]
                .into_u16()
                .copied()
                .expect("Expected column 10 to be a uint16!"),
            Unknown8: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
            Unknown9: row
                .columns[11]
                .into_u8()
                .copied()
                .expect("Expected column 11 to be a uint8!"),
            Unknown10: row
                .columns[12]
                .into_u8()
                .copied()
                .expect("Expected column 12 to be a uint8!"),
            Unknown11: row
                .columns[13]
                .into_u8()
                .copied()
                .expect("Expected column 13 to be a uint8!"),
            Unknown12: row
                .columns[14]
                .into_u8()
                .copied()
                .expect("Expected column 14 to be a uint8!"),
            Unknown13: row
                .columns[15]
                .into_u8()
                .copied()
                .expect("Expected column 15 to be a uint8!"),
            Unknown14: row
                .columns[16]
                .into_u8()
                .copied()
                .expect("Expected column 16 to be a uint8!"),
            Unknown15: row
                .columns[17]
                .into_u8()
                .copied()
                .expect("Expected column 17 to be a uint8!"),
            Unknown16: row
                .columns[18]
                .into_u8()
                .copied()
                .expect("Expected column 18 to be a uint8!"),
            Unknown17: row
                .columns[19]
                .into_u8()
                .copied()
                .expect("Expected column 19 to be a uint8!"),
            Unknown18: row
                .columns[20]
                .into_u8()
                .copied()
                .expect("Expected column 20 to be a uint8!"),
            Unknown19: row
                .columns[21]
                .into_u8()
                .copied()
                .expect("Expected column 21 to be a uint8!"),
            Unknown20: row
                .columns[22]
                .into_u8()
                .copied()
                .expect("Expected column 22 to be a uint8!"),
            Unknown21: row
                .columns[23]
                .into_u8()
                .copied()
                .expect("Expected column 23 to be a uint8!"),
            Unknown22: row
                .columns[24]
                .into_u8()
                .copied()
                .expect("Expected column 24 to be a uint8!"),
            Unknown23: row
                .columns[25]
                .into_u8()
                .copied()
                .expect("Expected column 25 to be a uint8!"),
            Unknown24: row
                .columns[26]
                .into_u8()
                .copied()
                .expect("Expected column 26 to be a uint8!"),
            Unknown25: row
                .columns[27]
                .into_u8()
                .copied()
                .expect("Expected column 27 to be a uint8!"),
            Unknown26: row
                .columns[28]
                .into_u8()
                .copied()
                .expect("Expected column 28 to be a uint8!"),
            Unknown27: row
                .columns[29]
                .into_u8()
                .copied()
                .expect("Expected column 29 to be a uint8!"),
            Unknown28: row
                .columns[30]
                .into_u8()
                .copied()
                .expect("Expected column 30 to be a uint8!"),
            Unknown29: row
                .columns[31]
                .into_u8()
                .copied()
                .expect("Expected column 31 to be a uint8!"),
            Unknown30: row
                .columns[32]
                .into_u8()
                .copied()
                .expect("Expected column 32 to be a uint8!"),
            Unknown31: row
                .columns[33]
                .into_u8()
                .copied()
                .expect("Expected column 33 to be a uint8!"),
            Unknown32: row
                .columns[34]
                .into_u8()
                .copied()
                .expect("Expected column 34 to be a uint8!"),
            Unknown33: row
                .columns[35]
                .into_u8()
                .copied()
                .expect("Expected column 35 to be a uint8!"),
            Unknown34: row
                .columns[36]
                .into_u8()
                .copied()
                .expect("Expected column 36 to be a uint8!"),
            DyeMainHand: row
                .columns[38]
                .into_u8()
                .copied()
                .expect("Expected column 38 to be a uint8!"),
            Dye2MainHand: row
                .columns[39]
                .into_u8()
                .copied()
                .expect("Expected column 39 to be a uint8!"),
            DyeOffHand: row
                .columns[41]
                .into_u8()
                .copied()
                .expect("Expected column 41 to be a uint8!"),
            Dye2OffHand: row
                .columns[42]
                .into_u8()
                .copied()
                .expect("Expected column 42 to be a uint8!"),
            DyeHead: row
                .columns[44]
                .into_u8()
                .copied()
                .expect("Expected column 44 to be a uint8!"),
            DyeBody: row
                .columns[47]
                .into_u8()
                .copied()
                .expect("Expected column 47 to be a uint8!"),
            DyeHands: row
                .columns[50]
                .into_u8()
                .copied()
                .expect("Expected column 50 to be a uint8!"),
            DyeLegs: row
                .columns[53]
                .into_u8()
                .copied()
                .expect("Expected column 53 to be a uint8!"),
            DyeFeet: row
                .columns[56]
                .into_u8()
                .copied()
                .expect("Expected column 56 to be a uint8!"),
            DyeEars: row
                .columns[59]
                .into_u8()
                .copied()
                .expect("Expected column 59 to be a uint8!"),
            DyeNeck: row
                .columns[62]
                .into_u8()
                .copied()
                .expect("Expected column 62 to be a uint8!"),
            DyeWrists: row
                .columns[65]
                .into_u8()
                .copied()
                .expect("Expected column 65 to be a uint8!"),
            DyeLeftRing: row
                .columns[68]
                .into_u8()
                .copied()
                .expect("Expected column 68 to be a uint8!"),
            DyeRightRing: row
                .columns[71]
                .into_u8()
                .copied()
                .expect("Expected column 71 to be a uint8!"),
            Dye2Head: row
                .columns[45]
                .into_u8()
                .copied()
                .expect("Expected column 45 to be a uint8!"),
            Dye2Body: row
                .columns[48]
                .into_u8()
                .copied()
                .expect("Expected column 48 to be a uint8!"),
            Dye2Hands: row
                .columns[51]
                .into_u8()
                .copied()
                .expect("Expected column 51 to be a uint8!"),
            Dye2Legs: row
                .columns[54]
                .into_u8()
                .copied()
                .expect("Expected column 54 to be a uint8!"),
            Dye2Feet: row
                .columns[57]
                .into_u8()
                .copied()
                .expect("Expected column 57 to be a uint8!"),
            Dye2Ears: row
                .columns[60]
                .into_u8()
                .copied()
                .expect("Expected column 60 to be a uint8!"),
            Dye2Neck: row
                .columns[63]
                .into_u8()
                .copied()
                .expect("Expected column 63 to be a uint8!"),
            Dye2Wrists: row
                .columns[66]
                .into_u8()
                .copied()
                .expect("Expected column 66 to be a uint8!"),
            Dye2LeftRing: row
                .columns[69]
                .into_u8()
                .copied()
                .expect("Expected column 69 to be a uint8!"),
            Dye2RightRing: row
                .columns[72]
                .into_u8()
                .copied()
                .expect("Expected column 72 to be a uint8!"),
            Unknown40: row
                .columns[1]
                .into_bool()
                .copied()
                .expect("Expected column 1 to be a bool!"),
            Unknown41: row
                .columns[2]
                .into_bool()
                .copied()
                .expect("Expected column 2 to be a bool!"),
            Unknown42: row
                .columns[3]
                .into_bool()
                .copied()
                .expect("Expected column 3 to be a bool!"),
            Unknown43: row
                .columns[4]
                .into_bool()
                .copied()
                .expect("Expected column 4 to be a bool!"),
            Unknown44: row
                .columns[5]
                .into_bool()
                .copied()
                .expect("Expected column 5 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a ENpcDressUpDressSheet {
    type Item = (u32, Vec<(u16, ENpcDressUpDressRow)>);
    type IntoIter = StructuredSheetIterator<'a, ENpcDressUpDressSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ENpcDressUpDressSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ENpcDressUpDressRow {
    ///""
    pub ModelMainHand: u64,
    ///""
    pub ModelOffHand: u64,
    ///""
    pub Unknown0: u32,
    ///""
    pub ENpc: u32,
    ///""
    pub ModelHead: u32,
    ///""
    pub ModelBody: u32,
    ///""
    pub ModelHands: u32,
    ///""
    pub ModelLegs: u32,
    ///""
    pub ModelFeet: u32,
    ///""
    pub Unknown1: u32,
    ///""
    pub Unknown2: u32,
    ///""
    pub Unknown3: u32,
    ///""
    pub Unknown4: u32,
    ///""
    pub Unknown5: u32,
    ///""
    pub Unknown6: u16,
    ///""
    pub Behavior: u16,
    ///""
    pub Unknown7: u16,
    ///""
    pub Unknown8: u8,
    ///""
    pub Unknown9: u8,
    ///""
    pub Unknown10: u8,
    ///""
    pub Unknown11: u8,
    ///""
    pub Unknown12: u8,
    ///""
    pub Unknown13: u8,
    ///""
    pub Unknown14: u8,
    ///""
    pub Unknown15: u8,
    ///""
    pub Unknown16: u8,
    ///""
    pub Unknown17: u8,
    ///""
    pub Unknown18: u8,
    ///""
    pub Unknown19: u8,
    ///""
    pub Unknown20: u8,
    ///""
    pub Unknown21: u8,
    ///""
    pub Unknown22: u8,
    ///""
    pub Unknown23: u8,
    ///""
    pub Unknown24: u8,
    ///""
    pub Unknown25: u8,
    ///""
    pub Unknown26: u8,
    ///""
    pub Unknown27: u8,
    ///""
    pub Unknown28: u8,
    ///""
    pub Unknown29: u8,
    ///""
    pub Unknown30: u8,
    ///""
    pub Unknown31: u8,
    ///""
    pub Unknown32: u8,
    ///""
    pub Unknown33: u8,
    ///""
    pub Unknown34: u8,
    ///""
    pub DyeMainHand: u8,
    ///""
    pub Dye2MainHand: u8,
    ///""
    pub DyeOffHand: u8,
    ///""
    pub Dye2OffHand: u8,
    ///""
    pub DyeHead: u8,
    ///""
    pub DyeBody: u8,
    ///""
    pub DyeHands: u8,
    ///""
    pub DyeLegs: u8,
    ///""
    pub DyeFeet: u8,
    ///""
    pub DyeEars: u8,
    ///""
    pub DyeNeck: u8,
    ///""
    pub DyeWrists: u8,
    ///""
    pub DyeLeftRing: u8,
    ///""
    pub DyeRightRing: u8,
    ///""
    pub Dye2Head: u8,
    ///""
    pub Dye2Body: u8,
    ///""
    pub Dye2Hands: u8,
    ///""
    pub Dye2Legs: u8,
    ///""
    pub Dye2Feet: u8,
    ///""
    pub Dye2Ears: u8,
    ///""
    pub Dye2Neck: u8,
    ///""
    pub Dye2Wrists: u8,
    ///""
    pub Dye2LeftRing: u8,
    ///""
    pub Dye2RightRing: u8,
    ///""
    pub Unknown40: bool,
    ///""
    pub Unknown41: bool,
    ///""
    pub Unknown42: bool,
    ///""
    pub Unknown43: bool,
    ///""
    pub Unknown44: bool,
}
