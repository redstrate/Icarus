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
pub struct ZoneSharedGroupSheet {
    sheet: Sheet,
}
impl ZoneSharedGroupSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ZoneSharedGroup")?;
        let sheet = resolver.read_excel_sheet(&exh, "ZoneSharedGroup", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ZoneSharedGroupRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ZoneSharedGroupRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for ZoneSharedGroupSheet {
    type Row = ZoneSharedGroupRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a ZoneSharedGroupSheet {
    type Item = (u32, Vec<(u16, ZoneSharedGroupRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, ZoneSharedGroupSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ZoneSharedGroupSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ZoneSharedGroupRow<'a> {
    row: &'a Row,
}
impl<'a> ZoneSharedGroupRow<'a> {
    pub fn LGBSharedGroup(&'a self) -> u32 {
        self.row.columns[0].into_u32().copied().unwrap()
    }
    pub fn RequirementRow(&'a self) -> [u32; 6] {
        [
            self.row.columns[2].into_u32().copied().unwrap(),
            self.row.columns[6].into_u32().copied().unwrap(),
            self.row.columns[10].into_u32().copied().unwrap(),
            self.row.columns[14].into_u32().copied().unwrap(),
            self.row.columns[18].into_u32().copied().unwrap(),
            self.row.columns[22].into_u32().copied().unwrap(),
        ]
    }
    pub fn Unknown0(&'a self) -> u32 {
        self.row.columns[26].into_u32().copied().unwrap()
    }
    pub fn RequirementQuestSequence(&'a self) -> [u32; 6] {
        [
            self.row.columns[3].into_u32().copied().unwrap(),
            self.row.columns[7].into_u32().copied().unwrap(),
            self.row.columns[11].into_u32().copied().unwrap(),
            self.row.columns[15].into_u32().copied().unwrap(),
            self.row.columns[19].into_u32().copied().unwrap(),
            self.row.columns[23].into_u32().copied().unwrap(),
        ]
    }
    pub fn Unknown1(&'a self) -> u32 {
        self.row.columns[27].into_u32().copied().unwrap()
    }
    /// 1 = Quest
    /// 2 = Quest with specific Sequence
    /// 3 = AetherCurrent
    /// 4 = EurekaStoryProgress
    /// 5 = DomaStoryProgress
    ///
    pub fn RequirementType(&'a self) -> [u8; 6] {
        [
            self.row.columns[1].into_u8().copied().unwrap(),
            self.row.columns[5].into_u8().copied().unwrap(),
            self.row.columns[9].into_u8().copied().unwrap(),
            self.row.columns[13].into_u8().copied().unwrap(),
            self.row.columns[17].into_u8().copied().unwrap(),
            self.row.columns[21].into_u8().copied().unwrap(),
        ]
    }
    pub fn Unknown8(&'a self) -> u8 {
        self.row.columns[25].into_u8().copied().unwrap()
    }
    pub fn Unknown9(&'a self) -> bool {
        self.row.columns[4].into_bool().copied().unwrap()
    }
    pub fn Unknown10(&'a self) -> bool {
        self.row.columns[8].into_bool().copied().unwrap()
    }
    pub fn Unknown11(&'a self) -> bool {
        self.row.columns[12].into_bool().copied().unwrap()
    }
    pub fn Unknown12(&'a self) -> bool {
        self.row.columns[16].into_bool().copied().unwrap()
    }
    pub fn Unknown13(&'a self) -> bool {
        self.row.columns[20].into_bool().copied().unwrap()
    }
    pub fn Unknown14(&'a self) -> bool {
        self.row.columns[24].into_bool().copied().unwrap()
    }
    pub fn Unknown15(&'a self) -> bool {
        self.row.columns[28].into_bool().copied().unwrap()
    }
}
