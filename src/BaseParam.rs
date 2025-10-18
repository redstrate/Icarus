//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct BaseParamSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl BaseParamSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "BaseParam")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages.push(read_excel_sheet(resource, "BaseParam", &exh, language, i)?);
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<BaseParamRow> {
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
        Some(BaseParamRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<BaseParamRow> {
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
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<BaseParamRow> {
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
pub struct BaseParamRow {
    columns: Vec<ColumnData>,
}
impl BaseParamRow {
    pub fn Name<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn Description<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn OneHandWeaponPercent<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn OffHandPercent<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn HeadPercent<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn ChestPercent<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn HandsPercent<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn WaistPercent<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn LegsPercent<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn FeetPercent<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn EarringPercent<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn NecklacePercent<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn BraceletPercent<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn RingPercent<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn TwoHandWeaponPercent<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn UnderArmorPercent<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn ChestHeadPercent<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn ChestHeadLegsFeetPercent<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn LegsFeetPercent<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn HeadChestHandsLegsFeetPercent<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn ChestLegsGlovesPercent<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn ChestLegsFeetPercent<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[23]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[24]
    }
    pub fn OrderPriority<'a>(&'a self) -> &'a ColumnData {
        &self.columns[25]
    }
    pub fn MeldParam<'a>(&'a self) -> [&'a ColumnData; 13] {
        [
            &self.columns[26],
            &self.columns[27],
            &self.columns[28],
            &self.columns[29],
            &self.columns[30],
            &self.columns[31],
            &self.columns[32],
            &self.columns[33],
            &self.columns[34],
            &self.columns[35],
            &self.columns[36],
            &self.columns[37],
            &self.columns[38],
        ]
    }
    pub fn PacketIndex<'a>(&'a self) -> &'a ColumnData {
        &self.columns[39]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[40]
    }
}
