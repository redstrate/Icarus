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
pub struct PetMirageSheet {
    sheet: Sheet,
}
impl PetMirageSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("PetMirage")?;
        let sheet = resolver.read_excel_sheet(&exh, "PetMirage", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<PetMirageRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<PetMirageRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for PetMirageSheet {
    type Row = PetMirageRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[2]
                .into_string()
                .cloned()
                .expect("Expected column 2 to be a string!"),
            Unknown0: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            Unknown1: row
                .columns[33]
                .into_u16()
                .copied()
                .expect("Expected column 33 to be a uint16!"),
            Unknown2: row
                .columns[48]
                .into_u16()
                .copied()
                .expect("Expected column 48 to be a uint16!"),
            Unknown3: row
                .columns[18]
                .into_u8()
                .copied()
                .expect("Expected column 18 to be a uint8!"),
            Unknown4: row
                .columns[4]
                .into_u16()
                .copied()
                .expect("Expected column 4 to be a uint16!"),
            Unknown5: row
                .columns[34]
                .into_u16()
                .copied()
                .expect("Expected column 34 to be a uint16!"),
            Unknown6: row
                .columns[49]
                .into_u16()
                .copied()
                .expect("Expected column 49 to be a uint16!"),
            Unknown7: row
                .columns[19]
                .into_u8()
                .copied()
                .expect("Expected column 19 to be a uint8!"),
            Unknown8: row
                .columns[5]
                .into_u16()
                .copied()
                .expect("Expected column 5 to be a uint16!"),
            Unknown9: row
                .columns[35]
                .into_u16()
                .copied()
                .expect("Expected column 35 to be a uint16!"),
            Unknown10: row
                .columns[50]
                .into_u16()
                .copied()
                .expect("Expected column 50 to be a uint16!"),
            Unknown11: row
                .columns[20]
                .into_u8()
                .copied()
                .expect("Expected column 20 to be a uint8!"),
            Unknown12: row
                .columns[6]
                .into_u16()
                .copied()
                .expect("Expected column 6 to be a uint16!"),
            Unknown13: row
                .columns[36]
                .into_u16()
                .copied()
                .expect("Expected column 36 to be a uint16!"),
            Unknown14: row
                .columns[51]
                .into_u16()
                .copied()
                .expect("Expected column 51 to be a uint16!"),
            Unknown15: row
                .columns[21]
                .into_u8()
                .copied()
                .expect("Expected column 21 to be a uint8!"),
            Unknown16: row
                .columns[7]
                .into_u16()
                .copied()
                .expect("Expected column 7 to be a uint16!"),
            Unknown17: row
                .columns[37]
                .into_u16()
                .copied()
                .expect("Expected column 37 to be a uint16!"),
            Unknown18: row
                .columns[52]
                .into_u16()
                .copied()
                .expect("Expected column 52 to be a uint16!"),
            Unknown19: row
                .columns[22]
                .into_u8()
                .copied()
                .expect("Expected column 22 to be a uint8!"),
            Unknown20: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            Unknown21: row
                .columns[38]
                .into_u16()
                .copied()
                .expect("Expected column 38 to be a uint16!"),
            Unknown22: row
                .columns[53]
                .into_u16()
                .copied()
                .expect("Expected column 53 to be a uint16!"),
            Unknown23: row
                .columns[23]
                .into_u8()
                .copied()
                .expect("Expected column 23 to be a uint8!"),
            Unknown24: row
                .columns[9]
                .into_u16()
                .copied()
                .expect("Expected column 9 to be a uint16!"),
            Unknown25: row
                .columns[39]
                .into_u16()
                .copied()
                .expect("Expected column 39 to be a uint16!"),
            Unknown26: row
                .columns[54]
                .into_u16()
                .copied()
                .expect("Expected column 54 to be a uint16!"),
            Unknown27: row
                .columns[24]
                .into_u8()
                .copied()
                .expect("Expected column 24 to be a uint8!"),
            Unknown28: row
                .columns[10]
                .into_u16()
                .copied()
                .expect("Expected column 10 to be a uint16!"),
            Unknown29: row
                .columns[40]
                .into_u16()
                .copied()
                .expect("Expected column 40 to be a uint16!"),
            Unknown30: row
                .columns[55]
                .into_u16()
                .copied()
                .expect("Expected column 55 to be a uint16!"),
            Unknown31: row
                .columns[25]
                .into_u8()
                .copied()
                .expect("Expected column 25 to be a uint8!"),
            Unknown32: row
                .columns[11]
                .into_u16()
                .copied()
                .expect("Expected column 11 to be a uint16!"),
            Unknown33: row
                .columns[41]
                .into_u16()
                .copied()
                .expect("Expected column 41 to be a uint16!"),
            Unknown34: row
                .columns[56]
                .into_u16()
                .copied()
                .expect("Expected column 56 to be a uint16!"),
            Unknown35: row
                .columns[26]
                .into_u8()
                .copied()
                .expect("Expected column 26 to be a uint8!"),
            Unknown36: row
                .columns[12]
                .into_u16()
                .copied()
                .expect("Expected column 12 to be a uint16!"),
            Unknown37: row
                .columns[42]
                .into_u16()
                .copied()
                .expect("Expected column 42 to be a uint16!"),
            Unknown38: row
                .columns[57]
                .into_u16()
                .copied()
                .expect("Expected column 57 to be a uint16!"),
            Unknown39: row
                .columns[27]
                .into_u8()
                .copied()
                .expect("Expected column 27 to be a uint8!"),
            Unknown40: row
                .columns[13]
                .into_u16()
                .copied()
                .expect("Expected column 13 to be a uint16!"),
            Unknown41: row
                .columns[43]
                .into_u16()
                .copied()
                .expect("Expected column 43 to be a uint16!"),
            Unknown42: row
                .columns[58]
                .into_u16()
                .copied()
                .expect("Expected column 58 to be a uint16!"),
            Unknown43: row
                .columns[28]
                .into_u8()
                .copied()
                .expect("Expected column 28 to be a uint8!"),
            Unknown44: row
                .columns[14]
                .into_u16()
                .copied()
                .expect("Expected column 14 to be a uint16!"),
            Unknown45: row
                .columns[44]
                .into_u16()
                .copied()
                .expect("Expected column 44 to be a uint16!"),
            Unknown46: row
                .columns[59]
                .into_u16()
                .copied()
                .expect("Expected column 59 to be a uint16!"),
            Unknown47: row
                .columns[29]
                .into_u8()
                .copied()
                .expect("Expected column 29 to be a uint8!"),
            Unknown48: row
                .columns[15]
                .into_u16()
                .copied()
                .expect("Expected column 15 to be a uint16!"),
            Unknown49: row
                .columns[45]
                .into_u16()
                .copied()
                .expect("Expected column 45 to be a uint16!"),
            Unknown50: row
                .columns[60]
                .into_u16()
                .copied()
                .expect("Expected column 60 to be a uint16!"),
            Unknown51: row
                .columns[30]
                .into_u8()
                .copied()
                .expect("Expected column 30 to be a uint8!"),
            Unknown52: row
                .columns[16]
                .into_u16()
                .copied()
                .expect("Expected column 16 to be a uint16!"),
            Unknown53: row
                .columns[46]
                .into_u16()
                .copied()
                .expect("Expected column 46 to be a uint16!"),
            Unknown54: row
                .columns[61]
                .into_u16()
                .copied()
                .expect("Expected column 61 to be a uint16!"),
            Unknown55: row
                .columns[31]
                .into_u8()
                .copied()
                .expect("Expected column 31 to be a uint8!"),
            Unknown56: row
                .columns[17]
                .into_u16()
                .copied()
                .expect("Expected column 17 to be a uint16!"),
            Unknown57: row
                .columns[47]
                .into_u16()
                .copied()
                .expect("Expected column 47 to be a uint16!"),
            Unknown58: row
                .columns[62]
                .into_u16()
                .copied()
                .expect("Expected column 62 to be a uint16!"),
            Unknown59: row
                .columns[32]
                .into_u8()
                .copied()
                .expect("Expected column 32 to be a uint8!"),
            Scale: row
                .columns[0]
                .into_f32()
                .copied()
                .expect("Expected column 0 to be a float32!"),
            ModelChara: row
                .columns[1]
                .into_u16()
                .copied()
                .expect("Expected column 1 to be a uint16!"),
        })
    }
}
impl<'a> IntoIterator for &'a PetMirageSheet {
    type Item = (u32, Vec<(u16, PetMirageRow)>);
    type IntoIter = StructuredSheetIterator<'a, PetMirageSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, PetMirageSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PetMirageRow {
    ///""
    pub Name: String,
    ///""
    pub Unknown0: u16,
    ///""
    pub Unknown1: u16,
    ///""
    pub Unknown2: u16,
    ///""
    pub Unknown3: u8,
    ///""
    pub Unknown4: u16,
    ///""
    pub Unknown5: u16,
    ///""
    pub Unknown6: u16,
    ///""
    pub Unknown7: u8,
    ///""
    pub Unknown8: u16,
    ///""
    pub Unknown9: u16,
    ///""
    pub Unknown10: u16,
    ///""
    pub Unknown11: u8,
    ///""
    pub Unknown12: u16,
    ///""
    pub Unknown13: u16,
    ///""
    pub Unknown14: u16,
    ///""
    pub Unknown15: u8,
    ///""
    pub Unknown16: u16,
    ///""
    pub Unknown17: u16,
    ///""
    pub Unknown18: u16,
    ///""
    pub Unknown19: u8,
    ///""
    pub Unknown20: u16,
    ///""
    pub Unknown21: u16,
    ///""
    pub Unknown22: u16,
    ///""
    pub Unknown23: u8,
    ///""
    pub Unknown24: u16,
    ///""
    pub Unknown25: u16,
    ///""
    pub Unknown26: u16,
    ///""
    pub Unknown27: u8,
    ///""
    pub Unknown28: u16,
    ///""
    pub Unknown29: u16,
    ///""
    pub Unknown30: u16,
    ///""
    pub Unknown31: u8,
    ///""
    pub Unknown32: u16,
    ///""
    pub Unknown33: u16,
    ///""
    pub Unknown34: u16,
    ///""
    pub Unknown35: u8,
    ///""
    pub Unknown36: u16,
    ///""
    pub Unknown37: u16,
    ///""
    pub Unknown38: u16,
    ///""
    pub Unknown39: u8,
    ///""
    pub Unknown40: u16,
    ///""
    pub Unknown41: u16,
    ///""
    pub Unknown42: u16,
    ///""
    pub Unknown43: u8,
    ///""
    pub Unknown44: u16,
    ///""
    pub Unknown45: u16,
    ///""
    pub Unknown46: u16,
    ///""
    pub Unknown47: u8,
    ///""
    pub Unknown48: u16,
    ///""
    pub Unknown49: u16,
    ///""
    pub Unknown50: u16,
    ///""
    pub Unknown51: u8,
    ///""
    pub Unknown52: u16,
    ///""
    pub Unknown53: u16,
    ///""
    pub Unknown54: u16,
    ///""
    pub Unknown55: u8,
    ///""
    pub Unknown56: u16,
    ///""
    pub Unknown57: u16,
    ///""
    pub Unknown58: u16,
    ///""
    pub Unknown59: u8,
    ///""
    pub Scale: f32,
    ///""
    pub ModelChara: u16,
}
