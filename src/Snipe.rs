//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Clone, Debug, PartialEq)]
pub struct SnipeDataElement {
    pub DataEventNPC: u32,
    pub Unknown0: u16,
    pub Unknown1: u16,
    pub Unknown2: u16,
    pub Unknown3: u16,
    pub Unknown4: u8,
    pub Unknown5: u8,
}
#[derive(Debug, Clone)]
pub struct SnipeSheet {
    sheet: Sheet,
}
impl SnipeSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 1u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Snipe")?;
        let sheet = resolver.read_excel_sheet(&exh, "Snipe", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<SnipeRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<SnipeRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for SnipeSheet {
    type Row = SnipeRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            SnipeData: [
                SnipeDataElement {
                    DataEventNPC: row
                        .columns[17]
                        .into_u32()
                        .copied()
                        .expect("Expected column 17 to be a uint32!"),
                    Unknown0: row
                        .columns[25]
                        .into_u16()
                        .copied()
                        .expect("Expected column 25 to be a uint16!"),
                    Unknown1: row
                        .columns[33]
                        .into_u16()
                        .copied()
                        .expect("Expected column 33 to be a uint16!"),
                    Unknown2: row
                        .columns[49]
                        .into_u16()
                        .copied()
                        .expect("Expected column 49 to be a uint16!"),
                    Unknown3: row
                        .columns[57]
                        .into_u16()
                        .copied()
                        .expect("Expected column 57 to be a uint16!"),
                    Unknown4: row
                        .columns[41]
                        .into_u8()
                        .copied()
                        .expect("Expected column 41 to be a uint8!"),
                    Unknown5: row
                        .columns[65]
                        .into_u8()
                        .copied()
                        .expect("Expected column 65 to be a uint8!"),
                },
                SnipeDataElement {
                    DataEventNPC: row
                        .columns[18]
                        .into_u32()
                        .copied()
                        .expect("Expected column 18 to be a uint32!"),
                    Unknown0: row
                        .columns[26]
                        .into_u16()
                        .copied()
                        .expect("Expected column 26 to be a uint16!"),
                    Unknown1: row
                        .columns[34]
                        .into_u16()
                        .copied()
                        .expect("Expected column 34 to be a uint16!"),
                    Unknown2: row
                        .columns[50]
                        .into_u16()
                        .copied()
                        .expect("Expected column 50 to be a uint16!"),
                    Unknown3: row
                        .columns[58]
                        .into_u16()
                        .copied()
                        .expect("Expected column 58 to be a uint16!"),
                    Unknown4: row
                        .columns[42]
                        .into_u8()
                        .copied()
                        .expect("Expected column 42 to be a uint8!"),
                    Unknown5: row
                        .columns[66]
                        .into_u8()
                        .copied()
                        .expect("Expected column 66 to be a uint8!"),
                },
                SnipeDataElement {
                    DataEventNPC: row
                        .columns[19]
                        .into_u32()
                        .copied()
                        .expect("Expected column 19 to be a uint32!"),
                    Unknown0: row
                        .columns[27]
                        .into_u16()
                        .copied()
                        .expect("Expected column 27 to be a uint16!"),
                    Unknown1: row
                        .columns[35]
                        .into_u16()
                        .copied()
                        .expect("Expected column 35 to be a uint16!"),
                    Unknown2: row
                        .columns[51]
                        .into_u16()
                        .copied()
                        .expect("Expected column 51 to be a uint16!"),
                    Unknown3: row
                        .columns[59]
                        .into_u16()
                        .copied()
                        .expect("Expected column 59 to be a uint16!"),
                    Unknown4: row
                        .columns[43]
                        .into_u8()
                        .copied()
                        .expect("Expected column 43 to be a uint8!"),
                    Unknown5: row
                        .columns[67]
                        .into_u8()
                        .copied()
                        .expect("Expected column 67 to be a uint8!"),
                },
                SnipeDataElement {
                    DataEventNPC: row
                        .columns[20]
                        .into_u32()
                        .copied()
                        .expect("Expected column 20 to be a uint32!"),
                    Unknown0: row
                        .columns[28]
                        .into_u16()
                        .copied()
                        .expect("Expected column 28 to be a uint16!"),
                    Unknown1: row
                        .columns[36]
                        .into_u16()
                        .copied()
                        .expect("Expected column 36 to be a uint16!"),
                    Unknown2: row
                        .columns[52]
                        .into_u16()
                        .copied()
                        .expect("Expected column 52 to be a uint16!"),
                    Unknown3: row
                        .columns[60]
                        .into_u16()
                        .copied()
                        .expect("Expected column 60 to be a uint16!"),
                    Unknown4: row
                        .columns[44]
                        .into_u8()
                        .copied()
                        .expect("Expected column 44 to be a uint8!"),
                    Unknown5: row
                        .columns[68]
                        .into_u8()
                        .copied()
                        .expect("Expected column 68 to be a uint8!"),
                },
                SnipeDataElement {
                    DataEventNPC: row
                        .columns[21]
                        .into_u32()
                        .copied()
                        .expect("Expected column 21 to be a uint32!"),
                    Unknown0: row
                        .columns[29]
                        .into_u16()
                        .copied()
                        .expect("Expected column 29 to be a uint16!"),
                    Unknown1: row
                        .columns[37]
                        .into_u16()
                        .copied()
                        .expect("Expected column 37 to be a uint16!"),
                    Unknown2: row
                        .columns[53]
                        .into_u16()
                        .copied()
                        .expect("Expected column 53 to be a uint16!"),
                    Unknown3: row
                        .columns[61]
                        .into_u16()
                        .copied()
                        .expect("Expected column 61 to be a uint16!"),
                    Unknown4: row
                        .columns[45]
                        .into_u8()
                        .copied()
                        .expect("Expected column 45 to be a uint8!"),
                    Unknown5: row
                        .columns[69]
                        .into_u8()
                        .copied()
                        .expect("Expected column 69 to be a uint8!"),
                },
                SnipeDataElement {
                    DataEventNPC: row
                        .columns[22]
                        .into_u32()
                        .copied()
                        .expect("Expected column 22 to be a uint32!"),
                    Unknown0: row
                        .columns[30]
                        .into_u16()
                        .copied()
                        .expect("Expected column 30 to be a uint16!"),
                    Unknown1: row
                        .columns[38]
                        .into_u16()
                        .copied()
                        .expect("Expected column 38 to be a uint16!"),
                    Unknown2: row
                        .columns[54]
                        .into_u16()
                        .copied()
                        .expect("Expected column 54 to be a uint16!"),
                    Unknown3: row
                        .columns[62]
                        .into_u16()
                        .copied()
                        .expect("Expected column 62 to be a uint16!"),
                    Unknown4: row
                        .columns[46]
                        .into_u8()
                        .copied()
                        .expect("Expected column 46 to be a uint8!"),
                    Unknown5: row
                        .columns[70]
                        .into_u8()
                        .copied()
                        .expect("Expected column 70 to be a uint8!"),
                },
                SnipeDataElement {
                    DataEventNPC: row
                        .columns[23]
                        .into_u32()
                        .copied()
                        .expect("Expected column 23 to be a uint32!"),
                    Unknown0: row
                        .columns[31]
                        .into_u16()
                        .copied()
                        .expect("Expected column 31 to be a uint16!"),
                    Unknown1: row
                        .columns[39]
                        .into_u16()
                        .copied()
                        .expect("Expected column 39 to be a uint16!"),
                    Unknown2: row
                        .columns[55]
                        .into_u16()
                        .copied()
                        .expect("Expected column 55 to be a uint16!"),
                    Unknown3: row
                        .columns[63]
                        .into_u16()
                        .copied()
                        .expect("Expected column 63 to be a uint16!"),
                    Unknown4: row
                        .columns[47]
                        .into_u8()
                        .copied()
                        .expect("Expected column 47 to be a uint8!"),
                    Unknown5: row
                        .columns[71]
                        .into_u8()
                        .copied()
                        .expect("Expected column 71 to be a uint8!"),
                },
                SnipeDataElement {
                    DataEventNPC: row
                        .columns[24]
                        .into_u32()
                        .copied()
                        .expect("Expected column 24 to be a uint32!"),
                    Unknown0: row
                        .columns[32]
                        .into_u16()
                        .copied()
                        .expect("Expected column 32 to be a uint16!"),
                    Unknown1: row
                        .columns[40]
                        .into_u16()
                        .copied()
                        .expect("Expected column 40 to be a uint16!"),
                    Unknown2: row
                        .columns[56]
                        .into_u16()
                        .copied()
                        .expect("Expected column 56 to be a uint16!"),
                    Unknown3: row
                        .columns[64]
                        .into_u16()
                        .copied()
                        .expect("Expected column 64 to be a uint16!"),
                    Unknown4: row
                        .columns[48]
                        .into_u8()
                        .copied()
                        .expect("Expected column 48 to be a uint8!"),
                    Unknown5: row
                        .columns[72]
                        .into_u8()
                        .copied()
                        .expect("Expected column 72 to be a uint8!"),
                },
            ],
            EventNPC: [
                row
                    .columns[73]
                    .into_u32()
                    .copied()
                    .expect("Expected column 73 to be a uint32!"),
                row
                    .columns[74]
                    .into_u32()
                    .copied()
                    .expect("Expected column 74 to be a uint32!"),
                row
                    .columns[75]
                    .into_u32()
                    .copied()
                    .expect("Expected column 75 to be a uint32!"),
                row
                    .columns[76]
                    .into_u32()
                    .copied()
                    .expect("Expected column 76 to be a uint32!"),
                row
                    .columns[77]
                    .into_u32()
                    .copied()
                    .expect("Expected column 77 to be a uint32!"),
                row
                    .columns[78]
                    .into_u32()
                    .copied()
                    .expect("Expected column 78 to be a uint32!"),
                row
                    .columns[79]
                    .into_u32()
                    .copied()
                    .expect("Expected column 79 to be a uint32!"),
                row
                    .columns[80]
                    .into_u32()
                    .copied()
                    .expect("Expected column 80 to be a uint32!"),
            ],
            Unknown0: row
                .columns[89]
                .into_u32()
                .copied()
                .expect("Expected column 89 to be a uint32!"),
            Unknown1: row
                .columns[81]
                .into_u16()
                .copied()
                .expect("Expected column 81 to be a uint16!"),
            Unknown2: row
                .columns[85]
                .into_u8()
                .copied()
                .expect("Expected column 85 to be a uint8!"),
            Unknown3: row
                .columns[90]
                .into_u32()
                .copied()
                .expect("Expected column 90 to be a uint32!"),
            Unknown4: row
                .columns[82]
                .into_u16()
                .copied()
                .expect("Expected column 82 to be a uint16!"),
            Unknown5: row
                .columns[86]
                .into_u8()
                .copied()
                .expect("Expected column 86 to be a uint8!"),
            Unknown6: row
                .columns[91]
                .into_u32()
                .copied()
                .expect("Expected column 91 to be a uint32!"),
            Unknown7: row
                .columns[83]
                .into_u16()
                .copied()
                .expect("Expected column 83 to be a uint16!"),
            Unknown8: row
                .columns[87]
                .into_u8()
                .copied()
                .expect("Expected column 87 to be a uint8!"),
            Unknown9: row
                .columns[92]
                .into_u32()
                .copied()
                .expect("Expected column 92 to be a uint32!"),
            Unknown10: row
                .columns[84]
                .into_u16()
                .copied()
                .expect("Expected column 84 to be a uint16!"),
            Unknown11: row
                .columns[88]
                .into_u8()
                .copied()
                .expect("Expected column 88 to be a uint8!"),
            Objective0: row
                .columns[93]
                .into_string()
                .cloned()
                .expect("Expected column 93 to be a string!"),
            Hint0: row
                .columns[94]
                .into_string()
                .cloned()
                .expect("Expected column 94 to be a string!"),
            Objective1: row
                .columns[95]
                .into_string()
                .cloned()
                .expect("Expected column 95 to be a string!"),
            Hint1: row
                .columns[96]
                .into_string()
                .cloned()
                .expect("Expected column 96 to be a string!"),
            Unknown12: row
                .columns[97]
                .into_string()
                .cloned()
                .expect("Expected column 97 to be a string!"),
            Unknown13: row
                .columns[98]
                .into_string()
                .cloned()
                .expect("Expected column 98 to be a string!"),
            Unknown14: row
                .columns[99]
                .into_string()
                .cloned()
                .expect("Expected column 99 to be a string!"),
            Unknown15: row
                .columns[100]
                .into_string()
                .cloned()
                .expect("Expected column 100 to be a string!"),
            Unknown16: row
                .columns[101]
                .into_string()
                .cloned()
                .expect("Expected column 101 to be a string!"),
            Unknown17: row
                .columns[102]
                .into_string()
                .cloned()
                .expect("Expected column 102 to be a string!"),
            Unknown18: row
                .columns[103]
                .into_string()
                .cloned()
                .expect("Expected column 103 to be a string!"),
            ActionText: row
                .columns[104]
                .into_string()
                .cloned()
                .expect("Expected column 104 to be a string!"),
            Unknown19: row
                .columns[105]
                .into_u8()
                .copied()
                .expect("Expected column 105 to be a uint8!"),
            Unknown20: row
                .columns[106]
                .into_u8()
                .copied()
                .expect("Expected column 106 to be a uint8!"),
            VFXFire: row
                .columns[11]
                .into_string()
                .cloned()
                .expect("Expected column 11 to be a string!"),
            VFXHit: row
                .columns[12]
                .into_string()
                .cloned()
                .expect("Expected column 12 to be a string!"),
            VFXMiss: row
                .columns[13]
                .into_string()
                .cloned()
                .expect("Expected column 13 to be a string!"),
            VFXAdditional: row
                .columns[14]
                .into_string()
                .cloned()
                .expect("Expected column 14 to be a string!"),
            LGBTargetMarker: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
            Unknown21: row
                .columns[1]
                .into_u16()
                .copied()
                .expect("Expected column 1 to be a uint16!"),
            Unknown22: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            Unknown23: row
                .columns[15]
                .into_u16()
                .copied()
                .expect("Expected column 15 to be a uint16!"),
            Unknown24: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
            Unknown25: row
                .columns[4]
                .into_u8()
                .copied()
                .expect("Expected column 4 to be a uint8!"),
            Unknown26: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            Unknown27: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
            Unknown28: row
                .columns[7]
                .into_u8()
                .copied()
                .expect("Expected column 7 to be a uint8!"),
            Unknown29: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
            Unknown30: row
                .columns[10]
                .into_u8()
                .copied()
                .expect("Expected column 10 to be a uint8!"),
            Unknown31: row
                .columns[2]
                .into_bool()
                .copied()
                .expect("Expected column 2 to be a bool!"),
            Unknown32: row
                .columns[16]
                .into_bool()
                .copied()
                .expect("Expected column 16 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a SnipeSheet {
    type Item = (u32, Vec<(u16, SnipeRow)>);
    type IntoIter = StructuredSheetIterator<'a, SnipeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, SnipeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SnipeRow {
    ///""
    pub SnipeData: [SnipeDataElement; 8],
    ///""
    pub EventNPC: [u32; 8],
    ///""
    pub Unknown0: u32,
    ///""
    pub Unknown1: u16,
    ///""
    pub Unknown2: u8,
    ///""
    pub Unknown3: u32,
    ///""
    pub Unknown4: u16,
    ///""
    pub Unknown5: u8,
    ///""
    pub Unknown6: u32,
    ///""
    pub Unknown7: u16,
    ///""
    pub Unknown8: u8,
    ///""
    pub Unknown9: u32,
    ///""
    pub Unknown10: u16,
    ///""
    pub Unknown11: u8,
    ///""
    pub Objective0: String,
    ///""
    pub Hint0: String,
    ///""
    pub Objective1: String,
    ///""
    pub Hint1: String,
    ///""
    pub Unknown12: String,
    ///""
    pub Unknown13: String,
    ///""
    pub Unknown14: String,
    ///""
    pub Unknown15: String,
    ///""
    pub Unknown16: String,
    ///""
    pub Unknown17: String,
    ///""
    pub Unknown18: String,
    ///""
    pub ActionText: String,
    ///""
    pub Unknown19: u8,
    ///""
    pub Unknown20: u8,
    ///""
    pub VFXFire: String,
    ///""
    pub VFXHit: String,
    ///""
    pub VFXMiss: String,
    ///""
    pub VFXAdditional: String,
    ///""
    pub LGBTargetMarker: u32,
    ///""
    pub Unknown21: u16,
    ///""
    pub Unknown22: u16,
    ///""
    pub Unknown23: u16,
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
    pub Unknown31: bool,
    ///""
    pub Unknown32: bool,
}
