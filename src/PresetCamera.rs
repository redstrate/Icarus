//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct PresetCameraSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl PresetCameraSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "PresetCamera")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages.push(read_excel_sheet(resource, "PresetCamera", &exh, language, i)?);
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<PresetCameraRow> {
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
        Some(PresetCameraRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<PresetCameraRow> {
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
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<PresetCameraRow> {
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
pub struct PresetCameraRow {
    columns: Vec<ColumnData>,
}
impl PresetCameraRow {
    pub fn PosX<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn PosY<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn PosZ<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn Elezen<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn Lalafell<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn Miqote<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn Roe<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn Hrothgar<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn Viera<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn Hyur_F<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn Elezen_F<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn Lalafell_F<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn Miqote_F<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn Roe_F<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn Hrothgar_F<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn Viera_F<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn Unknown_70<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn EID<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
}
