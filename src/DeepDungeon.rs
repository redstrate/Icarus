//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
pub struct DeepDungeonSheet {
    sheet: ExcelSheet,
}
impl DeepDungeonSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("DeepDungeon")?;
        let sheet = resolver.read_excel_sheet(exh, "DeepDungeon", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<DeepDungeonRow> {
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
        Some(DeepDungeonRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<DeepDungeonRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<DeepDungeonRow> {
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
pub struct DeepDungeonRow {
    columns: Vec<ColumnData>,
}
impl DeepDungeonRow {
    pub fn PomanderSlot<'a>(&'a self) -> [&'a ColumnData; 16] {
        [
            &self.columns[0],
            &self.columns[1],
            &self.columns[2],
            &self.columns[3],
            &self.columns[4],
            &self.columns[5],
            &self.columns[6],
            &self.columns[7],
            &self.columns[8],
            &self.columns[9],
            &self.columns[10],
            &self.columns[11],
            &self.columns[12],
            &self.columns[13],
            &self.columns[14],
            &self.columns[15],
        ]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a ColumnData {
        &self.columns[23]
    }
    pub fn Unknown10<'a>(&'a self) -> &'a ColumnData {
        &self.columns[24]
    }
    pub fn Unknown11<'a>(&'a self) -> &'a ColumnData {
        &self.columns[25]
    }
    pub fn Unknown12<'a>(&'a self) -> &'a ColumnData {
        &self.columns[26]
    }
    pub fn Unknown13<'a>(&'a self) -> &'a ColumnData {
        &self.columns[27]
    }
    pub fn MagiciteSlot<'a>(&'a self) -> [&'a ColumnData; 4] {
        [&self.columns[28], &self.columns[29], &self.columns[30], &self.columns[31]]
    }
    pub fn Name<'a>(&'a self) -> &'a ColumnData {
        &self.columns[32]
    }
    pub fn Unknown14<'a>(&'a self) -> &'a ColumnData {
        &self.columns[33]
    }
    pub fn AetherpoolArm<'a>(&'a self) -> &'a ColumnData {
        &self.columns[34]
    }
    pub fn AetherpoolArmor<'a>(&'a self) -> &'a ColumnData {
        &self.columns[35]
    }
    pub fn DeepDungeonType<'a>(&'a self) -> &'a ColumnData {
        &self.columns[36]
    }
}
