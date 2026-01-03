//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
#[derive(Debug, Clone)]
pub struct MKDSupportJobSheet {
    sheet: ExcelSheet,
}
impl MKDSupportJobSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MKDSupportJob")?;
        let sheet = resolver.read_excel_sheet(exh, "MKDSupportJob", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<MKDSupportJobRow> {
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
        Some(MKDSupportJobRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<MKDSupportJobRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<MKDSupportJobRow> {
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
pub struct MKDSupportJobRow {
    columns: Vec<ColumnData>,
}
impl MKDSupportJobRow {
    pub fn Name<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn NameShort<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn NameFemale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn Description<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn NameEnglish<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn Action<'a>(&'a self) -> [&'a ColumnData; 5] {
        [
            &self.columns[5],
            &self.columns[6],
            &self.columns[7],
            &self.columns[8],
            &self.columns[9],
        ]
    }
    pub fn LevelMax<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn JobIndex<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn LevelUnlock<'a>(&'a self) -> [&'a ColumnData; 5] {
        [
            &self.columns[12],
            &self.columns[13],
            &self.columns[14],
            &self.columns[15],
            &self.columns[16],
        ]
    }
}
