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
pub struct ContentRewardConditionSheet {
    sheet: Sheet,
}
impl ContentRewardConditionSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ContentRewardCondition")?;
        let sheet = resolver.read_excel_sheet(&exh, "ContentRewardCondition", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ContentRewardConditionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<ContentRewardConditionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for ContentRewardConditionSheet {
    type Row = ContentRewardConditionRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            MaxLevelCondition: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a ContentRewardConditionSheet {
    type Item = (u32, Vec<(u16, ContentRewardConditionRow)>);
    type IntoIter = StructuredSheetIterator<'a, ContentRewardConditionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ContentRewardConditionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ContentRewardConditionRow {
    ///"0 = Always rewarded\n /// 1 = Character has reached current expansions max level\n /// 2 = Character has reached at least 50\n /// 3 = Character has reached at least 60\n /// 4 = Character has reached at least 70\n /// 5 = Character has reached at least 80\n /// 6 = Character has reached at least 90\n /// "
    pub MaxLevelCondition: u8,
}
