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
pub struct EmoteSheet {
    sheet: Sheet,
}
impl EmoteSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Emote")?;
        let sheet = resolver.read_excel_sheet(&exh, "Emote", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<EmoteRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<EmoteRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for EmoteSheet {
    type Row = EmoteRow;
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
impl<'a> IntoIterator for &'a EmoteSheet {
    type Item = (u32, Vec<(u16, EmoteRow)>);
    type IntoIter = StructuredSheetIterator<'a, EmoteSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, EmoteSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct EmoteRow {
    columns: Vec<Field>,
}
impl EmoteRow {
    pub fn Name<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn Icon<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn UnlockLink<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn TextCommand<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn ActionTimeline<'a>(&'a self) -> [&'a Field; 7] {
        [
            &self.columns[4],
            &self.columns[5],
            &self.columns[6],
            &self.columns[7],
            &self.columns[8],
            &self.columns[9],
            &self.columns[10],
        ]
    }
    pub fn Order<'a>(&'a self) -> &'a Field {
        &self.columns[11]
    }
    pub fn LogMessageTargeted<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    pub fn LogMessageUntargeted<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
    pub fn Patch<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    pub fn EmoteCategory<'a>(&'a self) -> &'a Field {
        &self.columns[15]
    }
    pub fn EmoteMode<'a>(&'a self) -> &'a Field {
        &self.columns[16]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a Field {
        &self.columns[17]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a Field {
        &self.columns[18]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a Field {
        &self.columns[19]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a Field {
        &self.columns[20]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a Field {
        &self.columns[21]
    }
    pub fn HasCancelEmote<'a>(&'a self) -> &'a Field {
        &self.columns[22]
    }
    pub fn DrawsWeapon<'a>(&'a self) -> &'a Field {
        &self.columns[23]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a Field {
        &self.columns[24]
    }
}
