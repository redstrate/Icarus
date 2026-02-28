//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct CompanyLeveStructElement<'a> {
    pub BNpcName: &'a Field,
    pub ToDoParam: [&'a Field; 6],
    pub BaseID: &'a Field,
    pub ItemsInvolved: &'a Field,
    pub EnemyLevel: &'a Field,
    pub ItemsInvolvedQty: &'a Field,
    pub ItemDropRate: &'a Field,
    pub NumOfAppearance: [&'a Field; 8],
}
#[derive(Debug, Clone)]
pub struct CompanyLeveSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl CompanyLeveSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("CompanyLeve")?;
        let sheet = resolver.read_excel_sheet(&exh, "CompanyLeve", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<CompanyLeveRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<CompanyLeveRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for CompanyLeveSheet {
    type Row = CompanyLeveRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a CompanyLeveSheet {
    type Item = (u32, Vec<(u16, CompanyLeveRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, CompanyLeveSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, CompanyLeveSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct CompanyLeveRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> CompanyLeveRow<'a> {
    pub fn RoutePointTime(&'a self) -> [&'a Field; 8] {
        [
            &self.row.columns[self.index_mapping[0]],
            &self.row.columns[self.index_mapping[1]],
            &self.row.columns[self.index_mapping[2]],
            &self.row.columns[self.index_mapping[3]],
            &self.row.columns[self.index_mapping[4]],
            &self.row.columns[self.index_mapping[5]],
            &self.row.columns[self.index_mapping[6]],
            &self.row.columns[self.index_mapping[7]],
        ]
    }
    pub fn CompanyLeveStruct(&'a self) -> [CompanyLeveStructElement<'a>; 8] {
        [
            CompanyLeveStructElement {
                BNpcName: &self.row.columns[self.index_mapping[8]],
                ToDoParam: [
                    &self.row.columns[self.index_mapping[9]],
                    &self.row.columns[self.index_mapping[10]],
                    &self.row.columns[self.index_mapping[11]],
                    &self.row.columns[self.index_mapping[12]],
                    &self.row.columns[self.index_mapping[13]],
                    &self.row.columns[self.index_mapping[14]],
                ],
                BaseID: &self.row.columns[self.index_mapping[15]],
                ItemsInvolved: &self.row.columns[self.index_mapping[16]],
                EnemyLevel: &self.row.columns[self.index_mapping[17]],
                ItemsInvolvedQty: &self.row.columns[self.index_mapping[18]],
                ItemDropRate: &self.row.columns[self.index_mapping[19]],
                NumOfAppearance: [
                    &self.row.columns[self.index_mapping[20]],
                    &self.row.columns[self.index_mapping[21]],
                    &self.row.columns[self.index_mapping[22]],
                    &self.row.columns[self.index_mapping[23]],
                    &self.row.columns[self.index_mapping[24]],
                    &self.row.columns[self.index_mapping[25]],
                    &self.row.columns[self.index_mapping[26]],
                    &self.row.columns[self.index_mapping[27]],
                ],
            },
            CompanyLeveStructElement {
                BNpcName: &self.row.columns[self.index_mapping[28]],
                ToDoParam: [
                    &self.row.columns[self.index_mapping[29]],
                    &self.row.columns[self.index_mapping[30]],
                    &self.row.columns[self.index_mapping[31]],
                    &self.row.columns[self.index_mapping[32]],
                    &self.row.columns[self.index_mapping[33]],
                    &self.row.columns[self.index_mapping[34]],
                ],
                BaseID: &self.row.columns[self.index_mapping[35]],
                ItemsInvolved: &self.row.columns[self.index_mapping[36]],
                EnemyLevel: &self.row.columns[self.index_mapping[37]],
                ItemsInvolvedQty: &self.row.columns[self.index_mapping[38]],
                ItemDropRate: &self.row.columns[self.index_mapping[39]],
                NumOfAppearance: [
                    &self.row.columns[self.index_mapping[40]],
                    &self.row.columns[self.index_mapping[41]],
                    &self.row.columns[self.index_mapping[42]],
                    &self.row.columns[self.index_mapping[43]],
                    &self.row.columns[self.index_mapping[44]],
                    &self.row.columns[self.index_mapping[45]],
                    &self.row.columns[self.index_mapping[46]],
                    &self.row.columns[self.index_mapping[47]],
                ],
            },
            CompanyLeveStructElement {
                BNpcName: &self.row.columns[self.index_mapping[48]],
                ToDoParam: [
                    &self.row.columns[self.index_mapping[49]],
                    &self.row.columns[self.index_mapping[50]],
                    &self.row.columns[self.index_mapping[51]],
                    &self.row.columns[self.index_mapping[52]],
                    &self.row.columns[self.index_mapping[53]],
                    &self.row.columns[self.index_mapping[54]],
                ],
                BaseID: &self.row.columns[self.index_mapping[55]],
                ItemsInvolved: &self.row.columns[self.index_mapping[56]],
                EnemyLevel: &self.row.columns[self.index_mapping[57]],
                ItemsInvolvedQty: &self.row.columns[self.index_mapping[58]],
                ItemDropRate: &self.row.columns[self.index_mapping[59]],
                NumOfAppearance: [
                    &self.row.columns[self.index_mapping[60]],
                    &self.row.columns[self.index_mapping[61]],
                    &self.row.columns[self.index_mapping[62]],
                    &self.row.columns[self.index_mapping[63]],
                    &self.row.columns[self.index_mapping[64]],
                    &self.row.columns[self.index_mapping[65]],
                    &self.row.columns[self.index_mapping[66]],
                    &self.row.columns[self.index_mapping[67]],
                ],
            },
            CompanyLeveStructElement {
                BNpcName: &self.row.columns[self.index_mapping[68]],
                ToDoParam: [
                    &self.row.columns[self.index_mapping[69]],
                    &self.row.columns[self.index_mapping[70]],
                    &self.row.columns[self.index_mapping[71]],
                    &self.row.columns[self.index_mapping[72]],
                    &self.row.columns[self.index_mapping[73]],
                    &self.row.columns[self.index_mapping[74]],
                ],
                BaseID: &self.row.columns[self.index_mapping[75]],
                ItemsInvolved: &self.row.columns[self.index_mapping[76]],
                EnemyLevel: &self.row.columns[self.index_mapping[77]],
                ItemsInvolvedQty: &self.row.columns[self.index_mapping[78]],
                ItemDropRate: &self.row.columns[self.index_mapping[79]],
                NumOfAppearance: [
                    &self.row.columns[self.index_mapping[80]],
                    &self.row.columns[self.index_mapping[81]],
                    &self.row.columns[self.index_mapping[82]],
                    &self.row.columns[self.index_mapping[83]],
                    &self.row.columns[self.index_mapping[84]],
                    &self.row.columns[self.index_mapping[85]],
                    &self.row.columns[self.index_mapping[86]],
                    &self.row.columns[self.index_mapping[87]],
                ],
            },
            CompanyLeveStructElement {
                BNpcName: &self.row.columns[self.index_mapping[88]],
                ToDoParam: [
                    &self.row.columns[self.index_mapping[89]],
                    &self.row.columns[self.index_mapping[90]],
                    &self.row.columns[self.index_mapping[91]],
                    &self.row.columns[self.index_mapping[92]],
                    &self.row.columns[self.index_mapping[93]],
                    &self.row.columns[self.index_mapping[94]],
                ],
                BaseID: &self.row.columns[self.index_mapping[95]],
                ItemsInvolved: &self.row.columns[self.index_mapping[96]],
                EnemyLevel: &self.row.columns[self.index_mapping[97]],
                ItemsInvolvedQty: &self.row.columns[self.index_mapping[98]],
                ItemDropRate: &self.row.columns[self.index_mapping[99]],
                NumOfAppearance: [
                    &self.row.columns[self.index_mapping[100]],
                    &self.row.columns[self.index_mapping[101]],
                    &self.row.columns[self.index_mapping[102]],
                    &self.row.columns[self.index_mapping[103]],
                    &self.row.columns[self.index_mapping[104]],
                    &self.row.columns[self.index_mapping[105]],
                    &self.row.columns[self.index_mapping[106]],
                    &self.row.columns[self.index_mapping[107]],
                ],
            },
            CompanyLeveStructElement {
                BNpcName: &self.row.columns[self.index_mapping[108]],
                ToDoParam: [
                    &self.row.columns[self.index_mapping[109]],
                    &self.row.columns[self.index_mapping[110]],
                    &self.row.columns[self.index_mapping[111]],
                    &self.row.columns[self.index_mapping[112]],
                    &self.row.columns[self.index_mapping[113]],
                    &self.row.columns[self.index_mapping[114]],
                ],
                BaseID: &self.row.columns[self.index_mapping[115]],
                ItemsInvolved: &self.row.columns[self.index_mapping[116]],
                EnemyLevel: &self.row.columns[self.index_mapping[117]],
                ItemsInvolvedQty: &self.row.columns[self.index_mapping[118]],
                ItemDropRate: &self.row.columns[self.index_mapping[119]],
                NumOfAppearance: [
                    &self.row.columns[self.index_mapping[120]],
                    &self.row.columns[self.index_mapping[121]],
                    &self.row.columns[self.index_mapping[122]],
                    &self.row.columns[self.index_mapping[123]],
                    &self.row.columns[self.index_mapping[124]],
                    &self.row.columns[self.index_mapping[125]],
                    &self.row.columns[self.index_mapping[126]],
                    &self.row.columns[self.index_mapping[127]],
                ],
            },
            CompanyLeveStructElement {
                BNpcName: &self.row.columns[self.index_mapping[128]],
                ToDoParam: [
                    &self.row.columns[self.index_mapping[129]],
                    &self.row.columns[self.index_mapping[130]],
                    &self.row.columns[self.index_mapping[131]],
                    &self.row.columns[self.index_mapping[132]],
                    &self.row.columns[self.index_mapping[133]],
                    &self.row.columns[self.index_mapping[134]],
                ],
                BaseID: &self.row.columns[self.index_mapping[135]],
                ItemsInvolved: &self.row.columns[self.index_mapping[136]],
                EnemyLevel: &self.row.columns[self.index_mapping[137]],
                ItemsInvolvedQty: &self.row.columns[self.index_mapping[138]],
                ItemDropRate: &self.row.columns[self.index_mapping[139]],
                NumOfAppearance: [
                    &self.row.columns[self.index_mapping[140]],
                    &self.row.columns[self.index_mapping[141]],
                    &self.row.columns[self.index_mapping[142]],
                    &self.row.columns[self.index_mapping[143]],
                    &self.row.columns[self.index_mapping[144]],
                    &self.row.columns[self.index_mapping[145]],
                    &self.row.columns[self.index_mapping[146]],
                    &self.row.columns[self.index_mapping[147]],
                ],
            },
            CompanyLeveStructElement {
                BNpcName: &self.row.columns[self.index_mapping[148]],
                ToDoParam: [
                    &self.row.columns[self.index_mapping[149]],
                    &self.row.columns[self.index_mapping[150]],
                    &self.row.columns[self.index_mapping[151]],
                    &self.row.columns[self.index_mapping[152]],
                    &self.row.columns[self.index_mapping[153]],
                    &self.row.columns[self.index_mapping[154]],
                ],
                BaseID: &self.row.columns[self.index_mapping[155]],
                ItemsInvolved: &self.row.columns[self.index_mapping[156]],
                EnemyLevel: &self.row.columns[self.index_mapping[157]],
                ItemsInvolvedQty: &self.row.columns[self.index_mapping[158]],
                ItemDropRate: &self.row.columns[self.index_mapping[159]],
                NumOfAppearance: [
                    &self.row.columns[self.index_mapping[160]],
                    &self.row.columns[self.index_mapping[161]],
                    &self.row.columns[self.index_mapping[162]],
                    &self.row.columns[self.index_mapping[163]],
                    &self.row.columns[self.index_mapping[164]],
                    &self.row.columns[self.index_mapping[165]],
                    &self.row.columns[self.index_mapping[166]],
                    &self.row.columns[self.index_mapping[167]],
                ],
            },
        ]
    }
    pub fn ToDoSequence(&'a self) -> [&'a Field; 8] {
        [
            &self.row.columns[self.index_mapping[168]],
            &self.row.columns[self.index_mapping[169]],
            &self.row.columns[self.index_mapping[170]],
            &self.row.columns[self.index_mapping[171]],
            &self.row.columns[self.index_mapping[172]],
            &self.row.columns[self.index_mapping[173]],
            &self.row.columns[self.index_mapping[174]],
            &self.row.columns[self.index_mapping[175]],
        ]
    }
    pub fn Rule(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[176]]
    }
    pub fn RuleParam(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[177]]
    }
}
