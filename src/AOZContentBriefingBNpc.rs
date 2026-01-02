//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
pub struct AOZContentBriefingBNpcSheet {
    sheet: ExcelSheet,
}
impl AOZContentBriefingBNpcSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("AOZContentBriefingBNpc")?;
        let sheet = resolver.read_excel_sheet(exh, "AOZContentBriefingBNpc", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<AOZContentBriefingBNpcRow> {
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
        Some(AOZContentBriefingBNpcRow {
            columns,
        })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<AOZContentBriefingBNpcRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<AOZContentBriefingBNpcRow> {
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
pub struct AOZContentBriefingBNpcRow {
    columns: Vec<ColumnData>,
}
impl AOZContentBriefingBNpcRow {
    pub fn BNpcName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn TargetSmall<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn TargetLarge<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn Endurance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn Fire<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn Ice<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn Wind<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn Earth<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn Thunder<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn Water<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn Slashing<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn Piercing<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn Blunt<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn Magic<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn HideStats<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn SlowVuln<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn PetrificationVuln<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn ParalysisVuln<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn InterruptionVuln<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn BlindVuln<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn StunVuln<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn SleepVuln<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn BindVuln<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
    pub fn HeavyVuln<'a>(&'a self) -> &'a ColumnData {
        &self.columns[23]
    }
    pub fn FlatOrDeathVuln<'a>(&'a self) -> &'a ColumnData {
        &self.columns[24]
    }
}
