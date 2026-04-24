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
pub struct ExpeditionParamsElement {
    pub RewardItem: i32,
    pub RequiredPhysical: u16,
    pub RequiredMental: u16,
    pub RequiredTactical: u16,
    pub RewardQuantity: u8,
    pub PercentPhysicalMet: u8,
    pub PercentMentalMet: u8,
    pub PercentTacticalMet: u8,
    pub PercentAllMet: u8,
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
impl<'a> StructuredSheet<'a> for GcArmyExpeditionSheet {
    type Row = GcArmyExpeditionRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
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
}
impl<'a> GcArmyExpeditionRow<'a> {
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[8].into_string().unwrap()
    }
    pub fn Description(&'a self) -> &'a str {
        self.row.columns[9].into_string().unwrap()
    }
    pub fn ExpeditionParams(&'a self) -> [ExpeditionParamsElement; 6] {
        [
            ExpeditionParamsElement {
                RewardItem: self.row.columns[10].into_i32().copied().unwrap(),
                RequiredPhysical: self.row.columns[22].into_u16().copied().unwrap(),
                RequiredMental: self.row.columns[34].into_u16().copied().unwrap(),
                RequiredTactical: self.row.columns[46].into_u16().copied().unwrap(),
                RewardQuantity: self.row.columns[16].into_u8().copied().unwrap(),
                PercentPhysicalMet: self.row.columns[28].into_u8().copied().unwrap(),
                PercentMentalMet: self.row.columns[40].into_u8().copied().unwrap(),
                PercentTacticalMet: self.row.columns[52].into_u8().copied().unwrap(),
                PercentAllMet: self.row.columns[58].into_u8().copied().unwrap(),
            },
            ExpeditionParamsElement {
                RewardItem: self.row.columns[11].into_i32().copied().unwrap(),
                RequiredPhysical: self.row.columns[23].into_u16().copied().unwrap(),
                RequiredMental: self.row.columns[35].into_u16().copied().unwrap(),
                RequiredTactical: self.row.columns[47].into_u16().copied().unwrap(),
                RewardQuantity: self.row.columns[17].into_u8().copied().unwrap(),
                PercentPhysicalMet: self.row.columns[29].into_u8().copied().unwrap(),
                PercentMentalMet: self.row.columns[41].into_u8().copied().unwrap(),
                PercentTacticalMet: self.row.columns[53].into_u8().copied().unwrap(),
                PercentAllMet: self.row.columns[59].into_u8().copied().unwrap(),
            },
            ExpeditionParamsElement {
                RewardItem: self.row.columns[12].into_i32().copied().unwrap(),
                RequiredPhysical: self.row.columns[24].into_u16().copied().unwrap(),
                RequiredMental: self.row.columns[36].into_u16().copied().unwrap(),
                RequiredTactical: self.row.columns[48].into_u16().copied().unwrap(),
                RewardQuantity: self.row.columns[18].into_u8().copied().unwrap(),
                PercentPhysicalMet: self.row.columns[30].into_u8().copied().unwrap(),
                PercentMentalMet: self.row.columns[42].into_u8().copied().unwrap(),
                PercentTacticalMet: self.row.columns[54].into_u8().copied().unwrap(),
                PercentAllMet: self.row.columns[60].into_u8().copied().unwrap(),
            },
            ExpeditionParamsElement {
                RewardItem: self.row.columns[13].into_i32().copied().unwrap(),
                RequiredPhysical: self.row.columns[25].into_u16().copied().unwrap(),
                RequiredMental: self.row.columns[37].into_u16().copied().unwrap(),
                RequiredTactical: self.row.columns[49].into_u16().copied().unwrap(),
                RewardQuantity: self.row.columns[19].into_u8().copied().unwrap(),
                PercentPhysicalMet: self.row.columns[31].into_u8().copied().unwrap(),
                PercentMentalMet: self.row.columns[43].into_u8().copied().unwrap(),
                PercentTacticalMet: self.row.columns[55].into_u8().copied().unwrap(),
                PercentAllMet: self.row.columns[61].into_u8().copied().unwrap(),
            },
            ExpeditionParamsElement {
                RewardItem: self.row.columns[14].into_i32().copied().unwrap(),
                RequiredPhysical: self.row.columns[26].into_u16().copied().unwrap(),
                RequiredMental: self.row.columns[38].into_u16().copied().unwrap(),
                RequiredTactical: self.row.columns[50].into_u16().copied().unwrap(),
                RewardQuantity: self.row.columns[20].into_u8().copied().unwrap(),
                PercentPhysicalMet: self.row.columns[32].into_u8().copied().unwrap(),
                PercentMentalMet: self.row.columns[44].into_u8().copied().unwrap(),
                PercentTacticalMet: self.row.columns[56].into_u8().copied().unwrap(),
                PercentAllMet: self.row.columns[62].into_u8().copied().unwrap(),
            },
            ExpeditionParamsElement {
                RewardItem: self.row.columns[15].into_i32().copied().unwrap(),
                RequiredPhysical: self.row.columns[27].into_u16().copied().unwrap(),
                RequiredMental: self.row.columns[39].into_u16().copied().unwrap(),
                RequiredTactical: self.row.columns[51].into_u16().copied().unwrap(),
                RewardQuantity: self.row.columns[21].into_u8().copied().unwrap(),
                PercentPhysicalMet: self.row.columns[33].into_u8().copied().unwrap(),
                PercentMentalMet: self.row.columns[45].into_u8().copied().unwrap(),
                PercentTacticalMet: self.row.columns[57].into_u8().copied().unwrap(),
                PercentAllMet: self.row.columns[63].into_u8().copied().unwrap(),
            },
        ]
    }
    pub fn RewardExperience(&'a self) -> u32 {
        self.row.columns[4].into_u32().copied().unwrap()
    }
    pub fn RequiredSeals(&'a self) -> u16 {
        self.row.columns[3].into_u16().copied().unwrap()
    }
    pub fn RequiredFlag(&'a self) -> u8 {
        self.row.columns[0].into_u8().copied().unwrap()
    }
    pub fn UnlockFlag(&'a self) -> u8 {
        self.row.columns[1].into_u8().copied().unwrap()
    }
    pub fn RequiredLevel(&'a self) -> u8 {
        self.row.columns[2].into_u8().copied().unwrap()
    }
    pub fn PercentBase(&'a self) -> u8 {
        self.row.columns[5].into_u8().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> u8 {
        self.row.columns[6].into_u8().copied().unwrap()
    }
    pub fn GcArmyExpeditionType(&'a self) -> u8 {
        self.row.columns[7].into_u8().copied().unwrap()
    }
}
