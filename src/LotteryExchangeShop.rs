//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct LotteryExchangeParamsElement<'a> {
    pub AmountAccepted: &'a Field,
    pub ItemAccepted: &'a Field,
    pub Unknown0: &'a Field,
    pub Unknown1: &'a Field,
}
#[derive(Debug, Clone)]
pub struct LotteryExchangeShopSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl LotteryExchangeShopSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("LotteryExchangeShop")?;
        let sheet = resolver.read_excel_sheet(&exh, "LotteryExchangeShop", language)?;
        let mut index_mapping: Vec<(usize, &ExcelColumnDefinition)> = sheet
            .exh
            .column_definitions
            .iter()
            .enumerate()
            .collect();
        index_mapping.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let index_mapping: Vec<usize> = index_mapping
            .iter()
            .map(|(index, _)| *index)
            .collect();
        Ok(Self { sheet, index_mapping })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<LotteryExchangeShopRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<LotteryExchangeShopRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for LotteryExchangeShopSheet {
    type Row = LotteryExchangeShopRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a LotteryExchangeShopSheet {
    type Item = (u32, Vec<(u16, LotteryExchangeShopRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, LotteryExchangeShopSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, LotteryExchangeShopSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct LotteryExchangeShopRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> LotteryExchangeShopRow<'a> {
    pub fn Name(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn LotteryExchangeParams(&'a self) -> [LotteryExchangeParamsElement<'a>; 32] {
        [
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[1]],
                ItemAccepted: &self.row.columns[self.index_mapping[2]],
                Unknown0: &self.row.columns[self.index_mapping[3]],
                Unknown1: &self.row.columns[self.index_mapping[4]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[5]],
                ItemAccepted: &self.row.columns[self.index_mapping[6]],
                Unknown0: &self.row.columns[self.index_mapping[7]],
                Unknown1: &self.row.columns[self.index_mapping[8]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[9]],
                ItemAccepted: &self.row.columns[self.index_mapping[10]],
                Unknown0: &self.row.columns[self.index_mapping[11]],
                Unknown1: &self.row.columns[self.index_mapping[12]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[13]],
                ItemAccepted: &self.row.columns[self.index_mapping[14]],
                Unknown0: &self.row.columns[self.index_mapping[15]],
                Unknown1: &self.row.columns[self.index_mapping[16]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[17]],
                ItemAccepted: &self.row.columns[self.index_mapping[18]],
                Unknown0: &self.row.columns[self.index_mapping[19]],
                Unknown1: &self.row.columns[self.index_mapping[20]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[21]],
                ItemAccepted: &self.row.columns[self.index_mapping[22]],
                Unknown0: &self.row.columns[self.index_mapping[23]],
                Unknown1: &self.row.columns[self.index_mapping[24]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[25]],
                ItemAccepted: &self.row.columns[self.index_mapping[26]],
                Unknown0: &self.row.columns[self.index_mapping[27]],
                Unknown1: &self.row.columns[self.index_mapping[28]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[29]],
                ItemAccepted: &self.row.columns[self.index_mapping[30]],
                Unknown0: &self.row.columns[self.index_mapping[31]],
                Unknown1: &self.row.columns[self.index_mapping[32]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[33]],
                ItemAccepted: &self.row.columns[self.index_mapping[34]],
                Unknown0: &self.row.columns[self.index_mapping[35]],
                Unknown1: &self.row.columns[self.index_mapping[36]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[37]],
                ItemAccepted: &self.row.columns[self.index_mapping[38]],
                Unknown0: &self.row.columns[self.index_mapping[39]],
                Unknown1: &self.row.columns[self.index_mapping[40]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[41]],
                ItemAccepted: &self.row.columns[self.index_mapping[42]],
                Unknown0: &self.row.columns[self.index_mapping[43]],
                Unknown1: &self.row.columns[self.index_mapping[44]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[45]],
                ItemAccepted: &self.row.columns[self.index_mapping[46]],
                Unknown0: &self.row.columns[self.index_mapping[47]],
                Unknown1: &self.row.columns[self.index_mapping[48]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[49]],
                ItemAccepted: &self.row.columns[self.index_mapping[50]],
                Unknown0: &self.row.columns[self.index_mapping[51]],
                Unknown1: &self.row.columns[self.index_mapping[52]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[53]],
                ItemAccepted: &self.row.columns[self.index_mapping[54]],
                Unknown0: &self.row.columns[self.index_mapping[55]],
                Unknown1: &self.row.columns[self.index_mapping[56]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[57]],
                ItemAccepted: &self.row.columns[self.index_mapping[58]],
                Unknown0: &self.row.columns[self.index_mapping[59]],
                Unknown1: &self.row.columns[self.index_mapping[60]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[61]],
                ItemAccepted: &self.row.columns[self.index_mapping[62]],
                Unknown0: &self.row.columns[self.index_mapping[63]],
                Unknown1: &self.row.columns[self.index_mapping[64]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[65]],
                ItemAccepted: &self.row.columns[self.index_mapping[66]],
                Unknown0: &self.row.columns[self.index_mapping[67]],
                Unknown1: &self.row.columns[self.index_mapping[68]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[69]],
                ItemAccepted: &self.row.columns[self.index_mapping[70]],
                Unknown0: &self.row.columns[self.index_mapping[71]],
                Unknown1: &self.row.columns[self.index_mapping[72]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[73]],
                ItemAccepted: &self.row.columns[self.index_mapping[74]],
                Unknown0: &self.row.columns[self.index_mapping[75]],
                Unknown1: &self.row.columns[self.index_mapping[76]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[77]],
                ItemAccepted: &self.row.columns[self.index_mapping[78]],
                Unknown0: &self.row.columns[self.index_mapping[79]],
                Unknown1: &self.row.columns[self.index_mapping[80]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[81]],
                ItemAccepted: &self.row.columns[self.index_mapping[82]],
                Unknown0: &self.row.columns[self.index_mapping[83]],
                Unknown1: &self.row.columns[self.index_mapping[84]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[85]],
                ItemAccepted: &self.row.columns[self.index_mapping[86]],
                Unknown0: &self.row.columns[self.index_mapping[87]],
                Unknown1: &self.row.columns[self.index_mapping[88]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[89]],
                ItemAccepted: &self.row.columns[self.index_mapping[90]],
                Unknown0: &self.row.columns[self.index_mapping[91]],
                Unknown1: &self.row.columns[self.index_mapping[92]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[93]],
                ItemAccepted: &self.row.columns[self.index_mapping[94]],
                Unknown0: &self.row.columns[self.index_mapping[95]],
                Unknown1: &self.row.columns[self.index_mapping[96]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[97]],
                ItemAccepted: &self.row.columns[self.index_mapping[98]],
                Unknown0: &self.row.columns[self.index_mapping[99]],
                Unknown1: &self.row.columns[self.index_mapping[100]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[101]],
                ItemAccepted: &self.row.columns[self.index_mapping[102]],
                Unknown0: &self.row.columns[self.index_mapping[103]],
                Unknown1: &self.row.columns[self.index_mapping[104]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[105]],
                ItemAccepted: &self.row.columns[self.index_mapping[106]],
                Unknown0: &self.row.columns[self.index_mapping[107]],
                Unknown1: &self.row.columns[self.index_mapping[108]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[109]],
                ItemAccepted: &self.row.columns[self.index_mapping[110]],
                Unknown0: &self.row.columns[self.index_mapping[111]],
                Unknown1: &self.row.columns[self.index_mapping[112]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[113]],
                ItemAccepted: &self.row.columns[self.index_mapping[114]],
                Unknown0: &self.row.columns[self.index_mapping[115]],
                Unknown1: &self.row.columns[self.index_mapping[116]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[117]],
                ItemAccepted: &self.row.columns[self.index_mapping[118]],
                Unknown0: &self.row.columns[self.index_mapping[119]],
                Unknown1: &self.row.columns[self.index_mapping[120]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[121]],
                ItemAccepted: &self.row.columns[self.index_mapping[122]],
                Unknown0: &self.row.columns[self.index_mapping[123]],
                Unknown1: &self.row.columns[self.index_mapping[124]],
            },
            LotteryExchangeParamsElement {
                AmountAccepted: &self.row.columns[self.index_mapping[125]],
                ItemAccepted: &self.row.columns[self.index_mapping[126]],
                Unknown0: &self.row.columns[self.index_mapping[127]],
                Unknown1: &self.row.columns[self.index_mapping[128]],
            },
        ]
    }
    pub fn Script(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[129]]
    }
    pub fn LogMessage(&'a self) -> [&'a Field; 3] {
        [
            &self.row.columns[self.index_mapping[130]],
            &self.row.columns[self.index_mapping[131]],
            &self.row.columns[self.index_mapping[132]],
        ]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[133]]
    }
}
