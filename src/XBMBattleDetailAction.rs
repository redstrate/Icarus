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
pub struct XBMBattleDetailActionSheet {
    sheet: Sheet,
}
impl XBMBattleDetailActionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("XBMBattleDetailAction")?;
        let sheet = resolver.read_excel_sheet(&exh, "XBMBattleDetailAction", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<XBMBattleDetailActionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<XBMBattleDetailActionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for XBMBattleDetailActionSheet {
    type Row = XBMBattleDetailActionRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Action: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
            Status: row
                .columns[3]
                .into_u32()
                .copied()
                .expect("Expected column 3 to be a uint32!"),
            ActionTarget: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            ActionEffectType: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a XBMBattleDetailActionSheet {
    type Item = (u32, Vec<(u16, XBMBattleDetailActionRow)>);
    type IntoIter = StructuredSheetIterator<'a, XBMBattleDetailActionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, XBMBattleDetailActionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct XBMBattleDetailActionRow {
    ///""
    pub Action: u32,
    ///""
    pub Status: u32,
    ///""
    pub ActionTarget: u8,
    ///""
    pub ActionEffectType: u8,
}
