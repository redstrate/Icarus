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
pub struct MJICraftworksPopularitySheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl MJICraftworksPopularitySheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MJICraftworksPopularity")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "MJICraftworksPopularity", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<MJICraftworksPopularityRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<MJICraftworksPopularityRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for MJICraftworksPopularitySheet {
    type Row = MJICraftworksPopularityRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a MJICraftworksPopularitySheet {
    type Item = (u32, Vec<(u16, MJICraftworksPopularityRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, MJICraftworksPopularitySheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MJICraftworksPopularitySheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MJICraftworksPopularityRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> MJICraftworksPopularityRow<'a> {
    pub fn Popularity(&'a self) -> [&'a Field; 91] {
        [
            &self.row.columns[self.index_mapping[0]],
            &self.row.columns[self.index_mapping[1]],
            &self.row.columns[self.index_mapping[2]],
            &self.row.columns[self.index_mapping[3]],
            &self.row.columns[self.index_mapping[4]],
            &self.row.columns[self.index_mapping[5]],
            &self.row.columns[self.index_mapping[6]],
            &self.row.columns[self.index_mapping[7]],
            &self.row.columns[self.index_mapping[8]],
            &self.row.columns[self.index_mapping[9]],
            &self.row.columns[self.index_mapping[10]],
            &self.row.columns[self.index_mapping[11]],
            &self.row.columns[self.index_mapping[12]],
            &self.row.columns[self.index_mapping[13]],
            &self.row.columns[self.index_mapping[14]],
            &self.row.columns[self.index_mapping[15]],
            &self.row.columns[self.index_mapping[16]],
            &self.row.columns[self.index_mapping[17]],
            &self.row.columns[self.index_mapping[18]],
            &self.row.columns[self.index_mapping[19]],
            &self.row.columns[self.index_mapping[20]],
            &self.row.columns[self.index_mapping[21]],
            &self.row.columns[self.index_mapping[22]],
            &self.row.columns[self.index_mapping[23]],
            &self.row.columns[self.index_mapping[24]],
            &self.row.columns[self.index_mapping[25]],
            &self.row.columns[self.index_mapping[26]],
            &self.row.columns[self.index_mapping[27]],
            &self.row.columns[self.index_mapping[28]],
            &self.row.columns[self.index_mapping[29]],
            &self.row.columns[self.index_mapping[30]],
            &self.row.columns[self.index_mapping[31]],
            &self.row.columns[self.index_mapping[32]],
            &self.row.columns[self.index_mapping[33]],
            &self.row.columns[self.index_mapping[34]],
            &self.row.columns[self.index_mapping[35]],
            &self.row.columns[self.index_mapping[36]],
            &self.row.columns[self.index_mapping[37]],
            &self.row.columns[self.index_mapping[38]],
            &self.row.columns[self.index_mapping[39]],
            &self.row.columns[self.index_mapping[40]],
            &self.row.columns[self.index_mapping[41]],
            &self.row.columns[self.index_mapping[42]],
            &self.row.columns[self.index_mapping[43]],
            &self.row.columns[self.index_mapping[44]],
            &self.row.columns[self.index_mapping[45]],
            &self.row.columns[self.index_mapping[46]],
            &self.row.columns[self.index_mapping[47]],
            &self.row.columns[self.index_mapping[48]],
            &self.row.columns[self.index_mapping[49]],
            &self.row.columns[self.index_mapping[50]],
            &self.row.columns[self.index_mapping[51]],
            &self.row.columns[self.index_mapping[52]],
            &self.row.columns[self.index_mapping[53]],
            &self.row.columns[self.index_mapping[54]],
            &self.row.columns[self.index_mapping[55]],
            &self.row.columns[self.index_mapping[56]],
            &self.row.columns[self.index_mapping[57]],
            &self.row.columns[self.index_mapping[58]],
            &self.row.columns[self.index_mapping[59]],
            &self.row.columns[self.index_mapping[60]],
            &self.row.columns[self.index_mapping[61]],
            &self.row.columns[self.index_mapping[62]],
            &self.row.columns[self.index_mapping[63]],
            &self.row.columns[self.index_mapping[64]],
            &self.row.columns[self.index_mapping[65]],
            &self.row.columns[self.index_mapping[66]],
            &self.row.columns[self.index_mapping[67]],
            &self.row.columns[self.index_mapping[68]],
            &self.row.columns[self.index_mapping[69]],
            &self.row.columns[self.index_mapping[70]],
            &self.row.columns[self.index_mapping[71]],
            &self.row.columns[self.index_mapping[72]],
            &self.row.columns[self.index_mapping[73]],
            &self.row.columns[self.index_mapping[74]],
            &self.row.columns[self.index_mapping[75]],
            &self.row.columns[self.index_mapping[76]],
            &self.row.columns[self.index_mapping[77]],
            &self.row.columns[self.index_mapping[78]],
            &self.row.columns[self.index_mapping[79]],
            &self.row.columns[self.index_mapping[80]],
            &self.row.columns[self.index_mapping[81]],
            &self.row.columns[self.index_mapping[82]],
            &self.row.columns[self.index_mapping[83]],
            &self.row.columns[self.index_mapping[84]],
            &self.row.columns[self.index_mapping[85]],
            &self.row.columns[self.index_mapping[86]],
            &self.row.columns[self.index_mapping[87]],
            &self.row.columns[self.index_mapping[88]],
            &self.row.columns[self.index_mapping[89]],
            &self.row.columns[self.index_mapping[90]],
        ]
    }
}
