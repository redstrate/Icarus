//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct HugeCraftworksTurnInParamElement<'a> {
    pub RequestedItem: &'a Field,
    pub Unknown0: &'a Field,
    pub RequestedQuantity: &'a Field,
    pub Unknown1: &'a Field,
    pub Unknown2: &'a Field,
    pub Unknown3: &'a Field,
    pub Unknown4: &'a Field,
    pub Unknown5: &'a Field,
    pub Unknown6: &'a Field,
}
pub struct HugeCraftworksRewardParamElement<'a> {
    pub RewardItem: [&'a Field; 2],
    pub RewardQuantity: [&'a Field; 2],
    pub RewardHQ: [&'a Field; 2],
}
#[derive(Debug, Clone)]
pub struct HugeCraftworksNpcSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl HugeCraftworksNpcSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("HugeCraftworksNpc")?;
        let sheet = resolver.read_excel_sheet(&exh, "HugeCraftworksNpc", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<HugeCraftworksNpcRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<HugeCraftworksNpcRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for HugeCraftworksNpcSheet {
    type Row = HugeCraftworksNpcRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a HugeCraftworksNpcSheet {
    type Item = (u32, Vec<(u16, HugeCraftworksNpcRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, HugeCraftworksNpcSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, HugeCraftworksNpcSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct HugeCraftworksNpcRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> HugeCraftworksNpcRow<'a> {
    pub fn HugeCraftworksTurnInParam(
        &'a self,
    ) -> [HugeCraftworksTurnInParamElement<'a>; 6] {
        [
            HugeCraftworksTurnInParamElement {
                RequestedItem: &self.row.columns[self.index_mapping[0]],
                Unknown0: &self.row.columns[self.index_mapping[1]],
                RequestedQuantity: &self.row.columns[self.index_mapping[2]],
                Unknown1: &self.row.columns[self.index_mapping[3]],
                Unknown2: &self.row.columns[self.index_mapping[4]],
                Unknown3: &self.row.columns[self.index_mapping[5]],
                Unknown4: &self.row.columns[self.index_mapping[6]],
                Unknown5: &self.row.columns[self.index_mapping[7]],
                Unknown6: &self.row.columns[self.index_mapping[8]],
            },
            HugeCraftworksTurnInParamElement {
                RequestedItem: &self.row.columns[self.index_mapping[9]],
                Unknown0: &self.row.columns[self.index_mapping[10]],
                RequestedQuantity: &self.row.columns[self.index_mapping[11]],
                Unknown1: &self.row.columns[self.index_mapping[12]],
                Unknown2: &self.row.columns[self.index_mapping[13]],
                Unknown3: &self.row.columns[self.index_mapping[14]],
                Unknown4: &self.row.columns[self.index_mapping[15]],
                Unknown5: &self.row.columns[self.index_mapping[16]],
                Unknown6: &self.row.columns[self.index_mapping[17]],
            },
            HugeCraftworksTurnInParamElement {
                RequestedItem: &self.row.columns[self.index_mapping[18]],
                Unknown0: &self.row.columns[self.index_mapping[19]],
                RequestedQuantity: &self.row.columns[self.index_mapping[20]],
                Unknown1: &self.row.columns[self.index_mapping[21]],
                Unknown2: &self.row.columns[self.index_mapping[22]],
                Unknown3: &self.row.columns[self.index_mapping[23]],
                Unknown4: &self.row.columns[self.index_mapping[24]],
                Unknown5: &self.row.columns[self.index_mapping[25]],
                Unknown6: &self.row.columns[self.index_mapping[26]],
            },
            HugeCraftworksTurnInParamElement {
                RequestedItem: &self.row.columns[self.index_mapping[27]],
                Unknown0: &self.row.columns[self.index_mapping[28]],
                RequestedQuantity: &self.row.columns[self.index_mapping[29]],
                Unknown1: &self.row.columns[self.index_mapping[30]],
                Unknown2: &self.row.columns[self.index_mapping[31]],
                Unknown3: &self.row.columns[self.index_mapping[32]],
                Unknown4: &self.row.columns[self.index_mapping[33]],
                Unknown5: &self.row.columns[self.index_mapping[34]],
                Unknown6: &self.row.columns[self.index_mapping[35]],
            },
            HugeCraftworksTurnInParamElement {
                RequestedItem: &self.row.columns[self.index_mapping[36]],
                Unknown0: &self.row.columns[self.index_mapping[37]],
                RequestedQuantity: &self.row.columns[self.index_mapping[38]],
                Unknown1: &self.row.columns[self.index_mapping[39]],
                Unknown2: &self.row.columns[self.index_mapping[40]],
                Unknown3: &self.row.columns[self.index_mapping[41]],
                Unknown4: &self.row.columns[self.index_mapping[42]],
                Unknown5: &self.row.columns[self.index_mapping[43]],
                Unknown6: &self.row.columns[self.index_mapping[44]],
            },
            HugeCraftworksTurnInParamElement {
                RequestedItem: &self.row.columns[self.index_mapping[45]],
                Unknown0: &self.row.columns[self.index_mapping[46]],
                RequestedQuantity: &self.row.columns[self.index_mapping[47]],
                Unknown1: &self.row.columns[self.index_mapping[48]],
                Unknown2: &self.row.columns[self.index_mapping[49]],
                Unknown3: &self.row.columns[self.index_mapping[50]],
                Unknown4: &self.row.columns[self.index_mapping[51]],
                Unknown5: &self.row.columns[self.index_mapping[52]],
                Unknown6: &self.row.columns[self.index_mapping[53]],
            },
        ]
    }
    pub fn HugeCraftworksRewardParam(
        &'a self,
    ) -> [HugeCraftworksRewardParamElement<'a>; 6] {
        [
            HugeCraftworksRewardParamElement {
                RewardItem: [
                    &self.row.columns[self.index_mapping[54]],
                    &self.row.columns[self.index_mapping[55]],
                ],
                RewardQuantity: [
                    &self.row.columns[self.index_mapping[56]],
                    &self.row.columns[self.index_mapping[57]],
                ],
                RewardHQ: [
                    &self.row.columns[self.index_mapping[58]],
                    &self.row.columns[self.index_mapping[59]],
                ],
            },
            HugeCraftworksRewardParamElement {
                RewardItem: [
                    &self.row.columns[self.index_mapping[60]],
                    &self.row.columns[self.index_mapping[61]],
                ],
                RewardQuantity: [
                    &self.row.columns[self.index_mapping[62]],
                    &self.row.columns[self.index_mapping[63]],
                ],
                RewardHQ: [
                    &self.row.columns[self.index_mapping[64]],
                    &self.row.columns[self.index_mapping[65]],
                ],
            },
            HugeCraftworksRewardParamElement {
                RewardItem: [
                    &self.row.columns[self.index_mapping[66]],
                    &self.row.columns[self.index_mapping[67]],
                ],
                RewardQuantity: [
                    &self.row.columns[self.index_mapping[68]],
                    &self.row.columns[self.index_mapping[69]],
                ],
                RewardHQ: [
                    &self.row.columns[self.index_mapping[70]],
                    &self.row.columns[self.index_mapping[71]],
                ],
            },
            HugeCraftworksRewardParamElement {
                RewardItem: [
                    &self.row.columns[self.index_mapping[72]],
                    &self.row.columns[self.index_mapping[73]],
                ],
                RewardQuantity: [
                    &self.row.columns[self.index_mapping[74]],
                    &self.row.columns[self.index_mapping[75]],
                ],
                RewardHQ: [
                    &self.row.columns[self.index_mapping[76]],
                    &self.row.columns[self.index_mapping[77]],
                ],
            },
            HugeCraftworksRewardParamElement {
                RewardItem: [
                    &self.row.columns[self.index_mapping[78]],
                    &self.row.columns[self.index_mapping[79]],
                ],
                RewardQuantity: [
                    &self.row.columns[self.index_mapping[80]],
                    &self.row.columns[self.index_mapping[81]],
                ],
                RewardHQ: [
                    &self.row.columns[self.index_mapping[82]],
                    &self.row.columns[self.index_mapping[83]],
                ],
            },
            HugeCraftworksRewardParamElement {
                RewardItem: [
                    &self.row.columns[self.index_mapping[84]],
                    &self.row.columns[self.index_mapping[85]],
                ],
                RewardQuantity: [
                    &self.row.columns[self.index_mapping[86]],
                    &self.row.columns[self.index_mapping[87]],
                ],
                RewardHQ: [
                    &self.row.columns[self.index_mapping[88]],
                    &self.row.columns[self.index_mapping[89]],
                ],
            },
        ]
    }
    pub fn Transient(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[90]]
    }
    pub fn EventNpc(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[91]]
    }
    pub fn ClassJobCategory(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[92]]
    }
}
