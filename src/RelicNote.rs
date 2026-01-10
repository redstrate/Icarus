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
pub struct RelicNoteSheet {
    sheet: Sheet,
}
impl RelicNoteSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("RelicNote")?;
        let sheet = resolver.read_excel_sheet(&exh, "RelicNote", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<RelicNoteRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<RelicNoteRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for RelicNoteSheet {
    type Row = RelicNoteRow;
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
impl<'a> IntoIterator for &'a RelicNoteSheet {
    type Item = (u32, Vec<(u16, RelicNoteRow)>);
    type IntoIter = StructuredSheetIterator<'a, RelicNoteSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, RelicNoteSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct RelicNoteRow {
    columns: Vec<Field>,
}
impl RelicNoteRow {
    pub fn EventItem<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn MonsterNoteTargetCommon<'a>(&'a self) -> [&'a Field; 10] {
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
    pub fn MonsterNoteTargetNM<'a>(&'a self) -> [&'a Field; 3] {
        [&self.columns[11], &self.columns[12], &self.columns[13]]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    pub fn Fate<'a>(&'a self) -> [&'a Field; 3] {
        [&self.columns[15], &self.columns[16], &self.columns[17]]
    }
    pub fn PlaceNameFate<'a>(&'a self) -> [&'a Field; 3] {
        [&self.columns[18], &self.columns[19], &self.columns[20]]
    }
    pub fn Leve<'a>(&'a self) -> [&'a Field; 3] {
        [&self.columns[21], &self.columns[22], &self.columns[23]]
    }
    pub fn MonsterCount<'a>(&'a self) -> [&'a Field; 10] {
        [
            &self.columns[24],
            &self.columns[25],
            &self.columns[26],
            &self.columns[27],
            &self.columns[28],
            &self.columns[29],
            &self.columns[30],
            &self.columns[31],
            &self.columns[32],
            &self.columns[33],
        ]
    }
}
