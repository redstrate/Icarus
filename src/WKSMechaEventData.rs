//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct WKSMechaEventDataSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl WKSMechaEventDataSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "WKSMechaEventData")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages
                .push(
                    read_excel_sheet(resource, "WKSMechaEventData", &exh, language, i)?,
                );
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<WKSMechaEventDataRow> {
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
        Some(WKSMechaEventDataRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<WKSMechaEventDataRow> {
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
    ) -> Option<WKSMechaEventDataRow> {
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
pub struct WKSMechaEventDataRow {
    columns: Vec<ColumnData>,
}
impl WKSMechaEventDataRow {
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn Unknown10<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn Unknown11<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn Unknown12<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn Unknown13<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn Unknown14<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn Unknown15<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn Unknown16<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn Unknown17<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn Unknown18<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn Unknown19<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn Unknown20<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn Unknown21<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn Unknown22<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
    pub fn Unknown23<'a>(&'a self) -> &'a ColumnData {
        &self.columns[23]
    }
    pub fn Unknown24<'a>(&'a self) -> &'a ColumnData {
        &self.columns[24]
    }
    pub fn Unknown25<'a>(&'a self) -> &'a ColumnData {
        &self.columns[25]
    }
    pub fn Unknown26<'a>(&'a self) -> &'a ColumnData {
        &self.columns[26]
    }
    pub fn Unknown27<'a>(&'a self) -> &'a ColumnData {
        &self.columns[27]
    }
    pub fn Unknown28<'a>(&'a self) -> &'a ColumnData {
        &self.columns[28]
    }
    pub fn Unknown29<'a>(&'a self) -> &'a ColumnData {
        &self.columns[29]
    }
    pub fn Unknown30<'a>(&'a self) -> &'a ColumnData {
        &self.columns[30]
    }
    pub fn Unknown31<'a>(&'a self) -> &'a ColumnData {
        &self.columns[31]
    }
    pub fn Unknown32<'a>(&'a self) -> &'a ColumnData {
        &self.columns[32]
    }
    pub fn Unknown33<'a>(&'a self) -> &'a ColumnData {
        &self.columns[33]
    }
    pub fn Unknown34<'a>(&'a self) -> &'a ColumnData {
        &self.columns[34]
    }
    pub fn Unknown35<'a>(&'a self) -> &'a ColumnData {
        &self.columns[35]
    }
    pub fn Unknown36<'a>(&'a self) -> &'a ColumnData {
        &self.columns[36]
    }
    pub fn Unknown37<'a>(&'a self) -> &'a ColumnData {
        &self.columns[37]
    }
    pub fn Unknown38<'a>(&'a self) -> &'a ColumnData {
        &self.columns[38]
    }
    pub fn Unknown39<'a>(&'a self) -> &'a ColumnData {
        &self.columns[39]
    }
    pub fn Unknown40<'a>(&'a self) -> &'a ColumnData {
        &self.columns[40]
    }
    pub fn Unknown41<'a>(&'a self) -> &'a ColumnData {
        &self.columns[41]
    }
    pub fn Unknown42<'a>(&'a self) -> &'a ColumnData {
        &self.columns[42]
    }
    pub fn Unknown43<'a>(&'a self) -> &'a ColumnData {
        &self.columns[43]
    }
    pub fn Unknown44<'a>(&'a self) -> &'a ColumnData {
        &self.columns[44]
    }
    pub fn Unknown45<'a>(&'a self) -> &'a ColumnData {
        &self.columns[45]
    }
    pub fn Unknown46<'a>(&'a self) -> &'a ColumnData {
        &self.columns[46]
    }
    pub fn Unknown47<'a>(&'a self) -> &'a ColumnData {
        &self.columns[47]
    }
    pub fn Unknown48<'a>(&'a self) -> &'a ColumnData {
        &self.columns[48]
    }
    pub fn Unknown49<'a>(&'a self) -> &'a ColumnData {
        &self.columns[49]
    }
    pub fn Unknown50<'a>(&'a self) -> &'a ColumnData {
        &self.columns[50]
    }
    pub fn Unknown51<'a>(&'a self) -> &'a ColumnData {
        &self.columns[51]
    }
    pub fn Unknown52<'a>(&'a self) -> &'a ColumnData {
        &self.columns[52]
    }
}
