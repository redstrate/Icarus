//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct RankDataElement<'a> {
    SGB: [&'a ColumnData; 4],
    Unknown0: &'a ColumnData,
    Unknown1: &'a ColumnData,
    Unknown2: &'a ColumnData,
    Unknown3: &'a ColumnData,
    Unknown4: &'a ColumnData,
    Unknown5: &'a ColumnData,
    Unknown6: &'a ColumnData,
    Unknown7: &'a ColumnData,
}
pub struct MJIFarmPastureRankSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl MJIFarmPastureRankSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "MJIFarmPastureRank")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages
                .push(
                    read_excel_sheet(resource, "MJIFarmPastureRank", &exh, language, i)?,
                );
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<MJIFarmPastureRankRow> {
        let column_defs = &self.exh.column_definitions;
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
        for page in &self.pages {
            let Some(row) = &page.get_row(row_id) else {
                continue;
            };
            let row = match row {
                ExcelRowKind::SingleRow(row) => row,
                ExcelRowKind::SubRows(rows) => &rows.first()?.1,
            };
            return self.read_row(row);
        }
        None
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<MJIFarmPastureRankRow> {
        for page in &self.pages {
            let Some(row) = &page.get_row(row_id) else {
                continue;
            };
            let row = match row {
                ExcelRowKind::SingleRow(row) => return None,
                ExcelRowKind::SubRows(subrows) => {
                    &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1
                }
            };
            return self.read_row(row);
        }
        None
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.row_count
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
