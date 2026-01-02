//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
pub struct AozActionTransientSheet {
    sheet: ExcelSheet,
}
impl AozActionTransientSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("AozActionTransient")?;
        let sheet = resolver.read_excel_sheet(exh, "AozActionTransient", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<AozActionTransientRow> {
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
        Some(AozActionTransientRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<AozActionTransientRow> {
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
    ) -> Option<AozActionTransientRow> {
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
pub struct AozActionTransientRow {
    columns: Vec<ColumnData>,
}
impl AozActionTransientRow {
    pub fn Stats<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn Description<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn Icon<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn RequiredForQuest<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn PreviousQuest<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn Location<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn Number<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn LocationKey<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn TargetsEnemy<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn TargetsSelfOrAlly<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn CauseSlow<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn CausePetrify<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn CauseParalysis<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn CauseInterrupt<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn CauseBlind<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn CauseStun<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn CauseSleep<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn CauseBind<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn CauseHeavy<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn CauseDeath<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
}
