//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct QuestClassJobRewardSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl QuestClassJobRewardSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "QuestClassJobReward")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages
                .push(
                    read_excel_sheet(resource, "QuestClassJobReward", &exh, language, i)?,
                );
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<QuestClassJobRewardRow> {
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
        Some(QuestClassJobRewardRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<QuestClassJobRewardRow> {
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
    ) -> Option<QuestClassJobRewardRow> {
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
pub struct QuestClassJobRewardRow {
    columns: Vec<ColumnData>,
}
impl QuestClassJobRewardRow {
    pub fn RewardItem<'a>(&'a self) -> [&'a ColumnData; 4] {
        [&self.columns[0], &self.columns[1], &self.columns[2], &self.columns[3]]
    }
    pub fn RequiredItem<'a>(&'a self) -> [&'a ColumnData; 4] {
        [&self.columns[4], &self.columns[5], &self.columns[6], &self.columns[7]]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn ClassJobCategory<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn RewardAmount<'a>(&'a self) -> [&'a ColumnData; 4] {
        [&self.columns[11], &self.columns[12], &self.columns[13], &self.columns[14]]
    }
    pub fn RequiredAmount<'a>(&'a self) -> [&'a ColumnData; 4] {
        [&self.columns[15], &self.columns[16], &self.columns[17], &self.columns[18]]
    }
    pub fn Unknown10<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn Unknown11<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[23]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[24]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a ColumnData {
        &self.columns[25]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a ColumnData {
        &self.columns[26]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a ColumnData {
        &self.columns[27]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a ColumnData {
        &self.columns[28]
    }
    pub fn Unknown12<'a>(&'a self) -> &'a ColumnData {
        &self.columns[29]
    }
    pub fn Unknown13<'a>(&'a self) -> &'a ColumnData {
        &self.columns[30]
    }
    pub fn Unknown14<'a>(&'a self) -> &'a ColumnData {
        &self.columns[31]
    }
    pub fn Unknown15<'a>(&'a self) -> &'a ColumnData {
        &self.columns[32]
    }
}
