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
pub struct CraftActionSheet {
    sheet: Sheet,
}
impl CraftActionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("CraftAction")?;
        let sheet = resolver.read_excel_sheet(&exh, "CraftAction", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<CraftActionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<CraftActionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for CraftActionSheet {
    type Row = CraftActionRow;
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
impl<'a> IntoIterator for &'a CraftActionSheet {
    type Item = (u32, Vec<(u16, CraftActionRow)>);
    type IntoIter = StructuredSheetIterator<'a, CraftActionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, CraftActionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct CraftActionRow {
    columns: Vec<Field>,
}
impl CraftActionRow {
    pub fn Name<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn Description<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn QuestRequirement<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn CRP<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn BSM<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn ARM<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
    pub fn GSM<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    pub fn LTW<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn WVR<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn ALC<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn CUL<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
    pub fn AnimationStart<'a>(&'a self) -> &'a Field {
        &self.columns[11]
    }
    pub fn AnimationEnd<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    pub fn Icon<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
    pub fn RequiredStatus<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    pub fn ClassJobCategory<'a>(&'a self) -> &'a Field {
        &self.columns[15]
    }
    pub fn ClassJobLevel<'a>(&'a self) -> &'a Field {
        &self.columns[16]
    }
    pub fn Cost<'a>(&'a self) -> &'a Field {
        &self.columns[17]
    }
    pub fn ClassJob<'a>(&'a self) -> &'a Field {
        &self.columns[18]
    }
    pub fn Specialist<'a>(&'a self) -> &'a Field {
        &self.columns[19]
    }
}
