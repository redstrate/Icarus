//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct StagesElement<'a> {
    pub RequiredAmount: [&'a Field; 6],
    pub MaxAmount: [&'a Field; 6],
}
#[derive(Debug, Clone)]
pub struct WKSCosmoToolDataAmountSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl WKSCosmoToolDataAmountSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WKSCosmoToolDataAmount")?;
        let sheet = resolver.read_excel_sheet(&exh, "WKSCosmoToolDataAmount", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<WKSCosmoToolDataAmountRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<WKSCosmoToolDataAmountRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for WKSCosmoToolDataAmountSheet {
    type Row = WKSCosmoToolDataAmountRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a WKSCosmoToolDataAmountSheet {
    type Item = (u32, Vec<(u16, WKSCosmoToolDataAmountRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, WKSCosmoToolDataAmountSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WKSCosmoToolDataAmountSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct WKSCosmoToolDataAmountRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> WKSCosmoToolDataAmountRow<'a> {
    pub fn Stages(&'a self) -> [StagesElement<'a>; 17] {
        [
            StagesElement {
                RequiredAmount: [
                    &self.row.columns[self.index_mapping[0]],
                    &self.row.columns[self.index_mapping[1]],
                    &self.row.columns[self.index_mapping[2]],
                    &self.row.columns[self.index_mapping[3]],
                    &self.row.columns[self.index_mapping[4]],
                    &self.row.columns[self.index_mapping[5]],
                ],
                MaxAmount: [
                    &self.row.columns[self.index_mapping[6]],
                    &self.row.columns[self.index_mapping[7]],
                    &self.row.columns[self.index_mapping[8]],
                    &self.row.columns[self.index_mapping[9]],
                    &self.row.columns[self.index_mapping[10]],
                    &self.row.columns[self.index_mapping[11]],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.row.columns[self.index_mapping[12]],
                    &self.row.columns[self.index_mapping[13]],
                    &self.row.columns[self.index_mapping[14]],
                    &self.row.columns[self.index_mapping[15]],
                    &self.row.columns[self.index_mapping[16]],
                    &self.row.columns[self.index_mapping[17]],
                ],
                MaxAmount: [
                    &self.row.columns[self.index_mapping[18]],
                    &self.row.columns[self.index_mapping[19]],
                    &self.row.columns[self.index_mapping[20]],
                    &self.row.columns[self.index_mapping[21]],
                    &self.row.columns[self.index_mapping[22]],
                    &self.row.columns[self.index_mapping[23]],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.row.columns[self.index_mapping[24]],
                    &self.row.columns[self.index_mapping[25]],
                    &self.row.columns[self.index_mapping[26]],
                    &self.row.columns[self.index_mapping[27]],
                    &self.row.columns[self.index_mapping[28]],
                    &self.row.columns[self.index_mapping[29]],
                ],
                MaxAmount: [
                    &self.row.columns[self.index_mapping[30]],
                    &self.row.columns[self.index_mapping[31]],
                    &self.row.columns[self.index_mapping[32]],
                    &self.row.columns[self.index_mapping[33]],
                    &self.row.columns[self.index_mapping[34]],
                    &self.row.columns[self.index_mapping[35]],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.row.columns[self.index_mapping[36]],
                    &self.row.columns[self.index_mapping[37]],
                    &self.row.columns[self.index_mapping[38]],
                    &self.row.columns[self.index_mapping[39]],
                    &self.row.columns[self.index_mapping[40]],
                    &self.row.columns[self.index_mapping[41]],
                ],
                MaxAmount: [
                    &self.row.columns[self.index_mapping[42]],
                    &self.row.columns[self.index_mapping[43]],
                    &self.row.columns[self.index_mapping[44]],
                    &self.row.columns[self.index_mapping[45]],
                    &self.row.columns[self.index_mapping[46]],
                    &self.row.columns[self.index_mapping[47]],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.row.columns[self.index_mapping[48]],
                    &self.row.columns[self.index_mapping[49]],
                    &self.row.columns[self.index_mapping[50]],
                    &self.row.columns[self.index_mapping[51]],
                    &self.row.columns[self.index_mapping[52]],
                    &self.row.columns[self.index_mapping[53]],
                ],
                MaxAmount: [
                    &self.row.columns[self.index_mapping[54]],
                    &self.row.columns[self.index_mapping[55]],
                    &self.row.columns[self.index_mapping[56]],
                    &self.row.columns[self.index_mapping[57]],
                    &self.row.columns[self.index_mapping[58]],
                    &self.row.columns[self.index_mapping[59]],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.row.columns[self.index_mapping[60]],
                    &self.row.columns[self.index_mapping[61]],
                    &self.row.columns[self.index_mapping[62]],
                    &self.row.columns[self.index_mapping[63]],
                    &self.row.columns[self.index_mapping[64]],
                    &self.row.columns[self.index_mapping[65]],
                ],
                MaxAmount: [
                    &self.row.columns[self.index_mapping[66]],
                    &self.row.columns[self.index_mapping[67]],
                    &self.row.columns[self.index_mapping[68]],
                    &self.row.columns[self.index_mapping[69]],
                    &self.row.columns[self.index_mapping[70]],
                    &self.row.columns[self.index_mapping[71]],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.row.columns[self.index_mapping[72]],
                    &self.row.columns[self.index_mapping[73]],
                    &self.row.columns[self.index_mapping[74]],
                    &self.row.columns[self.index_mapping[75]],
                    &self.row.columns[self.index_mapping[76]],
                    &self.row.columns[self.index_mapping[77]],
                ],
                MaxAmount: [
                    &self.row.columns[self.index_mapping[78]],
                    &self.row.columns[self.index_mapping[79]],
                    &self.row.columns[self.index_mapping[80]],
                    &self.row.columns[self.index_mapping[81]],
                    &self.row.columns[self.index_mapping[82]],
                    &self.row.columns[self.index_mapping[83]],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.row.columns[self.index_mapping[84]],
                    &self.row.columns[self.index_mapping[85]],
                    &self.row.columns[self.index_mapping[86]],
                    &self.row.columns[self.index_mapping[87]],
                    &self.row.columns[self.index_mapping[88]],
                    &self.row.columns[self.index_mapping[89]],
                ],
                MaxAmount: [
                    &self.row.columns[self.index_mapping[90]],
                    &self.row.columns[self.index_mapping[91]],
                    &self.row.columns[self.index_mapping[92]],
                    &self.row.columns[self.index_mapping[93]],
                    &self.row.columns[self.index_mapping[94]],
                    &self.row.columns[self.index_mapping[95]],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.row.columns[self.index_mapping[96]],
                    &self.row.columns[self.index_mapping[97]],
                    &self.row.columns[self.index_mapping[98]],
                    &self.row.columns[self.index_mapping[99]],
                    &self.row.columns[self.index_mapping[100]],
                    &self.row.columns[self.index_mapping[101]],
                ],
                MaxAmount: [
                    &self.row.columns[self.index_mapping[102]],
                    &self.row.columns[self.index_mapping[103]],
                    &self.row.columns[self.index_mapping[104]],
                    &self.row.columns[self.index_mapping[105]],
                    &self.row.columns[self.index_mapping[106]],
                    &self.row.columns[self.index_mapping[107]],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.row.columns[self.index_mapping[108]],
                    &self.row.columns[self.index_mapping[109]],
                    &self.row.columns[self.index_mapping[110]],
                    &self.row.columns[self.index_mapping[111]],
                    &self.row.columns[self.index_mapping[112]],
                    &self.row.columns[self.index_mapping[113]],
                ],
                MaxAmount: [
                    &self.row.columns[self.index_mapping[114]],
                    &self.row.columns[self.index_mapping[115]],
                    &self.row.columns[self.index_mapping[116]],
                    &self.row.columns[self.index_mapping[117]],
                    &self.row.columns[self.index_mapping[118]],
                    &self.row.columns[self.index_mapping[119]],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.row.columns[self.index_mapping[120]],
                    &self.row.columns[self.index_mapping[121]],
                    &self.row.columns[self.index_mapping[122]],
                    &self.row.columns[self.index_mapping[123]],
                    &self.row.columns[self.index_mapping[124]],
                    &self.row.columns[self.index_mapping[125]],
                ],
                MaxAmount: [
                    &self.row.columns[self.index_mapping[126]],
                    &self.row.columns[self.index_mapping[127]],
                    &self.row.columns[self.index_mapping[128]],
                    &self.row.columns[self.index_mapping[129]],
                    &self.row.columns[self.index_mapping[130]],
                    &self.row.columns[self.index_mapping[131]],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.row.columns[self.index_mapping[132]],
                    &self.row.columns[self.index_mapping[133]],
                    &self.row.columns[self.index_mapping[134]],
                    &self.row.columns[self.index_mapping[135]],
                    &self.row.columns[self.index_mapping[136]],
                    &self.row.columns[self.index_mapping[137]],
                ],
                MaxAmount: [
                    &self.row.columns[self.index_mapping[138]],
                    &self.row.columns[self.index_mapping[139]],
                    &self.row.columns[self.index_mapping[140]],
                    &self.row.columns[self.index_mapping[141]],
                    &self.row.columns[self.index_mapping[142]],
                    &self.row.columns[self.index_mapping[143]],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.row.columns[self.index_mapping[144]],
                    &self.row.columns[self.index_mapping[145]],
                    &self.row.columns[self.index_mapping[146]],
                    &self.row.columns[self.index_mapping[147]],
                    &self.row.columns[self.index_mapping[148]],
                    &self.row.columns[self.index_mapping[149]],
                ],
                MaxAmount: [
                    &self.row.columns[self.index_mapping[150]],
                    &self.row.columns[self.index_mapping[151]],
                    &self.row.columns[self.index_mapping[152]],
                    &self.row.columns[self.index_mapping[153]],
                    &self.row.columns[self.index_mapping[154]],
                    &self.row.columns[self.index_mapping[155]],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.row.columns[self.index_mapping[156]],
                    &self.row.columns[self.index_mapping[157]],
                    &self.row.columns[self.index_mapping[158]],
                    &self.row.columns[self.index_mapping[159]],
                    &self.row.columns[self.index_mapping[160]],
                    &self.row.columns[self.index_mapping[161]],
                ],
                MaxAmount: [
                    &self.row.columns[self.index_mapping[162]],
                    &self.row.columns[self.index_mapping[163]],
                    &self.row.columns[self.index_mapping[164]],
                    &self.row.columns[self.index_mapping[165]],
                    &self.row.columns[self.index_mapping[166]],
                    &self.row.columns[self.index_mapping[167]],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.row.columns[self.index_mapping[168]],
                    &self.row.columns[self.index_mapping[169]],
                    &self.row.columns[self.index_mapping[170]],
                    &self.row.columns[self.index_mapping[171]],
                    &self.row.columns[self.index_mapping[172]],
                    &self.row.columns[self.index_mapping[173]],
                ],
                MaxAmount: [
                    &self.row.columns[self.index_mapping[174]],
                    &self.row.columns[self.index_mapping[175]],
                    &self.row.columns[self.index_mapping[176]],
                    &self.row.columns[self.index_mapping[177]],
                    &self.row.columns[self.index_mapping[178]],
                    &self.row.columns[self.index_mapping[179]],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.row.columns[self.index_mapping[180]],
                    &self.row.columns[self.index_mapping[181]],
                    &self.row.columns[self.index_mapping[182]],
                    &self.row.columns[self.index_mapping[183]],
                    &self.row.columns[self.index_mapping[184]],
                    &self.row.columns[self.index_mapping[185]],
                ],
                MaxAmount: [
                    &self.row.columns[self.index_mapping[186]],
                    &self.row.columns[self.index_mapping[187]],
                    &self.row.columns[self.index_mapping[188]],
                    &self.row.columns[self.index_mapping[189]],
                    &self.row.columns[self.index_mapping[190]],
                    &self.row.columns[self.index_mapping[191]],
                ],
            },
            StagesElement {
                RequiredAmount: [
                    &self.row.columns[self.index_mapping[192]],
                    &self.row.columns[self.index_mapping[193]],
                    &self.row.columns[self.index_mapping[194]],
                    &self.row.columns[self.index_mapping[195]],
                    &self.row.columns[self.index_mapping[196]],
                    &self.row.columns[self.index_mapping[197]],
                ],
                MaxAmount: [
                    &self.row.columns[self.index_mapping[198]],
                    &self.row.columns[self.index_mapping[199]],
                    &self.row.columns[self.index_mapping[200]],
                    &self.row.columns[self.index_mapping[201]],
                    &self.row.columns[self.index_mapping[202]],
                    &self.row.columns[self.index_mapping[203]],
                ],
            },
        ]
    }
}
