//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
#[derive(Debug, Clone)]
pub struct BenchmarkOverrideEquipmentSheet {
    sheet: ExcelSheet,
}
impl BenchmarkOverrideEquipmentSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BenchmarkOverrideEquipment")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "BenchmarkOverrideEquipment", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<BenchmarkOverrideEquipmentRow> {
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
        Some(BenchmarkOverrideEquipmentRow {
            columns,
        })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<BenchmarkOverrideEquipmentRow> {
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
    ) -> Option<BenchmarkOverrideEquipmentRow> {
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
pub struct BenchmarkOverrideEquipmentRow {
    columns: Vec<ColumnData>,
}
impl BenchmarkOverrideEquipmentRow {
    pub fn ModelMainHand<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn ModelOffHand<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn ModelHead<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn ModelBody<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn ModelHands<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn ModelLegs<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn ModelFeet<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn ModelEars<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn ModelNeck<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn ModelWrists<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn ModelLeftRing<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn ModelRightRing<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn DyeMainHand<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn DyeOffHand<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn DyeHead<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn DyeBody<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn DyeHands<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn DyeLegs<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
    pub fn DyeFeet<'a>(&'a self) -> &'a ColumnData {
        &self.columns[23]
    }
    pub fn DyeEars<'a>(&'a self) -> &'a ColumnData {
        &self.columns[24]
    }
    pub fn DyeNeck<'a>(&'a self) -> &'a ColumnData {
        &self.columns[25]
    }
    pub fn DyeWrists<'a>(&'a self) -> &'a ColumnData {
        &self.columns[26]
    }
    pub fn DyeLeftRing<'a>(&'a self) -> &'a ColumnData {
        &self.columns[27]
    }
    pub fn DyeRightRing<'a>(&'a self) -> &'a ColumnData {
        &self.columns[28]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a ColumnData {
        &self.columns[29]
    }
}
