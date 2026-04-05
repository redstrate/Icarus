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
pub struct WKSDevGradeBGSetSheet {
    sheet: Sheet,
}
impl WKSDevGradeBGSetSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WKSDevGradeBGSet")?;
        let sheet = resolver.read_excel_sheet(&exh, "WKSDevGradeBGSet", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<WKSDevGradeBGSetRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<WKSDevGradeBGSetRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for WKSDevGradeBGSetSheet {
    type Row = WKSDevGradeBGSetRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a WKSDevGradeBGSetSheet {
    type Item = (u32, Vec<(u16, WKSDevGradeBGSetRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, WKSDevGradeBGSetSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WKSDevGradeBGSetSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct WKSDevGradeBGSetRow<'a> {
    row: &'a Row,
}
impl<'a> WKSDevGradeBGSetRow<'a> {
    pub fn Unknown0(&'a self) -> u16 {
        self.row.columns[0].into_u16().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> bool {
        self.row.columns[1].into_bool().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> bool {
        self.row.columns[2].into_bool().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> bool {
        self.row.columns[3].into_bool().copied().unwrap()
    }
    pub fn Unknown4(&'a self) -> bool {
        self.row.columns[4].into_bool().copied().unwrap()
    }
    pub fn Unknown5(&'a self) -> bool {
        self.row.columns[5].into_bool().copied().unwrap()
    }
    pub fn Unknown6(&'a self) -> bool {
        self.row.columns[6].into_bool().copied().unwrap()
    }
    pub fn Unknown7(&'a self) -> bool {
        self.row.columns[7].into_bool().copied().unwrap()
    }
    pub fn Unknown8(&'a self) -> bool {
        self.row.columns[8].into_bool().copied().unwrap()
    }
    pub fn Unknown9(&'a self) -> bool {
        self.row.columns[9].into_bool().copied().unwrap()
    }
    pub fn Unknown10(&'a self) -> bool {
        self.row.columns[10].into_bool().copied().unwrap()
    }
    pub fn Unknown11(&'a self) -> bool {
        self.row.columns[11].into_bool().copied().unwrap()
    }
    pub fn Unknown12(&'a self) -> bool {
        self.row.columns[12].into_bool().copied().unwrap()
    }
    pub fn Unknown13(&'a self) -> bool {
        self.row.columns[13].into_bool().copied().unwrap()
    }
    pub fn Unknown14(&'a self) -> bool {
        self.row.columns[14].into_bool().copied().unwrap()
    }
    pub fn Unknown15(&'a self) -> bool {
        self.row.columns[15].into_bool().copied().unwrap()
    }
    pub fn Unknown16(&'a self) -> bool {
        self.row.columns[16].into_bool().copied().unwrap()
    }
    pub fn Unknown17(&'a self) -> bool {
        self.row.columns[17].into_bool().copied().unwrap()
    }
    pub fn Unknown18(&'a self) -> bool {
        self.row.columns[18].into_bool().copied().unwrap()
    }
    pub fn Unknown19(&'a self) -> bool {
        self.row.columns[19].into_bool().copied().unwrap()
    }
    pub fn Unknown20(&'a self) -> bool {
        self.row.columns[20].into_bool().copied().unwrap()
    }
    pub fn Unknown21(&'a self) -> bool {
        self.row.columns[21].into_bool().copied().unwrap()
    }
    pub fn Unknown22(&'a self) -> bool {
        self.row.columns[22].into_bool().copied().unwrap()
    }
    pub fn Unknown23(&'a self) -> bool {
        self.row.columns[23].into_bool().copied().unwrap()
    }
    pub fn Unknown24(&'a self) -> bool {
        self.row.columns[24].into_bool().copied().unwrap()
    }
    pub fn Unknown25(&'a self) -> bool {
        self.row.columns[25].into_bool().copied().unwrap()
    }
    pub fn Unknown26(&'a self) -> bool {
        self.row.columns[26].into_bool().copied().unwrap()
    }
    pub fn Unknown27(&'a self) -> bool {
        self.row.columns[27].into_bool().copied().unwrap()
    }
    pub fn Unknown28(&'a self) -> bool {
        self.row.columns[28].into_bool().copied().unwrap()
    }
    pub fn Unknown29(&'a self) -> bool {
        self.row.columns[29].into_bool().copied().unwrap()
    }
    pub fn Unknown30(&'a self) -> bool {
        self.row.columns[30].into_bool().copied().unwrap()
    }
    pub fn Unknown31(&'a self) -> bool {
        self.row.columns[31].into_bool().copied().unwrap()
    }
    pub fn Unknown32(&'a self) -> bool {
        self.row.columns[32].into_bool().copied().unwrap()
    }
    pub fn Unknown33(&'a self) -> bool {
        self.row.columns[33].into_bool().copied().unwrap()
    }
    pub fn Unknown34(&'a self) -> bool {
        self.row.columns[34].into_bool().copied().unwrap()
    }
    pub fn Unknown35(&'a self) -> bool {
        self.row.columns[35].into_bool().copied().unwrap()
    }
    pub fn Unknown36(&'a self) -> bool {
        self.row.columns[36].into_bool().copied().unwrap()
    }
    pub fn Unknown37(&'a self) -> bool {
        self.row.columns[37].into_bool().copied().unwrap()
    }
    pub fn Unknown38(&'a self) -> bool {
        self.row.columns[38].into_bool().copied().unwrap()
    }
    pub fn Unknown39(&'a self) -> bool {
        self.row.columns[39].into_bool().copied().unwrap()
    }
    pub fn Unknown40(&'a self) -> bool {
        self.row.columns[40].into_bool().copied().unwrap()
    }
}
