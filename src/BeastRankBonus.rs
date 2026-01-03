//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
#[derive(Debug, Clone)]
pub struct BeastRankBonusSheet {
    sheet: ExcelSheet,
}
impl BeastRankBonusSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BeastRankBonus")?;
        let sheet = resolver.read_excel_sheet(exh, "BeastRankBonus", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<BeastRankBonusRow> {
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
        Some(BeastRankBonusRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<BeastRankBonusRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<BeastRankBonusRow> {
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
pub struct BeastRankBonusRow {
    columns: Vec<ColumnData>,
}
impl BeastRankBonusRow {
    pub fn Item<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn Neutral<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn Recognized<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn Friendly<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn Trusted<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn Respected<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn Honored<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn Sworn<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn AlliedBloodsworn<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn ItemQuantity<'a>(&'a self) -> [&'a ColumnData; 8] {
        [
            &self.columns[9],
            &self.columns[10],
            &self.columns[11],
            &self.columns[12],
            &self.columns[13],
            &self.columns[14],
            &self.columns[15],
            &self.columns[16],
        ]
    }
}
