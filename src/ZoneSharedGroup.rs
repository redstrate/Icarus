//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct ZoneSharedGroupSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl ZoneSharedGroupSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "ZoneSharedGroup")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages
                .push(read_excel_sheet(resource, "ZoneSharedGroup", &exh, language, i)?);
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<ZoneSharedGroupRow> {
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
        Some(ZoneSharedGroupRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<ZoneSharedGroupRow> {
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
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<ZoneSharedGroupRow> {
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
pub struct ZoneSharedGroupRow {
    columns: Vec<ColumnData>,
}
impl ZoneSharedGroupRow {
    pub fn LGBSharedGroup<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn RequirementRow<'a>(&'a self) -> [&'a ColumnData; 6] {
        [
            &self.columns[1],
            &self.columns[2],
            &self.columns[3],
            &self.columns[4],
            &self.columns[5],
            &self.columns[6],
        ]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn RequirementQuestSequence<'a>(&'a self) -> [&'a ColumnData; 6] {
        [
            &self.columns[8],
            &self.columns[9],
            &self.columns[10],
            &self.columns[11],
            &self.columns[12],
            &self.columns[13],
        ]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    /// 1 = Quest
    /// 2 = Quest with specific Sequence
    /// 3 = AetherCurrent
    /// 4 = EurekaStoryProgress
    /// 5 = DomaStoryProgress
    ///
    pub fn RequirementType<'a>(&'a self) -> [&'a ColumnData; 6] {
        [
            &self.columns[15],
            &self.columns[16],
            &self.columns[17],
            &self.columns[18],
            &self.columns[19],
            &self.columns[20],
        ]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
    pub fn Unknown10<'a>(&'a self) -> &'a ColumnData {
        &self.columns[23]
    }
    pub fn Unknown11<'a>(&'a self) -> &'a ColumnData {
        &self.columns[24]
    }
    pub fn Unknown12<'a>(&'a self) -> &'a ColumnData {
        &self.columns[25]
    }
    pub fn Unknown13<'a>(&'a self) -> &'a ColumnData {
        &self.columns[26]
    }
    pub fn Unknown14<'a>(&'a self) -> &'a ColumnData {
        &self.columns[27]
    }
    pub fn Unknown15<'a>(&'a self) -> &'a ColumnData {
        &self.columns[28]
    }
}
