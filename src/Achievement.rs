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
pub struct AchievementSheet {
    sheet: Sheet,
}
impl AchievementSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Achievement")?;
        let sheet = resolver.read_excel_sheet(&exh, "Achievement", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<AchievementRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<AchievementRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for AchievementSheet {
    type Row = AchievementRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a AchievementSheet {
    type Item = (u32, Vec<(u16, AchievementRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, AchievementSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, AchievementSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AchievementRow<'a> {
    row: &'a Row,
}
impl<'a> AchievementRow<'a> {
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[1].into_string().unwrap()
    }
    pub fn Description(&'a self) -> &'a str {
        self.row.columns[2].into_string().unwrap()
    }
    pub fn Item(&'a self) -> u32 {
        self.row.columns[7].into_u32().copied().unwrap()
    }
    pub fn Icon(&'a self) -> u32 {
        self.row.columns[11].into_u32().copied().unwrap()
    }
    pub fn Key(&'a self) -> i32 {
        self.row.columns[14].into_i32().copied().unwrap()
    }
    pub fn Data(&'a self) -> [i32; 8] {
        [
            self.row.columns[15].into_i32().copied().unwrap(),
            self.row.columns[16].into_i32().copied().unwrap(),
            self.row.columns[17].into_i32().copied().unwrap(),
            self.row.columns[18].into_i32().copied().unwrap(),
            self.row.columns[19].into_i32().copied().unwrap(),
            self.row.columns[20].into_i32().copied().unwrap(),
            self.row.columns[21].into_i32().copied().unwrap(),
            self.row.columns[22].into_i32().copied().unwrap(),
        ]
    }
    pub fn Title(&'a self) -> u16 {
        self.row.columns[6].into_u16().copied().unwrap()
    }
    pub fn Order(&'a self) -> u16 {
        self.row.columns[23].into_u16().copied().unwrap()
    }
    pub fn AchievementCategory(&'a self) -> u8 {
        self.row.columns[0].into_u8().copied().unwrap()
    }
    pub fn AchievementTarget(&'a self) -> u8 {
        self.row.columns[3].into_u8().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> u8 {
        self.row.columns[4].into_u8().copied().unwrap()
    }
    pub fn Points(&'a self) -> u8 {
        self.row.columns[5].into_u8().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> u8 {
        self.row.columns[8].into_u8().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> u8 {
        self.row.columns[9].into_u8().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> u8 {
        self.row.columns[10].into_u8().copied().unwrap()
    }
    pub fn Unknown4(&'a self) -> u8 {
        self.row.columns[12].into_u8().copied().unwrap()
    }
    pub fn Type(&'a self) -> u8 {
        self.row.columns[13].into_u8().copied().unwrap()
    }
    pub fn Unknown5(&'a self) -> u8 {
        self.row.columns[24].into_u8().copied().unwrap()
    }
    pub fn AchievementHideCondition(&'a self) -> u8 {
        self.row.columns[25].into_u8().copied().unwrap()
    }
}
