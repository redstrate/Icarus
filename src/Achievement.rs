//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct AchievementSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl AchievementSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "Achievement")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages.push(read_excel_sheet(resource, "Achievement", &exh, language, i)?);
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<AchievementRow> {
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
        Some(AchievementRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<AchievementRow> {
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
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<AchievementRow> {
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
pub struct AchievementRow {
    columns: Vec<ColumnData>,
}
impl AchievementRow {
    pub fn Name<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn Description<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn Item<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn Icon<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn Key<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn Data<'a>(&'a self) -> [&'a ColumnData; 8] {
        [
            &self.columns[5],
            &self.columns[6],
            &self.columns[7],
            &self.columns[8],
            &self.columns[9],
            &self.columns[10],
            &self.columns[11],
            &self.columns[12],
        ]
    }
    pub fn Title<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn Order<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn AchievementCategory<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn AchievementTarget<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn Points<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
    pub fn Type<'a>(&'a self) -> &'a ColumnData {
        &self.columns[23]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a ColumnData {
        &self.columns[24]
    }
    pub fn AchievementHideCondition<'a>(&'a self) -> &'a ColumnData {
        &self.columns[25]
    }
}
