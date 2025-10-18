//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct WKSMissionUnitSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl WKSMissionUnitSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "WKSMissionUnit")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages.push(read_excel_sheet(resource, "WKSMissionUnit", &exh, language, i)?);
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<WKSMissionUnitRow> {
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
        Some(WKSMissionUnitRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<WKSMissionUnitRow> {
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
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<WKSMissionUnitRow> {
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
pub struct WKSMissionUnitRow {
    columns: Vec<ColumnData>,
}
impl WKSMissionUnitRow {
    pub fn Name<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn ClassJobCategory<'a>(&'a self) -> [&'a ColumnData; 2] {
        [&self.columns[1], &self.columns[2]]
    }
    pub fn MissionTime<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn MissionReward<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn SilverStarRequirement<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn GoldStarRequirement<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn MissionToDo<'a>(&'a self) -> [&'a ColumnData; 3] {
        [&self.columns[7], &self.columns[8], &self.columns[9]]
    }
    pub fn LockedBehind<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn WKSMissionSupplyItem<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn WKSMissionRecipe<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn PlaceName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn SortKey<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn WKSMissionText<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn WKSFunction<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn LevelGroup<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn WKSMissionLotterySpecialCond<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn IsSynced<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn IsSpecialQuest<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
}
