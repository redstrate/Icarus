//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct StagesElement<'a> {
    pub Unknown0: &'a ColumnData,
    pub Item: &'a ColumnData,
    pub Name: &'a ColumnData,
}
pub struct TypesElement<'a> {
    pub Icon: &'a ColumnData,
    pub Name: &'a ColumnData,
    pub CosmicName: &'a ColumnData,
}
pub struct WKSCosmoToolClassSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl WKSCosmoToolClassSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "WKSCosmoToolClass")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages
                .push(
                    read_excel_sheet(resource, "WKSCosmoToolClass", &exh, language, i)?,
                );
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<WKSCosmoToolClassRow> {
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
        Some(WKSCosmoToolClassRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<WKSCosmoToolClassRow> {
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
    ) -> Option<WKSCosmoToolClassRow> {
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
pub struct WKSCosmoToolClassRow {
    columns: Vec<ColumnData>,
}
impl WKSCosmoToolClassRow {
    pub fn Stages<'a>(&'a self) -> [StagesElement<'a>; 17] {
        [
            StagesElement {
                Unknown0: &self.columns[0],
                Item: &self.columns[1],
                Name: &self.columns[2],
            },
            StagesElement {
                Unknown0: &self.columns[3],
                Item: &self.columns[4],
                Name: &self.columns[5],
            },
            StagesElement {
                Unknown0: &self.columns[6],
                Item: &self.columns[7],
                Name: &self.columns[8],
            },
            StagesElement {
                Unknown0: &self.columns[9],
                Item: &self.columns[10],
                Name: &self.columns[11],
            },
            StagesElement {
                Unknown0: &self.columns[12],
                Item: &self.columns[13],
                Name: &self.columns[14],
            },
            StagesElement {
                Unknown0: &self.columns[15],
                Item: &self.columns[16],
                Name: &self.columns[17],
            },
            StagesElement {
                Unknown0: &self.columns[18],
                Item: &self.columns[19],
                Name: &self.columns[20],
            },
            StagesElement {
                Unknown0: &self.columns[21],
                Item: &self.columns[22],
                Name: &self.columns[23],
            },
            StagesElement {
                Unknown0: &self.columns[24],
                Item: &self.columns[25],
                Name: &self.columns[26],
            },
            StagesElement {
                Unknown0: &self.columns[27],
                Item: &self.columns[28],
                Name: &self.columns[29],
            },
            StagesElement {
                Unknown0: &self.columns[30],
                Item: &self.columns[31],
                Name: &self.columns[32],
            },
            StagesElement {
                Unknown0: &self.columns[33],
                Item: &self.columns[34],
                Name: &self.columns[35],
            },
            StagesElement {
                Unknown0: &self.columns[36],
                Item: &self.columns[37],
                Name: &self.columns[38],
            },
            StagesElement {
                Unknown0: &self.columns[39],
                Item: &self.columns[40],
                Name: &self.columns[41],
            },
            StagesElement {
                Unknown0: &self.columns[42],
                Item: &self.columns[43],
                Name: &self.columns[44],
            },
            StagesElement {
                Unknown0: &self.columns[45],
                Item: &self.columns[46],
                Name: &self.columns[47],
            },
            StagesElement {
                Unknown0: &self.columns[48],
                Item: &self.columns[49],
                Name: &self.columns[50],
            },
        ]
    }
    pub fn Types<'a>(&'a self) -> [TypesElement<'a>; 6] {
        [
            TypesElement {
                Icon: &self.columns[51],
                Name: &self.columns[52],
                CosmicName: &self.columns[53],
            },
            TypesElement {
                Icon: &self.columns[54],
                Name: &self.columns[55],
                CosmicName: &self.columns[56],
            },
            TypesElement {
                Icon: &self.columns[57],
                Name: &self.columns[58],
                CosmicName: &self.columns[59],
            },
            TypesElement {
                Icon: &self.columns[60],
                Name: &self.columns[61],
                CosmicName: &self.columns[62],
            },
            TypesElement {
                Icon: &self.columns[63],
                Name: &self.columns[64],
                CosmicName: &self.columns[65],
            },
            TypesElement {
                Icon: &self.columns[66],
                Name: &self.columns[67],
                CosmicName: &self.columns[68],
            },
        ]
    }
    pub fn Name<'a>(&'a self) -> &'a ColumnData {
        &self.columns[69]
    }
    pub fn DataAmount<'a>(&'a self) -> &'a ColumnData {
        &self.columns[70]
    }
}
