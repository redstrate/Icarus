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
pub struct AnimaWeapon5SpiritTalkTypeSheet {
    sheet: Sheet,
}
impl AnimaWeapon5SpiritTalkTypeSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("AnimaWeapon5SpiritTalkType")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "AnimaWeapon5SpiritTalkType", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<AnimaWeapon5SpiritTalkTypeRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<AnimaWeapon5SpiritTalkTypeRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for AnimaWeapon5SpiritTalkTypeSheet {
    type Row = AnimaWeapon5SpiritTalkTypeRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a AnimaWeapon5SpiritTalkTypeSheet {
    type Item = (u32, Vec<(u16, AnimaWeapon5SpiritTalkTypeRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, AnimaWeapon5SpiritTalkTypeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, AnimaWeapon5SpiritTalkTypeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AnimaWeapon5SpiritTalkTypeRow<'a> {
    row: &'a Row,
}
impl<'a> AnimaWeapon5SpiritTalkTypeRow<'a> {
    pub fn Unknown0(&'a self) -> u8 {
        self.row.columns[0].into_u8().copied().unwrap()
    }
}
