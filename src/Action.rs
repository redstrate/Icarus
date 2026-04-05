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
}
impl ActionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Action")?;
        let sheet = resolver.read_excel_sheet(&exh, "Action", language)?;
        Ok(Self { sheet })
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
        Some(Self::Row { row })
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
}
impl<'a> ActionRow<'a> {
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[0].into_string().unwrap()
    }
    pub fn UnlockLink(&'a self) -> u32 {
        self.row.columns[49].into_u32().copied().unwrap()
    }
    pub fn Icon(&'a self) -> u16 {
        self.row.columns[2].into_u16().copied().unwrap()
    }
    pub fn VFX(&'a self) -> u16 {
        self.row.columns[6].into_u16().copied().unwrap()
    }
    pub fn ActionTimelineHit(&'a self) -> u16 {
        self.row.columns[8].into_u16().copied().unwrap()
    }
    pub fn PrimaryCostValue(&'a self) -> u16 {
        self.row.columns[33].into_u16().copied().unwrap()
    }
    pub fn SecondaryCostValue(&'a self) -> u16 {
        self.row.columns[35].into_u16().copied().unwrap()
    }
    pub fn ActionCombo(&'a self) -> u16 {
        self.row.columns[36].into_u16().copied().unwrap()
    }
    pub fn Cast100ms(&'a self) -> u16 {
        self.row.columns[38].into_u16().copied().unwrap()
    }
    pub fn Recast100ms(&'a self) -> u16 {
        self.row.columns[40].into_u16().copied().unwrap()
    }
    pub fn ActionProcStatus(&'a self) -> u16 {
        self.row.columns[46].into_u16().copied().unwrap()
    }
    pub fn StatusGainSelf(&'a self) -> u16 {
        self.row.columns[48].into_u16().copied().unwrap()
    }
    pub fn Omen(&'a self) -> u16 {
        self.row.columns[54].into_u16().copied().unwrap()
    }
    pub fn OmenAlt(&'a self) -> u16 {
        self.row.columns[55].into_u16().copied().unwrap()
    }
    pub fn AnimationEnd(&'a self) -> i16 {
        self.row.columns[7].into_i16().copied().unwrap()
    }
    pub fn ActionCategory(&'a self) -> u8 {
        self.row.columns[3].into_u8().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> u8 {
        self.row.columns[4].into_u8().copied().unwrap()
    }
    pub fn AnimationStart(&'a self) -> u8 {
        self.row.columns[5].into_u8().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> u8 {
        self.row.columns[9].into_u8().copied().unwrap()
    }
    pub fn BehaviourType(&'a self) -> u8 {
        self.row.columns[11].into_u8().copied().unwrap()
    }
    pub fn ClassJobLevel(&'a self) -> u8 {
        self.row.columns[12].into_u8().copied().unwrap()
    }
    pub fn CastType(&'a self) -> u8 {
        self.row.columns[28].into_u8().copied().unwrap()
    }
    pub fn EffectRange(&'a self) -> u8 {
        self.row.columns[29].into_u8().copied().unwrap()
    }
    pub fn XAxisModifier(&'a self) -> u8 {
        self.row.columns[30].into_u8().copied().unwrap()
    }
    pub fn PrimaryCostType(&'a self) -> u8 {
        self.row.columns[32].into_u8().copied().unwrap()
    }
    pub fn SecondaryCostType(&'a self) -> u8 {
        self.row.columns[34].into_u8().copied().unwrap()
    }
    pub fn ExtraCastTime100ms(&'a self) -> u8 {
        self.row.columns[39].into_u8().copied().unwrap()
    }
    pub fn CooldownGroup(&'a self) -> u8 {
        self.row.columns[41].into_u8().copied().unwrap()
    }
    pub fn AdditionalCooldownGroup(&'a self) -> u8 {
        self.row.columns[42].into_u8().copied().unwrap()
    }
    pub fn MaxCharges(&'a self) -> u8 {
        self.row.columns[43].into_u8().copied().unwrap()
    }
    pub fn Aspect(&'a self) -> u8 {
        self.row.columns[45].into_u8().copied().unwrap()
    }
    pub fn Unknown4(&'a self) -> u8 {
        self.row.columns[47].into_u8().copied().unwrap()
    }
    pub fn ClassJobCategory(&'a self) -> u8 {
        self.row.columns[50].into_u8().copied().unwrap()
    }
    /// 0 = no effect on auto attacks, 1 = typical cast (unsheathe weapon, but don't touch autos), 2 = typical weaponskill (unsheathe, start autos if action target is primary target), 3 = sleep-like effect (unsheathe, stop autos), 4 = typical point-blank aoe (unsheathe, start autos if primary target is within action range), 5 = ??? (doesn't touch weapon state), 6 = typical channeled action (unsheathe, stop autos, do not auto-face target), 7 = force sheathe and stop autos, 8 = ??? (unused)
    pub fn AutoAttackBehaviour(&'a self) -> u8 {
        self.row.columns[51].into_u8().copied().unwrap()
    }
    /// set for eg crafting actions, when different jobs have different rows, but they need to be considered interchangeable
    pub fn EquivalenceGroup(&'a self) -> u8 {
        self.row.columns[65].into_u8().copied().unwrap()
    }
    pub fn Unknown_70(&'a self) -> u8 {
        self.row.columns[70].into_u8().copied().unwrap()
    }
    pub fn ClassJob(&'a self) -> i8 {
        self.row.columns[10].into_i8().copied().unwrap()
    }
    pub fn Range(&'a self) -> i8 {
        self.row.columns[15].into_i8().copied().unwrap()
    }
    /// 0 = can not target dead, 1 = can only target dead players (+ some other conditions), 2 = ???
    pub fn DeadTargetBehaviour(&'a self) -> i8 {
        self.row.columns[25].into_i8().copied().unwrap()
    }
    pub fn AttackType(&'a self) -> i8 {
        self.row.columns[44].into_i8().copied().unwrap()
    }
    pub fn Unknown8(&'a self) -> bool {
        self.row.columns[1].into_bool().copied().unwrap()
    }
    pub fn IsRoleAction(&'a self) -> bool {
        self.row.columns[13].into_bool().copied().unwrap()
    }
    pub fn Unknown28(&'a self) -> bool {
        self.row.columns[14].into_bool().copied().unwrap()
    }
    pub fn CanTargetSelf(&'a self) -> bool {
        self.row.columns[16].into_bool().copied().unwrap()
    }
    pub fn CanTargetParty(&'a self) -> bool {
        self.row.columns[17].into_bool().copied().unwrap()
    }
    pub fn CanTargetAlliance(&'a self) -> bool {
        self.row.columns[18].into_bool().copied().unwrap()
    }
    pub fn CanTargetHostile(&'a self) -> bool {
        self.row.columns[19].into_bool().copied().unwrap()
    }
    pub fn CanTargetAlly(&'a self) -> bool {
        self.row.columns[20].into_bool().copied().unwrap()
    }
    pub fn Unknown10(&'a self) -> bool {
        self.row.columns[21].into_bool().copied().unwrap()
    }
    pub fn TargetArea(&'a self) -> bool {
        self.row.columns[22].into_bool().copied().unwrap()
    }
    pub fn CanTargetOwnPet(&'a self) -> bool {
        self.row.columns[23].into_bool().copied().unwrap()
    }
    pub fn CanTargetPartyPet(&'a self) -> bool {
        self.row.columns[24].into_bool().copied().unwrap()
    }
    pub fn RequiresLineOfSight(&'a self) -> bool {
        self.row.columns[26].into_bool().copied().unwrap()
    }
    pub fn NeedToFaceTarget(&'a self) -> bool {
        self.row.columns[27].into_bool().copied().unwrap()
    }
    pub fn Unknown14(&'a self) -> bool {
        self.row.columns[31].into_bool().copied().unwrap()
    }
    pub fn PreservesCombo(&'a self) -> bool {
        self.row.columns[37].into_bool().copied().unwrap()
    }
    pub fn Unknown15(&'a self) -> bool {
        self.row.columns[52].into_bool().copied().unwrap()
    }
    pub fn AffectsPosition(&'a self) -> bool {
        self.row.columns[53].into_bool().copied().unwrap()
    }
    pub fn IsPvP(&'a self) -> bool {
        self.row.columns[56].into_bool().copied().unwrap()
    }
    pub fn Unknown16(&'a self) -> bool {
        self.row.columns[57].into_bool().copied().unwrap()
    }
    pub fn LogCastMessage(&'a self) -> bool {
        self.row.columns[58].into_bool().copied().unwrap()
    }
    pub fn Unknown18(&'a self) -> bool {
        self.row.columns[59].into_bool().copied().unwrap()
    }
    pub fn LogMissMessage(&'a self) -> bool {
        self.row.columns[60].into_bool().copied().unwrap()
    }
    pub fn LogActionMessage(&'a self) -> bool {
        self.row.columns[61].into_bool().copied().unwrap()
    }
    pub fn Unknown21(&'a self) -> bool {
        self.row.columns[62].into_bool().copied().unwrap()
    }
    pub fn Unknown22(&'a self) -> bool {
        self.row.columns[63].into_bool().copied().unwrap()
    }
    pub fn Unknown23(&'a self) -> bool {
        self.row.columns[64].into_bool().copied().unwrap()
    }
    pub fn CanUseWhileMounted(&'a self) -> bool {
        self.row.columns[66].into_bool().copied().unwrap()
    }
    pub fn Unknown25(&'a self) -> bool {
        self.row.columns[67].into_bool().copied().unwrap()
    }
    pub fn IsPlayerAction(&'a self) -> bool {
        self.row.columns[68].into_bool().copied().unwrap()
    }
    pub fn Unknown27(&'a self) -> bool {
        self.row.columns[69].into_bool().copied().unwrap()
    }
}
