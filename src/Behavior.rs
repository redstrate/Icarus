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
pub struct BehaviorSheet {
    sheet: Sheet,
}
impl BehaviorSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Behavior")?;
        let sheet = resolver.read_excel_sheet(&exh, "Behavior", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BehaviorRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BehaviorRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for BehaviorSheet {
    type Row = BehaviorRow;
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
impl<'a> IntoIterator for &'a BehaviorSheet {
    type Item = (u32, Vec<(u16, BehaviorRow)>);
    type IntoIter = StructuredSheetIterator<'a, BehaviorSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BehaviorSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BehaviorRow {
    columns: Vec<Field>,
}
impl BehaviorRow {
    pub fn ContentArgument0<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn Balloon<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn Condition0Target<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
    pub fn Condition0Type<'a>(&'a self) -> &'a Field {
        &self.columns[11]
    }
    pub fn Condition1Target<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    pub fn Condition1Type<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
    pub fn ContentArgument1<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a Field {
        &self.columns[15]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a Field {
        &self.columns[16]
    }
}
