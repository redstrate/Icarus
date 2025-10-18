//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct GatheringNotebookListSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl GatheringNotebookListSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "GatheringNotebookList")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages
                .push(
                    read_excel_sheet(
                        resource,
                        "GatheringNotebookList",
                        &exh,
                        language,
                        i,
                    )?,
                );
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<GatheringNotebookListRow> {
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
        Some(GatheringNotebookListRow {
            columns,
        })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<GatheringNotebookListRow> {
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
    ) -> Option<GatheringNotebookListRow> {
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
pub struct GatheringNotebookListRow {
    columns: Vec<ColumnData>,
}
impl GatheringNotebookListRow {
    pub fn GatheringItem<'a>(&'a self) -> [&'a ColumnData; 100] {
        [
            &self.columns[0],
            &self.columns[1],
            &self.columns[2],
            &self.columns[3],
            &self.columns[4],
            &self.columns[5],
            &self.columns[6],
            &self.columns[7],
            &self.columns[8],
            &self.columns[9],
            &self.columns[10],
            &self.columns[11],
            &self.columns[12],
            &self.columns[13],
            &self.columns[14],
            &self.columns[15],
            &self.columns[16],
            &self.columns[17],
            &self.columns[18],
            &self.columns[19],
            &self.columns[20],
            &self.columns[21],
            &self.columns[22],
            &self.columns[23],
            &self.columns[24],
            &self.columns[25],
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
            &self.columns[39],
            &self.columns[40],
            &self.columns[41],
            &self.columns[42],
            &self.columns[43],
            &self.columns[44],
            &self.columns[45],
            &self.columns[46],
            &self.columns[47],
            &self.columns[48],
            &self.columns[49],
            &self.columns[50],
            &self.columns[51],
            &self.columns[52],
            &self.columns[53],
            &self.columns[54],
            &self.columns[55],
            &self.columns[56],
            &self.columns[57],
            &self.columns[58],
            &self.columns[59],
            &self.columns[60],
            &self.columns[61],
            &self.columns[62],
            &self.columns[63],
            &self.columns[64],
            &self.columns[65],
            &self.columns[66],
            &self.columns[67],
            &self.columns[68],
            &self.columns[69],
            &self.columns[70],
            &self.columns[71],
            &self.columns[72],
            &self.columns[73],
            &self.columns[74],
            &self.columns[75],
            &self.columns[76],
            &self.columns[77],
            &self.columns[78],
            &self.columns[79],
            &self.columns[80],
            &self.columns[81],
            &self.columns[82],
            &self.columns[83],
            &self.columns[84],
            &self.columns[85],
            &self.columns[86],
            &self.columns[87],
            &self.columns[88],
            &self.columns[89],
            &self.columns[90],
            &self.columns[91],
            &self.columns[92],
            &self.columns[93],
            &self.columns[94],
            &self.columns[95],
            &self.columns[96],
            &self.columns[97],
            &self.columns[98],
            &self.columns[99],
        ]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[100]
    }
}
