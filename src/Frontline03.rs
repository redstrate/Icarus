//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
pub struct OvooDataElement<'a> {
    pub EmptyIcon: &'a ColumnData,
    pub MaelstromIcon: &'a ColumnData,
    pub TwinAdderIcon: &'a ColumnData,
    pub ImmortalFlamesIcon: &'a ColumnData,
    pub Unknown0: &'a ColumnData,
    pub Unknown1: &'a ColumnData,
    pub Unknown2: &'a ColumnData,
}
#[derive(Debug, Clone)]
pub struct Frontline03Sheet {
    sheet: ExcelSheet,
}
impl Frontline03Sheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Frontline03")?;
        let sheet = resolver.read_excel_sheet(exh, "Frontline03", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<Frontline03Row> {
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
        Some(Frontline03Row { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<Frontline03Row> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<Frontline03Row> {
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
pub struct Frontline03Row {
    columns: Vec<ColumnData>,
}
impl Frontline03Row {
    pub fn OvooData<'a>(&'a self) -> [OvooDataElement<'a>; 3] {
        [
            OvooDataElement {
                EmptyIcon: &self.columns[0],
                MaelstromIcon: &self.columns[1],
                TwinAdderIcon: &self.columns[2],
                ImmortalFlamesIcon: &self.columns[3],
                Unknown0: &self.columns[4],
                Unknown1: &self.columns[5],
                Unknown2: &self.columns[6],
            },
            OvooDataElement {
                EmptyIcon: &self.columns[7],
                MaelstromIcon: &self.columns[8],
                TwinAdderIcon: &self.columns[9],
                ImmortalFlamesIcon: &self.columns[10],
                Unknown0: &self.columns[11],
                Unknown1: &self.columns[12],
                Unknown2: &self.columns[13],
            },
            OvooDataElement {
                EmptyIcon: &self.columns[14],
                MaelstromIcon: &self.columns[15],
                TwinAdderIcon: &self.columns[16],
                ImmortalFlamesIcon: &self.columns[17],
                Unknown0: &self.columns[18],
                Unknown1: &self.columns[19],
                Unknown2: &self.columns[20],
            },
        ]
    }
}
