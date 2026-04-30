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
pub struct RideShootingParamsElement {
    pub Unknown0: u32,
    pub PopRange: u32,
    pub ENpc: u32,
    pub Unknown1: u32,
    pub Unknown2: u32,
    pub Unknown3: u32,
    pub Unknown4: u32,
    pub Unknown5: u32,
    pub ENpcScale: u8,
    pub Unknown6: u8,
    pub Unknown7: u8,
    pub Unknown8: u8,
    pub Unknown9: u8,
    pub Unknown10: u8,
}
#[derive(Debug, Clone)]
pub struct RideShootingSheet {
    sheet: Sheet,
}
impl RideShootingSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("RideShooting")?;
        let sheet = resolver.read_excel_sheet(&exh, "RideShooting", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<RideShootingRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<RideShootingRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for RideShootingSheet {
    type Row = RideShootingRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            RideShootingParams: [
                RideShootingParamsElement {
                    Unknown0: row
                        .columns[6]
                        .into_u32()
                        .copied()
                        .expect("Expected column 6 to be a uint32!"),
                    PopRange: row
                        .columns[14]
                        .into_u32()
                        .copied()
                        .expect("Expected column 14 to be a uint32!"),
                    ENpc: row
                        .columns[22]
                        .into_u32()
                        .copied()
                        .expect("Expected column 22 to be a uint32!"),
                    Unknown1: row
                        .columns[38]
                        .into_u32()
                        .copied()
                        .expect("Expected column 38 to be a uint32!"),
                    Unknown2: row
                        .columns[54]
                        .into_u32()
                        .copied()
                        .expect("Expected column 54 to be a uint32!"),
                    Unknown3: row
                        .columns[70]
                        .into_u32()
                        .copied()
                        .expect("Expected column 70 to be a uint32!"),
                    Unknown4: row
                        .columns[86]
                        .into_u32()
                        .copied()
                        .expect("Expected column 86 to be a uint32!"),
                    Unknown5: row
                        .columns[102]
                        .into_u32()
                        .copied()
                        .expect("Expected column 102 to be a uint32!"),
                    ENpcScale: row
                        .columns[30]
                        .into_u8()
                        .copied()
                        .expect("Expected column 30 to be a uint8!"),
                    Unknown6: row
                        .columns[46]
                        .into_u8()
                        .copied()
                        .expect("Expected column 46 to be a uint8!"),
                    Unknown7: row
                        .columns[62]
                        .into_u8()
                        .copied()
                        .expect("Expected column 62 to be a uint8!"),
                    Unknown8: row
                        .columns[78]
                        .into_u8()
                        .copied()
                        .expect("Expected column 78 to be a uint8!"),
                    Unknown9: row
                        .columns[94]
                        .into_u8()
                        .copied()
                        .expect("Expected column 94 to be a uint8!"),
                    Unknown10: row
                        .columns[110]
                        .into_u8()
                        .copied()
                        .expect("Expected column 110 to be a uint8!"),
                },
                RideShootingParamsElement {
                    Unknown0: row
                        .columns[7]
                        .into_u32()
                        .copied()
                        .expect("Expected column 7 to be a uint32!"),
                    PopRange: row
                        .columns[15]
                        .into_u32()
                        .copied()
                        .expect("Expected column 15 to be a uint32!"),
                    ENpc: row
                        .columns[23]
                        .into_u32()
                        .copied()
                        .expect("Expected column 23 to be a uint32!"),
                    Unknown1: row
                        .columns[39]
                        .into_u32()
                        .copied()
                        .expect("Expected column 39 to be a uint32!"),
                    Unknown2: row
                        .columns[55]
                        .into_u32()
                        .copied()
                        .expect("Expected column 55 to be a uint32!"),
                    Unknown3: row
                        .columns[71]
                        .into_u32()
                        .copied()
                        .expect("Expected column 71 to be a uint32!"),
                    Unknown4: row
                        .columns[87]
                        .into_u32()
                        .copied()
                        .expect("Expected column 87 to be a uint32!"),
                    Unknown5: row
                        .columns[103]
                        .into_u32()
                        .copied()
                        .expect("Expected column 103 to be a uint32!"),
                    ENpcScale: row
                        .columns[31]
                        .into_u8()
                        .copied()
                        .expect("Expected column 31 to be a uint8!"),
                    Unknown6: row
                        .columns[47]
                        .into_u8()
                        .copied()
                        .expect("Expected column 47 to be a uint8!"),
                    Unknown7: row
                        .columns[63]
                        .into_u8()
                        .copied()
                        .expect("Expected column 63 to be a uint8!"),
                    Unknown8: row
                        .columns[79]
                        .into_u8()
                        .copied()
                        .expect("Expected column 79 to be a uint8!"),
                    Unknown9: row
                        .columns[95]
                        .into_u8()
                        .copied()
                        .expect("Expected column 95 to be a uint8!"),
                    Unknown10: row
                        .columns[111]
                        .into_u8()
                        .copied()
                        .expect("Expected column 111 to be a uint8!"),
                },
                RideShootingParamsElement {
                    Unknown0: row
                        .columns[8]
                        .into_u32()
                        .copied()
                        .expect("Expected column 8 to be a uint32!"),
                    PopRange: row
                        .columns[16]
                        .into_u32()
                        .copied()
                        .expect("Expected column 16 to be a uint32!"),
                    ENpc: row
                        .columns[24]
                        .into_u32()
                        .copied()
                        .expect("Expected column 24 to be a uint32!"),
                    Unknown1: row
                        .columns[40]
                        .into_u32()
                        .copied()
                        .expect("Expected column 40 to be a uint32!"),
                    Unknown2: row
                        .columns[56]
                        .into_u32()
                        .copied()
                        .expect("Expected column 56 to be a uint32!"),
                    Unknown3: row
                        .columns[72]
                        .into_u32()
                        .copied()
                        .expect("Expected column 72 to be a uint32!"),
                    Unknown4: row
                        .columns[88]
                        .into_u32()
                        .copied()
                        .expect("Expected column 88 to be a uint32!"),
                    Unknown5: row
                        .columns[104]
                        .into_u32()
                        .copied()
                        .expect("Expected column 104 to be a uint32!"),
                    ENpcScale: row
                        .columns[32]
                        .into_u8()
                        .copied()
                        .expect("Expected column 32 to be a uint8!"),
                    Unknown6: row
                        .columns[48]
                        .into_u8()
                        .copied()
                        .expect("Expected column 48 to be a uint8!"),
                    Unknown7: row
                        .columns[64]
                        .into_u8()
                        .copied()
                        .expect("Expected column 64 to be a uint8!"),
                    Unknown8: row
                        .columns[80]
                        .into_u8()
                        .copied()
                        .expect("Expected column 80 to be a uint8!"),
                    Unknown9: row
                        .columns[96]
                        .into_u8()
                        .copied()
                        .expect("Expected column 96 to be a uint8!"),
                    Unknown10: row
                        .columns[112]
                        .into_u8()
                        .copied()
                        .expect("Expected column 112 to be a uint8!"),
                },
                RideShootingParamsElement {
                    Unknown0: row
                        .columns[9]
                        .into_u32()
                        .copied()
                        .expect("Expected column 9 to be a uint32!"),
                    PopRange: row
                        .columns[17]
                        .into_u32()
                        .copied()
                        .expect("Expected column 17 to be a uint32!"),
                    ENpc: row
                        .columns[25]
                        .into_u32()
                        .copied()
                        .expect("Expected column 25 to be a uint32!"),
                    Unknown1: row
                        .columns[41]
                        .into_u32()
                        .copied()
                        .expect("Expected column 41 to be a uint32!"),
                    Unknown2: row
                        .columns[57]
                        .into_u32()
                        .copied()
                        .expect("Expected column 57 to be a uint32!"),
                    Unknown3: row
                        .columns[73]
                        .into_u32()
                        .copied()
                        .expect("Expected column 73 to be a uint32!"),
                    Unknown4: row
                        .columns[89]
                        .into_u32()
                        .copied()
                        .expect("Expected column 89 to be a uint32!"),
                    Unknown5: row
                        .columns[105]
                        .into_u32()
                        .copied()
                        .expect("Expected column 105 to be a uint32!"),
                    ENpcScale: row
                        .columns[33]
                        .into_u8()
                        .copied()
                        .expect("Expected column 33 to be a uint8!"),
                    Unknown6: row
                        .columns[49]
                        .into_u8()
                        .copied()
                        .expect("Expected column 49 to be a uint8!"),
                    Unknown7: row
                        .columns[65]
                        .into_u8()
                        .copied()
                        .expect("Expected column 65 to be a uint8!"),
                    Unknown8: row
                        .columns[81]
                        .into_u8()
                        .copied()
                        .expect("Expected column 81 to be a uint8!"),
                    Unknown9: row
                        .columns[97]
                        .into_u8()
                        .copied()
                        .expect("Expected column 97 to be a uint8!"),
                    Unknown10: row
                        .columns[113]
                        .into_u8()
                        .copied()
                        .expect("Expected column 113 to be a uint8!"),
                },
                RideShootingParamsElement {
                    Unknown0: row
                        .columns[10]
                        .into_u32()
                        .copied()
                        .expect("Expected column 10 to be a uint32!"),
                    PopRange: row
                        .columns[18]
                        .into_u32()
                        .copied()
                        .expect("Expected column 18 to be a uint32!"),
                    ENpc: row
                        .columns[26]
                        .into_u32()
                        .copied()
                        .expect("Expected column 26 to be a uint32!"),
                    Unknown1: row
                        .columns[42]
                        .into_u32()
                        .copied()
                        .expect("Expected column 42 to be a uint32!"),
                    Unknown2: row
                        .columns[58]
                        .into_u32()
                        .copied()
                        .expect("Expected column 58 to be a uint32!"),
                    Unknown3: row
                        .columns[74]
                        .into_u32()
                        .copied()
                        .expect("Expected column 74 to be a uint32!"),
                    Unknown4: row
                        .columns[90]
                        .into_u32()
                        .copied()
                        .expect("Expected column 90 to be a uint32!"),
                    Unknown5: row
                        .columns[106]
                        .into_u32()
                        .copied()
                        .expect("Expected column 106 to be a uint32!"),
                    ENpcScale: row
                        .columns[34]
                        .into_u8()
                        .copied()
                        .expect("Expected column 34 to be a uint8!"),
                    Unknown6: row
                        .columns[50]
                        .into_u8()
                        .copied()
                        .expect("Expected column 50 to be a uint8!"),
                    Unknown7: row
                        .columns[66]
                        .into_u8()
                        .copied()
                        .expect("Expected column 66 to be a uint8!"),
                    Unknown8: row
                        .columns[82]
                        .into_u8()
                        .copied()
                        .expect("Expected column 82 to be a uint8!"),
                    Unknown9: row
                        .columns[98]
                        .into_u8()
                        .copied()
                        .expect("Expected column 98 to be a uint8!"),
                    Unknown10: row
                        .columns[114]
                        .into_u8()
                        .copied()
                        .expect("Expected column 114 to be a uint8!"),
                },
                RideShootingParamsElement {
                    Unknown0: row
                        .columns[11]
                        .into_u32()
                        .copied()
                        .expect("Expected column 11 to be a uint32!"),
                    PopRange: row
                        .columns[19]
                        .into_u32()
                        .copied()
                        .expect("Expected column 19 to be a uint32!"),
                    ENpc: row
                        .columns[27]
                        .into_u32()
                        .copied()
                        .expect("Expected column 27 to be a uint32!"),
                    Unknown1: row
                        .columns[43]
                        .into_u32()
                        .copied()
                        .expect("Expected column 43 to be a uint32!"),
                    Unknown2: row
                        .columns[59]
                        .into_u32()
                        .copied()
                        .expect("Expected column 59 to be a uint32!"),
                    Unknown3: row
                        .columns[75]
                        .into_u32()
                        .copied()
                        .expect("Expected column 75 to be a uint32!"),
                    Unknown4: row
                        .columns[91]
                        .into_u32()
                        .copied()
                        .expect("Expected column 91 to be a uint32!"),
                    Unknown5: row
                        .columns[107]
                        .into_u32()
                        .copied()
                        .expect("Expected column 107 to be a uint32!"),
                    ENpcScale: row
                        .columns[35]
                        .into_u8()
                        .copied()
                        .expect("Expected column 35 to be a uint8!"),
                    Unknown6: row
                        .columns[51]
                        .into_u8()
                        .copied()
                        .expect("Expected column 51 to be a uint8!"),
                    Unknown7: row
                        .columns[67]
                        .into_u8()
                        .copied()
                        .expect("Expected column 67 to be a uint8!"),
                    Unknown8: row
                        .columns[83]
                        .into_u8()
                        .copied()
                        .expect("Expected column 83 to be a uint8!"),
                    Unknown9: row
                        .columns[99]
                        .into_u8()
                        .copied()
                        .expect("Expected column 99 to be a uint8!"),
                    Unknown10: row
                        .columns[115]
                        .into_u8()
                        .copied()
                        .expect("Expected column 115 to be a uint8!"),
                },
                RideShootingParamsElement {
                    Unknown0: row
                        .columns[12]
                        .into_u32()
                        .copied()
                        .expect("Expected column 12 to be a uint32!"),
                    PopRange: row
                        .columns[20]
                        .into_u32()
                        .copied()
                        .expect("Expected column 20 to be a uint32!"),
                    ENpc: row
                        .columns[28]
                        .into_u32()
                        .copied()
                        .expect("Expected column 28 to be a uint32!"),
                    Unknown1: row
                        .columns[44]
                        .into_u32()
                        .copied()
                        .expect("Expected column 44 to be a uint32!"),
                    Unknown2: row
                        .columns[60]
                        .into_u32()
                        .copied()
                        .expect("Expected column 60 to be a uint32!"),
                    Unknown3: row
                        .columns[76]
                        .into_u32()
                        .copied()
                        .expect("Expected column 76 to be a uint32!"),
                    Unknown4: row
                        .columns[92]
                        .into_u32()
                        .copied()
                        .expect("Expected column 92 to be a uint32!"),
                    Unknown5: row
                        .columns[108]
                        .into_u32()
                        .copied()
                        .expect("Expected column 108 to be a uint32!"),
                    ENpcScale: row
                        .columns[36]
                        .into_u8()
                        .copied()
                        .expect("Expected column 36 to be a uint8!"),
                    Unknown6: row
                        .columns[52]
                        .into_u8()
                        .copied()
                        .expect("Expected column 52 to be a uint8!"),
                    Unknown7: row
                        .columns[68]
                        .into_u8()
                        .copied()
                        .expect("Expected column 68 to be a uint8!"),
                    Unknown8: row
                        .columns[84]
                        .into_u8()
                        .copied()
                        .expect("Expected column 84 to be a uint8!"),
                    Unknown9: row
                        .columns[100]
                        .into_u8()
                        .copied()
                        .expect("Expected column 100 to be a uint8!"),
                    Unknown10: row
                        .columns[116]
                        .into_u8()
                        .copied()
                        .expect("Expected column 116 to be a uint8!"),
                },
                RideShootingParamsElement {
                    Unknown0: row
                        .columns[13]
                        .into_u32()
                        .copied()
                        .expect("Expected column 13 to be a uint32!"),
                    PopRange: row
                        .columns[21]
                        .into_u32()
                        .copied()
                        .expect("Expected column 21 to be a uint32!"),
                    ENpc: row
                        .columns[29]
                        .into_u32()
                        .copied()
                        .expect("Expected column 29 to be a uint32!"),
                    Unknown1: row
                        .columns[45]
                        .into_u32()
                        .copied()
                        .expect("Expected column 45 to be a uint32!"),
                    Unknown2: row
                        .columns[61]
                        .into_u32()
                        .copied()
                        .expect("Expected column 61 to be a uint32!"),
                    Unknown3: row
                        .columns[77]
                        .into_u32()
                        .copied()
                        .expect("Expected column 77 to be a uint32!"),
                    Unknown4: row
                        .columns[93]
                        .into_u32()
                        .copied()
                        .expect("Expected column 93 to be a uint32!"),
                    Unknown5: row
                        .columns[109]
                        .into_u32()
                        .copied()
                        .expect("Expected column 109 to be a uint32!"),
                    ENpcScale: row
                        .columns[37]
                        .into_u8()
                        .copied()
                        .expect("Expected column 37 to be a uint8!"),
                    Unknown6: row
                        .columns[53]
                        .into_u8()
                        .copied()
                        .expect("Expected column 53 to be a uint8!"),
                    Unknown7: row
                        .columns[69]
                        .into_u8()
                        .copied()
                        .expect("Expected column 69 to be a uint8!"),
                    Unknown8: row
                        .columns[85]
                        .into_u8()
                        .copied()
                        .expect("Expected column 85 to be a uint8!"),
                    Unknown9: row
                        .columns[101]
                        .into_u8()
                        .copied()
                        .expect("Expected column 101 to be a uint8!"),
                    Unknown10: row
                        .columns[117]
                        .into_u8()
                        .copied()
                        .expect("Expected column 117 to be a uint8!"),
                },
            ],
            GFateRideShooting: row
                .columns[0]
                .into_u16()
                .copied()
                .expect("Expected column 0 to be a uint16!"),
            Unknown0: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            Unknown1: row
                .columns[4]
                .into_u16()
                .copied()
                .expect("Expected column 4 to be a uint16!"),
            StartText: row
                .columns[5]
                .into_u16()
                .copied()
                .expect("Expected column 5 to be a uint16!"),
            Unknown2: row
                .columns[1]
                .into_i16()
                .copied()
                .expect("Expected column 1 to be a int16!"),
            Unknown3: row
                .columns[2]
                .into_i16()
                .copied()
                .expect("Expected column 2 to be a int16!"),
        })
    }
}
impl<'a> IntoIterator for &'a RideShootingSheet {
    type Item = (u32, Vec<(u16, RideShootingRow)>);
    type IntoIter = StructuredSheetIterator<'a, RideShootingSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, RideShootingSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RideShootingRow {
    ///""
    pub RideShootingParams: [RideShootingParamsElement; 8],
    ///""
    pub GFateRideShooting: u16,
    ///""
    pub Unknown0: u16,
    ///""
    pub Unknown1: u16,
    ///""
    pub StartText: u16,
    ///""
    pub Unknown2: i16,
    ///""
    pub Unknown3: i16,
}
