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
pub struct GoldSaucerTalkSheet {
    sheet: Sheet,
}
impl GoldSaucerTalkSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GoldSaucerTalk")?;
        let sheet = resolver.read_excel_sheet(&exh, "GoldSaucerTalk", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<GoldSaucerTalkRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<GoldSaucerTalkRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for GoldSaucerTalkSheet {
    type Row = GoldSaucerTalkRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<Field>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(Self::Row { columns })
    }
}
impl<'a> IntoIterator for &'a GoldSaucerTalkSheet {
    type Item = (u32, Vec<(u16, GoldSaucerTalkRow)>);
    type IntoIter = StructuredSheetIterator<'a, GoldSaucerTalkSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GoldSaucerTalkSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct GoldSaucerTalkRow {
    columns: Vec<Field>,
}
impl GoldSaucerTalkRow {
    pub fn Message<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn ChoicesText<'a>(&'a self) -> [&'a Field; 10] {
        [
            &self.columns[1],
            &self.columns[2],
            &self.columns[3],
            &self.columns[4],
            &self.columns[5],
            &self.columns[6],
            &self.columns[7],
            &self.columns[8],
            &self.columns[9],
            &self.columns[10],
        ]
    }
    /// The next GoldSaucerTalk message.
    pub fn NextTalk<'a>(&'a self) -> &'a Field {
        &self.columns[11]
    }
    pub fn ActionTimeline<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    /// The next GoldSaucerTalk for the ChoicesText of the same index.
    pub fn ChoicesTalk<'a>(&'a self) -> [&'a Field; 10] {
        [
            &self.columns[13],
            &self.columns[14],
            &self.columns[15],
            &self.columns[16],
            &self.columns[17],
            &self.columns[18],
            &self.columns[19],
            &self.columns[20],
            &self.columns[21],
            &self.columns[22],
        ]
    }
    pub fn Unknown23<'a>(&'a self) -> &'a Field {
        &self.columns[23]
    }
    pub fn Unknown24<'a>(&'a self) -> &'a Field {
        &self.columns[24]
    }
    pub fn Unknown25<'a>(&'a self) -> &'a Field {
        &self.columns[25]
    }
    pub fn Unknown26<'a>(&'a self) -> &'a Field {
        &self.columns[26]
    }
    pub fn Unknown27<'a>(&'a self) -> &'a Field {
        &self.columns[27]
    }
}
