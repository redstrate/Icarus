//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct WKSFunctionSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl WKSFunctionSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "WKSFunction")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages.push(read_excel_sheet(resource, "WKSFunction", &exh, language, i)?);
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<WKSFunctionRow> {
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
        Some(WKSFunctionRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<WKSFunctionRow> {
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
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<WKSFunctionRow> {
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
pub struct WKSFunctionRow {
    columns: Vec<ColumnData>,
}
impl WKSFunctionRow {
    /// Required quest needed for function to return true
    pub fn RequiredQuests0<'a>(&'a self) -> [&'a ColumnData; 4] {
        [&self.columns[0], &self.columns[1], &self.columns[2], &self.columns[3]]
    }
    /// Required quest needed for function to return true
    pub fn RequiredQuests1<'a>(&'a self) -> [&'a ColumnData; 4] {
        [&self.columns[4], &self.columns[5], &self.columns[6], &self.columns[7]]
    }
    /// Best guess for now as no references are easily found in the game files
    pub fn RequiredQuests2<'a>(&'a self) -> [&'a ColumnData; 4] {
        [&self.columns[8], &self.columns[9], &self.columns[10], &self.columns[11]]
    }
    /// Needs this grade to be completed for function to be valid. Game checks for WKSManager.DevGrade < RequiredDevGrade[]
    pub fn RequiredDevGrade<'a>(&'a self) -> [&'a ColumnData; 4] {
        [&self.columns[12], &self.columns[13], &self.columns[14], &self.columns[15]]
    }
    /// Seems to only control one function tree for now relating to something in WKSManager but is also slated to check for '- 2 >= 2' and fail
    pub fn Unknown12<'a>(&'a self) -> [&'a ColumnData; 4] {
        [&self.columns[16], &self.columns[17], &self.columns[18], &self.columns[19]]
    }
}
