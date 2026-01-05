//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
#[derive(Debug, Clone)]
pub struct StatusSheet {
    sheet: ExcelSheet,
}
impl StatusSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Status")?;
        let sheet = resolver.read_excel_sheet(&exh, "Status", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<StatusRow> {
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
        Some(StatusRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<StatusRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<StatusRow> {
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
pub struct StatusRow {
    columns: Vec<ColumnData>,
}
impl StatusRow {
    pub fn Name<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn Description<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn Icon<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn ParamModifier<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn VFX<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn Log<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn MaxStacks<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn ClassJobCategory<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn StatusCategory<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn HitEffect<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn PartyListPriority<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn CanIncreaseRewards<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn ParamEffect<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn TargetType<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    /// actually an index of the flag
    pub fn Flags<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn Flag2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn Unknown_70_1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn LockMovement<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn LockActions<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn LockControl<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
    pub fn Transfiguration<'a>(&'a self) -> &'a ColumnData {
        &self.columns[23]
    }
    pub fn IsGaze<'a>(&'a self) -> &'a ColumnData {
        &self.columns[24]
    }
    pub fn CanDispel<'a>(&'a self) -> &'a ColumnData {
        &self.columns[25]
    }
    pub fn InflictedByActor<'a>(&'a self) -> &'a ColumnData {
        &self.columns[26]
    }
    pub fn IsPermanent<'a>(&'a self) -> &'a ColumnData {
        &self.columns[27]
    }
    pub fn NoLogVfx<'a>(&'a self) -> &'a ColumnData {
        &self.columns[28]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a ColumnData {
        &self.columns[29]
    }
    pub fn CanStatusOff<'a>(&'a self) -> &'a ColumnData {
        &self.columns[30]
    }
    pub fn IsFcBuff<'a>(&'a self) -> &'a ColumnData {
        &self.columns[31]
    }
    pub fn Invisibility<'a>(&'a self) -> &'a ColumnData {
        &self.columns[32]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a ColumnData {
        &self.columns[33]
    }
    pub fn Unknown_70_2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[34]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a ColumnData {
        &self.columns[35]
    }
}
