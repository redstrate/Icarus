//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct StagesElement<'a> {
    RequiredAmount: [&'a ColumnData; 5],
    MaxAmount: [&'a ColumnData; 5],
}
pub struct WKSCosmoToolDataAmountSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl WKSCosmoToolDataAmountSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "WKSCosmoToolDataAmount")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages
                .push(
                    read_excel_sheet(
                        resource,
                        "WKSCosmoToolDataAmount",
                        &exh,
                        language,
                        i,
                    )?,
                );
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<WKSCosmoToolDataAmountRow> {
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
        Some(WKSCosmoToolDataAmountRow {
            columns,
        })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<WKSCosmoToolDataAmountRow> {
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
    ) -> Option<WKSCosmoToolDataAmountRow> {
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
pub struct WKSCosmoToolDataAmountRow {
    columns: Vec<ColumnData>,
}
impl WKSCosmoToolDataAmountRow {
    pub fn Stages<'a>(&'a self) -> [StagesElement<'a>; 14] {
        [
            StagesElement {
                RequiredAmount: [
                    &self.columns[0],
                    &self.columns[1],
                    &self.columns[2],
                    &self.columns[3],
                    &self.columns[4],
                ],
                MaxAmount: [
                    &self.columns[5],
                    &self.columns[6],
                    &self.columns[7],
                    &self.columns[8],
                    &self.columns[9],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.columns[10],
                    &self.columns[11],
                    &self.columns[12],
                    &self.columns[13],
                    &self.columns[14],
                ],
                MaxAmount: [
                    &self.columns[15],
                    &self.columns[16],
                    &self.columns[17],
                    &self.columns[18],
                    &self.columns[19],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.columns[20],
                    &self.columns[21],
                    &self.columns[22],
                    &self.columns[23],
                    &self.columns[24],
                ],
                MaxAmount: [
                    &self.columns[25],
                    &self.columns[26],
                    &self.columns[27],
                    &self.columns[28],
                    &self.columns[29],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.columns[30],
                    &self.columns[31],
                    &self.columns[32],
                    &self.columns[33],
                    &self.columns[34],
                ],
                MaxAmount: [
                    &self.columns[35],
                    &self.columns[36],
                    &self.columns[37],
                    &self.columns[38],
                    &self.columns[39],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.columns[40],
                    &self.columns[41],
                    &self.columns[42],
                    &self.columns[43],
                    &self.columns[44],
                ],
                MaxAmount: [
                    &self.columns[45],
                    &self.columns[46],
                    &self.columns[47],
                    &self.columns[48],
                    &self.columns[49],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.columns[50],
                    &self.columns[51],
                    &self.columns[52],
                    &self.columns[53],
                    &self.columns[54],
                ],
                MaxAmount: [
                    &self.columns[55],
                    &self.columns[56],
                    &self.columns[57],
                    &self.columns[58],
                    &self.columns[59],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.columns[60],
                    &self.columns[61],
                    &self.columns[62],
                    &self.columns[63],
                    &self.columns[64],
                ],
                MaxAmount: [
                    &self.columns[65],
                    &self.columns[66],
                    &self.columns[67],
                    &self.columns[68],
                    &self.columns[69],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.columns[70],
                    &self.columns[71],
                    &self.columns[72],
                    &self.columns[73],
                    &self.columns[74],
                ],
                MaxAmount: [
                    &self.columns[75],
                    &self.columns[76],
                    &self.columns[77],
                    &self.columns[78],
                    &self.columns[79],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.columns[80],
                    &self.columns[81],
                    &self.columns[82],
                    &self.columns[83],
                    &self.columns[84],
                ],
                MaxAmount: [
                    &self.columns[85],
                    &self.columns[86],
                    &self.columns[87],
                    &self.columns[88],
                    &self.columns[89],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.columns[90],
                    &self.columns[91],
                    &self.columns[92],
                    &self.columns[93],
                    &self.columns[94],
                ],
                MaxAmount: [
                    &self.columns[95],
                    &self.columns[96],
                    &self.columns[97],
                    &self.columns[98],
                    &self.columns[99],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.columns[100],
                    &self.columns[101],
                    &self.columns[102],
                    &self.columns[103],
                    &self.columns[104],
                ],
                MaxAmount: [
                    &self.columns[105],
                    &self.columns[106],
                    &self.columns[107],
                    &self.columns[108],
                    &self.columns[109],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.columns[110],
                    &self.columns[111],
                    &self.columns[112],
                    &self.columns[113],
                    &self.columns[114],
                ],
                MaxAmount: [
                    &self.columns[115],
                    &self.columns[116],
                    &self.columns[117],
                    &self.columns[118],
                    &self.columns[119],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.columns[120],
                    &self.columns[121],
                    &self.columns[122],
                    &self.columns[123],
                    &self.columns[124],
                ],
                MaxAmount: [
                    &self.columns[125],
                    &self.columns[126],
                    &self.columns[127],
                    &self.columns[128],
                    &self.columns[129],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.columns[130],
                    &self.columns[131],
                    &self.columns[132],
                    &self.columns[133],
                    &self.columns[134],
                ],
                MaxAmount: [
                    &self.columns[135],
                    &self.columns[136],
                    &self.columns[137],
                    &self.columns[138],
                    &self.columns[139],
                ],
            },
        ]
    }
}
