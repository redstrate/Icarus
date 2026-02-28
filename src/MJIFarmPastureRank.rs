//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct RankDataElement<'a> {
    pub SGB: [&'a Field; 4],
    pub Unknown0: &'a Field,
    pub Unknown1: &'a Field,
    pub Unknown2: &'a Field,
    pub Unknown3: &'a Field,
    pub Unknown4: &'a Field,
    pub Unknown5: &'a Field,
    pub Unknown6: &'a Field,
    pub Unknown7: &'a Field,
}
#[derive(Debug, Clone)]
pub struct MJIFarmPastureRankSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl MJIFarmPastureRankSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MJIFarmPastureRank")?;
        let sheet = resolver.read_excel_sheet(&exh, "MJIFarmPastureRank", language)?;
        let mut index_mapping: Vec<(usize, &ExcelColumnDefinition)> = sheet
            .exh
            .column_definitions
            .iter()
            .enumerate()
            .collect();
        index_mapping.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let index_mapping: Vec<usize> = index_mapping
            .iter()
            .map(|(index, _)| *index)
            .collect();
        Ok(Self { sheet, index_mapping })
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
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
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
    index_mapping: Vec<usize>,
}
impl<'a> MJIFarmPastureRankRow<'a> {
    pub fn RankData(&'a self) -> [RankDataElement<'a>; 4] {
        [
            RankDataElement {
                SGB: [
                    &self.row.columns[self.index_mapping[0]],
                    &self.row.columns[self.index_mapping[1]],
                    &self.row.columns[self.index_mapping[2]],
                    &self.row.columns[self.index_mapping[3]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[4]],
                Unknown1: &self.row.columns[self.index_mapping[5]],
                Unknown2: &self.row.columns[self.index_mapping[6]],
                Unknown3: &self.row.columns[self.index_mapping[7]],
                Unknown4: &self.row.columns[self.index_mapping[8]],
                Unknown5: &self.row.columns[self.index_mapping[9]],
                Unknown6: &self.row.columns[self.index_mapping[10]],
                Unknown7: &self.row.columns[self.index_mapping[11]],
            },
            RankDataElement {
                SGB: [
                    &self.row.columns[self.index_mapping[12]],
                    &self.row.columns[self.index_mapping[13]],
                    &self.row.columns[self.index_mapping[14]],
                    &self.row.columns[self.index_mapping[15]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[16]],
                Unknown1: &self.row.columns[self.index_mapping[17]],
                Unknown2: &self.row.columns[self.index_mapping[18]],
                Unknown3: &self.row.columns[self.index_mapping[19]],
                Unknown4: &self.row.columns[self.index_mapping[20]],
                Unknown5: &self.row.columns[self.index_mapping[21]],
                Unknown6: &self.row.columns[self.index_mapping[22]],
                Unknown7: &self.row.columns[self.index_mapping[23]],
            },
            RankDataElement {
                SGB: [
                    &self.row.columns[self.index_mapping[24]],
                    &self.row.columns[self.index_mapping[25]],
                    &self.row.columns[self.index_mapping[26]],
                    &self.row.columns[self.index_mapping[27]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[28]],
                Unknown1: &self.row.columns[self.index_mapping[29]],
                Unknown2: &self.row.columns[self.index_mapping[30]],
                Unknown3: &self.row.columns[self.index_mapping[31]],
                Unknown4: &self.row.columns[self.index_mapping[32]],
                Unknown5: &self.row.columns[self.index_mapping[33]],
                Unknown6: &self.row.columns[self.index_mapping[34]],
                Unknown7: &self.row.columns[self.index_mapping[35]],
            },
            RankDataElement {
                SGB: [
                    &self.row.columns[self.index_mapping[36]],
                    &self.row.columns[self.index_mapping[37]],
                    &self.row.columns[self.index_mapping[38]],
                    &self.row.columns[self.index_mapping[39]],
                ],
                Unknown0: &self.row.columns[self.index_mapping[40]],
                Unknown1: &self.row.columns[self.index_mapping[41]],
                Unknown2: &self.row.columns[self.index_mapping[42]],
                Unknown3: &self.row.columns[self.index_mapping[43]],
                Unknown4: &self.row.columns[self.index_mapping[44]],
                Unknown5: &self.row.columns[self.index_mapping[45]],
                Unknown6: &self.row.columns[self.index_mapping[46]],
                Unknown7: &self.row.columns[self.index_mapping[47]],
            },
        ]
    }
}
