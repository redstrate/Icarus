//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
#[derive(Debug, Clone)]
pub struct ActionSheet {
    sheet: ExcelSheet,
}
impl ActionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Action")?;
        let sheet = resolver.read_excel_sheet(exh, "Action", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<ActionRow> {
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
        Some(ActionRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<ActionRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<ActionRow> {
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
pub struct ActionRow {
    columns: Vec<ColumnData>,
}
impl ActionRow {
    pub fn Name<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn UnlockLink<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn Icon<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn VFX<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn ActionTimelineHit<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn PrimaryCostValue<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn SecondaryCostValue<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn ActionCombo<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn Cast100ms<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn Recast100ms<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn ActionProcStatus<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn StatusGainSelf<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn Omen<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn OmenAlt<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn AnimationEnd<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn ActionCategory<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn AnimationStart<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn BehaviourType<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn ClassJobLevel<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn CastType<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn EffectRange<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
    pub fn XAxisModifier<'a>(&'a self) -> &'a ColumnData {
        &self.columns[23]
    }
    pub fn PrimaryCostType<'a>(&'a self) -> &'a ColumnData {
        &self.columns[24]
    }
    pub fn SecondaryCostType<'a>(&'a self) -> &'a ColumnData {
        &self.columns[25]
    }
    pub fn ExtraCastTime100ms<'a>(&'a self) -> &'a ColumnData {
        &self.columns[26]
    }
    pub fn CooldownGroup<'a>(&'a self) -> &'a ColumnData {
        &self.columns[27]
    }
    pub fn AdditionalCooldownGroup<'a>(&'a self) -> &'a ColumnData {
        &self.columns[28]
    }
    pub fn MaxCharges<'a>(&'a self) -> &'a ColumnData {
        &self.columns[29]
    }
    pub fn Aspect<'a>(&'a self) -> &'a ColumnData {
        &self.columns[30]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a ColumnData {
        &self.columns[31]
    }
    pub fn ClassJobCategory<'a>(&'a self) -> &'a ColumnData {
        &self.columns[32]
    }
    /// 0 = no effect on auto attacks, 1 = typical cast (unsheathe weapon, but don't touch autos), 2 = typical weaponskill (unsheathe, start autos if action target is primary target), 3 = sleep-like effect (unsheathe, stop autos), 4 = typical point-blank aoe (unsheathe, start autos if primary target is within action range), 5 = ??? (doesn't touch weapon state), 6 = typical channeled action (unsheathe, stop autos, do not auto-face target), 7 = force sheathe and stop autos, 8 = ??? (unused)
    pub fn AutoAttackBehaviour<'a>(&'a self) -> &'a ColumnData {
        &self.columns[33]
    }
    /// set for eg crafting actions, when different jobs have different rows, but they need to be considered interchangeable
    pub fn EquivalenceGroup<'a>(&'a self) -> &'a ColumnData {
        &self.columns[34]
    }
    pub fn Unknown_70<'a>(&'a self) -> &'a ColumnData {
        &self.columns[35]
    }
    pub fn ClassJob<'a>(&'a self) -> &'a ColumnData {
        &self.columns[36]
    }
    pub fn Range<'a>(&'a self) -> &'a ColumnData {
        &self.columns[37]
    }
    /// 0 = can not target dead, 1 = can only target dead players (+ some other conditions), 2 = ???
    pub fn DeadTargetBehaviour<'a>(&'a self) -> &'a ColumnData {
        &self.columns[38]
    }
    pub fn AttackType<'a>(&'a self) -> &'a ColumnData {
        &self.columns[39]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a ColumnData {
        &self.columns[40]
    }
    pub fn IsRoleAction<'a>(&'a self) -> &'a ColumnData {
        &self.columns[41]
    }
    pub fn Unknown28<'a>(&'a self) -> &'a ColumnData {
        &self.columns[42]
    }
    pub fn CanTargetSelf<'a>(&'a self) -> &'a ColumnData {
        &self.columns[43]
    }
    pub fn CanTargetParty<'a>(&'a self) -> &'a ColumnData {
        &self.columns[44]
    }
    pub fn CanTargetAlliance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[45]
    }
    pub fn CanTargetHostile<'a>(&'a self) -> &'a ColumnData {
        &self.columns[46]
    }
    pub fn CanTargetAlly<'a>(&'a self) -> &'a ColumnData {
        &self.columns[47]
    }
    pub fn Unknown10<'a>(&'a self) -> &'a ColumnData {
        &self.columns[48]
    }
    pub fn TargetArea<'a>(&'a self) -> &'a ColumnData {
        &self.columns[49]
    }
    pub fn CanTargetOwnPet<'a>(&'a self) -> &'a ColumnData {
        &self.columns[50]
    }
    pub fn CanTargetPartyPet<'a>(&'a self) -> &'a ColumnData {
        &self.columns[51]
    }
    pub fn RequiresLineOfSight<'a>(&'a self) -> &'a ColumnData {
        &self.columns[52]
    }
    pub fn NeedToFaceTarget<'a>(&'a self) -> &'a ColumnData {
        &self.columns[53]
    }
    pub fn Unknown14<'a>(&'a self) -> &'a ColumnData {
        &self.columns[54]
    }
    pub fn PreservesCombo<'a>(&'a self) -> &'a ColumnData {
        &self.columns[55]
    }
    pub fn Unknown15<'a>(&'a self) -> &'a ColumnData {
        &self.columns[56]
    }
    pub fn AffectsPosition<'a>(&'a self) -> &'a ColumnData {
        &self.columns[57]
    }
    pub fn IsPvP<'a>(&'a self) -> &'a ColumnData {
        &self.columns[58]
    }
    pub fn Unknown16<'a>(&'a self) -> &'a ColumnData {
        &self.columns[59]
    }
    pub fn LogCastMessage<'a>(&'a self) -> &'a ColumnData {
        &self.columns[60]
    }
    pub fn Unknown18<'a>(&'a self) -> &'a ColumnData {
        &self.columns[61]
    }
    pub fn LogMissMessage<'a>(&'a self) -> &'a ColumnData {
        &self.columns[62]
    }
    pub fn LogActionMessage<'a>(&'a self) -> &'a ColumnData {
        &self.columns[63]
    }
    pub fn Unknown21<'a>(&'a self) -> &'a ColumnData {
        &self.columns[64]
    }
    pub fn Unknown22<'a>(&'a self) -> &'a ColumnData {
        &self.columns[65]
    }
    pub fn Unknown23<'a>(&'a self) -> &'a ColumnData {
        &self.columns[66]
    }
    pub fn CanUseWhileMounted<'a>(&'a self) -> &'a ColumnData {
        &self.columns[67]
    }
    pub fn Unknown25<'a>(&'a self) -> &'a ColumnData {
        &self.columns[68]
    }
    pub fn IsPlayerAction<'a>(&'a self) -> &'a ColumnData {
        &self.columns[69]
    }
    pub fn Unknown27<'a>(&'a self) -> &'a ColumnData {
        &self.columns[70]
    }
}
