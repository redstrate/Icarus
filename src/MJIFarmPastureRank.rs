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
pub struct RankDataElement {
    pub SGB: [u32; 4],
    pub Unknown0: u32,
    pub Unknown1: u32,
    pub Unknown2: u16,
    pub Unknown3: u16,
    pub Unknown4: u8,
    pub Unknown5: u8,
    pub Unknown6: u8,
    pub Unknown7: u8,
}
#[derive(Debug, Clone)]
pub struct MJIFarmPastureRankSheet {
    sheet: Sheet,
}
impl MJIFarmPastureRankSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MJIFarmPastureRank")?;
        let sheet = resolver.read_excel_sheet(&exh, "MJIFarmPastureRank", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<MJIFarmPastureRankRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<MJIFarmPastureRankRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for MJIFarmPastureRankSheet {
    type Row = MJIFarmPastureRankRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            RankData: [
                RankDataElement {
                    SGB: [
                        row
                            .columns[0]
                            .into_u32()
                            .copied()
                            .expect("Expected column 0 to be a uint32!"),
                        row
                            .columns[4]
                            .into_u32()
                            .copied()
                            .expect("Expected column 4 to be a uint32!"),
                        row
                            .columns[8]
                            .into_u32()
                            .copied()
                            .expect("Expected column 8 to be a uint32!"),
                        row
                            .columns[12]
                            .into_u32()
                            .copied()
                            .expect("Expected column 12 to be a uint32!"),
                    ],
                    Unknown0: row
                        .columns[28]
                        .into_u32()
                        .copied()
                        .expect("Expected column 28 to be a uint32!"),
                    Unknown1: row
                        .columns[32]
                        .into_u32()
                        .copied()
                        .expect("Expected column 32 to be a uint32!"),
                    Unknown2: row
                        .columns[40]
                        .into_u16()
                        .copied()
                        .expect("Expected column 40 to be a uint16!"),
                    Unknown3: row
                        .columns[44]
                        .into_u16()
                        .copied()
                        .expect("Expected column 44 to be a uint16!"),
                    Unknown4: row
                        .columns[16]
                        .into_u8()
                        .copied()
                        .expect("Expected column 16 to be a uint8!"),
                    Unknown5: row
                        .columns[20]
                        .into_u8()
                        .copied()
                        .expect("Expected column 20 to be a uint8!"),
                    Unknown6: row
                        .columns[24]
                        .into_u8()
                        .copied()
                        .expect("Expected column 24 to be a uint8!"),
                    Unknown7: row
                        .columns[36]
                        .into_u8()
                        .copied()
                        .expect("Expected column 36 to be a uint8!"),
                },
                RankDataElement {
                    SGB: [
                        row
                            .columns[1]
                            .into_u32()
                            .copied()
                            .expect("Expected column 1 to be a uint32!"),
                        row
                            .columns[5]
                            .into_u32()
                            .copied()
                            .expect("Expected column 5 to be a uint32!"),
                        row
                            .columns[9]
                            .into_u32()
                            .copied()
                            .expect("Expected column 9 to be a uint32!"),
                        row
                            .columns[13]
                            .into_u32()
                            .copied()
                            .expect("Expected column 13 to be a uint32!"),
                    ],
                    Unknown0: row
                        .columns[29]
                        .into_u32()
                        .copied()
                        .expect("Expected column 29 to be a uint32!"),
                    Unknown1: row
                        .columns[33]
                        .into_u32()
                        .copied()
                        .expect("Expected column 33 to be a uint32!"),
                    Unknown2: row
                        .columns[41]
                        .into_u16()
                        .copied()
                        .expect("Expected column 41 to be a uint16!"),
                    Unknown3: row
                        .columns[45]
                        .into_u16()
                        .copied()
                        .expect("Expected column 45 to be a uint16!"),
                    Unknown4: row
                        .columns[17]
                        .into_u8()
                        .copied()
                        .expect("Expected column 17 to be a uint8!"),
                    Unknown5: row
                        .columns[21]
                        .into_u8()
                        .copied()
                        .expect("Expected column 21 to be a uint8!"),
                    Unknown6: row
                        .columns[25]
                        .into_u8()
                        .copied()
                        .expect("Expected column 25 to be a uint8!"),
                    Unknown7: row
                        .columns[37]
                        .into_u8()
                        .copied()
                        .expect("Expected column 37 to be a uint8!"),
                },
                RankDataElement {
                    SGB: [
                        row
                            .columns[2]
                            .into_u32()
                            .copied()
                            .expect("Expected column 2 to be a uint32!"),
                        row
                            .columns[6]
                            .into_u32()
                            .copied()
                            .expect("Expected column 6 to be a uint32!"),
                        row
                            .columns[10]
                            .into_u32()
                            .copied()
                            .expect("Expected column 10 to be a uint32!"),
                        row
                            .columns[14]
                            .into_u32()
                            .copied()
                            .expect("Expected column 14 to be a uint32!"),
                    ],
                    Unknown0: row
                        .columns[30]
                        .into_u32()
                        .copied()
                        .expect("Expected column 30 to be a uint32!"),
                    Unknown1: row
                        .columns[34]
                        .into_u32()
                        .copied()
                        .expect("Expected column 34 to be a uint32!"),
                    Unknown2: row
                        .columns[42]
                        .into_u16()
                        .copied()
                        .expect("Expected column 42 to be a uint16!"),
                    Unknown3: row
                        .columns[46]
                        .into_u16()
                        .copied()
                        .expect("Expected column 46 to be a uint16!"),
                    Unknown4: row
                        .columns[18]
                        .into_u8()
                        .copied()
                        .expect("Expected column 18 to be a uint8!"),
                    Unknown5: row
                        .columns[22]
                        .into_u8()
                        .copied()
                        .expect("Expected column 22 to be a uint8!"),
                    Unknown6: row
                        .columns[26]
                        .into_u8()
                        .copied()
                        .expect("Expected column 26 to be a uint8!"),
                    Unknown7: row
                        .columns[38]
                        .into_u8()
                        .copied()
                        .expect("Expected column 38 to be a uint8!"),
                },
                RankDataElement {
                    SGB: [
                        row
                            .columns[3]
                            .into_u32()
                            .copied()
                            .expect("Expected column 3 to be a uint32!"),
                        row
                            .columns[7]
                            .into_u32()
                            .copied()
                            .expect("Expected column 7 to be a uint32!"),
                        row
                            .columns[11]
                            .into_u32()
                            .copied()
                            .expect("Expected column 11 to be a uint32!"),
                        row
                            .columns[15]
                            .into_u32()
                            .copied()
                            .expect("Expected column 15 to be a uint32!"),
                    ],
                    Unknown0: row
                        .columns[31]
                        .into_u32()
                        .copied()
                        .expect("Expected column 31 to be a uint32!"),
                    Unknown1: row
                        .columns[35]
                        .into_u32()
                        .copied()
                        .expect("Expected column 35 to be a uint32!"),
                    Unknown2: row
                        .columns[43]
                        .into_u16()
                        .copied()
                        .expect("Expected column 43 to be a uint16!"),
                    Unknown3: row
                        .columns[47]
                        .into_u16()
                        .copied()
                        .expect("Expected column 47 to be a uint16!"),
                    Unknown4: row
                        .columns[19]
                        .into_u8()
                        .copied()
                        .expect("Expected column 19 to be a uint8!"),
                    Unknown5: row
                        .columns[23]
                        .into_u8()
                        .copied()
                        .expect("Expected column 23 to be a uint8!"),
                    Unknown6: row
                        .columns[27]
                        .into_u8()
                        .copied()
                        .expect("Expected column 27 to be a uint8!"),
                    Unknown7: row
                        .columns[39]
                        .into_u8()
                        .copied()
                        .expect("Expected column 39 to be a uint8!"),
                },
            ],
        })
    }
}
impl<'a> IntoIterator for &'a MJIFarmPastureRankSheet {
    type Item = (u32, Vec<(u16, MJIFarmPastureRankRow)>);
    type IntoIter = StructuredSheetIterator<'a, MJIFarmPastureRankSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MJIFarmPastureRankSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MJIFarmPastureRankRow {
    ///""
    pub RankData: [RankDataElement; 4],
}
