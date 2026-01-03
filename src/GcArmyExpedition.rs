//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
pub struct ExpeditionParamsElement<'a> {
    pub RewardItem: &'a ColumnData,
    pub RequiredPhysical: &'a ColumnData,
    pub RequiredMental: &'a ColumnData,
    pub RequiredTactical: &'a ColumnData,
    pub RewardQuantity: &'a ColumnData,
    pub PercentPhysicalMet: &'a ColumnData,
    pub PercentMentalMet: &'a ColumnData,
    pub PercentTacticalMet: &'a ColumnData,
    pub PercentAllMet: &'a ColumnData,
}
#[derive(Debug, Clone)]
pub struct GcArmyExpeditionSheet {
    sheet: ExcelSheet,
}
impl GcArmyExpeditionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GcArmyExpedition")?;
        let sheet = resolver.read_excel_sheet(exh, "GcArmyExpedition", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<GcArmyExpeditionRow> {
        let column_defs = &self.sheet.exh.column_definitions;
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
        Some(GcArmyExpeditionRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<GcArmyExpeditionRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<GcArmyExpeditionRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => return None,
            ExcelRowKind::SubRows(subrows) => {
                &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1
            }
        };
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
pub struct GcArmyExpeditionRow {
    columns: Vec<ColumnData>,
}
impl GcArmyExpeditionRow {
    pub fn Name<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn Description<'a>(&'a self) -> &'a ColumnData {
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
    pub fn RewardExperience<'a>(&'a self) -> &'a ColumnData {
        &self.columns[56]
    }
    pub fn RequiredSeals<'a>(&'a self) -> &'a ColumnData {
        &self.columns[57]
    }
    pub fn RequiredFlag<'a>(&'a self) -> &'a ColumnData {
        &self.columns[58]
    }
    pub fn UnlockFlag<'a>(&'a self) -> &'a ColumnData {
        &self.columns[59]
    }
    pub fn RequiredLevel<'a>(&'a self) -> &'a ColumnData {
        &self.columns[60]
    }
    pub fn PercentBase<'a>(&'a self) -> &'a ColumnData {
        &self.columns[61]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[62]
    }
    pub fn GcArmyExpeditionType<'a>(&'a self) -> &'a ColumnData {
        &self.columns[63]
    }
}
