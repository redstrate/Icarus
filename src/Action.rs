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
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
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
impl StructuredSheet for ActionSheet {
    type Row = ActionRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            UnlockLink: row
                .columns[49]
                .into_u32()
                .copied()
                .expect("Expected column 49 to be a uint32!"),
            Icon: row
                .columns[2]
                .into_u16()
                .copied()
                .expect("Expected column 2 to be a uint16!"),
            VFX: row
                .columns[6]
                .into_u16()
                .copied()
                .expect("Expected column 6 to be a uint16!"),
            ActionTimelineHit: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            PrimaryCostValue: row
                .columns[33]
                .into_u16()
                .copied()
                .expect("Expected column 33 to be a uint16!"),
            SecondaryCostValue: row
                .columns[35]
                .into_u16()
                .copied()
                .expect("Expected column 35 to be a uint16!"),
            ActionCombo: row
                .columns[36]
                .into_u16()
                .copied()
                .expect("Expected column 36 to be a uint16!"),
            Cast100ms: row
                .columns[38]
                .into_u16()
                .copied()
                .expect("Expected column 38 to be a uint16!"),
            Recast100ms: row
                .columns[40]
                .into_u16()
                .copied()
                .expect("Expected column 40 to be a uint16!"),
            ActionProcStatus: row
                .columns[46]
                .into_u16()
                .copied()
                .expect("Expected column 46 to be a uint16!"),
            StatusGainSelf: row
                .columns[48]
                .into_u16()
                .copied()
                .expect("Expected column 48 to be a uint16!"),
            Omen: row
                .columns[54]
                .into_u16()
                .copied()
                .expect("Expected column 54 to be a uint16!"),
            OmenAlt: row
                .columns[55]
                .into_u16()
                .copied()
                .expect("Expected column 55 to be a uint16!"),
            AnimationEnd: row
                .columns[7]
                .into_i16()
                .copied()
                .expect("Expected column 7 to be a int16!"),
            ActionCategory: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
            Unknown1: row
                .columns[4]
                .into_u8()
                .copied()
                .expect("Expected column 4 to be a uint8!"),
            AnimationStart: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            Unknown2: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
            BehaviourType: row
                .columns[11]
                .into_u8()
                .copied()
                .expect("Expected column 11 to be a uint8!"),
            ClassJobLevel: row
                .columns[12]
                .into_u8()
                .copied()
                .expect("Expected column 12 to be a uint8!"),
            CastType: row
                .columns[28]
                .into_u8()
                .copied()
                .expect("Expected column 28 to be a uint8!"),
            EffectRange: row
                .columns[29]
                .into_u8()
                .copied()
                .expect("Expected column 29 to be a uint8!"),
            XAxisModifier: row
                .columns[30]
                .into_u8()
                .copied()
                .expect("Expected column 30 to be a uint8!"),
            PrimaryCostType: row
                .columns[32]
                .into_u8()
                .copied()
                .expect("Expected column 32 to be a uint8!"),
            SecondaryCostType: row
                .columns[34]
                .into_u8()
                .copied()
                .expect("Expected column 34 to be a uint8!"),
            ExtraCastTime100ms: row
                .columns[39]
                .into_u8()
                .copied()
                .expect("Expected column 39 to be a uint8!"),
            CooldownGroup: row
                .columns[41]
                .into_u8()
                .copied()
                .expect("Expected column 41 to be a uint8!"),
            AdditionalCooldownGroup: row
                .columns[42]
                .into_u8()
                .copied()
                .expect("Expected column 42 to be a uint8!"),
            MaxCharges: row
                .columns[43]
                .into_u8()
                .copied()
                .expect("Expected column 43 to be a uint8!"),
            Aspect: row
                .columns[45]
                .into_u8()
                .copied()
                .expect("Expected column 45 to be a uint8!"),
            Unknown4: row
                .columns[47]
                .into_u8()
                .copied()
                .expect("Expected column 47 to be a uint8!"),
            ClassJobCategory: row
                .columns[50]
                .into_u8()
                .copied()
                .expect("Expected column 50 to be a uint8!"),
            AutoAttackBehaviour: row
                .columns[51]
                .into_u8()
                .copied()
                .expect("Expected column 51 to be a uint8!"),
            EquivalenceGroup: row
                .columns[65]
                .into_u8()
                .copied()
                .expect("Expected column 65 to be a uint8!"),
            Unknown_70: row
                .columns[70]
                .into_u8()
                .copied()
                .expect("Expected column 70 to be a uint8!"),
            ClassJob: row
                .columns[10]
                .into_i8()
                .copied()
                .expect("Expected column 10 to be a int8!"),
            Range: row
                .columns[15]
                .into_i8()
                .copied()
                .expect("Expected column 15 to be a int8!"),
            DeadTargetBehaviour: row
                .columns[25]
                .into_i8()
                .copied()
                .expect("Expected column 25 to be a int8!"),
            AttackType: row
                .columns[44]
                .into_i8()
                .copied()
                .expect("Expected column 44 to be a int8!"),
            Unknown8: row
                .columns[1]
                .into_bool()
                .copied()
                .expect("Expected column 1 to be a bool!"),
            IsRoleAction: row
                .columns[13]
                .into_bool()
                .copied()
                .expect("Expected column 13 to be a bool!"),
            Unknown28: row
                .columns[14]
                .into_bool()
                .copied()
                .expect("Expected column 14 to be a bool!"),
            CanTargetSelf: row
                .columns[16]
                .into_bool()
                .copied()
                .expect("Expected column 16 to be a bool!"),
            CanTargetParty: row
                .columns[17]
                .into_bool()
                .copied()
                .expect("Expected column 17 to be a bool!"),
            CanTargetAlliance: row
                .columns[18]
                .into_bool()
                .copied()
                .expect("Expected column 18 to be a bool!"),
            CanTargetHostile: row
                .columns[19]
                .into_bool()
                .copied()
                .expect("Expected column 19 to be a bool!"),
            CanTargetAlly: row
                .columns[20]
                .into_bool()
                .copied()
                .expect("Expected column 20 to be a bool!"),
            Unknown10: row
                .columns[21]
                .into_bool()
                .copied()
                .expect("Expected column 21 to be a bool!"),
            TargetArea: row
                .columns[22]
                .into_bool()
                .copied()
                .expect("Expected column 22 to be a bool!"),
            CanTargetOwnPet: row
                .columns[23]
                .into_bool()
                .copied()
                .expect("Expected column 23 to be a bool!"),
            CanTargetPartyPet: row
                .columns[24]
                .into_bool()
                .copied()
                .expect("Expected column 24 to be a bool!"),
            RequiresLineOfSight: row
                .columns[26]
                .into_bool()
                .copied()
                .expect("Expected column 26 to be a bool!"),
            NeedToFaceTarget: row
                .columns[27]
                .into_bool()
                .copied()
                .expect("Expected column 27 to be a bool!"),
            Unknown14: row
                .columns[31]
                .into_bool()
                .copied()
                .expect("Expected column 31 to be a bool!"),
            PreservesCombo: row
                .columns[37]
                .into_bool()
                .copied()
                .expect("Expected column 37 to be a bool!"),
            Unknown15: row
                .columns[52]
                .into_bool()
                .copied()
                .expect("Expected column 52 to be a bool!"),
            AffectsPosition: row
                .columns[53]
                .into_bool()
                .copied()
                .expect("Expected column 53 to be a bool!"),
            IsPvP: row
                .columns[56]
                .into_bool()
                .copied()
                .expect("Expected column 56 to be a bool!"),
            Unknown16: row
                .columns[57]
                .into_bool()
                .copied()
                .expect("Expected column 57 to be a bool!"),
            LogCastMessage: row
                .columns[58]
                .into_bool()
                .copied()
                .expect("Expected column 58 to be a bool!"),
            Unknown18: row
                .columns[59]
                .into_bool()
                .copied()
                .expect("Expected column 59 to be a bool!"),
            LogMissMessage: row
                .columns[60]
                .into_bool()
                .copied()
                .expect("Expected column 60 to be a bool!"),
            LogActionMessage: row
                .columns[61]
                .into_bool()
                .copied()
                .expect("Expected column 61 to be a bool!"),
            Unknown21: row
                .columns[62]
                .into_bool()
                .copied()
                .expect("Expected column 62 to be a bool!"),
            Unknown22: row
                .columns[63]
                .into_bool()
                .copied()
                .expect("Expected column 63 to be a bool!"),
            Unknown23: row
                .columns[64]
                .into_bool()
                .copied()
                .expect("Expected column 64 to be a bool!"),
            CanUseWhileMounted: row
                .columns[66]
                .into_bool()
                .copied()
                .expect("Expected column 66 to be a bool!"),
            Unknown25: row
                .columns[67]
                .into_bool()
                .copied()
                .expect("Expected column 67 to be a bool!"),
            IsPlayerAction: row
                .columns[68]
                .into_bool()
                .copied()
                .expect("Expected column 68 to be a bool!"),
            Unknown27: row
                .columns[69]
                .into_bool()
                .copied()
                .expect("Expected column 69 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a ActionSheet {
    type Item = (u32, Vec<(u16, ActionRow)>);
    type IntoIter = StructuredSheetIterator<'a, ActionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ActionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ActionRow {
    ///""
    pub Name: String,
    ///""
    pub UnlockLink: u32,
    ///""
    pub Icon: u16,
    ///""
    pub VFX: u16,
    ///""
    pub ActionTimelineHit: u16,
    ///""
    pub PrimaryCostValue: u16,
    ///""
    pub SecondaryCostValue: u16,
    ///""
    pub ActionCombo: u16,
    ///""
    pub Cast100ms: u16,
    ///""
    pub Recast100ms: u16,
    ///""
    pub ActionProcStatus: u16,
    ///""
    pub StatusGainSelf: u16,
    ///""
    pub Omen: u16,
    ///""
    pub OmenAlt: u16,
    ///""
    pub AnimationEnd: i16,
    ///""
    pub ActionCategory: u8,
    ///""
    pub Unknown1: u8,
    ///""
    pub AnimationStart: u8,
    ///""
    pub Unknown2: u8,
    ///""
    pub BehaviourType: u8,
    ///""
    pub ClassJobLevel: u8,
    ///""
    pub CastType: u8,
    ///""
    pub EffectRange: u8,
    ///""
    pub XAxisModifier: u8,
    ///""
    pub PrimaryCostType: u8,
    ///""
    pub SecondaryCostType: u8,
    ///""
    pub ExtraCastTime100ms: u8,
    ///""
    pub CooldownGroup: u8,
    ///""
    pub AdditionalCooldownGroup: u8,
    ///""
    pub MaxCharges: u8,
    ///""
    pub Aspect: u8,
    ///""
    pub Unknown4: u8,
    ///""
    pub ClassJobCategory: u8,
    ///"0 = no effect on auto attacks, 1 = typical cast (unsheathe weapon, but don't touch autos), 2 = typical weaponskill (unsheathe, start autos if action target is primary target), 3 = sleep-like effect (unsheathe, stop autos), 4 = typical point-blank aoe (unsheathe, start autos if primary target is within action range), 5 = ??? (doesn't touch weapon state), 6 = typical channeled action (unsheathe, stop autos, do not auto-face target), 7 = force sheathe and stop autos, 8 = ??? (unused)"
    pub AutoAttackBehaviour: u8,
    ///"set for eg crafting actions, when different jobs have different rows, but they need to be considered interchangeable"
    pub EquivalenceGroup: u8,
    ///""
    pub Unknown_70: u8,
    ///""
    pub ClassJob: i8,
    ///""
    pub Range: i8,
    ///"0 = can not target dead, 1 = can only target dead players (+ some other conditions), 2 = ???"
    pub DeadTargetBehaviour: i8,
    ///""
    pub AttackType: i8,
    ///""
    pub Unknown8: bool,
    ///""
    pub IsRoleAction: bool,
    ///""
    pub Unknown28: bool,
    ///""
    pub CanTargetSelf: bool,
    ///""
    pub CanTargetParty: bool,
    ///""
    pub CanTargetAlliance: bool,
    ///""
    pub CanTargetHostile: bool,
    ///""
    pub CanTargetAlly: bool,
    ///""
    pub Unknown10: bool,
    ///""
    pub TargetArea: bool,
    ///""
    pub CanTargetOwnPet: bool,
    ///""
    pub CanTargetPartyPet: bool,
    ///""
    pub RequiresLineOfSight: bool,
    ///""
    pub NeedToFaceTarget: bool,
    ///""
    pub Unknown14: bool,
    ///""
    pub PreservesCombo: bool,
    ///""
    pub Unknown15: bool,
    ///""
    pub AffectsPosition: bool,
    ///""
    pub IsPvP: bool,
    ///""
    pub Unknown16: bool,
    ///""
    pub LogCastMessage: bool,
    ///""
    pub Unknown18: bool,
    ///""
    pub LogMissMessage: bool,
    ///""
    pub LogActionMessage: bool,
    ///""
    pub Unknown21: bool,
    ///""
    pub Unknown22: bool,
    ///""
    pub Unknown23: bool,
    ///""
    pub CanUseWhileMounted: bool,
    ///""
    pub Unknown25: bool,
    ///""
    pub IsPlayerAction: bool,
    ///""
    pub Unknown27: bool,
}
