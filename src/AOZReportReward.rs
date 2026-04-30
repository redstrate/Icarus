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
pub struct AOZReportRewardSheet {
    sheet: Sheet,
}
impl AOZReportRewardSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("AOZReportReward")?;
        let sheet = resolver.read_excel_sheet(&exh, "AOZReportReward", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<AOZReportRewardRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<AOZReportRewardRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for AOZReportRewardSheet {
    type Row = AOZReportRewardRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            FirstGilReward: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
            GeneralGilReward: row
                .columns[3]
                .into_u32()
                .copied()
                .expect("Expected column 3 to be a uint32!"),
            FirstAlliedSealsReward: row
                .columns[1]
                .into_u16()
                .copied()
                .expect("Expected column 1 to be a uint16!"),
            FirstTomestonesReward: row
                .columns[2]
                .into_u16()
                .copied()
                .expect("Expected column 2 to be a uint16!"),
            GeneralAlliedSealsReward: row
                .columns[4]
                .into_u16()
                .copied()
                .expect("Expected column 4 to be a uint16!"),
            GeneralTomestonesReward: row
                .columns[5]
                .into_u16()
                .copied()
                .expect("Expected column 5 to be a uint16!"),
        })
    }
}
impl<'a> IntoIterator for &'a AOZReportRewardSheet {
    type Item = (u32, Vec<(u16, AOZReportRewardRow)>);
    type IntoIter = StructuredSheetIterator<'a, AOZReportRewardSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, AOZReportRewardSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AOZReportRewardRow {
    ///""
    pub FirstGilReward: u32,
    ///""
    pub GeneralGilReward: u32,
    ///""
    pub FirstAlliedSealsReward: u16,
    ///""
    pub FirstTomestonesReward: u16,
    ///""
    pub GeneralAlliedSealsReward: u16,
    ///""
    pub GeneralTomestonesReward: u16,
}
