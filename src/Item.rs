//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct ItemSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl ItemSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "Item")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages.push(read_excel_sheet(resource, "Item", &exh, language, i)?);
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<ItemRow> {
        let column_defs = &self.exh.column_definitions;
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
        Some(ItemRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<ItemRow> {
        for page in &self.pages {
            let Some(row) = &page.get_row(row_id) else {
                continue;
            };
            let row = match row {
                ExcelRowKind::SingleRow(row) => row,
                ExcelRowKind::SubRows(rows) => &rows.first()?.1,
            };
            return self.read_row(row);
        }
        None
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<ItemRow> {
        for page in &self.pages {
            let Some(row) = &page.get_row(row_id) else {
                continue;
            };
            let row = match row {
                ExcelRowKind::SingleRow(row) => return None,
                ExcelRowKind::SubRows(subrows) => {
                    &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1
                }
            };
            return self.read_row(row);
        }
        None
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.row_count
    }
}
pub struct ItemRow {
    columns: Vec<ColumnData>,
}
impl ItemRow {
    pub fn Singular<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn Plural<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn Description<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn Name<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn Adjective<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn PossessivePronoun<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn StartsWithVowel<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn Pronoun<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn Article<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn ModelMain<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn ModelSub<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn DamagePhys<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn DamageMag<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn Delayms<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn BlockRate<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn Block<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn DefensePhys<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn DefenseMag<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn BaseParamValue<'a>(&'a self) -> [&'a ColumnData; 6] {
        [
            &self.columns[19],
            &self.columns[20],
            &self.columns[21],
            &self.columns[22],
            &self.columns[23],
            &self.columns[24],
        ]
    }
    pub fn BaseParamValueSpecial<'a>(&'a self) -> [&'a ColumnData; 6] {
        [
            &self.columns[25],
            &self.columns[26],
            &self.columns[27],
            &self.columns[28],
            &self.columns[29],
            &self.columns[30],
        ]
    }
    pub fn LevelEquip<'a>(&'a self) -> &'a ColumnData {
        &self.columns[31]
    }
    pub fn RequiredPvpRank<'a>(&'a self) -> &'a ColumnData {
        &self.columns[32]
    }
    pub fn EquipRestriction<'a>(&'a self) -> &'a ColumnData {
        &self.columns[33]
    }
    pub fn ClassJobCategory<'a>(&'a self) -> &'a ColumnData {
        &self.columns[34]
    }
    pub fn GrandCompany<'a>(&'a self) -> &'a ColumnData {
        &self.columns[35]
    }
    pub fn ItemSeries<'a>(&'a self) -> &'a ColumnData {
        &self.columns[36]
    }
    pub fn BaseParamModifier<'a>(&'a self) -> &'a ColumnData {
        &self.columns[37]
    }
    pub fn ClassJobUse<'a>(&'a self) -> &'a ColumnData {
        &self.columns[38]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[39]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[40]
    }
    pub fn BaseParam<'a>(&'a self) -> [&'a ColumnData; 6] {
        [
            &self.columns[41],
            &self.columns[42],
            &self.columns[43],
            &self.columns[44],
            &self.columns[45],
            &self.columns[46],
        ]
    }
    pub fn ItemSpecialBonus<'a>(&'a self) -> &'a ColumnData {
        &self.columns[47]
    }
    pub fn ItemSpecialBonusParam<'a>(&'a self) -> &'a ColumnData {
        &self.columns[48]
    }
    pub fn BaseParamSpecial<'a>(&'a self) -> [&'a ColumnData; 6] {
        [
            &self.columns[49],
            &self.columns[50],
            &self.columns[51],
            &self.columns[52],
            &self.columns[53],
            &self.columns[54],
        ]
    }
    pub fn MaterializeType<'a>(&'a self) -> &'a ColumnData {
        &self.columns[55]
    }
    pub fn MateriaSlotCount<'a>(&'a self) -> &'a ColumnData {
        &self.columns[56]
    }
    pub fn SubStatCategory<'a>(&'a self) -> &'a ColumnData {
        &self.columns[57]
    }
    pub fn IsAdvancedMeldingPermitted<'a>(&'a self) -> &'a ColumnData {
        &self.columns[58]
    }
    pub fn IsPvP<'a>(&'a self) -> &'a ColumnData {
        &self.columns[59]
    }
    pub fn IsGlamorous<'a>(&'a self) -> &'a ColumnData {
        &self.columns[60]
    }
    pub fn AdditionalData<'a>(&'a self) -> &'a ColumnData {
        &self.columns[61]
    }
    pub fn StackSize<'a>(&'a self) -> &'a ColumnData {
        &self.columns[62]
    }
    pub fn PriceMid<'a>(&'a self) -> &'a ColumnData {
        &self.columns[63]
    }
    pub fn PriceLow<'a>(&'a self) -> &'a ColumnData {
        &self.columns[64]
    }
    pub fn ItemRepair<'a>(&'a self) -> &'a ColumnData {
        &self.columns[65]
    }
    pub fn ItemGlamour<'a>(&'a self) -> &'a ColumnData {
        &self.columns[66]
    }
    pub fn Icon<'a>(&'a self) -> &'a ColumnData {
        &self.columns[67]
    }
    pub fn LevelItem<'a>(&'a self) -> &'a ColumnData {
        &self.columns[68]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a ColumnData {
        &self.columns[69]
    }
    pub fn ItemAction<'a>(&'a self) -> &'a ColumnData {
        &self.columns[70]
    }
    pub fn Cooldowns<'a>(&'a self) -> &'a ColumnData {
        &self.columns[71]
    }
    pub fn Desynth<'a>(&'a self) -> &'a ColumnData {
        &self.columns[72]
    }
    pub fn AetherialReduce<'a>(&'a self) -> &'a ColumnData {
        &self.columns[73]
    }
    pub fn Rarity<'a>(&'a self) -> &'a ColumnData {
        &self.columns[74]
    }
    pub fn FilterGroup<'a>(&'a self) -> &'a ColumnData {
        &self.columns[75]
    }
    pub fn ItemUICategory<'a>(&'a self) -> &'a ColumnData {
        &self.columns[76]
    }
    pub fn ItemSearchCategory<'a>(&'a self) -> &'a ColumnData {
        &self.columns[77]
    }
    pub fn EquipSlotCategory<'a>(&'a self) -> &'a ColumnData {
        &self.columns[78]
    }
    pub fn ItemSortCategory<'a>(&'a self) -> &'a ColumnData {
        &self.columns[79]
    }
    pub fn DyeCount<'a>(&'a self) -> &'a ColumnData {
        &self.columns[80]
    }
    pub fn CastTimeSeconds<'a>(&'a self) -> &'a ColumnData {
        &self.columns[81]
    }
    pub fn ClassJobRepair<'a>(&'a self) -> &'a ColumnData {
        &self.columns[82]
    }
    pub fn IsUnique<'a>(&'a self) -> &'a ColumnData {
        &self.columns[83]
    }
    pub fn IsUntradable<'a>(&'a self) -> &'a ColumnData {
        &self.columns[84]
    }
    pub fn IsIndisposable<'a>(&'a self) -> &'a ColumnData {
        &self.columns[85]
    }
    pub fn Lot<'a>(&'a self) -> &'a ColumnData {
        &self.columns[86]
    }
    pub fn CanBeHq<'a>(&'a self) -> &'a ColumnData {
        &self.columns[87]
    }
    pub fn IsCrestWorthy<'a>(&'a self) -> &'a ColumnData {
        &self.columns[88]
    }
    pub fn IsCollectable<'a>(&'a self) -> &'a ColumnData {
        &self.columns[89]
    }
    pub fn AlwaysCollectable<'a>(&'a self) -> &'a ColumnData {
        &self.columns[90]
    }
}
