//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl<'a> StructuredSheet<'a> for MJIFarmPastureRankSheet {
    type Row = MJIFarmPastureRankRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a MJIFarmPastureRankSheet {
    type Item = (u32, Vec<(u16, MJIFarmPastureRankRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, MJIFarmPastureRankSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MJIFarmPastureRankSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MJIFarmPastureRankRow<'a> {
    row: &'a Row,
}
impl<'a> MJIFarmPastureRankRow<'a> {
    pub fn RankData(&'a self) -> [RankDataElement; 4] {
        [
            RankDataElement {
                SGB: [
                    self.row.columns[0].into_u32().copied().unwrap(),
                    self.row.columns[4].into_u32().copied().unwrap(),
                    self.row.columns[8].into_u32().copied().unwrap(),
                    self.row.columns[12].into_u32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[28].into_u32().copied().unwrap(),
                Unknown1: self.row.columns[32].into_u32().copied().unwrap(),
                Unknown2: self.row.columns[40].into_u16().copied().unwrap(),
                Unknown3: self.row.columns[44].into_u16().copied().unwrap(),
                Unknown4: self.row.columns[16].into_u8().copied().unwrap(),
                Unknown5: self.row.columns[20].into_u8().copied().unwrap(),
                Unknown6: self.row.columns[24].into_u8().copied().unwrap(),
                Unknown7: self.row.columns[36].into_u8().copied().unwrap(),
            },
            RankDataElement {
                SGB: [
                    self.row.columns[1].into_u32().copied().unwrap(),
                    self.row.columns[5].into_u32().copied().unwrap(),
                    self.row.columns[9].into_u32().copied().unwrap(),
                    self.row.columns[13].into_u32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[29].into_u32().copied().unwrap(),
                Unknown1: self.row.columns[33].into_u32().copied().unwrap(),
                Unknown2: self.row.columns[41].into_u16().copied().unwrap(),
                Unknown3: self.row.columns[45].into_u16().copied().unwrap(),
                Unknown4: self.row.columns[17].into_u8().copied().unwrap(),
                Unknown5: self.row.columns[21].into_u8().copied().unwrap(),
                Unknown6: self.row.columns[25].into_u8().copied().unwrap(),
                Unknown7: self.row.columns[37].into_u8().copied().unwrap(),
            },
            RankDataElement {
                SGB: [
                    self.row.columns[2].into_u32().copied().unwrap(),
                    self.row.columns[6].into_u32().copied().unwrap(),
                    self.row.columns[10].into_u32().copied().unwrap(),
                    self.row.columns[14].into_u32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[30].into_u32().copied().unwrap(),
                Unknown1: self.row.columns[34].into_u32().copied().unwrap(),
                Unknown2: self.row.columns[42].into_u16().copied().unwrap(),
                Unknown3: self.row.columns[46].into_u16().copied().unwrap(),
                Unknown4: self.row.columns[18].into_u8().copied().unwrap(),
                Unknown5: self.row.columns[22].into_u8().copied().unwrap(),
                Unknown6: self.row.columns[26].into_u8().copied().unwrap(),
                Unknown7: self.row.columns[38].into_u8().copied().unwrap(),
            },
            RankDataElement {
                SGB: [
                    self.row.columns[3].into_u32().copied().unwrap(),
                    self.row.columns[7].into_u32().copied().unwrap(),
                    self.row.columns[11].into_u32().copied().unwrap(),
                    self.row.columns[15].into_u32().copied().unwrap(),
                ],
                Unknown0: self.row.columns[31].into_u32().copied().unwrap(),
                Unknown1: self.row.columns[35].into_u32().copied().unwrap(),
                Unknown2: self.row.columns[43].into_u16().copied().unwrap(),
                Unknown3: self.row.columns[47].into_u16().copied().unwrap(),
                Unknown4: self.row.columns[19].into_u8().copied().unwrap(),
                Unknown5: self.row.columns[23].into_u8().copied().unwrap(),
                Unknown6: self.row.columns[27].into_u8().copied().unwrap(),
                Unknown7: self.row.columns[39].into_u8().copied().unwrap(),
            },
        ]
    }
}
