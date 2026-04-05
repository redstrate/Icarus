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
pub struct AozActionTransientSheet {
    sheet: Sheet,
}
impl AozActionTransientSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("AozActionTransient")?;
        let sheet = resolver.read_excel_sheet(&exh, "AozActionTransient", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<AozActionTransientRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<AozActionTransientRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for AozActionTransientSheet {
    type Row = AozActionTransientRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a AozActionTransientSheet {
    type Item = (u32, Vec<(u16, AozActionTransientRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, AozActionTransientSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, AozActionTransientSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AozActionTransientRow<'a> {
    row: &'a Row,
}
impl<'a> AozActionTransientRow<'a> {
    pub fn Stats(&'a self) -> &'a str {
        self.row.columns[2].into_string().unwrap()
    }
    pub fn Description(&'a self) -> &'a str {
        self.row.columns[3].into_string().unwrap()
    }
    pub fn Icon(&'a self) -> u32 {
        self.row.columns[1].into_u32().copied().unwrap()
    }
    pub fn RequiredForQuest(&'a self) -> u32 {
        self.row.columns[6].into_u32().copied().unwrap()
    }
    pub fn PreviousQuest(&'a self) -> u32 {
        self.row.columns[7].into_u32().copied().unwrap()
    }
    pub fn Location(&'a self) -> u16 {
        self.row.columns[5].into_u16().copied().unwrap()
    }
    pub fn Number(&'a self) -> u8 {
        self.row.columns[0].into_u8().copied().unwrap()
    }
    pub fn LocationKey(&'a self) -> u8 {
        self.row.columns[4].into_u8().copied().unwrap()
    }
    pub fn TargetsEnemy(&'a self) -> bool {
        self.row.columns[8].into_bool().copied().unwrap()
    }
    pub fn TargetsSelfOrAlly(&'a self) -> bool {
        self.row.columns[9].into_bool().copied().unwrap()
    }
    pub fn CauseSlow(&'a self) -> bool {
        self.row.columns[10].into_bool().copied().unwrap()
    }
    pub fn CausePetrify(&'a self) -> bool {
        self.row.columns[11].into_bool().copied().unwrap()
    }
    pub fn CauseParalysis(&'a self) -> bool {
        self.row.columns[12].into_bool().copied().unwrap()
    }
    pub fn CauseInterrupt(&'a self) -> bool {
        self.row.columns[13].into_bool().copied().unwrap()
    }
    pub fn CauseBlind(&'a self) -> bool {
        self.row.columns[14].into_bool().copied().unwrap()
    }
    pub fn CauseStun(&'a self) -> bool {
        self.row.columns[15].into_bool().copied().unwrap()
    }
    pub fn CauseSleep(&'a self) -> bool {
        self.row.columns[16].into_bool().copied().unwrap()
    }
    pub fn CauseBind(&'a self) -> bool {
        self.row.columns[17].into_bool().copied().unwrap()
    }
    pub fn CauseHeavy(&'a self) -> bool {
        self.row.columns[18].into_bool().copied().unwrap()
    }
    pub fn CauseDeath(&'a self) -> bool {
        self.row.columns[19].into_bool().copied().unwrap()
    }
}
