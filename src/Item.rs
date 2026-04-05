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
impl<'a> StructuredSheet<'a> for ItemSheet {
    type Row = ItemRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a ItemSheet {
    type Item = (u32, Vec<(u16, ItemRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, ItemSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ItemSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ItemRow<'a> {
    row: &'a Row,
}
impl<'a> ItemRow<'a> {
    pub fn Singular(&'a self) -> &'a str {
        self.row.columns[0].into_string().unwrap()
    }
    pub fn Plural(&'a self) -> &'a str {
        self.row.columns[2].into_string().unwrap()
    }
    pub fn Description(&'a self) -> &'a str {
        self.row.columns[8].into_string().unwrap()
    }
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[9].into_string().unwrap()
    }
    pub fn Adjective(&'a self) -> i8 {
        self.row.columns[1].into_i8().copied().unwrap()
    }
    pub fn PossessivePronoun(&'a self) -> i8 {
        self.row.columns[3].into_i8().copied().unwrap()
    }
    pub fn StartsWithVowel(&'a self) -> i8 {
        self.row.columns[4].into_i8().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> i8 {
        self.row.columns[5].into_i8().copied().unwrap()
    }
    pub fn Pronoun(&'a self) -> i8 {
        self.row.columns[6].into_i8().copied().unwrap()
    }
    pub fn Article(&'a self) -> i8 {
        self.row.columns[7].into_i8().copied().unwrap()
    }
    pub fn ModelMain(&'a self) -> u64 {
        self.row.columns[47].into_u64().copied().unwrap()
    }
    pub fn ModelSub(&'a self) -> u64 {
        self.row.columns[48].into_u64().copied().unwrap()
    }
    pub fn DamagePhys(&'a self) -> u16 {
        self.row.columns[51].into_u16().copied().unwrap()
    }
    pub fn DamageMag(&'a self) -> u16 {
        self.row.columns[52].into_u16().copied().unwrap()
    }
    pub fn Delayms(&'a self) -> u16 {
        self.row.columns[53].into_u16().copied().unwrap()
    }
    pub fn BlockRate(&'a self) -> u16 {
        self.row.columns[55].into_u16().copied().unwrap()
    }
    pub fn Block(&'a self) -> u16 {
        self.row.columns[56].into_u16().copied().unwrap()
    }
    pub fn DefensePhys(&'a self) -> u16 {
        self.row.columns[57].into_u16().copied().unwrap()
    }
    pub fn DefenseMag(&'a self) -> u16 {
        self.row.columns[58].into_u16().copied().unwrap()
    }
    pub fn BaseParamValue(&'a self) -> [i16; 6] {
        [
            self.row.columns[60].into_i16().copied().unwrap(),
            self.row.columns[62].into_i16().copied().unwrap(),
            self.row.columns[64].into_i16().copied().unwrap(),
            self.row.columns[66].into_i16().copied().unwrap(),
            self.row.columns[68].into_i16().copied().unwrap(),
            self.row.columns[70].into_i16().copied().unwrap(),
        ]
    }
    pub fn BaseParamValueSpecial(&'a self) -> [i16; 6] {
        [
            self.row.columns[74].into_i16().copied().unwrap(),
            self.row.columns[76].into_i16().copied().unwrap(),
            self.row.columns[78].into_i16().copied().unwrap(),
            self.row.columns[80].into_i16().copied().unwrap(),
            self.row.columns[82].into_i16().copied().unwrap(),
            self.row.columns[84].into_i16().copied().unwrap(),
        ]
    }
    pub fn LevelEquip(&'a self) -> u8 {
        self.row.columns[40].into_u8().copied().unwrap()
    }
    pub fn RequiredPvpRank(&'a self) -> u8 {
        self.row.columns[41].into_u8().copied().unwrap()
    }
    pub fn EquipRestriction(&'a self) -> u8 {
        self.row.columns[42].into_u8().copied().unwrap()
    }
    pub fn ClassJobCategory(&'a self) -> u8 {
        self.row.columns[43].into_u8().copied().unwrap()
    }
    pub fn GrandCompany(&'a self) -> u8 {
        self.row.columns[44].into_u8().copied().unwrap()
    }
    pub fn ItemSeries(&'a self) -> u8 {
        self.row.columns[45].into_u8().copied().unwrap()
    }
    pub fn BaseParamModifier(&'a self) -> u8 {
        self.row.columns[46].into_u8().copied().unwrap()
    }
    pub fn ClassJobUse(&'a self) -> u8 {
        self.row.columns[49].into_u8().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> u8 {
        self.row.columns[50].into_u8().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> u8 {
        self.row.columns[54].into_u8().copied().unwrap()
    }
    pub fn BaseParam(&'a self) -> [u8; 6] {
        [
            self.row.columns[59].into_u8().copied().unwrap(),
            self.row.columns[61].into_u8().copied().unwrap(),
            self.row.columns[63].into_u8().copied().unwrap(),
            self.row.columns[65].into_u8().copied().unwrap(),
            self.row.columns[67].into_u8().copied().unwrap(),
            self.row.columns[69].into_u8().copied().unwrap(),
        ]
    }
    pub fn ItemSpecialBonus(&'a self) -> u8 {
        self.row.columns[71].into_u8().copied().unwrap()
    }
    pub fn ItemSpecialBonusParam(&'a self) -> u8 {
        self.row.columns[72].into_u8().copied().unwrap()
    }
    pub fn BaseParamSpecial(&'a self) -> [u8; 6] {
        [
            self.row.columns[73].into_u8().copied().unwrap(),
            self.row.columns[75].into_u8().copied().unwrap(),
            self.row.columns[77].into_u8().copied().unwrap(),
            self.row.columns[79].into_u8().copied().unwrap(),
            self.row.columns[81].into_u8().copied().unwrap(),
            self.row.columns[83].into_u8().copied().unwrap(),
        ]
    }
    pub fn MaterializeType(&'a self) -> u8 {
        self.row.columns[85].into_u8().copied().unwrap()
    }
    pub fn MateriaSlotCount(&'a self) -> u8 {
        self.row.columns[86].into_u8().copied().unwrap()
    }
    pub fn SubStatCategory(&'a self) -> u8 {
        self.row.columns[89].into_u8().copied().unwrap()
    }
    pub fn IsAdvancedMeldingPermitted(&'a self) -> bool {
        self.row.columns[87].into_bool().copied().unwrap()
    }
    pub fn IsPvP(&'a self) -> bool {
        self.row.columns[88].into_bool().copied().unwrap()
    }
    pub fn IsGlamorous(&'a self) -> bool {
        self.row.columns[90].into_bool().copied().unwrap()
    }
    pub fn AdditionalData(&'a self) -> u32 {
        self.row.columns[14].into_u32().copied().unwrap()
    }
    pub fn StackSize(&'a self) -> u32 {
        self.row.columns[20].into_u32().copied().unwrap()
    }
    pub fn PriceMid(&'a self) -> u32 {
        self.row.columns[25].into_u32().copied().unwrap()
    }
    pub fn PriceLow(&'a self) -> u32 {
        self.row.columns[26].into_u32().copied().unwrap()
    }
    pub fn ItemRepair(&'a self) -> i32 {
        self.row.columns[34].into_i32().copied().unwrap()
    }
    pub fn ItemGlamour(&'a self) -> i32 {
        self.row.columns[35].into_i32().copied().unwrap()
    }
    pub fn Icon(&'a self) -> u16 {
        self.row.columns[10].into_u16().copied().unwrap()
    }
    pub fn LevelItem(&'a self) -> u16 {
        self.row.columns[11].into_u16().copied().unwrap()
    }
    pub fn Unknown4(&'a self) -> u16 {
        self.row.columns[19].into_u16().copied().unwrap()
    }
    pub fn ItemAction(&'a self) -> u16 {
        self.row.columns[30].into_u16().copied().unwrap()
    }
    pub fn Cooldowns(&'a self) -> u16 {
        self.row.columns[32].into_u16().copied().unwrap()
    }
    pub fn Desynth(&'a self) -> u16 {
        self.row.columns[36].into_u16().copied().unwrap()
    }
    pub fn AetherialReduce(&'a self) -> u16 {
        self.row.columns[39].into_u16().copied().unwrap()
    }
    pub fn Rarity(&'a self) -> u8 {
        self.row.columns[12].into_u8().copied().unwrap()
    }
    /// 1 = Physical Weapon
    /// 2 = Magical Weapon
    /// 3 = Shield
    /// 4 = Gear
    /// 5 = Meal
    /// 6 = Medicine
    /// 7 = Deep Dungeon Usable (Manuals, Medicine, Potions)
    /// 8 = Potion (HP)
    /// 9 = Ether (MP)
    /// 10 = Elixir (HP+MP)
    /// 11 = Crystal
    /// 12 = Crafting Material
    /// 13 = Materia
    /// 14 = Housing
    /// 15 = Stain
    /// 16 = Misc (Various stuff)
    /// 17 = Fishing Bait
    /// 18 = Treasure Map
    /// 19 = Useables (Various stuff)
    /// 20 = Gardening Seed
    /// 21 = Gardening Soil
    /// 22 = Gardening Fertilizer
    /// 23 = Secret Recipe Book
    /// 24 = unused
    /// 25 = Aetherial Wheel
    /// 26 = Primed Aetherial Wheel
    /// 27 = Triple Triad Card
    /// 28 = Airship Component
    /// 29 = Currency
    /// 30 = Folklore Book
    /// 31 = Soul Crystal
    /// 32 = Orchestrion Roll
    /// 33 = Aquarium Tank Trimming
    /// 34 = Painting
    /// 35 = Tales Of Adventure Retainer
    /// 36 = Submersible Component
    /// 37 = Eureka Logos Action Ingredient
    /// 38 = Bozja Mettle
    /// 39 = Bozja Lost Action
    /// 40 = Bozjan Cluster
    /// 41 = unused
    /// 42 = unused
    /// 43 = Placeholder Item
    /// 44 = Belts
    /// 45 = ArchiveItem (RowId in AdditionalData)
    /// 46 = unused
    /// 47 = Sanctuary Cowrie
    /// 48 = Sanctuary Material
    /// 49 = Adventurers Parcel
    /// 50 = Cosmic Exploration Material
    /// 51 = Outfit
    /// 52 = Occult Crescent Knowledge
    /// 53 = Occult Crescent Phantom Experience
    /// 54 = Occult Crescent Enlightenment Piece
    /// 55 = Cosmic Exploration Cosmocredit
    /// 56 = Cosmic Exploration Lunar Credit
    /// 57 = Occult Crescent Sanguine Cipher
    ///
    pub fn FilterGroup(&'a self) -> u8 {
        self.row.columns[13].into_u8().copied().unwrap()
    }
    pub fn ItemUICategory(&'a self) -> u8 {
        self.row.columns[15].into_u8().copied().unwrap()
    }
    pub fn ItemSearchCategory(&'a self) -> u8 {
        self.row.columns[16].into_u8().copied().unwrap()
    }
    pub fn EquipSlotCategory(&'a self) -> u8 {
        self.row.columns[17].into_u8().copied().unwrap()
    }
    pub fn ItemSortCategory(&'a self) -> u8 {
        self.row.columns[18].into_u8().copied().unwrap()
    }
    pub fn DyeCount(&'a self) -> u8 {
        self.row.columns[28].into_u8().copied().unwrap()
    }
    pub fn CastTimeSeconds(&'a self) -> u8 {
        self.row.columns[31].into_u8().copied().unwrap()
    }
    pub fn ClassJobRepair(&'a self) -> u8 {
        self.row.columns[33].into_u8().copied().unwrap()
    }
    pub fn IsUnique(&'a self) -> bool {
        self.row.columns[21].into_bool().copied().unwrap()
    }
    pub fn IsUntradable(&'a self) -> bool {
        self.row.columns[22].into_bool().copied().unwrap()
    }
    pub fn IsIndisposable(&'a self) -> bool {
        self.row.columns[23].into_bool().copied().unwrap()
    }
    pub fn Lot(&'a self) -> bool {
        self.row.columns[24].into_bool().copied().unwrap()
    }
    pub fn CanBeHq(&'a self) -> bool {
        self.row.columns[27].into_bool().copied().unwrap()
    }
    pub fn IsCrestWorthy(&'a self) -> bool {
        self.row.columns[29].into_bool().copied().unwrap()
    }
    pub fn IsCollectable(&'a self) -> bool {
        self.row.columns[37].into_bool().copied().unwrap()
    }
    pub fn AlwaysCollectable(&'a self) -> bool {
        self.row.columns[38].into_bool().copied().unwrap()
    }
}
