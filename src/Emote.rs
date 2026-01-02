//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
pub struct EmoteSheet {
    sheet: ExcelSheet,
}
impl EmoteSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Emote")?;
        let sheet = resolver.read_excel_sheet(exh, "Emote", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<EmoteRow> {
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
        Some(EmoteRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<EmoteRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<EmoteRow> {
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
pub struct EmoteRow {
    columns: Vec<ColumnData>,
}
impl EmoteRow {
    pub fn Name<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn Icon<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn UnlockLink<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn TextCommand<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn ActionTimeline<'a>(&'a self) -> [&'a ColumnData; 7] {
        [
            &self.columns[4],
            &self.columns[5],
            &self.columns[6],
            &self.columns[7],
            &self.columns[8],
            &self.columns[9],
            &self.columns[10],
        ]
    }
    pub fn Order<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn LogMessageTargeted<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn LogMessageUntargeted<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn Patch<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn EmoteCategory<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn EmoteMode<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn HasCancelEmote<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
    pub fn DrawsWeapon<'a>(&'a self) -> &'a ColumnData {
        &self.columns[23]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a ColumnData {
        &self.columns[24]
    }
}
