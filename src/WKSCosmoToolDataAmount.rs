//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StagesElement {
    pub RequiredAmount: [u16; 6],
    pub MaxAmount: [u16; 6],
}
#[derive(Debug, Clone)]
pub struct WKSCosmoToolDataAmountSheet {
    sheet: Sheet,
}
impl WKSCosmoToolDataAmountSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WKSCosmoToolDataAmount")?;
        let sheet = resolver.read_excel_sheet(&exh, "WKSCosmoToolDataAmount", language)?;
        Ok(Self { sheet })
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
        Some(Self::Row { row })
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
}
impl<'a> WKSCosmoToolDataAmountRow<'a> {
    pub fn Stages(&'a self) -> [StagesElement; 17] {
        [
            StagesElement {
                RequiredAmount: [
                    self.row.columns[0].into_u16().copied().unwrap(),
                    self.row.columns[34].into_u16().copied().unwrap(),
                    self.row.columns[68].into_u16().copied().unwrap(),
                    self.row.columns[102].into_u16().copied().unwrap(),
                    self.row.columns[136].into_u16().copied().unwrap(),
                    self.row.columns[170].into_u16().copied().unwrap(),
                ],
                MaxAmount: [
                    self.row.columns[17].into_u16().copied().unwrap(),
                    self.row.columns[51].into_u16().copied().unwrap(),
                    self.row.columns[85].into_u16().copied().unwrap(),
                    self.row.columns[119].into_u16().copied().unwrap(),
                    self.row.columns[153].into_u16().copied().unwrap(),
                    self.row.columns[187].into_u16().copied().unwrap(),
                ],
            },
            StagesElement {
                RequiredAmount: [
                    self.row.columns[1].into_u16().copied().unwrap(),
                    self.row.columns[35].into_u16().copied().unwrap(),
                    self.row.columns[69].into_u16().copied().unwrap(),
                    self.row.columns[103].into_u16().copied().unwrap(),
                    self.row.columns[137].into_u16().copied().unwrap(),
                    self.row.columns[171].into_u16().copied().unwrap(),
                ],
                MaxAmount: [
                    self.row.columns[18].into_u16().copied().unwrap(),
                    self.row.columns[52].into_u16().copied().unwrap(),
                    self.row.columns[86].into_u16().copied().unwrap(),
                    self.row.columns[120].into_u16().copied().unwrap(),
                    self.row.columns[154].into_u16().copied().unwrap(),
                    self.row.columns[188].into_u16().copied().unwrap(),
                ],
            },
            StagesElement {
                RequiredAmount: [
                    self.row.columns[2].into_u16().copied().unwrap(),
                    self.row.columns[36].into_u16().copied().unwrap(),
                    self.row.columns[70].into_u16().copied().unwrap(),
                    self.row.columns[104].into_u16().copied().unwrap(),
                    self.row.columns[138].into_u16().copied().unwrap(),
                    self.row.columns[172].into_u16().copied().unwrap(),
                ],
                MaxAmount: [
                    self.row.columns[19].into_u16().copied().unwrap(),
                    self.row.columns[53].into_u16().copied().unwrap(),
                    self.row.columns[87].into_u16().copied().unwrap(),
                    self.row.columns[121].into_u16().copied().unwrap(),
                    self.row.columns[155].into_u16().copied().unwrap(),
                    self.row.columns[189].into_u16().copied().unwrap(),
                ],
            },
            StagesElement {
                RequiredAmount: [
                    self.row.columns[3].into_u16().copied().unwrap(),
                    self.row.columns[37].into_u16().copied().unwrap(),
                    self.row.columns[71].into_u16().copied().unwrap(),
                    self.row.columns[105].into_u16().copied().unwrap(),
                    self.row.columns[139].into_u16().copied().unwrap(),
                    self.row.columns[173].into_u16().copied().unwrap(),
                ],
                MaxAmount: [
                    self.row.columns[20].into_u16().copied().unwrap(),
                    self.row.columns[54].into_u16().copied().unwrap(),
                    self.row.columns[88].into_u16().copied().unwrap(),
                    self.row.columns[122].into_u16().copied().unwrap(),
                    self.row.columns[156].into_u16().copied().unwrap(),
                    self.row.columns[190].into_u16().copied().unwrap(),
                ],
            },
            StagesElement {
                RequiredAmount: [
                    self.row.columns[4].into_u16().copied().unwrap(),
                    self.row.columns[38].into_u16().copied().unwrap(),
                    self.row.columns[72].into_u16().copied().unwrap(),
                    self.row.columns[106].into_u16().copied().unwrap(),
                    self.row.columns[140].into_u16().copied().unwrap(),
                    self.row.columns[174].into_u16().copied().unwrap(),
                ],
                MaxAmount: [
                    self.row.columns[21].into_u16().copied().unwrap(),
                    self.row.columns[55].into_u16().copied().unwrap(),
                    self.row.columns[89].into_u16().copied().unwrap(),
                    self.row.columns[123].into_u16().copied().unwrap(),
                    self.row.columns[157].into_u16().copied().unwrap(),
                    self.row.columns[191].into_u16().copied().unwrap(),
                ],
            },
            StagesElement {
                RequiredAmount: [
                    self.row.columns[5].into_u16().copied().unwrap(),
                    self.row.columns[39].into_u16().copied().unwrap(),
                    self.row.columns[73].into_u16().copied().unwrap(),
                    self.row.columns[107].into_u16().copied().unwrap(),
                    self.row.columns[141].into_u16().copied().unwrap(),
                    self.row.columns[175].into_u16().copied().unwrap(),
                ],
                MaxAmount: [
                    self.row.columns[22].into_u16().copied().unwrap(),
                    self.row.columns[56].into_u16().copied().unwrap(),
                    self.row.columns[90].into_u16().copied().unwrap(),
                    self.row.columns[124].into_u16().copied().unwrap(),
                    self.row.columns[158].into_u16().copied().unwrap(),
                    self.row.columns[192].into_u16().copied().unwrap(),
                ],
            },
            StagesElement {
                RequiredAmount: [
                    self.row.columns[6].into_u16().copied().unwrap(),
                    self.row.columns[40].into_u16().copied().unwrap(),
                    self.row.columns[74].into_u16().copied().unwrap(),
                    self.row.columns[108].into_u16().copied().unwrap(),
                    self.row.columns[142].into_u16().copied().unwrap(),
                    self.row.columns[176].into_u16().copied().unwrap(),
                ],
                MaxAmount: [
                    self.row.columns[23].into_u16().copied().unwrap(),
                    self.row.columns[57].into_u16().copied().unwrap(),
                    self.row.columns[91].into_u16().copied().unwrap(),
                    self.row.columns[125].into_u16().copied().unwrap(),
                    self.row.columns[159].into_u16().copied().unwrap(),
                    self.row.columns[193].into_u16().copied().unwrap(),
                ],
            },
            StagesElement {
                RequiredAmount: [
                    self.row.columns[7].into_u16().copied().unwrap(),
                    self.row.columns[41].into_u16().copied().unwrap(),
                    self.row.columns[75].into_u16().copied().unwrap(),
                    self.row.columns[109].into_u16().copied().unwrap(),
                    self.row.columns[143].into_u16().copied().unwrap(),
                    self.row.columns[177].into_u16().copied().unwrap(),
                ],
                MaxAmount: [
                    self.row.columns[24].into_u16().copied().unwrap(),
                    self.row.columns[58].into_u16().copied().unwrap(),
                    self.row.columns[92].into_u16().copied().unwrap(),
                    self.row.columns[126].into_u16().copied().unwrap(),
                    self.row.columns[160].into_u16().copied().unwrap(),
                    self.row.columns[194].into_u16().copied().unwrap(),
                ],
            },
            StagesElement {
                RequiredAmount: [
                    self.row.columns[8].into_u16().copied().unwrap(),
                    self.row.columns[42].into_u16().copied().unwrap(),
                    self.row.columns[76].into_u16().copied().unwrap(),
                    self.row.columns[110].into_u16().copied().unwrap(),
                    self.row.columns[144].into_u16().copied().unwrap(),
                    self.row.columns[178].into_u16().copied().unwrap(),
                ],
                MaxAmount: [
                    self.row.columns[25].into_u16().copied().unwrap(),
                    self.row.columns[59].into_u16().copied().unwrap(),
                    self.row.columns[93].into_u16().copied().unwrap(),
                    self.row.columns[127].into_u16().copied().unwrap(),
                    self.row.columns[161].into_u16().copied().unwrap(),
                    self.row.columns[195].into_u16().copied().unwrap(),
                ],
            },
            StagesElement {
                RequiredAmount: [
                    self.row.columns[9].into_u16().copied().unwrap(),
                    self.row.columns[43].into_u16().copied().unwrap(),
                    self.row.columns[77].into_u16().copied().unwrap(),
                    self.row.columns[111].into_u16().copied().unwrap(),
                    self.row.columns[145].into_u16().copied().unwrap(),
                    self.row.columns[179].into_u16().copied().unwrap(),
                ],
                MaxAmount: [
                    self.row.columns[26].into_u16().copied().unwrap(),
                    self.row.columns[60].into_u16().copied().unwrap(),
                    self.row.columns[94].into_u16().copied().unwrap(),
                    self.row.columns[128].into_u16().copied().unwrap(),
                    self.row.columns[162].into_u16().copied().unwrap(),
                    self.row.columns[196].into_u16().copied().unwrap(),
                ],
            },
            StagesElement {
                RequiredAmount: [
                    self.row.columns[10].into_u16().copied().unwrap(),
                    self.row.columns[44].into_u16().copied().unwrap(),
                    self.row.columns[78].into_u16().copied().unwrap(),
                    self.row.columns[112].into_u16().copied().unwrap(),
                    self.row.columns[146].into_u16().copied().unwrap(),
                    self.row.columns[180].into_u16().copied().unwrap(),
                ],
                MaxAmount: [
                    self.row.columns[27].into_u16().copied().unwrap(),
                    self.row.columns[61].into_u16().copied().unwrap(),
                    self.row.columns[95].into_u16().copied().unwrap(),
                    self.row.columns[129].into_u16().copied().unwrap(),
                    self.row.columns[163].into_u16().copied().unwrap(),
                    self.row.columns[197].into_u16().copied().unwrap(),
                ],
            },
            StagesElement {
                RequiredAmount: [
                    self.row.columns[11].into_u16().copied().unwrap(),
                    self.row.columns[45].into_u16().copied().unwrap(),
                    self.row.columns[79].into_u16().copied().unwrap(),
                    self.row.columns[113].into_u16().copied().unwrap(),
                    self.row.columns[147].into_u16().copied().unwrap(),
                    self.row.columns[181].into_u16().copied().unwrap(),
                ],
                MaxAmount: [
                    self.row.columns[28].into_u16().copied().unwrap(),
                    self.row.columns[62].into_u16().copied().unwrap(),
                    self.row.columns[96].into_u16().copied().unwrap(),
                    self.row.columns[130].into_u16().copied().unwrap(),
                    self.row.columns[164].into_u16().copied().unwrap(),
                    self.row.columns[198].into_u16().copied().unwrap(),
                ],
            },
            StagesElement {
                RequiredAmount: [
                    self.row.columns[12].into_u16().copied().unwrap(),
                    self.row.columns[46].into_u16().copied().unwrap(),
                    self.row.columns[80].into_u16().copied().unwrap(),
                    self.row.columns[114].into_u16().copied().unwrap(),
                    self.row.columns[148].into_u16().copied().unwrap(),
                    self.row.columns[182].into_u16().copied().unwrap(),
                ],
                MaxAmount: [
                    self.row.columns[29].into_u16().copied().unwrap(),
                    self.row.columns[63].into_u16().copied().unwrap(),
                    self.row.columns[97].into_u16().copied().unwrap(),
                    self.row.columns[131].into_u16().copied().unwrap(),
                    self.row.columns[165].into_u16().copied().unwrap(),
                    self.row.columns[199].into_u16().copied().unwrap(),
                ],
            },
            StagesElement {
                RequiredAmount: [
                    self.row.columns[13].into_u16().copied().unwrap(),
                    self.row.columns[47].into_u16().copied().unwrap(),
                    self.row.columns[81].into_u16().copied().unwrap(),
                    self.row.columns[115].into_u16().copied().unwrap(),
                    self.row.columns[149].into_u16().copied().unwrap(),
                    self.row.columns[183].into_u16().copied().unwrap(),
                ],
                MaxAmount: [
                    self.row.columns[30].into_u16().copied().unwrap(),
                    self.row.columns[64].into_u16().copied().unwrap(),
                    self.row.columns[98].into_u16().copied().unwrap(),
                    self.row.columns[132].into_u16().copied().unwrap(),
                    self.row.columns[166].into_u16().copied().unwrap(),
                    self.row.columns[200].into_u16().copied().unwrap(),
                ],
            },
            StagesElement {
                RequiredAmount: [
                    self.row.columns[14].into_u16().copied().unwrap(),
                    self.row.columns[48].into_u16().copied().unwrap(),
                    self.row.columns[82].into_u16().copied().unwrap(),
                    self.row.columns[116].into_u16().copied().unwrap(),
                    self.row.columns[150].into_u16().copied().unwrap(),
                    self.row.columns[184].into_u16().copied().unwrap(),
                ],
                MaxAmount: [
                    self.row.columns[31].into_u16().copied().unwrap(),
                    self.row.columns[65].into_u16().copied().unwrap(),
                    self.row.columns[99].into_u16().copied().unwrap(),
                    self.row.columns[133].into_u16().copied().unwrap(),
                    self.row.columns[167].into_u16().copied().unwrap(),
                    self.row.columns[201].into_u16().copied().unwrap(),
                ],
            },
            StagesElement {
                RequiredAmount: [
                    self.row.columns[15].into_u16().copied().unwrap(),
                    self.row.columns[49].into_u16().copied().unwrap(),
                    self.row.columns[83].into_u16().copied().unwrap(),
                    self.row.columns[117].into_u16().copied().unwrap(),
                    self.row.columns[151].into_u16().copied().unwrap(),
                    self.row.columns[185].into_u16().copied().unwrap(),
                ],
                MaxAmount: [
                    self.row.columns[32].into_u16().copied().unwrap(),
                    self.row.columns[66].into_u16().copied().unwrap(),
                    self.row.columns[100].into_u16().copied().unwrap(),
                    self.row.columns[134].into_u16().copied().unwrap(),
                    self.row.columns[168].into_u16().copied().unwrap(),
                    self.row.columns[202].into_u16().copied().unwrap(),
                ],
            },
            StagesElement {
                RequiredAmount: [
                    self.row.columns[16].into_u16().copied().unwrap(),
                    self.row.columns[50].into_u16().copied().unwrap(),
                    self.row.columns[84].into_u16().copied().unwrap(),
                    self.row.columns[118].into_u16().copied().unwrap(),
                    self.row.columns[152].into_u16().copied().unwrap(),
                    self.row.columns[186].into_u16().copied().unwrap(),
                ],
                MaxAmount: [
                    self.row.columns[33].into_u16().copied().unwrap(),
                    self.row.columns[67].into_u16().copied().unwrap(),
                    self.row.columns[101].into_u16().copied().unwrap(),
                    self.row.columns[135].into_u16().copied().unwrap(),
                    self.row.columns[169].into_u16().copied().unwrap(),
                    self.row.columns[203].into_u16().copied().unwrap(),
                ],
            },
        ]
    }
}
