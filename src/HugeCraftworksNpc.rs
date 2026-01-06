//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    common::Language,
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
}
impl HugeCraftworksNpcSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("HugeCraftworksNpc")?;
        let sheet = resolver.read_excel_sheet(&exh, "HugeCraftworksNpc", language)?;
        Ok(Self { sheet })
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
impl StructuredSheet for HugeCraftworksNpcSheet {
    type Row = HugeCraftworksNpcRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<Field>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(Self::Row { columns })
    }
}
impl<'a> IntoIterator for &'a HugeCraftworksNpcSheet {
    type Item = (u32, Vec<(u16, HugeCraftworksNpcRow)>);
    type IntoIter = StructuredSheetIterator<'a, HugeCraftworksNpcSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, HugeCraftworksNpcSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct HugeCraftworksNpcRow {
    columns: Vec<Field>,
}
impl HugeCraftworksNpcRow {
    pub fn HugeCraftworksTurnInParam<'a>(
        &'a self,
    ) -> [HugeCraftworksTurnInParamElement<'a>; 6] {
        [
            HugeCraftworksTurnInParamElement {
                RequestedItem: &self.columns[0],
                Unknown0: &self.columns[1],
                RequestedQuantity: &self.columns[2],
                Unknown1: &self.columns[3],
                Unknown2: &self.columns[4],
                Unknown3: &self.columns[5],
                Unknown4: &self.columns[6],
                Unknown5: &self.columns[7],
                Unknown6: &self.columns[8],
            },
            HugeCraftworksTurnInParamElement {
                RequestedItem: &self.columns[9],
                Unknown0: &self.columns[10],
                RequestedQuantity: &self.columns[11],
                Unknown1: &self.columns[12],
                Unknown2: &self.columns[13],
                Unknown3: &self.columns[14],
                Unknown4: &self.columns[15],
                Unknown5: &self.columns[16],
                Unknown6: &self.columns[17],
            },
            HugeCraftworksTurnInParamElement {
                RequestedItem: &self.columns[18],
                Unknown0: &self.columns[19],
                RequestedQuantity: &self.columns[20],
                Unknown1: &self.columns[21],
                Unknown2: &self.columns[22],
                Unknown3: &self.columns[23],
                Unknown4: &self.columns[24],
                Unknown5: &self.columns[25],
                Unknown6: &self.columns[26],
            },
            HugeCraftworksTurnInParamElement {
                RequestedItem: &self.columns[27],
                Unknown0: &self.columns[28],
                RequestedQuantity: &self.columns[29],
                Unknown1: &self.columns[30],
                Unknown2: &self.columns[31],
                Unknown3: &self.columns[32],
                Unknown4: &self.columns[33],
                Unknown5: &self.columns[34],
                Unknown6: &self.columns[35],
            },
            HugeCraftworksTurnInParamElement {
                RequestedItem: &self.columns[36],
                Unknown0: &self.columns[37],
                RequestedQuantity: &self.columns[38],
                Unknown1: &self.columns[39],
                Unknown2: &self.columns[40],
                Unknown3: &self.columns[41],
                Unknown4: &self.columns[42],
                Unknown5: &self.columns[43],
                Unknown6: &self.columns[44],
            },
            HugeCraftworksTurnInParamElement {
                RequestedItem: &self.columns[45],
                Unknown0: &self.columns[46],
                RequestedQuantity: &self.columns[47],
                Unknown1: &self.columns[48],
                Unknown2: &self.columns[49],
                Unknown3: &self.columns[50],
                Unknown4: &self.columns[51],
                Unknown5: &self.columns[52],
                Unknown6: &self.columns[53],
            },
        ]
    }
    pub fn HugeCraftworksRewardParam<'a>(
        &'a self,
    ) -> [HugeCraftworksRewardParamElement<'a>; 6] {
        [
            HugeCraftworksRewardParamElement {
                RewardItem: [&self.columns[54], &self.columns[55]],
                RewardQuantity: [&self.columns[56], &self.columns[57]],
                RewardHQ: [&self.columns[58], &self.columns[59]],
            },
            HugeCraftworksRewardParamElement {
                RewardItem: [&self.columns[60], &self.columns[61]],
                RewardQuantity: [&self.columns[62], &self.columns[63]],
                RewardHQ: [&self.columns[64], &self.columns[65]],
            },
            HugeCraftworksRewardParamElement {
                RewardItem: [&self.columns[66], &self.columns[67]],
                RewardQuantity: [&self.columns[68], &self.columns[69]],
                RewardHQ: [&self.columns[70], &self.columns[71]],
            },
            HugeCraftworksRewardParamElement {
                RewardItem: [&self.columns[72], &self.columns[73]],
                RewardQuantity: [&self.columns[74], &self.columns[75]],
                RewardHQ: [&self.columns[76], &self.columns[77]],
            },
            HugeCraftworksRewardParamElement {
                RewardItem: [&self.columns[78], &self.columns[79]],
                RewardQuantity: [&self.columns[80], &self.columns[81]],
                RewardHQ: [&self.columns[82], &self.columns[83]],
            },
            HugeCraftworksRewardParamElement {
                RewardItem: [&self.columns[84], &self.columns[85]],
                RewardQuantity: [&self.columns[86], &self.columns[87]],
                RewardHQ: [&self.columns[88], &self.columns[89]],
            },
        ]
    }
    pub fn Transient<'a>(&'a self) -> &'a Field {
        &self.columns[90]
    }
    pub fn EventNpc<'a>(&'a self) -> &'a Field {
        &self.columns[91]
    }
    pub fn ClassJobCategory<'a>(&'a self) -> &'a Field {
        &self.columns[92]
    }
}
