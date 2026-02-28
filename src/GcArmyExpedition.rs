//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct ExpeditionParamsElement<'a> {
    pub RewardItem: &'a Field,
    pub RequiredPhysical: &'a Field,
    pub RequiredMental: &'a Field,
    pub RequiredTactical: &'a Field,
    pub RewardQuantity: &'a Field,
    pub PercentPhysicalMet: &'a Field,
    pub PercentMentalMet: &'a Field,
    pub PercentTacticalMet: &'a Field,
    pub PercentAllMet: &'a Field,
}
#[derive(Debug, Clone)]
pub struct GcArmyExpeditionSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl GcArmyExpeditionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GcArmyExpedition")?;
        let sheet = resolver.read_excel_sheet(&exh, "GcArmyExpedition", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<GcArmyExpeditionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<GcArmyExpeditionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for GcArmyExpeditionSheet {
    type Row = GcArmyExpeditionRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a GcArmyExpeditionSheet {
    type Item = (u32, Vec<(u16, GcArmyExpeditionRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, GcArmyExpeditionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GcArmyExpeditionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct GcArmyExpeditionRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> GcArmyExpeditionRow<'a> {
    pub fn Name(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn Description(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn ExpeditionParams(&'a self) -> [ExpeditionParamsElement<'a>; 6] {
        [
            ExpeditionParamsElement {
                RewardItem: &self.row.columns[self.index_mapping[2]],
                RequiredPhysical: &self.row.columns[self.index_mapping[3]],
                RequiredMental: &self.row.columns[self.index_mapping[4]],
                RequiredTactical: &self.row.columns[self.index_mapping[5]],
                RewardQuantity: &self.row.columns[self.index_mapping[6]],
                PercentPhysicalMet: &self.row.columns[self.index_mapping[7]],
                PercentMentalMet: &self.row.columns[self.index_mapping[8]],
                PercentTacticalMet: &self.row.columns[self.index_mapping[9]],
                PercentAllMet: &self.row.columns[self.index_mapping[10]],
            },
            ExpeditionParamsElement {
                RewardItem: &self.row.columns[self.index_mapping[11]],
                RequiredPhysical: &self.row.columns[self.index_mapping[12]],
                RequiredMental: &self.row.columns[self.index_mapping[13]],
                RequiredTactical: &self.row.columns[self.index_mapping[14]],
                RewardQuantity: &self.row.columns[self.index_mapping[15]],
                PercentPhysicalMet: &self.row.columns[self.index_mapping[16]],
                PercentMentalMet: &self.row.columns[self.index_mapping[17]],
                PercentTacticalMet: &self.row.columns[self.index_mapping[18]],
                PercentAllMet: &self.row.columns[self.index_mapping[19]],
            },
            ExpeditionParamsElement {
                RewardItem: &self.row.columns[self.index_mapping[20]],
                RequiredPhysical: &self.row.columns[self.index_mapping[21]],
                RequiredMental: &self.row.columns[self.index_mapping[22]],
                RequiredTactical: &self.row.columns[self.index_mapping[23]],
                RewardQuantity: &self.row.columns[self.index_mapping[24]],
                PercentPhysicalMet: &self.row.columns[self.index_mapping[25]],
                PercentMentalMet: &self.row.columns[self.index_mapping[26]],
                PercentTacticalMet: &self.row.columns[self.index_mapping[27]],
                PercentAllMet: &self.row.columns[self.index_mapping[28]],
            },
            ExpeditionParamsElement {
                RewardItem: &self.row.columns[self.index_mapping[29]],
                RequiredPhysical: &self.row.columns[self.index_mapping[30]],
                RequiredMental: &self.row.columns[self.index_mapping[31]],
                RequiredTactical: &self.row.columns[self.index_mapping[32]],
                RewardQuantity: &self.row.columns[self.index_mapping[33]],
                PercentPhysicalMet: &self.row.columns[self.index_mapping[34]],
                PercentMentalMet: &self.row.columns[self.index_mapping[35]],
                PercentTacticalMet: &self.row.columns[self.index_mapping[36]],
                PercentAllMet: &self.row.columns[self.index_mapping[37]],
            },
            ExpeditionParamsElement {
                RewardItem: &self.row.columns[self.index_mapping[38]],
                RequiredPhysical: &self.row.columns[self.index_mapping[39]],
                RequiredMental: &self.row.columns[self.index_mapping[40]],
                RequiredTactical: &self.row.columns[self.index_mapping[41]],
                RewardQuantity: &self.row.columns[self.index_mapping[42]],
                PercentPhysicalMet: &self.row.columns[self.index_mapping[43]],
                PercentMentalMet: &self.row.columns[self.index_mapping[44]],
                PercentTacticalMet: &self.row.columns[self.index_mapping[45]],
                PercentAllMet: &self.row.columns[self.index_mapping[46]],
            },
            ExpeditionParamsElement {
                RewardItem: &self.row.columns[self.index_mapping[47]],
                RequiredPhysical: &self.row.columns[self.index_mapping[48]],
                RequiredMental: &self.row.columns[self.index_mapping[49]],
                RequiredTactical: &self.row.columns[self.index_mapping[50]],
                RewardQuantity: &self.row.columns[self.index_mapping[51]],
                PercentPhysicalMet: &self.row.columns[self.index_mapping[52]],
                PercentMentalMet: &self.row.columns[self.index_mapping[53]],
                PercentTacticalMet: &self.row.columns[self.index_mapping[54]],
                PercentAllMet: &self.row.columns[self.index_mapping[55]],
            },
        ]
    }
    pub fn RewardExperience(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[56]]
    }
    pub fn RequiredSeals(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[57]]
    }
    pub fn RequiredFlag(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[58]]
    }
    pub fn UnlockFlag(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[59]]
    }
    pub fn RequiredLevel(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[60]]
    }
    pub fn PercentBase(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[61]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[62]]
    }
    pub fn GcArmyExpeditionType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[63]]
    }
}
