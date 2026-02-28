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
    index_mapping: Vec<usize>,
}
impl ItemSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Item")?;
        let sheet = resolver.read_excel_sheet(&exh, "Item", language)?;
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
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
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
    index_mapping: Vec<usize>,
}
impl<'a> ItemRow<'a> {
    pub fn Singular(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn Plural(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn Description(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn Name(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn Adjective(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn PossessivePronoun(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn StartsWithVowel(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn Pronoun(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn Article(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn ModelMain(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    pub fn ModelSub(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn DamagePhys(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn DamageMag(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn Delayms(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    pub fn BlockRate(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    pub fn Block(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[16]]
    }
    pub fn DefensePhys(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[17]]
    }
    pub fn DefenseMag(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[18]]
    }
    pub fn BaseParamValue(&'a self) -> [&'a Field; 6] {
        [
            &self.row.columns[self.index_mapping[19]],
            &self.row.columns[self.index_mapping[20]],
            &self.row.columns[self.index_mapping[21]],
            &self.row.columns[self.index_mapping[22]],
            &self.row.columns[self.index_mapping[23]],
            &self.row.columns[self.index_mapping[24]],
        ]
    }
    pub fn BaseParamValueSpecial(&'a self) -> [&'a Field; 6] {
        [
            &self.row.columns[self.index_mapping[25]],
            &self.row.columns[self.index_mapping[26]],
            &self.row.columns[self.index_mapping[27]],
            &self.row.columns[self.index_mapping[28]],
            &self.row.columns[self.index_mapping[29]],
            &self.row.columns[self.index_mapping[30]],
        ]
    }
    pub fn LevelEquip(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[31]]
    }
    pub fn RequiredPvpRank(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[32]]
    }
    pub fn EquipRestriction(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[33]]
    }
    pub fn ClassJobCategory(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[34]]
    }
    pub fn GrandCompany(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[35]]
    }
    pub fn ItemSeries(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[36]]
    }
    pub fn BaseParamModifier(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[37]]
    }
    pub fn ClassJobUse(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[38]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[39]]
    }
    pub fn Unknown3(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[40]]
    }
    pub fn BaseParam(&'a self) -> [&'a Field; 6] {
        [
            &self.row.columns[self.index_mapping[41]],
            &self.row.columns[self.index_mapping[42]],
            &self.row.columns[self.index_mapping[43]],
            &self.row.columns[self.index_mapping[44]],
            &self.row.columns[self.index_mapping[45]],
            &self.row.columns[self.index_mapping[46]],
        ]
    }
    pub fn ItemSpecialBonus(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[47]]
    }
    pub fn ItemSpecialBonusParam(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[48]]
    }
    pub fn BaseParamSpecial(&'a self) -> [&'a Field; 6] {
        [
            &self.row.columns[self.index_mapping[49]],
            &self.row.columns[self.index_mapping[50]],
            &self.row.columns[self.index_mapping[51]],
            &self.row.columns[self.index_mapping[52]],
            &self.row.columns[self.index_mapping[53]],
            &self.row.columns[self.index_mapping[54]],
        ]
    }
    pub fn MaterializeType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[55]]
    }
    pub fn MateriaSlotCount(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[56]]
    }
    pub fn SubStatCategory(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[57]]
    }
    pub fn IsAdvancedMeldingPermitted(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[58]]
    }
    pub fn IsPvP(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[59]]
    }
    pub fn IsGlamorous(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[60]]
    }
    pub fn AdditionalData(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[61]]
    }
    pub fn StackSize(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[62]]
    }
    pub fn PriceMid(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[63]]
    }
    pub fn PriceLow(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[64]]
    }
    pub fn ItemRepair(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[65]]
    }
    pub fn ItemGlamour(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[66]]
    }
    pub fn Icon(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[67]]
    }
    pub fn LevelItem(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[68]]
    }
    pub fn Unknown4(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[69]]
    }
    pub fn ItemAction(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[70]]
    }
    pub fn Cooldowns(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[71]]
    }
    pub fn Desynth(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[72]]
    }
    pub fn AetherialReduce(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[73]]
    }
    pub fn Rarity(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[74]]
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
    pub fn FilterGroup(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[75]]
    }
    pub fn ItemUICategory(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[76]]
    }
    pub fn ItemSearchCategory(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[77]]
    }
    pub fn EquipSlotCategory(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[78]]
    }
    pub fn ItemSortCategory(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[79]]
    }
    pub fn DyeCount(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[80]]
    }
    pub fn CastTimeSeconds(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[81]]
    }
    pub fn ClassJobRepair(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[82]]
    }
    pub fn IsUnique(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[83]]
    }
    pub fn IsUntradable(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[84]]
    }
    pub fn IsIndisposable(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[85]]
    }
    pub fn Lot(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[86]]
    }
    pub fn CanBeHq(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[87]]
    }
    pub fn IsCrestWorthy(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[88]]
    }
    pub fn IsCollectable(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[89]]
    }
    pub fn AlwaysCollectable(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[90]]
    }
}
