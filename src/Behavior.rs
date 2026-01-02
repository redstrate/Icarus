//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
pub struct BehaviorSheet {
    sheet: ExcelSheet,
}
impl BehaviorSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Behavior")?;
        let sheet = resolver.read_excel_sheet(exh, "Behavior", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<BehaviorRow> {
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
        Some(BehaviorRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<BehaviorRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<BehaviorRow> {
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
pub struct BehaviorRow {
    columns: Vec<ColumnData>,
}
impl BehaviorRow {
    pub fn ContentArgument0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn Balloon<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn Condition0Target<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn Condition0Type<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn Condition1Target<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn Condition1Type<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn ContentArgument1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
}
