//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
pub struct CutsceneMotionSheet {
    sheet: ExcelSheet,
}
impl CutsceneMotionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("CutsceneMotion")?;
        let sheet = resolver.read_excel_sheet(exh, "CutsceneMotion", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<CutsceneMotionRow> {
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
        Some(CutsceneMotionRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<CutsceneMotionRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<CutsceneMotionRow> {
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
pub struct CutsceneMotionRow {
    columns: Vec<ColumnData>,
}
impl CutsceneMotionRow {
    pub fn WALK_LOOP_SPEED<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn RUN_LOOP_SPEED<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn SLOWWALK_LOOP_SPEED<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn SLOWRUN_LOOP_SPEED<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn BATTLEWALK_LOOP_SPEED<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn BATTLERUN_LOOP_SPEED<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn DASH_LOOP_SPEED<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn TURN_CW90_FRAME<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn TURN_CCW90_FRAME<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn TURN_CW180_FRAME<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn TURN_CCW180_FRAME<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
}
