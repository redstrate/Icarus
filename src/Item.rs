//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    common::Language,
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
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<Field>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(Self::Row { columns })
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
#[derive(Debug, Clone)]
pub struct ItemRow {
    columns: Vec<Field>,
}
impl ItemRow {
    pub fn Singular<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn Plural<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn Description<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn Name<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn Adjective<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn PossessivePronoun<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
    pub fn StartsWithVowel<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn Pronoun<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn Article<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn ModelMain<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
    pub fn ModelSub<'a>(&'a self) -> &'a Field {
        &self.columns[11]
    }
    pub fn DamagePhys<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    pub fn DamageMag<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
    pub fn Delayms<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    pub fn BlockRate<'a>(&'a self) -> &'a Field {
        &self.columns[15]
    }
    pub fn Block<'a>(&'a self) -> &'a Field {
        &self.columns[16]
    }
    pub fn DefensePhys<'a>(&'a self) -> &'a Field {
        &self.columns[17]
    }
    pub fn DefenseMag<'a>(&'a self) -> &'a Field {
        &self.columns[18]
    }
    pub fn BaseParamValue<'a>(&'a self) -> [&'a Field; 6] {
        [
            &self.columns[19],
            &self.columns[20],
            &self.columns[21],
            &self.columns[22],
            &self.columns[23],
            &self.columns[24],
        ]
    }
    pub fn BaseParamValueSpecial<'a>(&'a self) -> [&'a Field; 6] {
        [
            &self.columns[25],
            &self.columns[26],
            &self.columns[27],
            &self.columns[28],
            &self.columns[29],
            &self.columns[30],
        ]
    }
    pub fn LevelEquip<'a>(&'a self) -> &'a Field {
        &self.columns[31]
    }
    pub fn RequiredPvpRank<'a>(&'a self) -> &'a Field {
        &self.columns[32]
    }
    pub fn EquipRestriction<'a>(&'a self) -> &'a Field {
        &self.columns[33]
    }
    pub fn ClassJobCategory<'a>(&'a self) -> &'a Field {
        &self.columns[34]
    }
    pub fn GrandCompany<'a>(&'a self) -> &'a Field {
        &self.columns[35]
    }
    pub fn ItemSeries<'a>(&'a self) -> &'a Field {
        &self.columns[36]
    }
    pub fn BaseParamModifier<'a>(&'a self) -> &'a Field {
        &self.columns[37]
    }
    pub fn ClassJobUse<'a>(&'a self) -> &'a Field {
        &self.columns[38]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a Field {
        &self.columns[39]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a Field {
        &self.columns[40]
    }
    pub fn BaseParam<'a>(&'a self) -> [&'a Field; 6] {
        [
            &self.columns[41],
            &self.columns[42],
            &self.columns[43],
            &self.columns[44],
            &self.columns[45],
            &self.columns[46],
        ]
    }
    pub fn ItemSpecialBonus<'a>(&'a self) -> &'a Field {
        &self.columns[47]
    }
    pub fn ItemSpecialBonusParam<'a>(&'a self) -> &'a Field {
        &self.columns[48]
    }
    pub fn BaseParamSpecial<'a>(&'a self) -> [&'a Field; 6] {
        [
            &self.columns[49],
            &self.columns[50],
            &self.columns[51],
            &self.columns[52],
            &self.columns[53],
            &self.columns[54],
        ]
    }
    pub fn MaterializeType<'a>(&'a self) -> &'a Field {
        &self.columns[55]
    }
    pub fn MateriaSlotCount<'a>(&'a self) -> &'a Field {
        &self.columns[56]
    }
    pub fn SubStatCategory<'a>(&'a self) -> &'a Field {
        &self.columns[57]
    }
    pub fn IsAdvancedMeldingPermitted<'a>(&'a self) -> &'a Field {
        &self.columns[58]
    }
    pub fn IsPvP<'a>(&'a self) -> &'a Field {
        &self.columns[59]
    }
    pub fn IsGlamorous<'a>(&'a self) -> &'a Field {
        &self.columns[60]
    }
    pub fn AdditionalData<'a>(&'a self) -> &'a Field {
        &self.columns[61]
    }
    pub fn StackSize<'a>(&'a self) -> &'a Field {
        &self.columns[62]
    }
    pub fn PriceMid<'a>(&'a self) -> &'a Field {
        &self.columns[63]
    }
    pub fn PriceLow<'a>(&'a self) -> &'a Field {
        &self.columns[64]
    }
    pub fn ItemRepair<'a>(&'a self) -> &'a Field {
        &self.columns[65]
    }
    pub fn ItemGlamour<'a>(&'a self) -> &'a Field {
        &self.columns[66]
    }
    pub fn Icon<'a>(&'a self) -> &'a Field {
        &self.columns[67]
    }
    pub fn LevelItem<'a>(&'a self) -> &'a Field {
        &self.columns[68]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a Field {
        &self.columns[69]
    }
    pub fn ItemAction<'a>(&'a self) -> &'a Field {
        &self.columns[70]
    }
    pub fn Cooldowns<'a>(&'a self) -> &'a Field {
        &self.columns[71]
    }
    pub fn Desynth<'a>(&'a self) -> &'a Field {
        &self.columns[72]
    }
    pub fn AetherialReduce<'a>(&'a self) -> &'a Field {
        &self.columns[73]
    }
    pub fn Rarity<'a>(&'a self) -> &'a Field {
        &self.columns[74]
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
    pub fn FilterGroup<'a>(&'a self) -> &'a Field {
        &self.columns[75]
    }
    pub fn ItemUICategory<'a>(&'a self) -> &'a Field {
        &self.columns[76]
    }
    pub fn ItemSearchCategory<'a>(&'a self) -> &'a Field {
        &self.columns[77]
    }
    pub fn EquipSlotCategory<'a>(&'a self) -> &'a Field {
        &self.columns[78]
    }
    pub fn ItemSortCategory<'a>(&'a self) -> &'a Field {
        &self.columns[79]
    }
    pub fn DyeCount<'a>(&'a self) -> &'a Field {
        &self.columns[80]
    }
    pub fn CastTimeSeconds<'a>(&'a self) -> &'a Field {
        &self.columns[81]
    }
    pub fn ClassJobRepair<'a>(&'a self) -> &'a Field {
        &self.columns[82]
    }
    pub fn IsUnique<'a>(&'a self) -> &'a Field {
        &self.columns[83]
    }
    pub fn IsUntradable<'a>(&'a self) -> &'a Field {
        &self.columns[84]
    }
    pub fn IsIndisposable<'a>(&'a self) -> &'a Field {
        &self.columns[85]
    }
    pub fn Lot<'a>(&'a self) -> &'a Field {
        &self.columns[86]
    }
    pub fn CanBeHq<'a>(&'a self) -> &'a Field {
        &self.columns[87]
    }
    pub fn IsCrestWorthy<'a>(&'a self) -> &'a Field {
        &self.columns[88]
    }
    pub fn IsCollectable<'a>(&'a self) -> &'a Field {
        &self.columns[89]
    }
    pub fn AlwaysCollectable<'a>(&'a self) -> &'a Field {
        &self.columns[90]
    }
}
