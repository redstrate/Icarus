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
pub struct ItemLevelSheet {
    sheet: Sheet,
}
impl ItemLevelSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ItemLevel")?;
        let sheet = resolver.read_excel_sheet(&exh, "ItemLevel", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ItemLevelRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ItemLevelRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for ItemLevelSheet {
    type Row = ItemLevelRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Strength: row
                .columns[0]
                .into_u16()
                .copied()
                .expect("Expected column 0 to be a uint16!"),
            Dexterity: row
                .columns[1]
                .into_u16()
                .copied()
                .expect("Expected column 1 to be a uint16!"),
            Vitality: row
                .columns[2]
                .into_u16()
                .copied()
                .expect("Expected column 2 to be a uint16!"),
            Intelligence: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            Mind: row
                .columns[4]
                .into_u16()
                .copied()
                .expect("Expected column 4 to be a uint16!"),
            Piety: row
                .columns[5]
                .into_u16()
                .copied()
                .expect("Expected column 5 to be a uint16!"),
            HP: row
                .columns[6]
                .into_u16()
                .copied()
                .expect("Expected column 6 to be a uint16!"),
            MP: row
                .columns[7]
                .into_u16()
                .copied()
                .expect("Expected column 7 to be a uint16!"),
            TP: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            GP: row
                .columns[9]
                .into_u16()
                .copied()
                .expect("Expected column 9 to be a uint16!"),
            CP: row
                .columns[10]
                .into_u16()
                .copied()
                .expect("Expected column 10 to be a uint16!"),
            PhysicalDamage: row
                .columns[11]
                .into_u16()
                .copied()
                .expect("Expected column 11 to be a uint16!"),
            MagicalDamage: row
                .columns[12]
                .into_u16()
                .copied()
                .expect("Expected column 12 to be a uint16!"),
            Delay: row
                .columns[13]
                .into_u16()
                .copied()
                .expect("Expected column 13 to be a uint16!"),
            AdditionalEffect: row
                .columns[14]
                .into_u16()
                .copied()
                .expect("Expected column 14 to be a uint16!"),
            AttackSpeed: row
                .columns[15]
                .into_u16()
                .copied()
                .expect("Expected column 15 to be a uint16!"),
            BlockRate: row
                .columns[16]
                .into_u16()
                .copied()
                .expect("Expected column 16 to be a uint16!"),
            BlockStrength: row
                .columns[17]
                .into_u16()
                .copied()
                .expect("Expected column 17 to be a uint16!"),
            Tenacity: row
                .columns[18]
                .into_u16()
                .copied()
                .expect("Expected column 18 to be a uint16!"),
            AttackPower: row
                .columns[19]
                .into_u16()
                .copied()
                .expect("Expected column 19 to be a uint16!"),
            Defense: row
                .columns[20]
                .into_u16()
                .copied()
                .expect("Expected column 20 to be a uint16!"),
            DirectHitRate: row
                .columns[21]
                .into_u16()
                .copied()
                .expect("Expected column 21 to be a uint16!"),
            Evasion: row
                .columns[22]
                .into_u16()
                .copied()
                .expect("Expected column 22 to be a uint16!"),
            MagicDefense: row
                .columns[23]
                .into_u16()
                .copied()
                .expect("Expected column 23 to be a uint16!"),
            CriticalHitPower: row
                .columns[24]
                .into_u16()
                .copied()
                .expect("Expected column 24 to be a uint16!"),
            CriticalHitResilience: row
                .columns[25]
                .into_u16()
                .copied()
                .expect("Expected column 25 to be a uint16!"),
            CriticalHit: row
                .columns[26]
                .into_u16()
                .copied()
                .expect("Expected column 26 to be a uint16!"),
            CriticalHitEvasion: row
                .columns[27]
                .into_u16()
                .copied()
                .expect("Expected column 27 to be a uint16!"),
            SlashingResistance: row
                .columns[28]
                .into_u16()
                .copied()
                .expect("Expected column 28 to be a uint16!"),
            PiercingResistance: row
                .columns[29]
                .into_u16()
                .copied()
                .expect("Expected column 29 to be a uint16!"),
            BluntResistance: row
                .columns[30]
                .into_u16()
                .copied()
                .expect("Expected column 30 to be a uint16!"),
            ProjectileResistance: row
                .columns[31]
                .into_u16()
                .copied()
                .expect("Expected column 31 to be a uint16!"),
            AttackMagicPotency: row
                .columns[32]
                .into_u16()
                .copied()
                .expect("Expected column 32 to be a uint16!"),
            HealingMagicPotency: row
                .columns[33]
                .into_u16()
                .copied()
                .expect("Expected column 33 to be a uint16!"),
            EnhancementMagicPotency: row
                .columns[34]
                .into_u16()
                .copied()
                .expect("Expected column 34 to be a uint16!"),
            EnfeeblingMagicPotency: row
                .columns[35]
                .into_u16()
                .copied()
                .expect("Expected column 35 to be a uint16!"),
            FireResistance: row
                .columns[36]
                .into_u16()
                .copied()
                .expect("Expected column 36 to be a uint16!"),
            IceResistance: row
                .columns[37]
                .into_u16()
                .copied()
                .expect("Expected column 37 to be a uint16!"),
            WindResistance: row
                .columns[38]
                .into_u16()
                .copied()
                .expect("Expected column 38 to be a uint16!"),
            EarthResistance: row
                .columns[39]
                .into_u16()
                .copied()
                .expect("Expected column 39 to be a uint16!"),
            LightningResistance: row
                .columns[40]
                .into_u16()
                .copied()
                .expect("Expected column 40 to be a uint16!"),
            WaterResistance: row
                .columns[41]
                .into_u16()
                .copied()
                .expect("Expected column 41 to be a uint16!"),
            MagicResistance: row
                .columns[42]
                .into_u16()
                .copied()
                .expect("Expected column 42 to be a uint16!"),
            Determination: row
                .columns[43]
                .into_u16()
                .copied()
                .expect("Expected column 43 to be a uint16!"),
            SkillSpeed: row
                .columns[44]
                .into_u16()
                .copied()
                .expect("Expected column 44 to be a uint16!"),
            SpellSpeed: row
                .columns[45]
                .into_u16()
                .copied()
                .expect("Expected column 45 to be a uint16!"),
            Haste: row
                .columns[46]
                .into_u16()
                .copied()
                .expect("Expected column 46 to be a uint16!"),
            Morale: row
                .columns[47]
                .into_u16()
                .copied()
                .expect("Expected column 47 to be a uint16!"),
            Enmity: row
                .columns[48]
                .into_u16()
                .copied()
                .expect("Expected column 48 to be a uint16!"),
            EnmityReduction: row
                .columns[49]
                .into_u16()
                .copied()
                .expect("Expected column 49 to be a uint16!"),
            CarefulDesynthesis: row
                .columns[50]
                .into_u16()
                .copied()
                .expect("Expected column 50 to be a uint16!"),
            EXPBonus: row
                .columns[51]
                .into_u16()
                .copied()
                .expect("Expected column 51 to be a uint16!"),
            Regen: row
                .columns[52]
                .into_u16()
                .copied()
                .expect("Expected column 52 to be a uint16!"),
            Refresh: row
                .columns[53]
                .into_u16()
                .copied()
                .expect("Expected column 53 to be a uint16!"),
            MovementSpeed: row
                .columns[54]
                .into_u16()
                .copied()
                .expect("Expected column 54 to be a uint16!"),
            Spikes: row
                .columns[55]
                .into_u16()
                .copied()
                .expect("Expected column 55 to be a uint16!"),
            SlowResistance: row
                .columns[56]
                .into_u16()
                .copied()
                .expect("Expected column 56 to be a uint16!"),
            PetrificationResistance: row
                .columns[57]
                .into_u16()
                .copied()
                .expect("Expected column 57 to be a uint16!"),
            ParalysisResistance: row
                .columns[58]
                .into_u16()
                .copied()
                .expect("Expected column 58 to be a uint16!"),
            SilenceResistance: row
                .columns[59]
                .into_u16()
                .copied()
                .expect("Expected column 59 to be a uint16!"),
            BlindResistance: row
                .columns[60]
                .into_u16()
                .copied()
                .expect("Expected column 60 to be a uint16!"),
            PoisonResistance: row
                .columns[61]
                .into_u16()
                .copied()
                .expect("Expected column 61 to be a uint16!"),
            StunResistance: row
                .columns[62]
                .into_u16()
                .copied()
                .expect("Expected column 62 to be a uint16!"),
            SleepResistance: row
                .columns[63]
                .into_u16()
                .copied()
                .expect("Expected column 63 to be a uint16!"),
            BindResistance: row
                .columns[64]
                .into_u16()
                .copied()
                .expect("Expected column 64 to be a uint16!"),
            HeavyResistance: row
                .columns[65]
                .into_u16()
                .copied()
                .expect("Expected column 65 to be a uint16!"),
            DoomResistance: row
                .columns[66]
                .into_u16()
                .copied()
                .expect("Expected column 66 to be a uint16!"),
            ReducedDurabilityLoss: row
                .columns[67]
                .into_u16()
                .copied()
                .expect("Expected column 67 to be a uint16!"),
            IncreasedSpiritbondGain: row
                .columns[68]
                .into_u16()
                .copied()
                .expect("Expected column 68 to be a uint16!"),
            Craftsmanship: row
                .columns[69]
                .into_u16()
                .copied()
                .expect("Expected column 69 to be a uint16!"),
            Control: row
                .columns[70]
                .into_u16()
                .copied()
                .expect("Expected column 70 to be a uint16!"),
            Gathering: row
                .columns[71]
                .into_u16()
                .copied()
                .expect("Expected column 71 to be a uint16!"),
            Perception: row
                .columns[72]
                .into_u16()
                .copied()
                .expect("Expected column 72 to be a uint16!"),
            Unknown0: row
                .columns[73]
                .into_u16()
                .copied()
                .expect("Expected column 73 to be a uint16!"),
        })
    }
}
impl<'a> IntoIterator for &'a ItemLevelSheet {
    type Item = (u32, Vec<(u16, ItemLevelRow)>);
    type IntoIter = StructuredSheetIterator<'a, ItemLevelSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ItemLevelSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ItemLevelRow {
    ///""
    pub Strength: u16,
    ///""
    pub Dexterity: u16,
    ///""
    pub Vitality: u16,
    ///""
    pub Intelligence: u16,
    ///""
    pub Mind: u16,
    ///""
    pub Piety: u16,
    ///""
    pub HP: u16,
    ///""
    pub MP: u16,
    ///""
    pub TP: u16,
    ///""
    pub GP: u16,
    ///""
    pub CP: u16,
    ///""
    pub PhysicalDamage: u16,
    ///""
    pub MagicalDamage: u16,
    ///""
    pub Delay: u16,
    ///""
    pub AdditionalEffect: u16,
    ///""
    pub AttackSpeed: u16,
    ///""
    pub BlockRate: u16,
    ///""
    pub BlockStrength: u16,
    ///""
    pub Tenacity: u16,
    ///""
    pub AttackPower: u16,
    ///""
    pub Defense: u16,
    ///""
    pub DirectHitRate: u16,
    ///""
    pub Evasion: u16,
    ///""
    pub MagicDefense: u16,
    ///""
    pub CriticalHitPower: u16,
    ///""
    pub CriticalHitResilience: u16,
    ///""
    pub CriticalHit: u16,
    ///""
    pub CriticalHitEvasion: u16,
    ///""
    pub SlashingResistance: u16,
    ///""
    pub PiercingResistance: u16,
    ///""
    pub BluntResistance: u16,
    ///""
    pub ProjectileResistance: u16,
    ///""
    pub AttackMagicPotency: u16,
    ///""
    pub HealingMagicPotency: u16,
    ///""
    pub EnhancementMagicPotency: u16,
    ///""
    pub EnfeeblingMagicPotency: u16,
    ///""
    pub FireResistance: u16,
    ///""
    pub IceResistance: u16,
    ///""
    pub WindResistance: u16,
    ///""
    pub EarthResistance: u16,
    ///""
    pub LightningResistance: u16,
    ///""
    pub WaterResistance: u16,
    ///""
    pub MagicResistance: u16,
    ///""
    pub Determination: u16,
    ///""
    pub SkillSpeed: u16,
    ///""
    pub SpellSpeed: u16,
    ///""
    pub Haste: u16,
    ///""
    pub Morale: u16,
    ///""
    pub Enmity: u16,
    ///""
    pub EnmityReduction: u16,
    ///""
    pub CarefulDesynthesis: u16,
    ///""
    pub EXPBonus: u16,
    ///""
    pub Regen: u16,
    ///""
    pub Refresh: u16,
    ///""
    pub MovementSpeed: u16,
    ///""
    pub Spikes: u16,
    ///""
    pub SlowResistance: u16,
    ///""
    pub PetrificationResistance: u16,
    ///""
    pub ParalysisResistance: u16,
    ///""
    pub SilenceResistance: u16,
    ///""
    pub BlindResistance: u16,
    ///""
    pub PoisonResistance: u16,
    ///""
    pub StunResistance: u16,
    ///""
    pub SleepResistance: u16,
    ///""
    pub BindResistance: u16,
    ///""
    pub HeavyResistance: u16,
    ///""
    pub DoomResistance: u16,
    ///""
    pub ReducedDurabilityLoss: u16,
    ///""
    pub IncreasedSpiritbondGain: u16,
    ///""
    pub Craftsmanship: u16,
    ///""
    pub Control: u16,
    ///""
    pub Gathering: u16,
    ///""
    pub Perception: u16,
    ///""
    pub Unknown0: u16,
}
