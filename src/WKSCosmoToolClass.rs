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
pub struct StagesElement {
    pub Unknown0: u32,
    pub Item: i32,
    pub Name: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TypesElement {
    pub Icon: u32,
    pub Name: u16,
    pub CosmicName: u16,
}
#[derive(Debug, Clone)]
pub struct WKSCosmoToolClassSheet {
    sheet: Sheet,
}
impl WKSCosmoToolClassSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WKSCosmoToolClass")?;
        let sheet = resolver.read_excel_sheet(&exh, "WKSCosmoToolClass", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<WKSCosmoToolClassRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<WKSCosmoToolClassRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for WKSCosmoToolClassSheet {
    type Row = WKSCosmoToolClassRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Stages: [
                StagesElement {
                    Unknown0: row
                        .columns[40]
                        .into_u32()
                        .copied()
                        .expect("Expected column 40 to be a uint32!"),
                    Item: row
                        .columns[20]
                        .into_i32()
                        .copied()
                        .expect("Expected column 20 to be a int32!"),
                    Name: row
                        .columns[0]
                        .into_u16()
                        .copied()
                        .expect("Expected column 0 to be a uint16!"),
                },
                StagesElement {
                    Unknown0: row
                        .columns[41]
                        .into_u32()
                        .copied()
                        .expect("Expected column 41 to be a uint32!"),
                    Item: row
                        .columns[21]
                        .into_i32()
                        .copied()
                        .expect("Expected column 21 to be a int32!"),
                    Name: row
                        .columns[1]
                        .into_u16()
                        .copied()
                        .expect("Expected column 1 to be a uint16!"),
                },
                StagesElement {
                    Unknown0: row
                        .columns[42]
                        .into_u32()
                        .copied()
                        .expect("Expected column 42 to be a uint32!"),
                    Item: row
                        .columns[22]
                        .into_i32()
                        .copied()
                        .expect("Expected column 22 to be a int32!"),
                    Name: row
                        .columns[2]
                        .into_u16()
                        .copied()
                        .expect("Expected column 2 to be a uint16!"),
                },
                StagesElement {
                    Unknown0: row
                        .columns[43]
                        .into_u32()
                        .copied()
                        .expect("Expected column 43 to be a uint32!"),
                    Item: row
                        .columns[23]
                        .into_i32()
                        .copied()
                        .expect("Expected column 23 to be a int32!"),
                    Name: row
                        .columns[3]
                        .into_u16()
                        .copied()
                        .expect("Expected column 3 to be a uint16!"),
                },
                StagesElement {
                    Unknown0: row
                        .columns[44]
                        .into_u32()
                        .copied()
                        .expect("Expected column 44 to be a uint32!"),
                    Item: row
                        .columns[24]
                        .into_i32()
                        .copied()
                        .expect("Expected column 24 to be a int32!"),
                    Name: row
                        .columns[4]
                        .into_u16()
                        .copied()
                        .expect("Expected column 4 to be a uint16!"),
                },
                StagesElement {
                    Unknown0: row
                        .columns[45]
                        .into_u32()
                        .copied()
                        .expect("Expected column 45 to be a uint32!"),
                    Item: row
                        .columns[25]
                        .into_i32()
                        .copied()
                        .expect("Expected column 25 to be a int32!"),
                    Name: row
                        .columns[5]
                        .into_u16()
                        .copied()
                        .expect("Expected column 5 to be a uint16!"),
                },
                StagesElement {
                    Unknown0: row
                        .columns[46]
                        .into_u32()
                        .copied()
                        .expect("Expected column 46 to be a uint32!"),
                    Item: row
                        .columns[26]
                        .into_i32()
                        .copied()
                        .expect("Expected column 26 to be a int32!"),
                    Name: row
                        .columns[6]
                        .into_u16()
                        .copied()
                        .expect("Expected column 6 to be a uint16!"),
                },
                StagesElement {
                    Unknown0: row
                        .columns[47]
                        .into_u32()
                        .copied()
                        .expect("Expected column 47 to be a uint32!"),
                    Item: row
                        .columns[27]
                        .into_i32()
                        .copied()
                        .expect("Expected column 27 to be a int32!"),
                    Name: row
                        .columns[7]
                        .into_u16()
                        .copied()
                        .expect("Expected column 7 to be a uint16!"),
                },
                StagesElement {
                    Unknown0: row
                        .columns[48]
                        .into_u32()
                        .copied()
                        .expect("Expected column 48 to be a uint32!"),
                    Item: row
                        .columns[28]
                        .into_i32()
                        .copied()
                        .expect("Expected column 28 to be a int32!"),
                    Name: row
                        .columns[8]
                        .into_u16()
                        .copied()
                        .expect("Expected column 8 to be a uint16!"),
                },
                StagesElement {
                    Unknown0: row
                        .columns[49]
                        .into_u32()
                        .copied()
                        .expect("Expected column 49 to be a uint32!"),
                    Item: row
                        .columns[29]
                        .into_i32()
                        .copied()
                        .expect("Expected column 29 to be a int32!"),
                    Name: row
                        .columns[9]
                        .into_u16()
                        .copied()
                        .expect("Expected column 9 to be a uint16!"),
                },
                StagesElement {
                    Unknown0: row
                        .columns[50]
                        .into_u32()
                        .copied()
                        .expect("Expected column 50 to be a uint32!"),
                    Item: row
                        .columns[30]
                        .into_i32()
                        .copied()
                        .expect("Expected column 30 to be a int32!"),
                    Name: row
                        .columns[10]
                        .into_u16()
                        .copied()
                        .expect("Expected column 10 to be a uint16!"),
                },
                StagesElement {
                    Unknown0: row
                        .columns[51]
                        .into_u32()
                        .copied()
                        .expect("Expected column 51 to be a uint32!"),
                    Item: row
                        .columns[31]
                        .into_i32()
                        .copied()
                        .expect("Expected column 31 to be a int32!"),
                    Name: row
                        .columns[11]
                        .into_u16()
                        .copied()
                        .expect("Expected column 11 to be a uint16!"),
                },
                StagesElement {
                    Unknown0: row
                        .columns[52]
                        .into_u32()
                        .copied()
                        .expect("Expected column 52 to be a uint32!"),
                    Item: row
                        .columns[32]
                        .into_i32()
                        .copied()
                        .expect("Expected column 32 to be a int32!"),
                    Name: row
                        .columns[12]
                        .into_u16()
                        .copied()
                        .expect("Expected column 12 to be a uint16!"),
                },
                StagesElement {
                    Unknown0: row
                        .columns[53]
                        .into_u32()
                        .copied()
                        .expect("Expected column 53 to be a uint32!"),
                    Item: row
                        .columns[33]
                        .into_i32()
                        .copied()
                        .expect("Expected column 33 to be a int32!"),
                    Name: row
                        .columns[13]
                        .into_u16()
                        .copied()
                        .expect("Expected column 13 to be a uint16!"),
                },
                StagesElement {
                    Unknown0: row
                        .columns[54]
                        .into_u32()
                        .copied()
                        .expect("Expected column 54 to be a uint32!"),
                    Item: row
                        .columns[34]
                        .into_i32()
                        .copied()
                        .expect("Expected column 34 to be a int32!"),
                    Name: row
                        .columns[14]
                        .into_u16()
                        .copied()
                        .expect("Expected column 14 to be a uint16!"),
                },
                StagesElement {
                    Unknown0: row
                        .columns[55]
                        .into_u32()
                        .copied()
                        .expect("Expected column 55 to be a uint32!"),
                    Item: row
                        .columns[35]
                        .into_i32()
                        .copied()
                        .expect("Expected column 35 to be a int32!"),
                    Name: row
                        .columns[15]
                        .into_u16()
                        .copied()
                        .expect("Expected column 15 to be a uint16!"),
                },
                StagesElement {
                    Unknown0: row
                        .columns[56]
                        .into_u32()
                        .copied()
                        .expect("Expected column 56 to be a uint32!"),
                    Item: row
                        .columns[36]
                        .into_i32()
                        .copied()
                        .expect("Expected column 36 to be a int32!"),
                    Name: row
                        .columns[16]
                        .into_u16()
                        .copied()
                        .expect("Expected column 16 to be a uint16!"),
                },
                StagesElement {
                    Unknown0: row
                        .columns[57]
                        .into_u32()
                        .copied()
                        .expect("Expected column 57 to be a uint32!"),
                    Item: row
                        .columns[37]
                        .into_i32()
                        .copied()
                        .expect("Expected column 37 to be a int32!"),
                    Name: row
                        .columns[17]
                        .into_u16()
                        .copied()
                        .expect("Expected column 17 to be a uint16!"),
                },
                StagesElement {
                    Unknown0: row
                        .columns[58]
                        .into_u32()
                        .copied()
                        .expect("Expected column 58 to be a uint32!"),
                    Item: row
                        .columns[38]
                        .into_i32()
                        .copied()
                        .expect("Expected column 38 to be a int32!"),
                    Name: row
                        .columns[18]
                        .into_u16()
                        .copied()
                        .expect("Expected column 18 to be a uint16!"),
                },
                StagesElement {
                    Unknown0: row
                        .columns[59]
                        .into_u32()
                        .copied()
                        .expect("Expected column 59 to be a uint32!"),
                    Item: row
                        .columns[39]
                        .into_i32()
                        .copied()
                        .expect("Expected column 39 to be a int32!"),
                    Name: row
                        .columns[19]
                        .into_u16()
                        .copied()
                        .expect("Expected column 19 to be a uint16!"),
                },
            ],
            Types: [
                TypesElement {
                    Icon: row
                        .columns[74]
                        .into_u32()
                        .copied()
                        .expect("Expected column 74 to be a uint32!"),
                    Name: row
                        .columns[60]
                        .into_u16()
                        .copied()
                        .expect("Expected column 60 to be a uint16!"),
                    CosmicName: row
                        .columns[67]
                        .into_u16()
                        .copied()
                        .expect("Expected column 67 to be a uint16!"),
                },
                TypesElement {
                    Icon: row
                        .columns[75]
                        .into_u32()
                        .copied()
                        .expect("Expected column 75 to be a uint32!"),
                    Name: row
                        .columns[61]
                        .into_u16()
                        .copied()
                        .expect("Expected column 61 to be a uint16!"),
                    CosmicName: row
                        .columns[68]
                        .into_u16()
                        .copied()
                        .expect("Expected column 68 to be a uint16!"),
                },
                TypesElement {
                    Icon: row
                        .columns[76]
                        .into_u32()
                        .copied()
                        .expect("Expected column 76 to be a uint32!"),
                    Name: row
                        .columns[62]
                        .into_u16()
                        .copied()
                        .expect("Expected column 62 to be a uint16!"),
                    CosmicName: row
                        .columns[69]
                        .into_u16()
                        .copied()
                        .expect("Expected column 69 to be a uint16!"),
                },
                TypesElement {
                    Icon: row
                        .columns[77]
                        .into_u32()
                        .copied()
                        .expect("Expected column 77 to be a uint32!"),
                    Name: row
                        .columns[63]
                        .into_u16()
                        .copied()
                        .expect("Expected column 63 to be a uint16!"),
                    CosmicName: row
                        .columns[70]
                        .into_u16()
                        .copied()
                        .expect("Expected column 70 to be a uint16!"),
                },
                TypesElement {
                    Icon: row
                        .columns[78]
                        .into_u32()
                        .copied()
                        .expect("Expected column 78 to be a uint32!"),
                    Name: row
                        .columns[64]
                        .into_u16()
                        .copied()
                        .expect("Expected column 64 to be a uint16!"),
                    CosmicName: row
                        .columns[71]
                        .into_u16()
                        .copied()
                        .expect("Expected column 71 to be a uint16!"),
                },
                TypesElement {
                    Icon: row
                        .columns[79]
                        .into_u32()
                        .copied()
                        .expect("Expected column 79 to be a uint32!"),
                    Name: row
                        .columns[65]
                        .into_u16()
                        .copied()
                        .expect("Expected column 65 to be a uint16!"),
                    CosmicName: row
                        .columns[72]
                        .into_u16()
                        .copied()
                        .expect("Expected column 72 to be a uint16!"),
                },
                TypesElement {
                    Icon: row
                        .columns[80]
                        .into_u32()
                        .copied()
                        .expect("Expected column 80 to be a uint32!"),
                    Name: row
                        .columns[66]
                        .into_u16()
                        .copied()
                        .expect("Expected column 66 to be a uint16!"),
                    CosmicName: row
                        .columns[73]
                        .into_u16()
                        .copied()
                        .expect("Expected column 73 to be a uint16!"),
                },
            ],
            Name: row
                .columns[82]
                .into_u16()
                .copied()
                .expect("Expected column 82 to be a uint16!"),
            DataAmount: row
                .columns[81]
                .into_u8()
                .copied()
                .expect("Expected column 81 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a WKSCosmoToolClassSheet {
    type Item = (u32, Vec<(u16, WKSCosmoToolClassRow)>);
    type IntoIter = StructuredSheetIterator<'a, WKSCosmoToolClassSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WKSCosmoToolClassSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WKSCosmoToolClassRow {
    ///""
    pub Stages: [StagesElement; 20],
    ///""
    pub Types: [TypesElement; 7],
    ///""
    pub Name: u16,
    ///""
    pub DataAmount: u8,
}
