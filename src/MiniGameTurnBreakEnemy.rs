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
pub struct MiniGameTurnBreakEnemySheet {
    sheet: Sheet,
}
impl MiniGameTurnBreakEnemySheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MiniGameTurnBreakEnemy")?;
        let sheet = resolver.read_excel_sheet(&exh, "MiniGameTurnBreakEnemy", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<MiniGameTurnBreakEnemyRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<MiniGameTurnBreakEnemyRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for MiniGameTurnBreakEnemySheet {
    type Row = MiniGameTurnBreakEnemyRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a MiniGameTurnBreakEnemySheet {
    type Item = (u32, Vec<(u16, MiniGameTurnBreakEnemyRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, MiniGameTurnBreakEnemySheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MiniGameTurnBreakEnemySheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MiniGameTurnBreakEnemyRow<'a> {
    row: &'a Row,
}
impl<'a> MiniGameTurnBreakEnemyRow<'a> {
    pub fn Unknown0(&'a self) -> i32 {
        self.row.columns[28].into_i32().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> u8 {
        self.row.columns[20].into_u8().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> u8 {
        self.row.columns[24].into_u8().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> bool {
        self.row.columns[12].into_bool().copied().unwrap()
    }
    pub fn Unknown4(&'a self) -> bool {
        self.row.columns[16].into_bool().copied().unwrap()
    }
    pub fn Unknown5(&'a self) -> i32 {
        self.row.columns[29].into_i32().copied().unwrap()
    }
    pub fn Unknown6(&'a self) -> u8 {
        self.row.columns[21].into_u8().copied().unwrap()
    }
    pub fn Unknown7(&'a self) -> u8 {
        self.row.columns[25].into_u8().copied().unwrap()
    }
    pub fn Unknown8(&'a self) -> bool {
        self.row.columns[13].into_bool().copied().unwrap()
    }
    pub fn Unknown9(&'a self) -> bool {
        self.row.columns[17].into_bool().copied().unwrap()
    }
    pub fn Unknown10(&'a self) -> i32 {
        self.row.columns[30].into_i32().copied().unwrap()
    }
    pub fn Unknown11(&'a self) -> u8 {
        self.row.columns[22].into_u8().copied().unwrap()
    }
    pub fn Unknown12(&'a self) -> u8 {
        self.row.columns[26].into_u8().copied().unwrap()
    }
    pub fn Unknown13(&'a self) -> bool {
        self.row.columns[14].into_bool().copied().unwrap()
    }
    pub fn Unknown14(&'a self) -> bool {
        self.row.columns[18].into_bool().copied().unwrap()
    }
    pub fn Unknown15(&'a self) -> i32 {
        self.row.columns[31].into_i32().copied().unwrap()
    }
    pub fn Unknown16(&'a self) -> u8 {
        self.row.columns[23].into_u8().copied().unwrap()
    }
    pub fn Unknown17(&'a self) -> u8 {
        self.row.columns[27].into_u8().copied().unwrap()
    }
    pub fn Unknown18(&'a self) -> bool {
        self.row.columns[15].into_bool().copied().unwrap()
    }
    pub fn Unknown19(&'a self) -> bool {
        self.row.columns[19].into_bool().copied().unwrap()
    }
    pub fn Unknown20(&'a self) -> i32 {
        self.row.columns[3].into_i32().copied().unwrap()
    }
    pub fn Unknown21(&'a self) -> i32 {
        self.row.columns[7].into_i32().copied().unwrap()
    }
    pub fn Unknown22(&'a self) -> u8 {
        self.row.columns[0].into_u8().copied().unwrap()
    }
    pub fn Unknown23(&'a self) -> u8 {
        self.row.columns[1].into_u8().copied().unwrap()
    }
    pub fn Unknown24(&'a self) -> u8 {
        self.row.columns[2].into_u8().copied().unwrap()
    }
    pub fn Unknown25(&'a self) -> u8 {
        self.row.columns[5].into_u8().copied().unwrap()
    }
    pub fn Unknown26(&'a self) -> u8 {
        self.row.columns[6].into_u8().copied().unwrap()
    }
    pub fn Unknown27(&'a self) -> u8 {
        self.row.columns[8].into_u8().copied().unwrap()
    }
    pub fn Unknown28(&'a self) -> u8 {
        self.row.columns[9].into_u8().copied().unwrap()
    }
    pub fn Unknown29(&'a self) -> u8 {
        self.row.columns[10].into_u8().copied().unwrap()
    }
    pub fn Unknown30(&'a self) -> u8 {
        self.row.columns[11].into_u8().copied().unwrap()
    }
    pub fn Unknown31(&'a self) -> u8 {
        self.row.columns[32].into_u8().copied().unwrap()
    }
    pub fn Unknown32(&'a self) -> u8 {
        self.row.columns[33].into_u8().copied().unwrap()
    }
    pub fn Unknown33(&'a self) -> u8 {
        self.row.columns[34].into_u8().copied().unwrap()
    }
    pub fn Unknown34(&'a self) -> bool {
        self.row.columns[4].into_bool().copied().unwrap()
    }
}
