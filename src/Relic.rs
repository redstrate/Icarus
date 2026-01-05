//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
#[derive(Debug, Clone)]
pub struct RelicSheet {
    sheet: ExcelSheet,
}
impl RelicSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Relic")?;
        let sheet = resolver.read_excel_sheet(&exh, "Relic", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<RelicRow> {
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
        Some(RelicRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<RelicRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<RelicRow> {
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
pub struct RelicRow {
    columns: Vec<ColumnData>,
}
impl RelicRow {
    pub fn ItemAtma<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn ItemAnimus<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn Icon<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn Materia0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn Materia1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn Materia2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn Materia3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn NoteMain0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn NoteSub0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn NoteSelection10<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn NoteMain1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn NoteSub1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn NoteSelection1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn NoteMain2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn NoteSub2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn NoteSelection3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
}
