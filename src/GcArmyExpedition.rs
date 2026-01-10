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
}
impl GcArmyExpeditionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GcArmyExpedition")?;
        let sheet = resolver.read_excel_sheet(&exh, "GcArmyExpedition", language)?;
        Ok(Self { sheet })
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
impl StructuredSheet for GcArmyExpeditionSheet {
    type Row = GcArmyExpeditionRow;
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
impl<'a> IntoIterator for &'a GcArmyExpeditionSheet {
    type Item = (u32, Vec<(u16, GcArmyExpeditionRow)>);
    type IntoIter = StructuredSheetIterator<'a, GcArmyExpeditionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GcArmyExpeditionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct GcArmyExpeditionRow {
    columns: Vec<Field>,
}
impl GcArmyExpeditionRow {
    pub fn Name<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn Description<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn ExpeditionParams<'a>(&'a self) -> [ExpeditionParamsElement<'a>; 6] {
        [
            ExpeditionParamsElement {
                RewardItem: &self.columns[2],
                RequiredPhysical: &self.columns[3],
                RequiredMental: &self.columns[4],
                RequiredTactical: &self.columns[5],
                RewardQuantity: &self.columns[6],
                PercentPhysicalMet: &self.columns[7],
                PercentMentalMet: &self.columns[8],
                PercentTacticalMet: &self.columns[9],
                PercentAllMet: &self.columns[10],
            },
            ExpeditionParamsElement {
                RewardItem: &self.columns[11],
                RequiredPhysical: &self.columns[12],
                RequiredMental: &self.columns[13],
                RequiredTactical: &self.columns[14],
                RewardQuantity: &self.columns[15],
                PercentPhysicalMet: &self.columns[16],
                PercentMentalMet: &self.columns[17],
                PercentTacticalMet: &self.columns[18],
                PercentAllMet: &self.columns[19],
            },
            ExpeditionParamsElement {
                RewardItem: &self.columns[20],
                RequiredPhysical: &self.columns[21],
                RequiredMental: &self.columns[22],
                RequiredTactical: &self.columns[23],
                RewardQuantity: &self.columns[24],
                PercentPhysicalMet: &self.columns[25],
                PercentMentalMet: &self.columns[26],
                PercentTacticalMet: &self.columns[27],
                PercentAllMet: &self.columns[28],
            },
            ExpeditionParamsElement {
                RewardItem: &self.columns[29],
                RequiredPhysical: &self.columns[30],
                RequiredMental: &self.columns[31],
                RequiredTactical: &self.columns[32],
                RewardQuantity: &self.columns[33],
                PercentPhysicalMet: &self.columns[34],
                PercentMentalMet: &self.columns[35],
                PercentTacticalMet: &self.columns[36],
                PercentAllMet: &self.columns[37],
            },
            ExpeditionParamsElement {
                RewardItem: &self.columns[38],
                RequiredPhysical: &self.columns[39],
                RequiredMental: &self.columns[40],
                RequiredTactical: &self.columns[41],
                RewardQuantity: &self.columns[42],
                PercentPhysicalMet: &self.columns[43],
                PercentMentalMet: &self.columns[44],
                PercentTacticalMet: &self.columns[45],
                PercentAllMet: &self.columns[46],
            },
            ExpeditionParamsElement {
                RewardItem: &self.columns[47],
                RequiredPhysical: &self.columns[48],
                RequiredMental: &self.columns[49],
                RequiredTactical: &self.columns[50],
                RewardQuantity: &self.columns[51],
                PercentPhysicalMet: &self.columns[52],
                PercentMentalMet: &self.columns[53],
                PercentTacticalMet: &self.columns[54],
                PercentAllMet: &self.columns[55],
            },
        ]
    }
    pub fn RewardExperience<'a>(&'a self) -> &'a Field {
        &self.columns[56]
    }
    pub fn RequiredSeals<'a>(&'a self) -> &'a Field {
        &self.columns[57]
    }
    pub fn RequiredFlag<'a>(&'a self) -> &'a Field {
        &self.columns[58]
    }
    pub fn UnlockFlag<'a>(&'a self) -> &'a Field {
        &self.columns[59]
    }
    pub fn RequiredLevel<'a>(&'a self) -> &'a Field {
        &self.columns[60]
    }
    pub fn PercentBase<'a>(&'a self) -> &'a Field {
        &self.columns[61]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[62]
    }
    pub fn GcArmyExpeditionType<'a>(&'a self) -> &'a Field {
        &self.columns[63]
    }
}
