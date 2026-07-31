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
pub struct ItemSheet {
    sheet: Sheet,
}
impl ItemSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Item")?;
        let sheet = resolver.read_excel_sheet(&exh, "Item", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ItemRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ItemRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for ItemSheet {
    type Row = ItemRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Singular: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Plural: row
                .columns[2]
                .into_string()
                .cloned()
                .expect("Expected column 2 to be a string!"),
            Description: row
                .columns[8]
                .into_string()
                .cloned()
                .expect("Expected column 8 to be a string!"),
            Name: row
                .columns[9]
                .into_string()
                .cloned()
                .expect("Expected column 9 to be a string!"),
            Adjective: row
                .columns[1]
                .into_i8()
                .copied()
                .expect("Expected column 1 to be a int8!"),
            PossessivePronoun: row
                .columns[3]
                .into_i8()
                .copied()
                .expect("Expected column 3 to be a int8!"),
            StartsWithVowel: row
                .columns[4]
                .into_i8()
                .copied()
                .expect("Expected column 4 to be a int8!"),
            Unknown0: row
                .columns[5]
                .into_i8()
                .copied()
                .expect("Expected column 5 to be a int8!"),
            Pronoun: row
                .columns[6]
                .into_i8()
                .copied()
                .expect("Expected column 6 to be a int8!"),
            Article: row
                .columns[7]
                .into_i8()
                .copied()
                .expect("Expected column 7 to be a int8!"),
            ModelMain: row
                .columns[47]
                .into_u64()
                .copied()
                .expect("Expected column 47 to be a uint64!"),
            ModelSub: row
                .columns[48]
                .into_u64()
                .copied()
                .expect("Expected column 48 to be a uint64!"),
            DamagePhys: row
                .columns[51]
                .into_u16()
                .copied()
                .expect("Expected column 51 to be a uint16!"),
            DamageMag: row
                .columns[52]
                .into_u16()
                .copied()
                .expect("Expected column 52 to be a uint16!"),
            Delayms: row
                .columns[53]
                .into_u16()
                .copied()
                .expect("Expected column 53 to be a uint16!"),
            BlockRate: row
                .columns[55]
                .into_u16()
                .copied()
                .expect("Expected column 55 to be a uint16!"),
            Block: row
                .columns[56]
                .into_u16()
                .copied()
                .expect("Expected column 56 to be a uint16!"),
            DefensePhys: row
                .columns[57]
                .into_u16()
                .copied()
                .expect("Expected column 57 to be a uint16!"),
            DefenseMag: row
                .columns[58]
                .into_u16()
                .copied()
                .expect("Expected column 58 to be a uint16!"),
            BaseParamValue: [
                row
                    .columns[60]
                    .into_i16()
                    .copied()
                    .expect("Expected column 60 to be a int16!"),
                row
                    .columns[62]
                    .into_i16()
                    .copied()
                    .expect("Expected column 62 to be a int16!"),
                row
                    .columns[64]
                    .into_i16()
                    .copied()
                    .expect("Expected column 64 to be a int16!"),
                row
                    .columns[66]
                    .into_i16()
                    .copied()
                    .expect("Expected column 66 to be a int16!"),
                row
                    .columns[68]
                    .into_i16()
                    .copied()
                    .expect("Expected column 68 to be a int16!"),
                row
                    .columns[70]
                    .into_i16()
                    .copied()
                    .expect("Expected column 70 to be a int16!"),
            ],
            BaseParamValueSpecial: [
                row
                    .columns[74]
                    .into_i16()
                    .copied()
                    .expect("Expected column 74 to be a int16!"),
                row
                    .columns[76]
                    .into_i16()
                    .copied()
                    .expect("Expected column 76 to be a int16!"),
                row
                    .columns[78]
                    .into_i16()
                    .copied()
                    .expect("Expected column 78 to be a int16!"),
                row
                    .columns[80]
                    .into_i16()
                    .copied()
                    .expect("Expected column 80 to be a int16!"),
                row
                    .columns[82]
                    .into_i16()
                    .copied()
                    .expect("Expected column 82 to be a int16!"),
                row
                    .columns[84]
                    .into_i16()
                    .copied()
                    .expect("Expected column 84 to be a int16!"),
            ],
            LevelEquip: row
                .columns[40]
                .into_u8()
                .copied()
                .expect("Expected column 40 to be a uint8!"),
            RequiredPvpRank: row
                .columns[41]
                .into_u8()
                .copied()
                .expect("Expected column 41 to be a uint8!"),
            EquipRestriction: row
                .columns[42]
                .into_u8()
                .copied()
                .expect("Expected column 42 to be a uint8!"),
            ClassJobCategory: row
                .columns[43]
                .into_u8()
                .copied()
                .expect("Expected column 43 to be a uint8!"),
            GrandCompany: row
                .columns[44]
                .into_u8()
                .copied()
                .expect("Expected column 44 to be a uint8!"),
            ItemSeries: row
                .columns[45]
                .into_u8()
                .copied()
                .expect("Expected column 45 to be a uint8!"),
            BaseParamModifier: row
                .columns[46]
                .into_u8()
                .copied()
                .expect("Expected column 46 to be a uint8!"),
            ClassJobUse: row
                .columns[49]
                .into_u8()
                .copied()
                .expect("Expected column 49 to be a uint8!"),
            Unknown2: row
                .columns[50]
                .into_u8()
                .copied()
                .expect("Expected column 50 to be a uint8!"),
            DefaultActionRange: row
                .columns[54]
                .into_u8()
                .copied()
                .expect("Expected column 54 to be a uint8!"),
            BaseParam: [
                row
                    .columns[59]
                    .into_u8()
                    .copied()
                    .expect("Expected column 59 to be a uint8!"),
                row
                    .columns[61]
                    .into_u8()
                    .copied()
                    .expect("Expected column 61 to be a uint8!"),
                row
                    .columns[63]
                    .into_u8()
                    .copied()
                    .expect("Expected column 63 to be a uint8!"),
                row
                    .columns[65]
                    .into_u8()
                    .copied()
                    .expect("Expected column 65 to be a uint8!"),
                row
                    .columns[67]
                    .into_u8()
                    .copied()
                    .expect("Expected column 67 to be a uint8!"),
                row
                    .columns[69]
                    .into_u8()
                    .copied()
                    .expect("Expected column 69 to be a uint8!"),
            ],
            ItemSpecialBonus: row
                .columns[71]
                .into_u8()
                .copied()
                .expect("Expected column 71 to be a uint8!"),
            ItemSpecialBonusParam: row
                .columns[72]
                .into_u8()
                .copied()
                .expect("Expected column 72 to be a uint8!"),
            BaseParamSpecial: [
                row
                    .columns[73]
                    .into_u8()
                    .copied()
                    .expect("Expected column 73 to be a uint8!"),
                row
                    .columns[75]
                    .into_u8()
                    .copied()
                    .expect("Expected column 75 to be a uint8!"),
                row
                    .columns[77]
                    .into_u8()
                    .copied()
                    .expect("Expected column 77 to be a uint8!"),
                row
                    .columns[79]
                    .into_u8()
                    .copied()
                    .expect("Expected column 79 to be a uint8!"),
                row
                    .columns[81]
                    .into_u8()
                    .copied()
                    .expect("Expected column 81 to be a uint8!"),
                row
                    .columns[83]
                    .into_u8()
                    .copied()
                    .expect("Expected column 83 to be a uint8!"),
            ],
            MaterializeType: row
                .columns[85]
                .into_u8()
                .copied()
                .expect("Expected column 85 to be a uint8!"),
            MateriaSlotCount: row
                .columns[86]
                .into_u8()
                .copied()
                .expect("Expected column 86 to be a uint8!"),
            SubStatCategory: row
                .columns[89]
                .into_u8()
                .copied()
                .expect("Expected column 89 to be a uint8!"),
            IsAdvancedMeldingPermitted: row
                .columns[87]
                .into_bool()
                .copied()
                .expect("Expected column 87 to be a bool!"),
            IsPvP: row
                .columns[88]
                .into_bool()
                .copied()
                .expect("Expected column 88 to be a bool!"),
            IsGlamorous: row
                .columns[90]
                .into_bool()
                .copied()
                .expect("Expected column 90 to be a bool!"),
            AdditionalData: row
                .columns[14]
                .into_u32()
                .copied()
                .expect("Expected column 14 to be a uint32!"),
            StackSize: row
                .columns[20]
                .into_u32()
                .copied()
                .expect("Expected column 20 to be a uint32!"),
            PriceMid: row
                .columns[25]
                .into_u32()
                .copied()
                .expect("Expected column 25 to be a uint32!"),
            PriceLow: row
                .columns[26]
                .into_u32()
                .copied()
                .expect("Expected column 26 to be a uint32!"),
            ItemRepair: row
                .columns[34]
                .into_i32()
                .copied()
                .expect("Expected column 34 to be a int32!"),
            ItemGlamour: row
                .columns[35]
                .into_i32()
                .copied()
                .expect("Expected column 35 to be a int32!"),
            Icon: row
                .columns[10]
                .into_u16()
                .copied()
                .expect("Expected column 10 to be a uint16!"),
            LevelItem: row
                .columns[11]
                .into_u16()
                .copied()
                .expect("Expected column 11 to be a uint16!"),
            SubcategorySort: row
                .columns[19]
                .into_u16()
                .copied()
                .expect("Expected column 19 to be a uint16!"),
            ItemAction: row
                .columns[30]
                .into_u16()
                .copied()
                .expect("Expected column 30 to be a uint16!"),
            Cooldowns: row
                .columns[32]
                .into_u16()
                .copied()
                .expect("Expected column 32 to be a uint16!"),
            Desynth: row
                .columns[36]
                .into_u16()
                .copied()
                .expect("Expected column 36 to be a uint16!"),
            AetherialReduce: row
                .columns[39]
                .into_u16()
                .copied()
                .expect("Expected column 39 to be a uint16!"),
            Rarity: row
                .columns[12]
                .into_u8()
                .copied()
                .expect("Expected column 12 to be a uint8!"),
            FilterGroup: row
                .columns[13]
                .into_u8()
                .copied()
                .expect("Expected column 13 to be a uint8!"),
            ItemUICategory: row
                .columns[15]
                .into_u8()
                .copied()
                .expect("Expected column 15 to be a uint8!"),
            ItemSearchCategory: row
                .columns[16]
                .into_u8()
                .copied()
                .expect("Expected column 16 to be a uint8!"),
            EquipSlotCategory: row
                .columns[17]
                .into_u8()
                .copied()
                .expect("Expected column 17 to be a uint8!"),
            ItemSortCategory: row
                .columns[18]
                .into_u8()
                .copied()
                .expect("Expected column 18 to be a uint8!"),
            DyeCount: row
                .columns[28]
                .into_u8()
                .copied()
                .expect("Expected column 28 to be a uint8!"),
            CastTimeSeconds: row
                .columns[31]
                .into_u8()
                .copied()
                .expect("Expected column 31 to be a uint8!"),
            ClassJobRepair: row
                .columns[33]
                .into_u8()
                .copied()
                .expect("Expected column 33 to be a uint8!"),
            IsUnique: row
                .columns[21]
                .into_bool()
                .copied()
                .expect("Expected column 21 to be a bool!"),
            IsUntradable: row
                .columns[22]
                .into_bool()
                .copied()
                .expect("Expected column 22 to be a bool!"),
            IsIndisposable: row
                .columns[23]
                .into_bool()
                .copied()
                .expect("Expected column 23 to be a bool!"),
            Lot: row
                .columns[24]
                .into_bool()
                .copied()
                .expect("Expected column 24 to be a bool!"),
            CanBeHq: row
                .columns[27]
                .into_bool()
                .copied()
                .expect("Expected column 27 to be a bool!"),
            IsCrestWorthy: row
                .columns[29]
                .into_bool()
                .copied()
                .expect("Expected column 29 to be a bool!"),
            IsCollectable: row
                .columns[37]
                .into_bool()
                .copied()
                .expect("Expected column 37 to be a bool!"),
            AlwaysCollectable: row
                .columns[38]
                .into_bool()
                .copied()
                .expect("Expected column 38 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a ItemSheet {
    type Item = (u32, Vec<(u16, ItemRow)>);
    type IntoIter = StructuredSheetIterator<'a, ItemSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ItemSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ItemRow {
    ///""
    pub Singular: String,
    ///""
    pub Plural: String,
    ///""
    pub Description: String,
    ///""
    pub Name: String,
    ///""
    pub Adjective: i8,
    ///""
    pub PossessivePronoun: i8,
    ///""
    pub StartsWithVowel: i8,
    ///""
    pub Unknown0: i8,
    ///""
    pub Pronoun: i8,
    ///""
    pub Article: i8,
    ///""
    pub ModelMain: u64,
    ///""
    pub ModelSub: u64,
    ///""
    pub DamagePhys: u16,
    ///""
    pub DamageMag: u16,
    ///""
    pub Delayms: u16,
    ///""
    pub BlockRate: u16,
    ///""
    pub Block: u16,
    ///""
    pub DefensePhys: u16,
    ///""
    pub DefenseMag: u16,
    ///""
    pub BaseParamValue: [i16; 6],
    ///""
    pub BaseParamValueSpecial: [i16; 6],
    ///""
    pub LevelEquip: u8,
    ///""
    pub RequiredPvpRank: u8,
    ///""
    pub EquipRestriction: u8,
    ///""
    pub ClassJobCategory: u8,
    ///""
    pub GrandCompany: u8,
    ///""
    pub ItemSeries: u8,
    ///""
    pub BaseParamModifier: u8,
    ///""
    pub ClassJobUse: u8,
    ///""
    pub Unknown2: u8,
    ///""
    pub DefaultActionRange: u8,
    ///""
    pub BaseParam: [u8; 6],
    ///""
    pub ItemSpecialBonus: u8,
    ///""
    pub ItemSpecialBonusParam: u8,
    ///""
    pub BaseParamSpecial: [u8; 6],
    ///""
    pub MaterializeType: u8,
    ///""
    pub MateriaSlotCount: u8,
    ///""
    pub SubStatCategory: u8,
    ///""
    pub IsAdvancedMeldingPermitted: bool,
    ///""
    pub IsPvP: bool,
    ///""
    pub IsGlamorous: bool,
    ///""
    pub AdditionalData: u32,
    ///""
    pub StackSize: u32,
    ///""
    pub PriceMid: u32,
    ///""
    pub PriceLow: u32,
    ///""
    pub ItemRepair: i32,
    ///""
    pub ItemGlamour: i32,
    ///""
    pub Icon: u16,
    ///""
    pub LevelItem: u16,
    ///"Used in conjunction with the ItemUICategory sortkeys to sort things like vendor lists"
    pub SubcategorySort: u16,
    ///""
    pub ItemAction: u16,
    ///""
    pub Cooldowns: u16,
    ///""
    pub Desynth: u16,
    ///""
    pub AetherialReduce: u16,
    ///""
    pub Rarity: u8,
    ///"1 = Physical Weapon\n /// 2 = Magical Weapon\n /// 3 = Shield\n /// 4 = Gear\n /// 5 = Meal\n /// 6 = Medicine\n /// 7 = Deep Dungeon Usable (Manuals, Medicine, Potions)\n /// 8 = Potion (HP)\n /// 9 = Ether (MP)\n /// 10 = Elixir (HP+MP)\n /// 11 = Crystal\n /// 12 = Crafting Material\n /// 13 = Materia\n /// 14 = Housing\n /// 15 = Dyes\n /// 16 = Misc (Various stuff)\n /// 17 = Fishing Bait\n /// 18 = Treasure Map\n /// 19 = Useables (Various stuff)\n /// 20 = Gardening Seed\n /// 21 = Gardening Soil\n /// 22 = Gardening Fertilizer\n /// 23 = Secret Recipe Book\n /// 24 = unused\n /// 25 = Aetherial Wheel\n /// 26 = Primed Aetherial Wheel\n /// 27 = Triple Triad Card\n /// 28 = Airship Component\n /// 29 = Currency\n /// 30 = Folklore Book\n /// 31 = Soul Crystal\n /// 32 = Orchestrion Roll\n /// 33 = Aquarium Tank Trimming\n /// 34 = Painting\n /// 35 = Tales Of Adventure Retainer\n /// 36 = Submersible Component\n /// 37 = Eureka Logos Action Ingredient\n /// 38 = Bozja Mettle\n /// 39 = Bozja Lost Action\n /// 40 = Bozjan Cluster\n /// 41 = unused\n /// 42 = unused\n /// 43 = Placeholder Item\n /// 44 = Belts\n /// 45 = ArchiveItem (RowId in AdditionalData)\n /// 46 = unused\n /// 47 = Sanctuary Cowrie\n /// 48 = Sanctuary Material\n /// 49 = Adventurers Parcel\n /// 50 = Cosmic Exploration Material\n /// 51 = Outfit\n /// 52 = Occult Crescent Knowledge\n /// 53 = Occult Crescent Phantom Experience\n /// 54 = Occult Crescent Enlightenment Piece\n /// 55 = Cosmic Exploration Cosmocredit\n /// 56 = Cosmic Exploration Lunar Credit\n /// 57 = Occult Crescent Sanguine Cipher\n /// 58 = Cosmic Exploration Dronebits\n /// 59 = Cosmic Exploration Tool Mastery Points\n /// 60 = Old Dyes\n /// "
    pub FilterGroup: u8,
    ///""
    pub ItemUICategory: u8,
    ///""
    pub ItemSearchCategory: u8,
    ///""
    pub EquipSlotCategory: u8,
    ///""
    pub ItemSortCategory: u8,
    ///""
    pub DyeCount: u8,
    ///""
    pub CastTimeSeconds: u8,
    ///"This applies to both repairs and desynthesis, so irreparable but desynthesizable items such as fish or furniture will have a ClassJobRepair value."
    pub ClassJobRepair: u8,
    ///""
    pub IsUnique: bool,
    ///""
    pub IsUntradable: bool,
    ///""
    pub IsIndisposable: bool,
    ///""
    pub Lot: bool,
    ///""
    pub CanBeHq: bool,
    ///""
    pub IsCrestWorthy: bool,
    ///""
    pub IsCollectable: bool,
    ///""
    pub AlwaysCollectable: bool,
}
