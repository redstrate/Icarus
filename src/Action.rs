//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Debug, Clone)]
pub struct ActionSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl ActionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Action")?;
        let sheet = resolver.read_excel_sheet(&exh, "Action", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<ActionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ActionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for ActionSheet {
    type Row = ActionRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a ActionSheet {
    type Item = (u32, Vec<(u16, ActionRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, ActionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ActionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ActionRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> ActionRow<'a> {
    pub fn Name(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn UnlockLink(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn Icon(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn VFX(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn ActionTimelineHit(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn PrimaryCostValue(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn SecondaryCostValue(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn ActionCombo(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn Cast100ms(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn Recast100ms(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn ActionProcStatus(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    pub fn StatusGainSelf(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn Omen(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn OmenAlt(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn AnimationEnd(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    pub fn ActionCategory(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[16]]
    }
    pub fn AnimationStart(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[17]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[18]]
    }
    pub fn BehaviourType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[19]]
    }
    pub fn ClassJobLevel(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[20]]
    }
    pub fn CastType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[21]]
    }
    pub fn EffectRange(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[22]]
    }
    pub fn XAxisModifier(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[23]]
    }
    pub fn PrimaryCostType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[24]]
    }
    pub fn SecondaryCostType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[25]]
    }
    pub fn ExtraCastTime100ms(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[26]]
    }
    pub fn CooldownGroup(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[27]]
    }
    pub fn AdditionalCooldownGroup(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[28]]
    }
    pub fn MaxCharges(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[29]]
    }
    pub fn Aspect(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[30]]
    }
    pub fn Unknown4(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[31]]
    }
    pub fn ClassJobCategory(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[32]]
    }
    /// 0 = no effect on auto attacks, 1 = typical cast (unsheathe weapon, but don't touch autos), 2 = typical weaponskill (unsheathe, start autos if action target is primary target), 3 = sleep-like effect (unsheathe, stop autos), 4 = typical point-blank aoe (unsheathe, start autos if primary target is within action range), 5 = ??? (doesn't touch weapon state), 6 = typical channeled action (unsheathe, stop autos, do not auto-face target), 7 = force sheathe and stop autos, 8 = ??? (unused)
    pub fn AutoAttackBehaviour(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[33]]
    }
    /// set for eg crafting actions, when different jobs have different rows, but they need to be considered interchangeable
    pub fn EquivalenceGroup(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[34]]
    }
    pub fn Unknown_70(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[35]]
    }
    pub fn ClassJob(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[36]]
    }
    pub fn Range(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[37]]
    }
    /// 0 = can not target dead, 1 = can only target dead players (+ some other conditions), 2 = ???
    pub fn DeadTargetBehaviour(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[38]]
    }
    pub fn AttackType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[39]]
    }
    pub fn Unknown8(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[40]]
    }
    pub fn IsRoleAction(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[41]]
    }
    pub fn Unknown28(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[42]]
    }
    pub fn CanTargetSelf(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[43]]
    }
    pub fn CanTargetParty(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[44]]
    }
    pub fn CanTargetAlliance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[45]]
    }
    pub fn CanTargetHostile(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[46]]
    }
    pub fn CanTargetAlly(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[47]]
    }
    pub fn Unknown10(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[48]]
    }
    pub fn TargetArea(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[49]]
    }
    pub fn CanTargetOwnPet(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[50]]
    }
    pub fn CanTargetPartyPet(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[51]]
    }
    pub fn RequiresLineOfSight(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[52]]
    }
    pub fn NeedToFaceTarget(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[53]]
    }
    pub fn Unknown14(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[54]]
    }
    pub fn PreservesCombo(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[55]]
    }
    pub fn Unknown15(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[56]]
    }
    pub fn AffectsPosition(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[57]]
    }
    pub fn IsPvP(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[58]]
    }
    pub fn Unknown16(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[59]]
    }
    pub fn LogCastMessage(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[60]]
    }
    pub fn Unknown18(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[61]]
    }
    pub fn LogMissMessage(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[62]]
    }
    pub fn LogActionMessage(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[63]]
    }
    pub fn Unknown21(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[64]]
    }
    pub fn Unknown22(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[65]]
    }
    pub fn Unknown23(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[66]]
    }
    pub fn CanUseWhileMounted(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[67]]
    }
    pub fn Unknown25(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[68]]
    }
    pub fn IsPlayerAction(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[69]]
    }
    pub fn Unknown27(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[70]]
    }
}
