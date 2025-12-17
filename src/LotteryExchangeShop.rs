//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct LotteryExchangeParamsElement<'a> {
    AmountAccepted: &'a ColumnData,
    ItemAccepted: &'a ColumnData,
    Unknown0: &'a ColumnData,
    Unknown1: &'a ColumnData,
}
pub struct LotteryExchangeShopSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl LotteryExchangeShopSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "LotteryExchangeShop")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages
                .push(
                    read_excel_sheet(resource, "LotteryExchangeShop", &exh, language, i)?,
                );
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<LotteryExchangeShopRow> {
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
        Some(LotteryExchangeShopRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<LotteryExchangeShopRow> {
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
    ) -> Option<LotteryExchangeShopRow> {
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
pub struct LotteryExchangeShopRow {
    columns: Vec<ColumnData>,
}
impl LotteryExchangeShopRow {
    pub fn Name<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn LotteryExchangeParams<'a>(
        &'a self,
    ) -> [LotteryExchangeParamsElement<'a>; 32] {
        [
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[1],
                ItemAccepted: &self.columns[2],
                Unknown0: &self.columns[3],
                Unknown1: &self.columns[4],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[5],
                ItemAccepted: &self.columns[6],
                Unknown0: &self.columns[7],
                Unknown1: &self.columns[8],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[9],
                ItemAccepted: &self.columns[10],
                Unknown0: &self.columns[11],
                Unknown1: &self.columns[12],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[13],
                ItemAccepted: &self.columns[14],
                Unknown0: &self.columns[15],
                Unknown1: &self.columns[16],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[17],
                ItemAccepted: &self.columns[18],
                Unknown0: &self.columns[19],
                Unknown1: &self.columns[20],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[21],
                ItemAccepted: &self.columns[22],
                Unknown0: &self.columns[23],
                Unknown1: &self.columns[24],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[25],
                ItemAccepted: &self.columns[26],
                Unknown0: &self.columns[27],
                Unknown1: &self.columns[28],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[29],
                ItemAccepted: &self.columns[30],
                Unknown0: &self.columns[31],
                Unknown1: &self.columns[32],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[33],
                ItemAccepted: &self.columns[34],
                Unknown0: &self.columns[35],
                Unknown1: &self.columns[36],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[37],
                ItemAccepted: &self.columns[38],
                Unknown0: &self.columns[39],
                Unknown1: &self.columns[40],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[41],
                ItemAccepted: &self.columns[42],
                Unknown0: &self.columns[43],
                Unknown1: &self.columns[44],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[45],
                ItemAccepted: &self.columns[46],
                Unknown0: &self.columns[47],
                Unknown1: &self.columns[48],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[49],
                ItemAccepted: &self.columns[50],
                Unknown0: &self.columns[51],
                Unknown1: &self.columns[52],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[53],
                ItemAccepted: &self.columns[54],
                Unknown0: &self.columns[55],
                Unknown1: &self.columns[56],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[57],
                ItemAccepted: &self.columns[58],
                Unknown0: &self.columns[59],
                Unknown1: &self.columns[60],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[61],
                ItemAccepted: &self.columns[62],
                Unknown0: &self.columns[63],
                Unknown1: &self.columns[64],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[65],
                ItemAccepted: &self.columns[66],
                Unknown0: &self.columns[67],
                Unknown1: &self.columns[68],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[69],
                ItemAccepted: &self.columns[70],
                Unknown0: &self.columns[71],
                Unknown1: &self.columns[72],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[73],
                ItemAccepted: &self.columns[74],
                Unknown0: &self.columns[75],
                Unknown1: &self.columns[76],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[77],
                ItemAccepted: &self.columns[78],
                Unknown0: &self.columns[79],
                Unknown1: &self.columns[80],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[81],
                ItemAccepted: &self.columns[82],
                Unknown0: &self.columns[83],
                Unknown1: &self.columns[84],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[85],
                ItemAccepted: &self.columns[86],
                Unknown0: &self.columns[87],
                Unknown1: &self.columns[88],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[89],
                ItemAccepted: &self.columns[90],
                Unknown0: &self.columns[91],
                Unknown1: &self.columns[92],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[93],
                ItemAccepted: &self.columns[94],
                Unknown0: &self.columns[95],
                Unknown1: &self.columns[96],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[97],
                ItemAccepted: &self.columns[98],
                Unknown0: &self.columns[99],
                Unknown1: &self.columns[100],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[101],
                ItemAccepted: &self.columns[102],
                Unknown0: &self.columns[103],
                Unknown1: &self.columns[104],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[105],
                ItemAccepted: &self.columns[106],
                Unknown0: &self.columns[107],
                Unknown1: &self.columns[108],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[109],
                ItemAccepted: &self.columns[110],
                Unknown0: &self.columns[111],
                Unknown1: &self.columns[112],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[113],
                ItemAccepted: &self.columns[114],
                Unknown0: &self.columns[115],
                Unknown1: &self.columns[116],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[117],
                ItemAccepted: &self.columns[118],
                Unknown0: &self.columns[119],
                Unknown1: &self.columns[120],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[121],
                ItemAccepted: &self.columns[122],
                Unknown0: &self.columns[123],
                Unknown1: &self.columns[124],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.columns[125],
                ItemAccepted: &self.columns[126],
                Unknown0: &self.columns[127],
                Unknown1: &self.columns[128],
            },
        ]
    }
    pub fn Script<'a>(&'a self) -> &'a ColumnData {
        &self.columns[129]
    }
    pub fn LogMessage<'a>(&'a self) -> [&'a ColumnData; 3] {
        [&self.columns[130], &self.columns[131], &self.columns[132]]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[133]
    }
}
