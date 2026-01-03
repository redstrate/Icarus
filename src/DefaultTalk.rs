//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
pub struct DefaultTalkParamsElement<'a> {
    pub ActionTimelinePose: &'a ColumnData,
    pub Unknown0: &'a ColumnData,
    pub Unknown1: &'a ColumnData,
    pub Unknown2: &'a ColumnData,
    pub Unknown3: &'a ColumnData,
    pub Unknown4: &'a ColumnData,
}
#[derive(Debug, Clone)]
pub struct DefaultTalkSheet {
    sheet: ExcelSheet,
}
impl DefaultTalkSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("DefaultTalk")?;
        let sheet = resolver.read_excel_sheet(exh, "DefaultTalk", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<DefaultTalkRow> {
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
        Some(DefaultTalkRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<DefaultTalkRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<DefaultTalkRow> {
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
pub struct DefaultTalkRow {
    columns: Vec<ColumnData>,
}
impl DefaultTalkRow {
    pub fn DefaultTalkParams<'a>(&'a self) -> [DefaultTalkParamsElement<'a>; 3] {
        [
            DefaultTalkParamsElement {
                ActionTimelinePose: &self.columns[0],
                Unknown0: &self.columns[1],
                Unknown1: &self.columns[2],
                Unknown2: &self.columns[3],
                Unknown3: &self.columns[4],
                Unknown4: &self.columns[5],
            },
            DefaultTalkParamsElement {
                ActionTimelinePose: &self.columns[6],
                Unknown0: &self.columns[7],
                Unknown1: &self.columns[8],
                Unknown2: &self.columns[9],
                Unknown3: &self.columns[10],
                Unknown4: &self.columns[11],
            },
            DefaultTalkParamsElement {
                ActionTimelinePose: &self.columns[12],
                Unknown0: &self.columns[13],
                Unknown1: &self.columns[14],
                Unknown2: &self.columns[15],
                Unknown3: &self.columns[16],
                Unknown4: &self.columns[17],
            },
        ]
    }
    pub fn Text<'a>(&'a self) -> [&'a ColumnData; 3] {
        [&self.columns[18], &self.columns[19], &self.columns[20]]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
}
