//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    common::Language,
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
impl StructuredSheet for AozActionTransientSheet {
    type Row = AozActionTransientRow;
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
impl<'a> IntoIterator for &'a AozActionTransientSheet {
    type Item = (u32, Vec<(u16, AozActionTransientRow)>);
    type IntoIter = StructuredSheetIterator<'a, AozActionTransientSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, AozActionTransientSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AozActionTransientRow {
    columns: Vec<Field>,
}
impl AozActionTransientRow {
    pub fn Stats<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn Description<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn Icon<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn RequiredForQuest<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn PreviousQuest<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn Location<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
    pub fn Number<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    pub fn LocationKey<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn TargetsEnemy<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn TargetsSelfOrAlly<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn CauseSlow<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
    pub fn CausePetrify<'a>(&'a self) -> &'a Field {
        &self.columns[11]
    }
    pub fn CauseParalysis<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    pub fn CauseInterrupt<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
    pub fn CauseBlind<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    pub fn CauseStun<'a>(&'a self) -> &'a Field {
        &self.columns[15]
    }
    pub fn CauseSleep<'a>(&'a self) -> &'a Field {
        &self.columns[16]
    }
    pub fn CauseBind<'a>(&'a self) -> &'a Field {
        &self.columns[17]
    }
    pub fn CauseHeavy<'a>(&'a self) -> &'a Field {
        &self.columns[18]
    }
    pub fn CauseDeath<'a>(&'a self) -> &'a Field {
        &self.columns[19]
    }
}
