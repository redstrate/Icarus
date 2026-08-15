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
pub struct ScriptElement {
    pub ScriptInstruction: String,
    pub ScriptArg: u32,
}
#[derive(Debug, Clone)]
pub struct CustomTalkSheet {
    sheet: Sheet,
}
impl CustomTalkSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 720896u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("CustomTalk")?;
        let sheet = resolver.read_excel_sheet(&exh, "CustomTalk", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<CustomTalkRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<CustomTalkRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for CustomTalkSheet {
    type Row = CustomTalkRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Script: [
                ScriptElement {
                    ScriptInstruction: row
                        .columns[3]
                        .into_string()
                        .cloned()
                        .expect("Expected column 3 to be a string!"),
                    ScriptArg: row
                        .columns[33]
                        .into_u32()
                        .copied()
                        .expect("Expected column 33 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[4]
                        .into_string()
                        .cloned()
                        .expect("Expected column 4 to be a string!"),
                    ScriptArg: row
                        .columns[34]
                        .into_u32()
                        .copied()
                        .expect("Expected column 34 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[5]
                        .into_string()
                        .cloned()
                        .expect("Expected column 5 to be a string!"),
                    ScriptArg: row
                        .columns[35]
                        .into_u32()
                        .copied()
                        .expect("Expected column 35 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[6]
                        .into_string()
                        .cloned()
                        .expect("Expected column 6 to be a string!"),
                    ScriptArg: row
                        .columns[36]
                        .into_u32()
                        .copied()
                        .expect("Expected column 36 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[7]
                        .into_string()
                        .cloned()
                        .expect("Expected column 7 to be a string!"),
                    ScriptArg: row
                        .columns[37]
                        .into_u32()
                        .copied()
                        .expect("Expected column 37 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[8]
                        .into_string()
                        .cloned()
                        .expect("Expected column 8 to be a string!"),
                    ScriptArg: row
                        .columns[38]
                        .into_u32()
                        .copied()
                        .expect("Expected column 38 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[9]
                        .into_string()
                        .cloned()
                        .expect("Expected column 9 to be a string!"),
                    ScriptArg: row
                        .columns[39]
                        .into_u32()
                        .copied()
                        .expect("Expected column 39 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[10]
                        .into_string()
                        .cloned()
                        .expect("Expected column 10 to be a string!"),
                    ScriptArg: row
                        .columns[40]
                        .into_u32()
                        .copied()
                        .expect("Expected column 40 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[11]
                        .into_string()
                        .cloned()
                        .expect("Expected column 11 to be a string!"),
                    ScriptArg: row
                        .columns[41]
                        .into_u32()
                        .copied()
                        .expect("Expected column 41 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[12]
                        .into_string()
                        .cloned()
                        .expect("Expected column 12 to be a string!"),
                    ScriptArg: row
                        .columns[42]
                        .into_u32()
                        .copied()
                        .expect("Expected column 42 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[13]
                        .into_string()
                        .cloned()
                        .expect("Expected column 13 to be a string!"),
                    ScriptArg: row
                        .columns[43]
                        .into_u32()
                        .copied()
                        .expect("Expected column 43 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[14]
                        .into_string()
                        .cloned()
                        .expect("Expected column 14 to be a string!"),
                    ScriptArg: row
                        .columns[44]
                        .into_u32()
                        .copied()
                        .expect("Expected column 44 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[15]
                        .into_string()
                        .cloned()
                        .expect("Expected column 15 to be a string!"),
                    ScriptArg: row
                        .columns[45]
                        .into_u32()
                        .copied()
                        .expect("Expected column 45 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[16]
                        .into_string()
                        .cloned()
                        .expect("Expected column 16 to be a string!"),
                    ScriptArg: row
                        .columns[46]
                        .into_u32()
                        .copied()
                        .expect("Expected column 46 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[17]
                        .into_string()
                        .cloned()
                        .expect("Expected column 17 to be a string!"),
                    ScriptArg: row
                        .columns[47]
                        .into_u32()
                        .copied()
                        .expect("Expected column 47 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[18]
                        .into_string()
                        .cloned()
                        .expect("Expected column 18 to be a string!"),
                    ScriptArg: row
                        .columns[48]
                        .into_u32()
                        .copied()
                        .expect("Expected column 48 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[19]
                        .into_string()
                        .cloned()
                        .expect("Expected column 19 to be a string!"),
                    ScriptArg: row
                        .columns[49]
                        .into_u32()
                        .copied()
                        .expect("Expected column 49 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[20]
                        .into_string()
                        .cloned()
                        .expect("Expected column 20 to be a string!"),
                    ScriptArg: row
                        .columns[50]
                        .into_u32()
                        .copied()
                        .expect("Expected column 50 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[21]
                        .into_string()
                        .cloned()
                        .expect("Expected column 21 to be a string!"),
                    ScriptArg: row
                        .columns[51]
                        .into_u32()
                        .copied()
                        .expect("Expected column 51 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[22]
                        .into_string()
                        .cloned()
                        .expect("Expected column 22 to be a string!"),
                    ScriptArg: row
                        .columns[52]
                        .into_u32()
                        .copied()
                        .expect("Expected column 52 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[23]
                        .into_string()
                        .cloned()
                        .expect("Expected column 23 to be a string!"),
                    ScriptArg: row
                        .columns[53]
                        .into_u32()
                        .copied()
                        .expect("Expected column 53 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[24]
                        .into_string()
                        .cloned()
                        .expect("Expected column 24 to be a string!"),
                    ScriptArg: row
                        .columns[54]
                        .into_u32()
                        .copied()
                        .expect("Expected column 54 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[25]
                        .into_string()
                        .cloned()
                        .expect("Expected column 25 to be a string!"),
                    ScriptArg: row
                        .columns[55]
                        .into_u32()
                        .copied()
                        .expect("Expected column 55 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[26]
                        .into_string()
                        .cloned()
                        .expect("Expected column 26 to be a string!"),
                    ScriptArg: row
                        .columns[56]
                        .into_u32()
                        .copied()
                        .expect("Expected column 56 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[27]
                        .into_string()
                        .cloned()
                        .expect("Expected column 27 to be a string!"),
                    ScriptArg: row
                        .columns[57]
                        .into_u32()
                        .copied()
                        .expect("Expected column 57 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[28]
                        .into_string()
                        .cloned()
                        .expect("Expected column 28 to be a string!"),
                    ScriptArg: row
                        .columns[58]
                        .into_u32()
                        .copied()
                        .expect("Expected column 58 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[29]
                        .into_string()
                        .cloned()
                        .expect("Expected column 29 to be a string!"),
                    ScriptArg: row
                        .columns[59]
                        .into_u32()
                        .copied()
                        .expect("Expected column 59 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[30]
                        .into_string()
                        .cloned()
                        .expect("Expected column 30 to be a string!"),
                    ScriptArg: row
                        .columns[60]
                        .into_u32()
                        .copied()
                        .expect("Expected column 60 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[31]
                        .into_string()
                        .cloned()
                        .expect("Expected column 31 to be a string!"),
                    ScriptArg: row
                        .columns[61]
                        .into_u32()
                        .copied()
                        .expect("Expected column 61 to be a uint32!"),
                },
                ScriptElement {
                    ScriptInstruction: row
                        .columns[32]
                        .into_string()
                        .cloned()
                        .expect("Expected column 32 to be a string!"),
                    ScriptArg: row
                        .columns[62]
                        .into_u32()
                        .copied()
                        .expect("Expected column 62 to be a uint32!"),
                },
            ],
            MainOption: row
                .columns[64]
                .into_string()
                .cloned()
                .expect("Expected column 64 to be a string!"),
            SubOption: row
                .columns[65]
                .into_string()
                .cloned()
                .expect("Expected column 65 to be a string!"),
            Name: row
                .columns[2]
                .into_string()
                .cloned()
                .expect("Expected column 2 to be a string!"),
            IconActor: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
            IconMap: row
                .columns[1]
                .into_u32()
                .copied()
                .expect("Expected column 1 to be a uint32!"),
            SpecialLinks: row
                .columns[75]
                .into_u32()
                .copied()
                .expect("Expected column 75 to be a uint32!"),
            Unknown0: row
                .columns[76]
                .into_u8()
                .copied()
                .expect("Expected column 76 to be a uint8!"),
            Unknown1: row
                .columns[77]
                .into_u8()
                .copied()
                .expect("Expected column 77 to be a uint8!"),
            Unknown2: row
                .columns[63]
                .into_bool()
                .copied()
                .expect("Expected column 63 to be a bool!"),
            Unknown3: row
                .columns[66]
                .into_bool()
                .copied()
                .expect("Expected column 66 to be a bool!"),
            Unknown4: row
                .columns[67]
                .into_bool()
                .copied()
                .expect("Expected column 67 to be a bool!"),
            Unknown5: row
                .columns[68]
                .into_bool()
                .copied()
                .expect("Expected column 68 to be a bool!"),
            Unknown6: row
                .columns[69]
                .into_bool()
                .copied()
                .expect("Expected column 69 to be a bool!"),
            Unknown7: row
                .columns[70]
                .into_bool()
                .copied()
                .expect("Expected column 70 to be a bool!"),
            Unknown8: row
                .columns[71]
                .into_bool()
                .copied()
                .expect("Expected column 71 to be a bool!"),
            Unknown9: row
                .columns[72]
                .into_bool()
                .copied()
                .expect("Expected column 72 to be a bool!"),
            Unknown10: row
                .columns[73]
                .into_bool()
                .copied()
                .expect("Expected column 73 to be a bool!"),
            Unknown11: row
                .columns[74]
                .into_bool()
                .copied()
                .expect("Expected column 74 to be a bool!"),
            Unknown12: row
                .columns[78]
                .into_bool()
                .copied()
                .expect("Expected column 78 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a CustomTalkSheet {
    type Item = (u32, Vec<(u16, CustomTalkRow)>);
    type IntoIter = StructuredSheetIterator<'a, CustomTalkSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, CustomTalkSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CustomTalkRow {
    ///""
    pub Script: [ScriptElement; 30],
    ///""
    pub MainOption: String,
    ///""
    pub SubOption: String,
    ///""
    pub Name: String,
    ///""
    pub IconActor: u32,
    ///""
    pub IconMap: u32,
    ///""
    pub SpecialLinks: u32,
    ///""
    pub Unknown0: u8,
    ///""
    pub Unknown1: u8,
    ///""
    pub Unknown2: bool,
    ///""
    pub Unknown3: bool,
    ///""
    pub Unknown4: bool,
    ///""
    pub Unknown5: bool,
    ///""
    pub Unknown6: bool,
    ///""
    pub Unknown7: bool,
    ///""
    pub Unknown8: bool,
    ///""
    pub Unknown9: bool,
    ///""
    pub Unknown10: bool,
    ///""
    pub Unknown11: bool,
    ///""
    pub Unknown12: bool,
}
