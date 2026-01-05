//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
#[derive(Debug, Clone)]
pub struct MJIBuildingSheet {
    sheet: ExcelSheet,
}
impl MJIBuildingSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MJIBuilding")?;
        let sheet = resolver.read_excel_sheet(&exh, "MJIBuilding", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<MJIBuildingRow> {
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
        Some(MJIBuildingRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<MJIBuildingRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<MJIBuildingRow> {
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
pub struct MJIBuildingRow {
    columns: Vec<ColumnData>,
}
impl MJIBuildingRow {
    pub fn Name<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn Icon<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn Sgb<'a>(&'a self) -> [&'a ColumnData; 5] {
        [
            &self.columns[3],
            &self.columns[4],
            &self.columns[5],
            &self.columns[6],
            &self.columns[7],
        ]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn Unknown10<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn Unknown11<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn Unknown12<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn Unknown13<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn Unknown14<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn Material<'a>(&'a self) -> [&'a ColumnData; 5] {
        [
            &self.columns[22],
            &self.columns[23],
            &self.columns[24],
            &self.columns[25],
            &self.columns[26],
        ]
    }
    pub fn Amount<'a>(&'a self) -> [&'a ColumnData; 5] {
        [
            &self.columns[27],
            &self.columns[28],
            &self.columns[29],
            &self.columns[30],
            &self.columns[31],
        ]
    }
}
