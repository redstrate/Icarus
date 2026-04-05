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
pub struct GatheringNotebookListSheet {
    sheet: Sheet,
}
impl GatheringNotebookListSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GatheringNotebookList")?;
        let sheet = resolver.read_excel_sheet(&exh, "GatheringNotebookList", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<GatheringNotebookListRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<GatheringNotebookListRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for GatheringNotebookListSheet {
    type Row = GatheringNotebookListRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a GatheringNotebookListSheet {
    type Item = (u32, Vec<(u16, GatheringNotebookListRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, GatheringNotebookListSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GatheringNotebookListSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct GatheringNotebookListRow<'a> {
    row: &'a Row,
}
impl<'a> GatheringNotebookListRow<'a> {
    pub fn GatheringItem(&'a self) -> [i32; 100] {
        [
            self.row.columns[1].into_i32().copied().unwrap(),
            self.row.columns[2].into_i32().copied().unwrap(),
            self.row.columns[3].into_i32().copied().unwrap(),
            self.row.columns[4].into_i32().copied().unwrap(),
            self.row.columns[5].into_i32().copied().unwrap(),
            self.row.columns[6].into_i32().copied().unwrap(),
            self.row.columns[7].into_i32().copied().unwrap(),
            self.row.columns[8].into_i32().copied().unwrap(),
            self.row.columns[9].into_i32().copied().unwrap(),
            self.row.columns[10].into_i32().copied().unwrap(),
            self.row.columns[11].into_i32().copied().unwrap(),
            self.row.columns[12].into_i32().copied().unwrap(),
            self.row.columns[13].into_i32().copied().unwrap(),
            self.row.columns[14].into_i32().copied().unwrap(),
            self.row.columns[15].into_i32().copied().unwrap(),
            self.row.columns[16].into_i32().copied().unwrap(),
            self.row.columns[17].into_i32().copied().unwrap(),
            self.row.columns[18].into_i32().copied().unwrap(),
            self.row.columns[19].into_i32().copied().unwrap(),
            self.row.columns[20].into_i32().copied().unwrap(),
            self.row.columns[21].into_i32().copied().unwrap(),
            self.row.columns[22].into_i32().copied().unwrap(),
            self.row.columns[23].into_i32().copied().unwrap(),
            self.row.columns[24].into_i32().copied().unwrap(),
            self.row.columns[25].into_i32().copied().unwrap(),
            self.row.columns[26].into_i32().copied().unwrap(),
            self.row.columns[27].into_i32().copied().unwrap(),
            self.row.columns[28].into_i32().copied().unwrap(),
            self.row.columns[29].into_i32().copied().unwrap(),
            self.row.columns[30].into_i32().copied().unwrap(),
            self.row.columns[31].into_i32().copied().unwrap(),
            self.row.columns[32].into_i32().copied().unwrap(),
            self.row.columns[33].into_i32().copied().unwrap(),
            self.row.columns[34].into_i32().copied().unwrap(),
            self.row.columns[35].into_i32().copied().unwrap(),
            self.row.columns[36].into_i32().copied().unwrap(),
            self.row.columns[37].into_i32().copied().unwrap(),
            self.row.columns[38].into_i32().copied().unwrap(),
            self.row.columns[39].into_i32().copied().unwrap(),
            self.row.columns[40].into_i32().copied().unwrap(),
            self.row.columns[41].into_i32().copied().unwrap(),
            self.row.columns[42].into_i32().copied().unwrap(),
            self.row.columns[43].into_i32().copied().unwrap(),
            self.row.columns[44].into_i32().copied().unwrap(),
            self.row.columns[45].into_i32().copied().unwrap(),
            self.row.columns[46].into_i32().copied().unwrap(),
            self.row.columns[47].into_i32().copied().unwrap(),
            self.row.columns[48].into_i32().copied().unwrap(),
            self.row.columns[49].into_i32().copied().unwrap(),
            self.row.columns[50].into_i32().copied().unwrap(),
            self.row.columns[51].into_i32().copied().unwrap(),
            self.row.columns[52].into_i32().copied().unwrap(),
            self.row.columns[53].into_i32().copied().unwrap(),
            self.row.columns[54].into_i32().copied().unwrap(),
            self.row.columns[55].into_i32().copied().unwrap(),
            self.row.columns[56].into_i32().copied().unwrap(),
            self.row.columns[57].into_i32().copied().unwrap(),
            self.row.columns[58].into_i32().copied().unwrap(),
            self.row.columns[59].into_i32().copied().unwrap(),
            self.row.columns[60].into_i32().copied().unwrap(),
            self.row.columns[61].into_i32().copied().unwrap(),
            self.row.columns[62].into_i32().copied().unwrap(),
            self.row.columns[63].into_i32().copied().unwrap(),
            self.row.columns[64].into_i32().copied().unwrap(),
            self.row.columns[65].into_i32().copied().unwrap(),
            self.row.columns[66].into_i32().copied().unwrap(),
            self.row.columns[67].into_i32().copied().unwrap(),
            self.row.columns[68].into_i32().copied().unwrap(),
            self.row.columns[69].into_i32().copied().unwrap(),
            self.row.columns[70].into_i32().copied().unwrap(),
            self.row.columns[71].into_i32().copied().unwrap(),
            self.row.columns[72].into_i32().copied().unwrap(),
            self.row.columns[73].into_i32().copied().unwrap(),
            self.row.columns[74].into_i32().copied().unwrap(),
            self.row.columns[75].into_i32().copied().unwrap(),
            self.row.columns[76].into_i32().copied().unwrap(),
            self.row.columns[77].into_i32().copied().unwrap(),
            self.row.columns[78].into_i32().copied().unwrap(),
            self.row.columns[79].into_i32().copied().unwrap(),
            self.row.columns[80].into_i32().copied().unwrap(),
            self.row.columns[81].into_i32().copied().unwrap(),
            self.row.columns[82].into_i32().copied().unwrap(),
            self.row.columns[83].into_i32().copied().unwrap(),
            self.row.columns[84].into_i32().copied().unwrap(),
            self.row.columns[85].into_i32().copied().unwrap(),
            self.row.columns[86].into_i32().copied().unwrap(),
            self.row.columns[87].into_i32().copied().unwrap(),
            self.row.columns[88].into_i32().copied().unwrap(),
            self.row.columns[89].into_i32().copied().unwrap(),
            self.row.columns[90].into_i32().copied().unwrap(),
            self.row.columns[91].into_i32().copied().unwrap(),
            self.row.columns[92].into_i32().copied().unwrap(),
            self.row.columns[93].into_i32().copied().unwrap(),
            self.row.columns[94].into_i32().copied().unwrap(),
            self.row.columns[95].into_i32().copied().unwrap(),
            self.row.columns[96].into_i32().copied().unwrap(),
            self.row.columns[97].into_i32().copied().unwrap(),
            self.row.columns[98].into_i32().copied().unwrap(),
            self.row.columns[99].into_i32().copied().unwrap(),
            self.row.columns[100].into_i32().copied().unwrap(),
        ]
    }
    pub fn Unknown0(&'a self) -> u8 {
        self.row.columns[0].into_u8().copied().unwrap()
    }
}
