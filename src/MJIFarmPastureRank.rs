//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
pub struct RankDataElement<'a> {
    pub SGB: [&'a ColumnData; 4],
    pub Unknown0: &'a ColumnData,
    pub Unknown1: &'a ColumnData,
    pub Unknown2: &'a ColumnData,
    pub Unknown3: &'a ColumnData,
    pub Unknown4: &'a ColumnData,
    pub Unknown5: &'a ColumnData,
    pub Unknown6: &'a ColumnData,
    pub Unknown7: &'a ColumnData,
}
#[derive(Debug, Clone)]
pub struct MJIFarmPastureRankSheet {
    sheet: ExcelSheet,
}
impl MJIFarmPastureRankSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MJIFarmPastureRank")?;
        let sheet = resolver.read_excel_sheet(exh, "MJIFarmPastureRank", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<MJIFarmPastureRankRow> {
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
        Some(MJIFarmPastureRankRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<MJIFarmPastureRankRow> {
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
    ) -> Option<MJIFarmPastureRankRow> {
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
pub struct MJIFarmPastureRankRow {
    columns: Vec<ColumnData>,
}
impl MJIFarmPastureRankRow {
    pub fn RankData<'a>(&'a self) -> [RankDataElement<'a>; 4] {
        [
            RankDataElement {
                SGB: [
                    &self.columns[0],
                    &self.columns[1],
                    &self.columns[2],
                    &self.columns[3],
                ],
                Unknown0: &self.columns[4],
                Unknown1: &self.columns[5],
                Unknown2: &self.columns[6],
                Unknown3: &self.columns[7],
                Unknown4: &self.columns[8],
                Unknown5: &self.columns[9],
                Unknown6: &self.columns[10],
                Unknown7: &self.columns[11],
            },
            RankDataElement {
                SGB: [
                    &self.columns[12],
                    &self.columns[13],
                    &self.columns[14],
                    &self.columns[15],
                ],
                Unknown0: &self.columns[16],
                Unknown1: &self.columns[17],
                Unknown2: &self.columns[18],
                Unknown3: &self.columns[19],
                Unknown4: &self.columns[20],
                Unknown5: &self.columns[21],
                Unknown6: &self.columns[22],
                Unknown7: &self.columns[23],
            },
            RankDataElement {
                SGB: [
                    &self.columns[24],
                    &self.columns[25],
                    &self.columns[26],
                    &self.columns[27],
                ],
                Unknown0: &self.columns[28],
                Unknown1: &self.columns[29],
                Unknown2: &self.columns[30],
                Unknown3: &self.columns[31],
                Unknown4: &self.columns[32],
                Unknown5: &self.columns[33],
                Unknown6: &self.columns[34],
                Unknown7: &self.columns[35],
            },
            RankDataElement {
                SGB: [
                    &self.columns[36],
                    &self.columns[37],
                    &self.columns[38],
                    &self.columns[39],
                ],
                Unknown0: &self.columns[40],
                Unknown1: &self.columns[41],
                Unknown2: &self.columns[42],
                Unknown3: &self.columns[43],
                Unknown4: &self.columns[44],
                Unknown5: &self.columns[45],
                Unknown6: &self.columns[46],
                Unknown7: &self.columns[47],
            },
        ]
    }
}
