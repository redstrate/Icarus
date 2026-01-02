//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
pub struct ActionTimelineSheet {
    sheet: ExcelSheet,
}
impl ActionTimelineSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ActionTimeline")?;
        let sheet = resolver.read_excel_sheet(exh, "ActionTimeline", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<ActionTimelineRow> {
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
        Some(ActionTimelineRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<ActionTimelineRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<ActionTimelineRow> {
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
pub struct ActionTimelineRow {
    columns: Vec<ColumnData>,
}
impl ActionTimelineRow {
    pub fn Key<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn WeaponTimeline<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn KillUpper<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn Unknown_70<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn Type<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn Priority<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn Stance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn Slot<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn LookAtMode<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn ActionTimelineIDMode<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn LoadType<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn StartAttach<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn ResidentPap<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn Pause<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn Resident<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn IsMotionCanceledByMoving<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn IsLoop<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
}
