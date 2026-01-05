//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
#[derive(Debug, Clone)]
pub struct ConfigKeySheet {
    sheet: ExcelSheet,
}
impl ConfigKeySheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ConfigKey")?;
        let sheet = resolver.read_excel_sheet(&exh, "ConfigKey", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<ConfigKeyRow> {
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(ConfigKeyRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<ConfigKeyRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<ConfigKeyRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => return None,
            ExcelRowKind::SubRows(subrows) => {
                &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1
            }
        };
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
pub struct ConfigKeyRow {
    columns: Vec<ColumnData>,
}
impl ConfigKeyRow {
    pub fn Text<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn Label<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn Param<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn Platform<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn Category<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn BacklightColor<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn Required<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
}
