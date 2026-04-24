//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ItemElement {
    pub ReceiveCount: [u32; 2],
    pub CurrencyCost: [u32; 3],
    pub Item: [i32; 2],
    pub Category: [i32; 2],
    pub ItemCost: [i32; 3],
    pub Quest: i32,
    pub Unknown0: [i32; 4],
    pub AchievementUnlock: i32,
    pub Unknown2: i32,
    pub CollectabilityCost: [u16; 3],
    pub PatchNumber: u16,
    pub HqCost: [u8; 3],
    pub Unknown1: [u8; 5],
    pub Order: u8,
    pub ReceiveHq: [bool; 2],
}
#[derive(Debug, Clone)]
pub struct SpecialShopSheet {
    sheet: Sheet,
}
impl SpecialShopSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("SpecialShop")?;
        let sheet = resolver.read_excel_sheet(&exh, "SpecialShop", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<SpecialShopRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<SpecialShopRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for SpecialShopSheet {
    type Row = SpecialShopRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a SpecialShopSheet {
    type Item = (u32, Vec<(u16, SpecialShopRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, SpecialShopSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, SpecialShopSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SpecialShopRow<'a> {
    row: &'a Row,
}
impl<'a> SpecialShopRow<'a> {
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[0].into_string().unwrap()
    }
    pub fn Item(&'a self) -> [ItemElement; 60] {
        [
            ItemElement {
                ReceiveCount: [
                    self.row.columns[61].into_u32().copied().unwrap(),
                    self.row.columns[301].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[541].into_u32().copied().unwrap(),
                    self.row.columns[781].into_u32().copied().unwrap(),
                    self.row.columns[1021].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[1].into_i32().copied().unwrap(),
                    self.row.columns[241].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[121].into_i32().copied().unwrap(),
                    self.row.columns[361].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[481].into_i32().copied().unwrap(),
                    self.row.columns[721].into_i32().copied().unwrap(),
                    self.row.columns[961].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1201].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1261].into_i32().copied().unwrap(),
                    self.row.columns[1501].into_i32().copied().unwrap(),
                    self.row.columns[1561].into_i32().copied().unwrap(),
                    self.row.columns[1681].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1741].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1861].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[661].into_u16().copied().unwrap(),
                    self.row.columns[901].into_u16().copied().unwrap(),
                    self.row.columns[1141].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[1981].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[601].into_u8().copied().unwrap(),
                    self.row.columns[841].into_u8().copied().unwrap(),
                    self.row.columns[1081].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1321].into_u8().copied().unwrap(),
                    self.row.columns[1381].into_u8().copied().unwrap(),
                    self.row.columns[1441].into_u8().copied().unwrap(),
                    self.row.columns[1621].into_u8().copied().unwrap(),
                    self.row.columns[1801].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1921].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[181].into_bool().copied().unwrap(),
                    self.row.columns[421].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[62].into_u32().copied().unwrap(),
                    self.row.columns[302].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[542].into_u32().copied().unwrap(),
                    self.row.columns[782].into_u32().copied().unwrap(),
                    self.row.columns[1022].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[2].into_i32().copied().unwrap(),
                    self.row.columns[242].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[122].into_i32().copied().unwrap(),
                    self.row.columns[362].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[482].into_i32().copied().unwrap(),
                    self.row.columns[722].into_i32().copied().unwrap(),
                    self.row.columns[962].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1202].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1262].into_i32().copied().unwrap(),
                    self.row.columns[1502].into_i32().copied().unwrap(),
                    self.row.columns[1562].into_i32().copied().unwrap(),
                    self.row.columns[1682].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1742].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1862].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[662].into_u16().copied().unwrap(),
                    self.row.columns[902].into_u16().copied().unwrap(),
                    self.row.columns[1142].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[1982].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[602].into_u8().copied().unwrap(),
                    self.row.columns[842].into_u8().copied().unwrap(),
                    self.row.columns[1082].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1322].into_u8().copied().unwrap(),
                    self.row.columns[1382].into_u8().copied().unwrap(),
                    self.row.columns[1442].into_u8().copied().unwrap(),
                    self.row.columns[1622].into_u8().copied().unwrap(),
                    self.row.columns[1802].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1922].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[182].into_bool().copied().unwrap(),
                    self.row.columns[422].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[63].into_u32().copied().unwrap(),
                    self.row.columns[303].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[543].into_u32().copied().unwrap(),
                    self.row.columns[783].into_u32().copied().unwrap(),
                    self.row.columns[1023].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[3].into_i32().copied().unwrap(),
                    self.row.columns[243].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[123].into_i32().copied().unwrap(),
                    self.row.columns[363].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[483].into_i32().copied().unwrap(),
                    self.row.columns[723].into_i32().copied().unwrap(),
                    self.row.columns[963].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1203].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1263].into_i32().copied().unwrap(),
                    self.row.columns[1503].into_i32().copied().unwrap(),
                    self.row.columns[1563].into_i32().copied().unwrap(),
                    self.row.columns[1683].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1743].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1863].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[663].into_u16().copied().unwrap(),
                    self.row.columns[903].into_u16().copied().unwrap(),
                    self.row.columns[1143].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[1983].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[603].into_u8().copied().unwrap(),
                    self.row.columns[843].into_u8().copied().unwrap(),
                    self.row.columns[1083].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1323].into_u8().copied().unwrap(),
                    self.row.columns[1383].into_u8().copied().unwrap(),
                    self.row.columns[1443].into_u8().copied().unwrap(),
                    self.row.columns[1623].into_u8().copied().unwrap(),
                    self.row.columns[1803].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1923].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[183].into_bool().copied().unwrap(),
                    self.row.columns[423].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[64].into_u32().copied().unwrap(),
                    self.row.columns[304].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[544].into_u32().copied().unwrap(),
                    self.row.columns[784].into_u32().copied().unwrap(),
                    self.row.columns[1024].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[4].into_i32().copied().unwrap(),
                    self.row.columns[244].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[124].into_i32().copied().unwrap(),
                    self.row.columns[364].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[484].into_i32().copied().unwrap(),
                    self.row.columns[724].into_i32().copied().unwrap(),
                    self.row.columns[964].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1204].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1264].into_i32().copied().unwrap(),
                    self.row.columns[1504].into_i32().copied().unwrap(),
                    self.row.columns[1564].into_i32().copied().unwrap(),
                    self.row.columns[1684].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1744].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1864].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[664].into_u16().copied().unwrap(),
                    self.row.columns[904].into_u16().copied().unwrap(),
                    self.row.columns[1144].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[1984].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[604].into_u8().copied().unwrap(),
                    self.row.columns[844].into_u8().copied().unwrap(),
                    self.row.columns[1084].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1324].into_u8().copied().unwrap(),
                    self.row.columns[1384].into_u8().copied().unwrap(),
                    self.row.columns[1444].into_u8().copied().unwrap(),
                    self.row.columns[1624].into_u8().copied().unwrap(),
                    self.row.columns[1804].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1924].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[184].into_bool().copied().unwrap(),
                    self.row.columns[424].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[65].into_u32().copied().unwrap(),
                    self.row.columns[305].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[545].into_u32().copied().unwrap(),
                    self.row.columns[785].into_u32().copied().unwrap(),
                    self.row.columns[1025].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[5].into_i32().copied().unwrap(),
                    self.row.columns[245].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[125].into_i32().copied().unwrap(),
                    self.row.columns[365].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[485].into_i32().copied().unwrap(),
                    self.row.columns[725].into_i32().copied().unwrap(),
                    self.row.columns[965].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1205].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1265].into_i32().copied().unwrap(),
                    self.row.columns[1505].into_i32().copied().unwrap(),
                    self.row.columns[1565].into_i32().copied().unwrap(),
                    self.row.columns[1685].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1745].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1865].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[665].into_u16().copied().unwrap(),
                    self.row.columns[905].into_u16().copied().unwrap(),
                    self.row.columns[1145].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[1985].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[605].into_u8().copied().unwrap(),
                    self.row.columns[845].into_u8().copied().unwrap(),
                    self.row.columns[1085].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1325].into_u8().copied().unwrap(),
                    self.row.columns[1385].into_u8().copied().unwrap(),
                    self.row.columns[1445].into_u8().copied().unwrap(),
                    self.row.columns[1625].into_u8().copied().unwrap(),
                    self.row.columns[1805].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1925].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[185].into_bool().copied().unwrap(),
                    self.row.columns[425].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[66].into_u32().copied().unwrap(),
                    self.row.columns[306].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[546].into_u32().copied().unwrap(),
                    self.row.columns[786].into_u32().copied().unwrap(),
                    self.row.columns[1026].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[6].into_i32().copied().unwrap(),
                    self.row.columns[246].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[126].into_i32().copied().unwrap(),
                    self.row.columns[366].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[486].into_i32().copied().unwrap(),
                    self.row.columns[726].into_i32().copied().unwrap(),
                    self.row.columns[966].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1206].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1266].into_i32().copied().unwrap(),
                    self.row.columns[1506].into_i32().copied().unwrap(),
                    self.row.columns[1566].into_i32().copied().unwrap(),
                    self.row.columns[1686].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1746].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1866].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[666].into_u16().copied().unwrap(),
                    self.row.columns[906].into_u16().copied().unwrap(),
                    self.row.columns[1146].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[1986].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[606].into_u8().copied().unwrap(),
                    self.row.columns[846].into_u8().copied().unwrap(),
                    self.row.columns[1086].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1326].into_u8().copied().unwrap(),
                    self.row.columns[1386].into_u8().copied().unwrap(),
                    self.row.columns[1446].into_u8().copied().unwrap(),
                    self.row.columns[1626].into_u8().copied().unwrap(),
                    self.row.columns[1806].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1926].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[186].into_bool().copied().unwrap(),
                    self.row.columns[426].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[67].into_u32().copied().unwrap(),
                    self.row.columns[307].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[547].into_u32().copied().unwrap(),
                    self.row.columns[787].into_u32().copied().unwrap(),
                    self.row.columns[1027].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[7].into_i32().copied().unwrap(),
                    self.row.columns[247].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[127].into_i32().copied().unwrap(),
                    self.row.columns[367].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[487].into_i32().copied().unwrap(),
                    self.row.columns[727].into_i32().copied().unwrap(),
                    self.row.columns[967].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1207].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1267].into_i32().copied().unwrap(),
                    self.row.columns[1507].into_i32().copied().unwrap(),
                    self.row.columns[1567].into_i32().copied().unwrap(),
                    self.row.columns[1687].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1747].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1867].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[667].into_u16().copied().unwrap(),
                    self.row.columns[907].into_u16().copied().unwrap(),
                    self.row.columns[1147].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[1987].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[607].into_u8().copied().unwrap(),
                    self.row.columns[847].into_u8().copied().unwrap(),
                    self.row.columns[1087].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1327].into_u8().copied().unwrap(),
                    self.row.columns[1387].into_u8().copied().unwrap(),
                    self.row.columns[1447].into_u8().copied().unwrap(),
                    self.row.columns[1627].into_u8().copied().unwrap(),
                    self.row.columns[1807].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1927].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[187].into_bool().copied().unwrap(),
                    self.row.columns[427].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[68].into_u32().copied().unwrap(),
                    self.row.columns[308].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[548].into_u32().copied().unwrap(),
                    self.row.columns[788].into_u32().copied().unwrap(),
                    self.row.columns[1028].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[8].into_i32().copied().unwrap(),
                    self.row.columns[248].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[128].into_i32().copied().unwrap(),
                    self.row.columns[368].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[488].into_i32().copied().unwrap(),
                    self.row.columns[728].into_i32().copied().unwrap(),
                    self.row.columns[968].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1208].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1268].into_i32().copied().unwrap(),
                    self.row.columns[1508].into_i32().copied().unwrap(),
                    self.row.columns[1568].into_i32().copied().unwrap(),
                    self.row.columns[1688].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1748].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1868].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[668].into_u16().copied().unwrap(),
                    self.row.columns[908].into_u16().copied().unwrap(),
                    self.row.columns[1148].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[1988].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[608].into_u8().copied().unwrap(),
                    self.row.columns[848].into_u8().copied().unwrap(),
                    self.row.columns[1088].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1328].into_u8().copied().unwrap(),
                    self.row.columns[1388].into_u8().copied().unwrap(),
                    self.row.columns[1448].into_u8().copied().unwrap(),
                    self.row.columns[1628].into_u8().copied().unwrap(),
                    self.row.columns[1808].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1928].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[188].into_bool().copied().unwrap(),
                    self.row.columns[428].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[69].into_u32().copied().unwrap(),
                    self.row.columns[309].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[549].into_u32().copied().unwrap(),
                    self.row.columns[789].into_u32().copied().unwrap(),
                    self.row.columns[1029].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[9].into_i32().copied().unwrap(),
                    self.row.columns[249].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[129].into_i32().copied().unwrap(),
                    self.row.columns[369].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[489].into_i32().copied().unwrap(),
                    self.row.columns[729].into_i32().copied().unwrap(),
                    self.row.columns[969].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1209].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1269].into_i32().copied().unwrap(),
                    self.row.columns[1509].into_i32().copied().unwrap(),
                    self.row.columns[1569].into_i32().copied().unwrap(),
                    self.row.columns[1689].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1749].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1869].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[669].into_u16().copied().unwrap(),
                    self.row.columns[909].into_u16().copied().unwrap(),
                    self.row.columns[1149].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[1989].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[609].into_u8().copied().unwrap(),
                    self.row.columns[849].into_u8().copied().unwrap(),
                    self.row.columns[1089].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1329].into_u8().copied().unwrap(),
                    self.row.columns[1389].into_u8().copied().unwrap(),
                    self.row.columns[1449].into_u8().copied().unwrap(),
                    self.row.columns[1629].into_u8().copied().unwrap(),
                    self.row.columns[1809].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1929].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[189].into_bool().copied().unwrap(),
                    self.row.columns[429].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[70].into_u32().copied().unwrap(),
                    self.row.columns[310].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[550].into_u32().copied().unwrap(),
                    self.row.columns[790].into_u32().copied().unwrap(),
                    self.row.columns[1030].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[10].into_i32().copied().unwrap(),
                    self.row.columns[250].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[130].into_i32().copied().unwrap(),
                    self.row.columns[370].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[490].into_i32().copied().unwrap(),
                    self.row.columns[730].into_i32().copied().unwrap(),
                    self.row.columns[970].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1210].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1270].into_i32().copied().unwrap(),
                    self.row.columns[1510].into_i32().copied().unwrap(),
                    self.row.columns[1570].into_i32().copied().unwrap(),
                    self.row.columns[1690].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1750].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1870].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[670].into_u16().copied().unwrap(),
                    self.row.columns[910].into_u16().copied().unwrap(),
                    self.row.columns[1150].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[1990].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[610].into_u8().copied().unwrap(),
                    self.row.columns[850].into_u8().copied().unwrap(),
                    self.row.columns[1090].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1330].into_u8().copied().unwrap(),
                    self.row.columns[1390].into_u8().copied().unwrap(),
                    self.row.columns[1450].into_u8().copied().unwrap(),
                    self.row.columns[1630].into_u8().copied().unwrap(),
                    self.row.columns[1810].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1930].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[190].into_bool().copied().unwrap(),
                    self.row.columns[430].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[71].into_u32().copied().unwrap(),
                    self.row.columns[311].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[551].into_u32().copied().unwrap(),
                    self.row.columns[791].into_u32().copied().unwrap(),
                    self.row.columns[1031].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[11].into_i32().copied().unwrap(),
                    self.row.columns[251].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[131].into_i32().copied().unwrap(),
                    self.row.columns[371].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[491].into_i32().copied().unwrap(),
                    self.row.columns[731].into_i32().copied().unwrap(),
                    self.row.columns[971].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1211].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1271].into_i32().copied().unwrap(),
                    self.row.columns[1511].into_i32().copied().unwrap(),
                    self.row.columns[1571].into_i32().copied().unwrap(),
                    self.row.columns[1691].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1751].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1871].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[671].into_u16().copied().unwrap(),
                    self.row.columns[911].into_u16().copied().unwrap(),
                    self.row.columns[1151].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[1991].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[611].into_u8().copied().unwrap(),
                    self.row.columns[851].into_u8().copied().unwrap(),
                    self.row.columns[1091].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1331].into_u8().copied().unwrap(),
                    self.row.columns[1391].into_u8().copied().unwrap(),
                    self.row.columns[1451].into_u8().copied().unwrap(),
                    self.row.columns[1631].into_u8().copied().unwrap(),
                    self.row.columns[1811].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1931].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[191].into_bool().copied().unwrap(),
                    self.row.columns[431].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[72].into_u32().copied().unwrap(),
                    self.row.columns[312].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[552].into_u32().copied().unwrap(),
                    self.row.columns[792].into_u32().copied().unwrap(),
                    self.row.columns[1032].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[12].into_i32().copied().unwrap(),
                    self.row.columns[252].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[132].into_i32().copied().unwrap(),
                    self.row.columns[372].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[492].into_i32().copied().unwrap(),
                    self.row.columns[732].into_i32().copied().unwrap(),
                    self.row.columns[972].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1212].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1272].into_i32().copied().unwrap(),
                    self.row.columns[1512].into_i32().copied().unwrap(),
                    self.row.columns[1572].into_i32().copied().unwrap(),
                    self.row.columns[1692].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1752].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1872].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[672].into_u16().copied().unwrap(),
                    self.row.columns[912].into_u16().copied().unwrap(),
                    self.row.columns[1152].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[1992].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[612].into_u8().copied().unwrap(),
                    self.row.columns[852].into_u8().copied().unwrap(),
                    self.row.columns[1092].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1332].into_u8().copied().unwrap(),
                    self.row.columns[1392].into_u8().copied().unwrap(),
                    self.row.columns[1452].into_u8().copied().unwrap(),
                    self.row.columns[1632].into_u8().copied().unwrap(),
                    self.row.columns[1812].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1932].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[192].into_bool().copied().unwrap(),
                    self.row.columns[432].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[73].into_u32().copied().unwrap(),
                    self.row.columns[313].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[553].into_u32().copied().unwrap(),
                    self.row.columns[793].into_u32().copied().unwrap(),
                    self.row.columns[1033].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[13].into_i32().copied().unwrap(),
                    self.row.columns[253].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[133].into_i32().copied().unwrap(),
                    self.row.columns[373].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[493].into_i32().copied().unwrap(),
                    self.row.columns[733].into_i32().copied().unwrap(),
                    self.row.columns[973].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1213].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1273].into_i32().copied().unwrap(),
                    self.row.columns[1513].into_i32().copied().unwrap(),
                    self.row.columns[1573].into_i32().copied().unwrap(),
                    self.row.columns[1693].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1753].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1873].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[673].into_u16().copied().unwrap(),
                    self.row.columns[913].into_u16().copied().unwrap(),
                    self.row.columns[1153].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[1993].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[613].into_u8().copied().unwrap(),
                    self.row.columns[853].into_u8().copied().unwrap(),
                    self.row.columns[1093].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1333].into_u8().copied().unwrap(),
                    self.row.columns[1393].into_u8().copied().unwrap(),
                    self.row.columns[1453].into_u8().copied().unwrap(),
                    self.row.columns[1633].into_u8().copied().unwrap(),
                    self.row.columns[1813].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1933].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[193].into_bool().copied().unwrap(),
                    self.row.columns[433].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[74].into_u32().copied().unwrap(),
                    self.row.columns[314].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[554].into_u32().copied().unwrap(),
                    self.row.columns[794].into_u32().copied().unwrap(),
                    self.row.columns[1034].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[14].into_i32().copied().unwrap(),
                    self.row.columns[254].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[134].into_i32().copied().unwrap(),
                    self.row.columns[374].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[494].into_i32().copied().unwrap(),
                    self.row.columns[734].into_i32().copied().unwrap(),
                    self.row.columns[974].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1214].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1274].into_i32().copied().unwrap(),
                    self.row.columns[1514].into_i32().copied().unwrap(),
                    self.row.columns[1574].into_i32().copied().unwrap(),
                    self.row.columns[1694].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1754].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1874].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[674].into_u16().copied().unwrap(),
                    self.row.columns[914].into_u16().copied().unwrap(),
                    self.row.columns[1154].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[1994].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[614].into_u8().copied().unwrap(),
                    self.row.columns[854].into_u8().copied().unwrap(),
                    self.row.columns[1094].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1334].into_u8().copied().unwrap(),
                    self.row.columns[1394].into_u8().copied().unwrap(),
                    self.row.columns[1454].into_u8().copied().unwrap(),
                    self.row.columns[1634].into_u8().copied().unwrap(),
                    self.row.columns[1814].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1934].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[194].into_bool().copied().unwrap(),
                    self.row.columns[434].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[75].into_u32().copied().unwrap(),
                    self.row.columns[315].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[555].into_u32().copied().unwrap(),
                    self.row.columns[795].into_u32().copied().unwrap(),
                    self.row.columns[1035].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[15].into_i32().copied().unwrap(),
                    self.row.columns[255].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[135].into_i32().copied().unwrap(),
                    self.row.columns[375].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[495].into_i32().copied().unwrap(),
                    self.row.columns[735].into_i32().copied().unwrap(),
                    self.row.columns[975].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1215].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1275].into_i32().copied().unwrap(),
                    self.row.columns[1515].into_i32().copied().unwrap(),
                    self.row.columns[1575].into_i32().copied().unwrap(),
                    self.row.columns[1695].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1755].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1875].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[675].into_u16().copied().unwrap(),
                    self.row.columns[915].into_u16().copied().unwrap(),
                    self.row.columns[1155].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[1995].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[615].into_u8().copied().unwrap(),
                    self.row.columns[855].into_u8().copied().unwrap(),
                    self.row.columns[1095].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1335].into_u8().copied().unwrap(),
                    self.row.columns[1395].into_u8().copied().unwrap(),
                    self.row.columns[1455].into_u8().copied().unwrap(),
                    self.row.columns[1635].into_u8().copied().unwrap(),
                    self.row.columns[1815].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1935].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[195].into_bool().copied().unwrap(),
                    self.row.columns[435].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[76].into_u32().copied().unwrap(),
                    self.row.columns[316].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[556].into_u32().copied().unwrap(),
                    self.row.columns[796].into_u32().copied().unwrap(),
                    self.row.columns[1036].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[16].into_i32().copied().unwrap(),
                    self.row.columns[256].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[136].into_i32().copied().unwrap(),
                    self.row.columns[376].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[496].into_i32().copied().unwrap(),
                    self.row.columns[736].into_i32().copied().unwrap(),
                    self.row.columns[976].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1216].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1276].into_i32().copied().unwrap(),
                    self.row.columns[1516].into_i32().copied().unwrap(),
                    self.row.columns[1576].into_i32().copied().unwrap(),
                    self.row.columns[1696].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1756].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1876].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[676].into_u16().copied().unwrap(),
                    self.row.columns[916].into_u16().copied().unwrap(),
                    self.row.columns[1156].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[1996].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[616].into_u8().copied().unwrap(),
                    self.row.columns[856].into_u8().copied().unwrap(),
                    self.row.columns[1096].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1336].into_u8().copied().unwrap(),
                    self.row.columns[1396].into_u8().copied().unwrap(),
                    self.row.columns[1456].into_u8().copied().unwrap(),
                    self.row.columns[1636].into_u8().copied().unwrap(),
                    self.row.columns[1816].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1936].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[196].into_bool().copied().unwrap(),
                    self.row.columns[436].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[77].into_u32().copied().unwrap(),
                    self.row.columns[317].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[557].into_u32().copied().unwrap(),
                    self.row.columns[797].into_u32().copied().unwrap(),
                    self.row.columns[1037].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[17].into_i32().copied().unwrap(),
                    self.row.columns[257].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[137].into_i32().copied().unwrap(),
                    self.row.columns[377].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[497].into_i32().copied().unwrap(),
                    self.row.columns[737].into_i32().copied().unwrap(),
                    self.row.columns[977].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1217].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1277].into_i32().copied().unwrap(),
                    self.row.columns[1517].into_i32().copied().unwrap(),
                    self.row.columns[1577].into_i32().copied().unwrap(),
                    self.row.columns[1697].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1757].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1877].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[677].into_u16().copied().unwrap(),
                    self.row.columns[917].into_u16().copied().unwrap(),
                    self.row.columns[1157].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[1997].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[617].into_u8().copied().unwrap(),
                    self.row.columns[857].into_u8().copied().unwrap(),
                    self.row.columns[1097].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1337].into_u8().copied().unwrap(),
                    self.row.columns[1397].into_u8().copied().unwrap(),
                    self.row.columns[1457].into_u8().copied().unwrap(),
                    self.row.columns[1637].into_u8().copied().unwrap(),
                    self.row.columns[1817].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1937].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[197].into_bool().copied().unwrap(),
                    self.row.columns[437].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[78].into_u32().copied().unwrap(),
                    self.row.columns[318].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[558].into_u32().copied().unwrap(),
                    self.row.columns[798].into_u32().copied().unwrap(),
                    self.row.columns[1038].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[18].into_i32().copied().unwrap(),
                    self.row.columns[258].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[138].into_i32().copied().unwrap(),
                    self.row.columns[378].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[498].into_i32().copied().unwrap(),
                    self.row.columns[738].into_i32().copied().unwrap(),
                    self.row.columns[978].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1218].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1278].into_i32().copied().unwrap(),
                    self.row.columns[1518].into_i32().copied().unwrap(),
                    self.row.columns[1578].into_i32().copied().unwrap(),
                    self.row.columns[1698].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1758].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1878].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[678].into_u16().copied().unwrap(),
                    self.row.columns[918].into_u16().copied().unwrap(),
                    self.row.columns[1158].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[1998].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[618].into_u8().copied().unwrap(),
                    self.row.columns[858].into_u8().copied().unwrap(),
                    self.row.columns[1098].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1338].into_u8().copied().unwrap(),
                    self.row.columns[1398].into_u8().copied().unwrap(),
                    self.row.columns[1458].into_u8().copied().unwrap(),
                    self.row.columns[1638].into_u8().copied().unwrap(),
                    self.row.columns[1818].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1938].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[198].into_bool().copied().unwrap(),
                    self.row.columns[438].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[79].into_u32().copied().unwrap(),
                    self.row.columns[319].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[559].into_u32().copied().unwrap(),
                    self.row.columns[799].into_u32().copied().unwrap(),
                    self.row.columns[1039].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[19].into_i32().copied().unwrap(),
                    self.row.columns[259].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[139].into_i32().copied().unwrap(),
                    self.row.columns[379].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[499].into_i32().copied().unwrap(),
                    self.row.columns[739].into_i32().copied().unwrap(),
                    self.row.columns[979].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1219].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1279].into_i32().copied().unwrap(),
                    self.row.columns[1519].into_i32().copied().unwrap(),
                    self.row.columns[1579].into_i32().copied().unwrap(),
                    self.row.columns[1699].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1759].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1879].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[679].into_u16().copied().unwrap(),
                    self.row.columns[919].into_u16().copied().unwrap(),
                    self.row.columns[1159].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[1999].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[619].into_u8().copied().unwrap(),
                    self.row.columns[859].into_u8().copied().unwrap(),
                    self.row.columns[1099].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1339].into_u8().copied().unwrap(),
                    self.row.columns[1399].into_u8().copied().unwrap(),
                    self.row.columns[1459].into_u8().copied().unwrap(),
                    self.row.columns[1639].into_u8().copied().unwrap(),
                    self.row.columns[1819].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1939].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[199].into_bool().copied().unwrap(),
                    self.row.columns[439].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[80].into_u32().copied().unwrap(),
                    self.row.columns[320].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[560].into_u32().copied().unwrap(),
                    self.row.columns[800].into_u32().copied().unwrap(),
                    self.row.columns[1040].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[20].into_i32().copied().unwrap(),
                    self.row.columns[260].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[140].into_i32().copied().unwrap(),
                    self.row.columns[380].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[500].into_i32().copied().unwrap(),
                    self.row.columns[740].into_i32().copied().unwrap(),
                    self.row.columns[980].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1220].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1280].into_i32().copied().unwrap(),
                    self.row.columns[1520].into_i32().copied().unwrap(),
                    self.row.columns[1580].into_i32().copied().unwrap(),
                    self.row.columns[1700].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1760].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1880].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[680].into_u16().copied().unwrap(),
                    self.row.columns[920].into_u16().copied().unwrap(),
                    self.row.columns[1160].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2000].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[620].into_u8().copied().unwrap(),
                    self.row.columns[860].into_u8().copied().unwrap(),
                    self.row.columns[1100].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1340].into_u8().copied().unwrap(),
                    self.row.columns[1400].into_u8().copied().unwrap(),
                    self.row.columns[1460].into_u8().copied().unwrap(),
                    self.row.columns[1640].into_u8().copied().unwrap(),
                    self.row.columns[1820].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1940].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[200].into_bool().copied().unwrap(),
                    self.row.columns[440].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[81].into_u32().copied().unwrap(),
                    self.row.columns[321].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[561].into_u32().copied().unwrap(),
                    self.row.columns[801].into_u32().copied().unwrap(),
                    self.row.columns[1041].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[21].into_i32().copied().unwrap(),
                    self.row.columns[261].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[141].into_i32().copied().unwrap(),
                    self.row.columns[381].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[501].into_i32().copied().unwrap(),
                    self.row.columns[741].into_i32().copied().unwrap(),
                    self.row.columns[981].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1221].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1281].into_i32().copied().unwrap(),
                    self.row.columns[1521].into_i32().copied().unwrap(),
                    self.row.columns[1581].into_i32().copied().unwrap(),
                    self.row.columns[1701].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1761].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1881].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[681].into_u16().copied().unwrap(),
                    self.row.columns[921].into_u16().copied().unwrap(),
                    self.row.columns[1161].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2001].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[621].into_u8().copied().unwrap(),
                    self.row.columns[861].into_u8().copied().unwrap(),
                    self.row.columns[1101].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1341].into_u8().copied().unwrap(),
                    self.row.columns[1401].into_u8().copied().unwrap(),
                    self.row.columns[1461].into_u8().copied().unwrap(),
                    self.row.columns[1641].into_u8().copied().unwrap(),
                    self.row.columns[1821].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1941].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[201].into_bool().copied().unwrap(),
                    self.row.columns[441].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[82].into_u32().copied().unwrap(),
                    self.row.columns[322].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[562].into_u32().copied().unwrap(),
                    self.row.columns[802].into_u32().copied().unwrap(),
                    self.row.columns[1042].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[22].into_i32().copied().unwrap(),
                    self.row.columns[262].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[142].into_i32().copied().unwrap(),
                    self.row.columns[382].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[502].into_i32().copied().unwrap(),
                    self.row.columns[742].into_i32().copied().unwrap(),
                    self.row.columns[982].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1222].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1282].into_i32().copied().unwrap(),
                    self.row.columns[1522].into_i32().copied().unwrap(),
                    self.row.columns[1582].into_i32().copied().unwrap(),
                    self.row.columns[1702].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1762].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1882].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[682].into_u16().copied().unwrap(),
                    self.row.columns[922].into_u16().copied().unwrap(),
                    self.row.columns[1162].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2002].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[622].into_u8().copied().unwrap(),
                    self.row.columns[862].into_u8().copied().unwrap(),
                    self.row.columns[1102].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1342].into_u8().copied().unwrap(),
                    self.row.columns[1402].into_u8().copied().unwrap(),
                    self.row.columns[1462].into_u8().copied().unwrap(),
                    self.row.columns[1642].into_u8().copied().unwrap(),
                    self.row.columns[1822].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1942].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[202].into_bool().copied().unwrap(),
                    self.row.columns[442].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[83].into_u32().copied().unwrap(),
                    self.row.columns[323].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[563].into_u32().copied().unwrap(),
                    self.row.columns[803].into_u32().copied().unwrap(),
                    self.row.columns[1043].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[23].into_i32().copied().unwrap(),
                    self.row.columns[263].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[143].into_i32().copied().unwrap(),
                    self.row.columns[383].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[503].into_i32().copied().unwrap(),
                    self.row.columns[743].into_i32().copied().unwrap(),
                    self.row.columns[983].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1223].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1283].into_i32().copied().unwrap(),
                    self.row.columns[1523].into_i32().copied().unwrap(),
                    self.row.columns[1583].into_i32().copied().unwrap(),
                    self.row.columns[1703].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1763].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1883].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[683].into_u16().copied().unwrap(),
                    self.row.columns[923].into_u16().copied().unwrap(),
                    self.row.columns[1163].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2003].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[623].into_u8().copied().unwrap(),
                    self.row.columns[863].into_u8().copied().unwrap(),
                    self.row.columns[1103].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1343].into_u8().copied().unwrap(),
                    self.row.columns[1403].into_u8().copied().unwrap(),
                    self.row.columns[1463].into_u8().copied().unwrap(),
                    self.row.columns[1643].into_u8().copied().unwrap(),
                    self.row.columns[1823].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1943].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[203].into_bool().copied().unwrap(),
                    self.row.columns[443].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[84].into_u32().copied().unwrap(),
                    self.row.columns[324].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[564].into_u32().copied().unwrap(),
                    self.row.columns[804].into_u32().copied().unwrap(),
                    self.row.columns[1044].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[24].into_i32().copied().unwrap(),
                    self.row.columns[264].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[144].into_i32().copied().unwrap(),
                    self.row.columns[384].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[504].into_i32().copied().unwrap(),
                    self.row.columns[744].into_i32().copied().unwrap(),
                    self.row.columns[984].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1224].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1284].into_i32().copied().unwrap(),
                    self.row.columns[1524].into_i32().copied().unwrap(),
                    self.row.columns[1584].into_i32().copied().unwrap(),
                    self.row.columns[1704].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1764].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1884].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[684].into_u16().copied().unwrap(),
                    self.row.columns[924].into_u16().copied().unwrap(),
                    self.row.columns[1164].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2004].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[624].into_u8().copied().unwrap(),
                    self.row.columns[864].into_u8().copied().unwrap(),
                    self.row.columns[1104].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1344].into_u8().copied().unwrap(),
                    self.row.columns[1404].into_u8().copied().unwrap(),
                    self.row.columns[1464].into_u8().copied().unwrap(),
                    self.row.columns[1644].into_u8().copied().unwrap(),
                    self.row.columns[1824].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1944].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[204].into_bool().copied().unwrap(),
                    self.row.columns[444].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[85].into_u32().copied().unwrap(),
                    self.row.columns[325].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[565].into_u32().copied().unwrap(),
                    self.row.columns[805].into_u32().copied().unwrap(),
                    self.row.columns[1045].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[25].into_i32().copied().unwrap(),
                    self.row.columns[265].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[145].into_i32().copied().unwrap(),
                    self.row.columns[385].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[505].into_i32().copied().unwrap(),
                    self.row.columns[745].into_i32().copied().unwrap(),
                    self.row.columns[985].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1225].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1285].into_i32().copied().unwrap(),
                    self.row.columns[1525].into_i32().copied().unwrap(),
                    self.row.columns[1585].into_i32().copied().unwrap(),
                    self.row.columns[1705].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1765].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1885].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[685].into_u16().copied().unwrap(),
                    self.row.columns[925].into_u16().copied().unwrap(),
                    self.row.columns[1165].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2005].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[625].into_u8().copied().unwrap(),
                    self.row.columns[865].into_u8().copied().unwrap(),
                    self.row.columns[1105].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1345].into_u8().copied().unwrap(),
                    self.row.columns[1405].into_u8().copied().unwrap(),
                    self.row.columns[1465].into_u8().copied().unwrap(),
                    self.row.columns[1645].into_u8().copied().unwrap(),
                    self.row.columns[1825].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1945].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[205].into_bool().copied().unwrap(),
                    self.row.columns[445].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[86].into_u32().copied().unwrap(),
                    self.row.columns[326].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[566].into_u32().copied().unwrap(),
                    self.row.columns[806].into_u32().copied().unwrap(),
                    self.row.columns[1046].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[26].into_i32().copied().unwrap(),
                    self.row.columns[266].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[146].into_i32().copied().unwrap(),
                    self.row.columns[386].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[506].into_i32().copied().unwrap(),
                    self.row.columns[746].into_i32().copied().unwrap(),
                    self.row.columns[986].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1226].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1286].into_i32().copied().unwrap(),
                    self.row.columns[1526].into_i32().copied().unwrap(),
                    self.row.columns[1586].into_i32().copied().unwrap(),
                    self.row.columns[1706].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1766].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1886].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[686].into_u16().copied().unwrap(),
                    self.row.columns[926].into_u16().copied().unwrap(),
                    self.row.columns[1166].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2006].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[626].into_u8().copied().unwrap(),
                    self.row.columns[866].into_u8().copied().unwrap(),
                    self.row.columns[1106].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1346].into_u8().copied().unwrap(),
                    self.row.columns[1406].into_u8().copied().unwrap(),
                    self.row.columns[1466].into_u8().copied().unwrap(),
                    self.row.columns[1646].into_u8().copied().unwrap(),
                    self.row.columns[1826].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1946].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[206].into_bool().copied().unwrap(),
                    self.row.columns[446].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[87].into_u32().copied().unwrap(),
                    self.row.columns[327].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[567].into_u32().copied().unwrap(),
                    self.row.columns[807].into_u32().copied().unwrap(),
                    self.row.columns[1047].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[27].into_i32().copied().unwrap(),
                    self.row.columns[267].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[147].into_i32().copied().unwrap(),
                    self.row.columns[387].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[507].into_i32().copied().unwrap(),
                    self.row.columns[747].into_i32().copied().unwrap(),
                    self.row.columns[987].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1227].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1287].into_i32().copied().unwrap(),
                    self.row.columns[1527].into_i32().copied().unwrap(),
                    self.row.columns[1587].into_i32().copied().unwrap(),
                    self.row.columns[1707].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1767].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1887].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[687].into_u16().copied().unwrap(),
                    self.row.columns[927].into_u16().copied().unwrap(),
                    self.row.columns[1167].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2007].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[627].into_u8().copied().unwrap(),
                    self.row.columns[867].into_u8().copied().unwrap(),
                    self.row.columns[1107].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1347].into_u8().copied().unwrap(),
                    self.row.columns[1407].into_u8().copied().unwrap(),
                    self.row.columns[1467].into_u8().copied().unwrap(),
                    self.row.columns[1647].into_u8().copied().unwrap(),
                    self.row.columns[1827].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1947].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[207].into_bool().copied().unwrap(),
                    self.row.columns[447].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[88].into_u32().copied().unwrap(),
                    self.row.columns[328].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[568].into_u32().copied().unwrap(),
                    self.row.columns[808].into_u32().copied().unwrap(),
                    self.row.columns[1048].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[28].into_i32().copied().unwrap(),
                    self.row.columns[268].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[148].into_i32().copied().unwrap(),
                    self.row.columns[388].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[508].into_i32().copied().unwrap(),
                    self.row.columns[748].into_i32().copied().unwrap(),
                    self.row.columns[988].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1228].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1288].into_i32().copied().unwrap(),
                    self.row.columns[1528].into_i32().copied().unwrap(),
                    self.row.columns[1588].into_i32().copied().unwrap(),
                    self.row.columns[1708].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1768].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1888].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[688].into_u16().copied().unwrap(),
                    self.row.columns[928].into_u16().copied().unwrap(),
                    self.row.columns[1168].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2008].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[628].into_u8().copied().unwrap(),
                    self.row.columns[868].into_u8().copied().unwrap(),
                    self.row.columns[1108].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1348].into_u8().copied().unwrap(),
                    self.row.columns[1408].into_u8().copied().unwrap(),
                    self.row.columns[1468].into_u8().copied().unwrap(),
                    self.row.columns[1648].into_u8().copied().unwrap(),
                    self.row.columns[1828].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1948].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[208].into_bool().copied().unwrap(),
                    self.row.columns[448].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[89].into_u32().copied().unwrap(),
                    self.row.columns[329].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[569].into_u32().copied().unwrap(),
                    self.row.columns[809].into_u32().copied().unwrap(),
                    self.row.columns[1049].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[29].into_i32().copied().unwrap(),
                    self.row.columns[269].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[149].into_i32().copied().unwrap(),
                    self.row.columns[389].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[509].into_i32().copied().unwrap(),
                    self.row.columns[749].into_i32().copied().unwrap(),
                    self.row.columns[989].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1229].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1289].into_i32().copied().unwrap(),
                    self.row.columns[1529].into_i32().copied().unwrap(),
                    self.row.columns[1589].into_i32().copied().unwrap(),
                    self.row.columns[1709].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1769].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1889].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[689].into_u16().copied().unwrap(),
                    self.row.columns[929].into_u16().copied().unwrap(),
                    self.row.columns[1169].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2009].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[629].into_u8().copied().unwrap(),
                    self.row.columns[869].into_u8().copied().unwrap(),
                    self.row.columns[1109].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1349].into_u8().copied().unwrap(),
                    self.row.columns[1409].into_u8().copied().unwrap(),
                    self.row.columns[1469].into_u8().copied().unwrap(),
                    self.row.columns[1649].into_u8().copied().unwrap(),
                    self.row.columns[1829].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1949].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[209].into_bool().copied().unwrap(),
                    self.row.columns[449].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[90].into_u32().copied().unwrap(),
                    self.row.columns[330].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[570].into_u32().copied().unwrap(),
                    self.row.columns[810].into_u32().copied().unwrap(),
                    self.row.columns[1050].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[30].into_i32().copied().unwrap(),
                    self.row.columns[270].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[150].into_i32().copied().unwrap(),
                    self.row.columns[390].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[510].into_i32().copied().unwrap(),
                    self.row.columns[750].into_i32().copied().unwrap(),
                    self.row.columns[990].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1230].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1290].into_i32().copied().unwrap(),
                    self.row.columns[1530].into_i32().copied().unwrap(),
                    self.row.columns[1590].into_i32().copied().unwrap(),
                    self.row.columns[1710].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1770].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1890].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[690].into_u16().copied().unwrap(),
                    self.row.columns[930].into_u16().copied().unwrap(),
                    self.row.columns[1170].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2010].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[630].into_u8().copied().unwrap(),
                    self.row.columns[870].into_u8().copied().unwrap(),
                    self.row.columns[1110].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1350].into_u8().copied().unwrap(),
                    self.row.columns[1410].into_u8().copied().unwrap(),
                    self.row.columns[1470].into_u8().copied().unwrap(),
                    self.row.columns[1650].into_u8().copied().unwrap(),
                    self.row.columns[1830].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1950].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[210].into_bool().copied().unwrap(),
                    self.row.columns[450].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[91].into_u32().copied().unwrap(),
                    self.row.columns[331].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[571].into_u32().copied().unwrap(),
                    self.row.columns[811].into_u32().copied().unwrap(),
                    self.row.columns[1051].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[31].into_i32().copied().unwrap(),
                    self.row.columns[271].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[151].into_i32().copied().unwrap(),
                    self.row.columns[391].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[511].into_i32().copied().unwrap(),
                    self.row.columns[751].into_i32().copied().unwrap(),
                    self.row.columns[991].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1231].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1291].into_i32().copied().unwrap(),
                    self.row.columns[1531].into_i32().copied().unwrap(),
                    self.row.columns[1591].into_i32().copied().unwrap(),
                    self.row.columns[1711].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1771].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1891].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[691].into_u16().copied().unwrap(),
                    self.row.columns[931].into_u16().copied().unwrap(),
                    self.row.columns[1171].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2011].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[631].into_u8().copied().unwrap(),
                    self.row.columns[871].into_u8().copied().unwrap(),
                    self.row.columns[1111].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1351].into_u8().copied().unwrap(),
                    self.row.columns[1411].into_u8().copied().unwrap(),
                    self.row.columns[1471].into_u8().copied().unwrap(),
                    self.row.columns[1651].into_u8().copied().unwrap(),
                    self.row.columns[1831].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1951].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[211].into_bool().copied().unwrap(),
                    self.row.columns[451].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[92].into_u32().copied().unwrap(),
                    self.row.columns[332].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[572].into_u32().copied().unwrap(),
                    self.row.columns[812].into_u32().copied().unwrap(),
                    self.row.columns[1052].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[32].into_i32().copied().unwrap(),
                    self.row.columns[272].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[152].into_i32().copied().unwrap(),
                    self.row.columns[392].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[512].into_i32().copied().unwrap(),
                    self.row.columns[752].into_i32().copied().unwrap(),
                    self.row.columns[992].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1232].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1292].into_i32().copied().unwrap(),
                    self.row.columns[1532].into_i32().copied().unwrap(),
                    self.row.columns[1592].into_i32().copied().unwrap(),
                    self.row.columns[1712].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1772].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1892].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[692].into_u16().copied().unwrap(),
                    self.row.columns[932].into_u16().copied().unwrap(),
                    self.row.columns[1172].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2012].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[632].into_u8().copied().unwrap(),
                    self.row.columns[872].into_u8().copied().unwrap(),
                    self.row.columns[1112].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1352].into_u8().copied().unwrap(),
                    self.row.columns[1412].into_u8().copied().unwrap(),
                    self.row.columns[1472].into_u8().copied().unwrap(),
                    self.row.columns[1652].into_u8().copied().unwrap(),
                    self.row.columns[1832].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1952].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[212].into_bool().copied().unwrap(),
                    self.row.columns[452].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[93].into_u32().copied().unwrap(),
                    self.row.columns[333].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[573].into_u32().copied().unwrap(),
                    self.row.columns[813].into_u32().copied().unwrap(),
                    self.row.columns[1053].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[33].into_i32().copied().unwrap(),
                    self.row.columns[273].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[153].into_i32().copied().unwrap(),
                    self.row.columns[393].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[513].into_i32().copied().unwrap(),
                    self.row.columns[753].into_i32().copied().unwrap(),
                    self.row.columns[993].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1233].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1293].into_i32().copied().unwrap(),
                    self.row.columns[1533].into_i32().copied().unwrap(),
                    self.row.columns[1593].into_i32().copied().unwrap(),
                    self.row.columns[1713].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1773].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1893].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[693].into_u16().copied().unwrap(),
                    self.row.columns[933].into_u16().copied().unwrap(),
                    self.row.columns[1173].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2013].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[633].into_u8().copied().unwrap(),
                    self.row.columns[873].into_u8().copied().unwrap(),
                    self.row.columns[1113].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1353].into_u8().copied().unwrap(),
                    self.row.columns[1413].into_u8().copied().unwrap(),
                    self.row.columns[1473].into_u8().copied().unwrap(),
                    self.row.columns[1653].into_u8().copied().unwrap(),
                    self.row.columns[1833].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1953].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[213].into_bool().copied().unwrap(),
                    self.row.columns[453].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[94].into_u32().copied().unwrap(),
                    self.row.columns[334].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[574].into_u32().copied().unwrap(),
                    self.row.columns[814].into_u32().copied().unwrap(),
                    self.row.columns[1054].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[34].into_i32().copied().unwrap(),
                    self.row.columns[274].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[154].into_i32().copied().unwrap(),
                    self.row.columns[394].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[514].into_i32().copied().unwrap(),
                    self.row.columns[754].into_i32().copied().unwrap(),
                    self.row.columns[994].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1234].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1294].into_i32().copied().unwrap(),
                    self.row.columns[1534].into_i32().copied().unwrap(),
                    self.row.columns[1594].into_i32().copied().unwrap(),
                    self.row.columns[1714].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1774].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1894].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[694].into_u16().copied().unwrap(),
                    self.row.columns[934].into_u16().copied().unwrap(),
                    self.row.columns[1174].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2014].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[634].into_u8().copied().unwrap(),
                    self.row.columns[874].into_u8().copied().unwrap(),
                    self.row.columns[1114].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1354].into_u8().copied().unwrap(),
                    self.row.columns[1414].into_u8().copied().unwrap(),
                    self.row.columns[1474].into_u8().copied().unwrap(),
                    self.row.columns[1654].into_u8().copied().unwrap(),
                    self.row.columns[1834].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1954].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[214].into_bool().copied().unwrap(),
                    self.row.columns[454].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[95].into_u32().copied().unwrap(),
                    self.row.columns[335].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[575].into_u32().copied().unwrap(),
                    self.row.columns[815].into_u32().copied().unwrap(),
                    self.row.columns[1055].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[35].into_i32().copied().unwrap(),
                    self.row.columns[275].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[155].into_i32().copied().unwrap(),
                    self.row.columns[395].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[515].into_i32().copied().unwrap(),
                    self.row.columns[755].into_i32().copied().unwrap(),
                    self.row.columns[995].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1235].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1295].into_i32().copied().unwrap(),
                    self.row.columns[1535].into_i32().copied().unwrap(),
                    self.row.columns[1595].into_i32().copied().unwrap(),
                    self.row.columns[1715].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1775].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1895].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[695].into_u16().copied().unwrap(),
                    self.row.columns[935].into_u16().copied().unwrap(),
                    self.row.columns[1175].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2015].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[635].into_u8().copied().unwrap(),
                    self.row.columns[875].into_u8().copied().unwrap(),
                    self.row.columns[1115].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1355].into_u8().copied().unwrap(),
                    self.row.columns[1415].into_u8().copied().unwrap(),
                    self.row.columns[1475].into_u8().copied().unwrap(),
                    self.row.columns[1655].into_u8().copied().unwrap(),
                    self.row.columns[1835].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1955].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[215].into_bool().copied().unwrap(),
                    self.row.columns[455].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[96].into_u32().copied().unwrap(),
                    self.row.columns[336].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[576].into_u32().copied().unwrap(),
                    self.row.columns[816].into_u32().copied().unwrap(),
                    self.row.columns[1056].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[36].into_i32().copied().unwrap(),
                    self.row.columns[276].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[156].into_i32().copied().unwrap(),
                    self.row.columns[396].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[516].into_i32().copied().unwrap(),
                    self.row.columns[756].into_i32().copied().unwrap(),
                    self.row.columns[996].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1236].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1296].into_i32().copied().unwrap(),
                    self.row.columns[1536].into_i32().copied().unwrap(),
                    self.row.columns[1596].into_i32().copied().unwrap(),
                    self.row.columns[1716].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1776].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1896].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[696].into_u16().copied().unwrap(),
                    self.row.columns[936].into_u16().copied().unwrap(),
                    self.row.columns[1176].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2016].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[636].into_u8().copied().unwrap(),
                    self.row.columns[876].into_u8().copied().unwrap(),
                    self.row.columns[1116].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1356].into_u8().copied().unwrap(),
                    self.row.columns[1416].into_u8().copied().unwrap(),
                    self.row.columns[1476].into_u8().copied().unwrap(),
                    self.row.columns[1656].into_u8().copied().unwrap(),
                    self.row.columns[1836].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1956].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[216].into_bool().copied().unwrap(),
                    self.row.columns[456].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[97].into_u32().copied().unwrap(),
                    self.row.columns[337].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[577].into_u32().copied().unwrap(),
                    self.row.columns[817].into_u32().copied().unwrap(),
                    self.row.columns[1057].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[37].into_i32().copied().unwrap(),
                    self.row.columns[277].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[157].into_i32().copied().unwrap(),
                    self.row.columns[397].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[517].into_i32().copied().unwrap(),
                    self.row.columns[757].into_i32().copied().unwrap(),
                    self.row.columns[997].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1237].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1297].into_i32().copied().unwrap(),
                    self.row.columns[1537].into_i32().copied().unwrap(),
                    self.row.columns[1597].into_i32().copied().unwrap(),
                    self.row.columns[1717].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1777].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1897].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[697].into_u16().copied().unwrap(),
                    self.row.columns[937].into_u16().copied().unwrap(),
                    self.row.columns[1177].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2017].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[637].into_u8().copied().unwrap(),
                    self.row.columns[877].into_u8().copied().unwrap(),
                    self.row.columns[1117].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1357].into_u8().copied().unwrap(),
                    self.row.columns[1417].into_u8().copied().unwrap(),
                    self.row.columns[1477].into_u8().copied().unwrap(),
                    self.row.columns[1657].into_u8().copied().unwrap(),
                    self.row.columns[1837].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1957].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[217].into_bool().copied().unwrap(),
                    self.row.columns[457].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[98].into_u32().copied().unwrap(),
                    self.row.columns[338].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[578].into_u32().copied().unwrap(),
                    self.row.columns[818].into_u32().copied().unwrap(),
                    self.row.columns[1058].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[38].into_i32().copied().unwrap(),
                    self.row.columns[278].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[158].into_i32().copied().unwrap(),
                    self.row.columns[398].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[518].into_i32().copied().unwrap(),
                    self.row.columns[758].into_i32().copied().unwrap(),
                    self.row.columns[998].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1238].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1298].into_i32().copied().unwrap(),
                    self.row.columns[1538].into_i32().copied().unwrap(),
                    self.row.columns[1598].into_i32().copied().unwrap(),
                    self.row.columns[1718].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1778].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1898].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[698].into_u16().copied().unwrap(),
                    self.row.columns[938].into_u16().copied().unwrap(),
                    self.row.columns[1178].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2018].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[638].into_u8().copied().unwrap(),
                    self.row.columns[878].into_u8().copied().unwrap(),
                    self.row.columns[1118].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1358].into_u8().copied().unwrap(),
                    self.row.columns[1418].into_u8().copied().unwrap(),
                    self.row.columns[1478].into_u8().copied().unwrap(),
                    self.row.columns[1658].into_u8().copied().unwrap(),
                    self.row.columns[1838].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1958].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[218].into_bool().copied().unwrap(),
                    self.row.columns[458].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[99].into_u32().copied().unwrap(),
                    self.row.columns[339].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[579].into_u32().copied().unwrap(),
                    self.row.columns[819].into_u32().copied().unwrap(),
                    self.row.columns[1059].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[39].into_i32().copied().unwrap(),
                    self.row.columns[279].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[159].into_i32().copied().unwrap(),
                    self.row.columns[399].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[519].into_i32().copied().unwrap(),
                    self.row.columns[759].into_i32().copied().unwrap(),
                    self.row.columns[999].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1239].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1299].into_i32().copied().unwrap(),
                    self.row.columns[1539].into_i32().copied().unwrap(),
                    self.row.columns[1599].into_i32().copied().unwrap(),
                    self.row.columns[1719].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1779].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1899].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[699].into_u16().copied().unwrap(),
                    self.row.columns[939].into_u16().copied().unwrap(),
                    self.row.columns[1179].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2019].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[639].into_u8().copied().unwrap(),
                    self.row.columns[879].into_u8().copied().unwrap(),
                    self.row.columns[1119].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1359].into_u8().copied().unwrap(),
                    self.row.columns[1419].into_u8().copied().unwrap(),
                    self.row.columns[1479].into_u8().copied().unwrap(),
                    self.row.columns[1659].into_u8().copied().unwrap(),
                    self.row.columns[1839].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1959].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[219].into_bool().copied().unwrap(),
                    self.row.columns[459].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[100].into_u32().copied().unwrap(),
                    self.row.columns[340].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[580].into_u32().copied().unwrap(),
                    self.row.columns[820].into_u32().copied().unwrap(),
                    self.row.columns[1060].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[40].into_i32().copied().unwrap(),
                    self.row.columns[280].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[160].into_i32().copied().unwrap(),
                    self.row.columns[400].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[520].into_i32().copied().unwrap(),
                    self.row.columns[760].into_i32().copied().unwrap(),
                    self.row.columns[1000].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1240].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1300].into_i32().copied().unwrap(),
                    self.row.columns[1540].into_i32().copied().unwrap(),
                    self.row.columns[1600].into_i32().copied().unwrap(),
                    self.row.columns[1720].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1780].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1900].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[700].into_u16().copied().unwrap(),
                    self.row.columns[940].into_u16().copied().unwrap(),
                    self.row.columns[1180].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2020].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[640].into_u8().copied().unwrap(),
                    self.row.columns[880].into_u8().copied().unwrap(),
                    self.row.columns[1120].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1360].into_u8().copied().unwrap(),
                    self.row.columns[1420].into_u8().copied().unwrap(),
                    self.row.columns[1480].into_u8().copied().unwrap(),
                    self.row.columns[1660].into_u8().copied().unwrap(),
                    self.row.columns[1840].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1960].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[220].into_bool().copied().unwrap(),
                    self.row.columns[460].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[101].into_u32().copied().unwrap(),
                    self.row.columns[341].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[581].into_u32().copied().unwrap(),
                    self.row.columns[821].into_u32().copied().unwrap(),
                    self.row.columns[1061].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[41].into_i32().copied().unwrap(),
                    self.row.columns[281].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[161].into_i32().copied().unwrap(),
                    self.row.columns[401].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[521].into_i32().copied().unwrap(),
                    self.row.columns[761].into_i32().copied().unwrap(),
                    self.row.columns[1001].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1241].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1301].into_i32().copied().unwrap(),
                    self.row.columns[1541].into_i32().copied().unwrap(),
                    self.row.columns[1601].into_i32().copied().unwrap(),
                    self.row.columns[1721].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1781].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1901].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[701].into_u16().copied().unwrap(),
                    self.row.columns[941].into_u16().copied().unwrap(),
                    self.row.columns[1181].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2021].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[641].into_u8().copied().unwrap(),
                    self.row.columns[881].into_u8().copied().unwrap(),
                    self.row.columns[1121].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1361].into_u8().copied().unwrap(),
                    self.row.columns[1421].into_u8().copied().unwrap(),
                    self.row.columns[1481].into_u8().copied().unwrap(),
                    self.row.columns[1661].into_u8().copied().unwrap(),
                    self.row.columns[1841].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1961].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[221].into_bool().copied().unwrap(),
                    self.row.columns[461].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[102].into_u32().copied().unwrap(),
                    self.row.columns[342].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[582].into_u32().copied().unwrap(),
                    self.row.columns[822].into_u32().copied().unwrap(),
                    self.row.columns[1062].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[42].into_i32().copied().unwrap(),
                    self.row.columns[282].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[162].into_i32().copied().unwrap(),
                    self.row.columns[402].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[522].into_i32().copied().unwrap(),
                    self.row.columns[762].into_i32().copied().unwrap(),
                    self.row.columns[1002].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1242].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1302].into_i32().copied().unwrap(),
                    self.row.columns[1542].into_i32().copied().unwrap(),
                    self.row.columns[1602].into_i32().copied().unwrap(),
                    self.row.columns[1722].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1782].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1902].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[702].into_u16().copied().unwrap(),
                    self.row.columns[942].into_u16().copied().unwrap(),
                    self.row.columns[1182].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2022].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[642].into_u8().copied().unwrap(),
                    self.row.columns[882].into_u8().copied().unwrap(),
                    self.row.columns[1122].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1362].into_u8().copied().unwrap(),
                    self.row.columns[1422].into_u8().copied().unwrap(),
                    self.row.columns[1482].into_u8().copied().unwrap(),
                    self.row.columns[1662].into_u8().copied().unwrap(),
                    self.row.columns[1842].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1962].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[222].into_bool().copied().unwrap(),
                    self.row.columns[462].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[103].into_u32().copied().unwrap(),
                    self.row.columns[343].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[583].into_u32().copied().unwrap(),
                    self.row.columns[823].into_u32().copied().unwrap(),
                    self.row.columns[1063].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[43].into_i32().copied().unwrap(),
                    self.row.columns[283].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[163].into_i32().copied().unwrap(),
                    self.row.columns[403].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[523].into_i32().copied().unwrap(),
                    self.row.columns[763].into_i32().copied().unwrap(),
                    self.row.columns[1003].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1243].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1303].into_i32().copied().unwrap(),
                    self.row.columns[1543].into_i32().copied().unwrap(),
                    self.row.columns[1603].into_i32().copied().unwrap(),
                    self.row.columns[1723].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1783].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1903].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[703].into_u16().copied().unwrap(),
                    self.row.columns[943].into_u16().copied().unwrap(),
                    self.row.columns[1183].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2023].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[643].into_u8().copied().unwrap(),
                    self.row.columns[883].into_u8().copied().unwrap(),
                    self.row.columns[1123].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1363].into_u8().copied().unwrap(),
                    self.row.columns[1423].into_u8().copied().unwrap(),
                    self.row.columns[1483].into_u8().copied().unwrap(),
                    self.row.columns[1663].into_u8().copied().unwrap(),
                    self.row.columns[1843].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1963].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[223].into_bool().copied().unwrap(),
                    self.row.columns[463].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[104].into_u32().copied().unwrap(),
                    self.row.columns[344].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[584].into_u32().copied().unwrap(),
                    self.row.columns[824].into_u32().copied().unwrap(),
                    self.row.columns[1064].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[44].into_i32().copied().unwrap(),
                    self.row.columns[284].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[164].into_i32().copied().unwrap(),
                    self.row.columns[404].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[524].into_i32().copied().unwrap(),
                    self.row.columns[764].into_i32().copied().unwrap(),
                    self.row.columns[1004].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1244].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1304].into_i32().copied().unwrap(),
                    self.row.columns[1544].into_i32().copied().unwrap(),
                    self.row.columns[1604].into_i32().copied().unwrap(),
                    self.row.columns[1724].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1784].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1904].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[704].into_u16().copied().unwrap(),
                    self.row.columns[944].into_u16().copied().unwrap(),
                    self.row.columns[1184].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2024].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[644].into_u8().copied().unwrap(),
                    self.row.columns[884].into_u8().copied().unwrap(),
                    self.row.columns[1124].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1364].into_u8().copied().unwrap(),
                    self.row.columns[1424].into_u8().copied().unwrap(),
                    self.row.columns[1484].into_u8().copied().unwrap(),
                    self.row.columns[1664].into_u8().copied().unwrap(),
                    self.row.columns[1844].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1964].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[224].into_bool().copied().unwrap(),
                    self.row.columns[464].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[105].into_u32().copied().unwrap(),
                    self.row.columns[345].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[585].into_u32().copied().unwrap(),
                    self.row.columns[825].into_u32().copied().unwrap(),
                    self.row.columns[1065].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[45].into_i32().copied().unwrap(),
                    self.row.columns[285].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[165].into_i32().copied().unwrap(),
                    self.row.columns[405].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[525].into_i32().copied().unwrap(),
                    self.row.columns[765].into_i32().copied().unwrap(),
                    self.row.columns[1005].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1245].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1305].into_i32().copied().unwrap(),
                    self.row.columns[1545].into_i32().copied().unwrap(),
                    self.row.columns[1605].into_i32().copied().unwrap(),
                    self.row.columns[1725].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1785].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1905].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[705].into_u16().copied().unwrap(),
                    self.row.columns[945].into_u16().copied().unwrap(),
                    self.row.columns[1185].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2025].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[645].into_u8().copied().unwrap(),
                    self.row.columns[885].into_u8().copied().unwrap(),
                    self.row.columns[1125].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1365].into_u8().copied().unwrap(),
                    self.row.columns[1425].into_u8().copied().unwrap(),
                    self.row.columns[1485].into_u8().copied().unwrap(),
                    self.row.columns[1665].into_u8().copied().unwrap(),
                    self.row.columns[1845].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1965].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[225].into_bool().copied().unwrap(),
                    self.row.columns[465].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[106].into_u32().copied().unwrap(),
                    self.row.columns[346].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[586].into_u32().copied().unwrap(),
                    self.row.columns[826].into_u32().copied().unwrap(),
                    self.row.columns[1066].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[46].into_i32().copied().unwrap(),
                    self.row.columns[286].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[166].into_i32().copied().unwrap(),
                    self.row.columns[406].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[526].into_i32().copied().unwrap(),
                    self.row.columns[766].into_i32().copied().unwrap(),
                    self.row.columns[1006].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1246].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1306].into_i32().copied().unwrap(),
                    self.row.columns[1546].into_i32().copied().unwrap(),
                    self.row.columns[1606].into_i32().copied().unwrap(),
                    self.row.columns[1726].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1786].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1906].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[706].into_u16().copied().unwrap(),
                    self.row.columns[946].into_u16().copied().unwrap(),
                    self.row.columns[1186].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2026].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[646].into_u8().copied().unwrap(),
                    self.row.columns[886].into_u8().copied().unwrap(),
                    self.row.columns[1126].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1366].into_u8().copied().unwrap(),
                    self.row.columns[1426].into_u8().copied().unwrap(),
                    self.row.columns[1486].into_u8().copied().unwrap(),
                    self.row.columns[1666].into_u8().copied().unwrap(),
                    self.row.columns[1846].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1966].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[226].into_bool().copied().unwrap(),
                    self.row.columns[466].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[107].into_u32().copied().unwrap(),
                    self.row.columns[347].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[587].into_u32().copied().unwrap(),
                    self.row.columns[827].into_u32().copied().unwrap(),
                    self.row.columns[1067].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[47].into_i32().copied().unwrap(),
                    self.row.columns[287].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[167].into_i32().copied().unwrap(),
                    self.row.columns[407].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[527].into_i32().copied().unwrap(),
                    self.row.columns[767].into_i32().copied().unwrap(),
                    self.row.columns[1007].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1247].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1307].into_i32().copied().unwrap(),
                    self.row.columns[1547].into_i32().copied().unwrap(),
                    self.row.columns[1607].into_i32().copied().unwrap(),
                    self.row.columns[1727].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1787].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1907].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[707].into_u16().copied().unwrap(),
                    self.row.columns[947].into_u16().copied().unwrap(),
                    self.row.columns[1187].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2027].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[647].into_u8().copied().unwrap(),
                    self.row.columns[887].into_u8().copied().unwrap(),
                    self.row.columns[1127].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1367].into_u8().copied().unwrap(),
                    self.row.columns[1427].into_u8().copied().unwrap(),
                    self.row.columns[1487].into_u8().copied().unwrap(),
                    self.row.columns[1667].into_u8().copied().unwrap(),
                    self.row.columns[1847].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1967].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[227].into_bool().copied().unwrap(),
                    self.row.columns[467].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[108].into_u32().copied().unwrap(),
                    self.row.columns[348].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[588].into_u32().copied().unwrap(),
                    self.row.columns[828].into_u32().copied().unwrap(),
                    self.row.columns[1068].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[48].into_i32().copied().unwrap(),
                    self.row.columns[288].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[168].into_i32().copied().unwrap(),
                    self.row.columns[408].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[528].into_i32().copied().unwrap(),
                    self.row.columns[768].into_i32().copied().unwrap(),
                    self.row.columns[1008].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1248].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1308].into_i32().copied().unwrap(),
                    self.row.columns[1548].into_i32().copied().unwrap(),
                    self.row.columns[1608].into_i32().copied().unwrap(),
                    self.row.columns[1728].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1788].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1908].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[708].into_u16().copied().unwrap(),
                    self.row.columns[948].into_u16().copied().unwrap(),
                    self.row.columns[1188].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2028].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[648].into_u8().copied().unwrap(),
                    self.row.columns[888].into_u8().copied().unwrap(),
                    self.row.columns[1128].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1368].into_u8().copied().unwrap(),
                    self.row.columns[1428].into_u8().copied().unwrap(),
                    self.row.columns[1488].into_u8().copied().unwrap(),
                    self.row.columns[1668].into_u8().copied().unwrap(),
                    self.row.columns[1848].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1968].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[228].into_bool().copied().unwrap(),
                    self.row.columns[468].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[109].into_u32().copied().unwrap(),
                    self.row.columns[349].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[589].into_u32().copied().unwrap(),
                    self.row.columns[829].into_u32().copied().unwrap(),
                    self.row.columns[1069].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[49].into_i32().copied().unwrap(),
                    self.row.columns[289].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[169].into_i32().copied().unwrap(),
                    self.row.columns[409].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[529].into_i32().copied().unwrap(),
                    self.row.columns[769].into_i32().copied().unwrap(),
                    self.row.columns[1009].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1249].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1309].into_i32().copied().unwrap(),
                    self.row.columns[1549].into_i32().copied().unwrap(),
                    self.row.columns[1609].into_i32().copied().unwrap(),
                    self.row.columns[1729].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1789].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1909].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[709].into_u16().copied().unwrap(),
                    self.row.columns[949].into_u16().copied().unwrap(),
                    self.row.columns[1189].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2029].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[649].into_u8().copied().unwrap(),
                    self.row.columns[889].into_u8().copied().unwrap(),
                    self.row.columns[1129].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1369].into_u8().copied().unwrap(),
                    self.row.columns[1429].into_u8().copied().unwrap(),
                    self.row.columns[1489].into_u8().copied().unwrap(),
                    self.row.columns[1669].into_u8().copied().unwrap(),
                    self.row.columns[1849].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1969].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[229].into_bool().copied().unwrap(),
                    self.row.columns[469].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[110].into_u32().copied().unwrap(),
                    self.row.columns[350].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[590].into_u32().copied().unwrap(),
                    self.row.columns[830].into_u32().copied().unwrap(),
                    self.row.columns[1070].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[50].into_i32().copied().unwrap(),
                    self.row.columns[290].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[170].into_i32().copied().unwrap(),
                    self.row.columns[410].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[530].into_i32().copied().unwrap(),
                    self.row.columns[770].into_i32().copied().unwrap(),
                    self.row.columns[1010].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1250].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1310].into_i32().copied().unwrap(),
                    self.row.columns[1550].into_i32().copied().unwrap(),
                    self.row.columns[1610].into_i32().copied().unwrap(),
                    self.row.columns[1730].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1790].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1910].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[710].into_u16().copied().unwrap(),
                    self.row.columns[950].into_u16().copied().unwrap(),
                    self.row.columns[1190].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2030].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[650].into_u8().copied().unwrap(),
                    self.row.columns[890].into_u8().copied().unwrap(),
                    self.row.columns[1130].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1370].into_u8().copied().unwrap(),
                    self.row.columns[1430].into_u8().copied().unwrap(),
                    self.row.columns[1490].into_u8().copied().unwrap(),
                    self.row.columns[1670].into_u8().copied().unwrap(),
                    self.row.columns[1850].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1970].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[230].into_bool().copied().unwrap(),
                    self.row.columns[470].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[111].into_u32().copied().unwrap(),
                    self.row.columns[351].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[591].into_u32().copied().unwrap(),
                    self.row.columns[831].into_u32().copied().unwrap(),
                    self.row.columns[1071].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[51].into_i32().copied().unwrap(),
                    self.row.columns[291].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[171].into_i32().copied().unwrap(),
                    self.row.columns[411].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[531].into_i32().copied().unwrap(),
                    self.row.columns[771].into_i32().copied().unwrap(),
                    self.row.columns[1011].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1251].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1311].into_i32().copied().unwrap(),
                    self.row.columns[1551].into_i32().copied().unwrap(),
                    self.row.columns[1611].into_i32().copied().unwrap(),
                    self.row.columns[1731].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1791].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1911].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[711].into_u16().copied().unwrap(),
                    self.row.columns[951].into_u16().copied().unwrap(),
                    self.row.columns[1191].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2031].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[651].into_u8().copied().unwrap(),
                    self.row.columns[891].into_u8().copied().unwrap(),
                    self.row.columns[1131].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1371].into_u8().copied().unwrap(),
                    self.row.columns[1431].into_u8().copied().unwrap(),
                    self.row.columns[1491].into_u8().copied().unwrap(),
                    self.row.columns[1671].into_u8().copied().unwrap(),
                    self.row.columns[1851].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1971].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[231].into_bool().copied().unwrap(),
                    self.row.columns[471].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[112].into_u32().copied().unwrap(),
                    self.row.columns[352].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[592].into_u32().copied().unwrap(),
                    self.row.columns[832].into_u32().copied().unwrap(),
                    self.row.columns[1072].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[52].into_i32().copied().unwrap(),
                    self.row.columns[292].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[172].into_i32().copied().unwrap(),
                    self.row.columns[412].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[532].into_i32().copied().unwrap(),
                    self.row.columns[772].into_i32().copied().unwrap(),
                    self.row.columns[1012].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1252].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1312].into_i32().copied().unwrap(),
                    self.row.columns[1552].into_i32().copied().unwrap(),
                    self.row.columns[1612].into_i32().copied().unwrap(),
                    self.row.columns[1732].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1792].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1912].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[712].into_u16().copied().unwrap(),
                    self.row.columns[952].into_u16().copied().unwrap(),
                    self.row.columns[1192].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2032].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[652].into_u8().copied().unwrap(),
                    self.row.columns[892].into_u8().copied().unwrap(),
                    self.row.columns[1132].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1372].into_u8().copied().unwrap(),
                    self.row.columns[1432].into_u8().copied().unwrap(),
                    self.row.columns[1492].into_u8().copied().unwrap(),
                    self.row.columns[1672].into_u8().copied().unwrap(),
                    self.row.columns[1852].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1972].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[232].into_bool().copied().unwrap(),
                    self.row.columns[472].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[113].into_u32().copied().unwrap(),
                    self.row.columns[353].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[593].into_u32().copied().unwrap(),
                    self.row.columns[833].into_u32().copied().unwrap(),
                    self.row.columns[1073].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[53].into_i32().copied().unwrap(),
                    self.row.columns[293].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[173].into_i32().copied().unwrap(),
                    self.row.columns[413].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[533].into_i32().copied().unwrap(),
                    self.row.columns[773].into_i32().copied().unwrap(),
                    self.row.columns[1013].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1253].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1313].into_i32().copied().unwrap(),
                    self.row.columns[1553].into_i32().copied().unwrap(),
                    self.row.columns[1613].into_i32().copied().unwrap(),
                    self.row.columns[1733].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1793].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1913].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[713].into_u16().copied().unwrap(),
                    self.row.columns[953].into_u16().copied().unwrap(),
                    self.row.columns[1193].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2033].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[653].into_u8().copied().unwrap(),
                    self.row.columns[893].into_u8().copied().unwrap(),
                    self.row.columns[1133].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1373].into_u8().copied().unwrap(),
                    self.row.columns[1433].into_u8().copied().unwrap(),
                    self.row.columns[1493].into_u8().copied().unwrap(),
                    self.row.columns[1673].into_u8().copied().unwrap(),
                    self.row.columns[1853].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1973].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[233].into_bool().copied().unwrap(),
                    self.row.columns[473].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[114].into_u32().copied().unwrap(),
                    self.row.columns[354].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[594].into_u32().copied().unwrap(),
                    self.row.columns[834].into_u32().copied().unwrap(),
                    self.row.columns[1074].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[54].into_i32().copied().unwrap(),
                    self.row.columns[294].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[174].into_i32().copied().unwrap(),
                    self.row.columns[414].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[534].into_i32().copied().unwrap(),
                    self.row.columns[774].into_i32().copied().unwrap(),
                    self.row.columns[1014].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1254].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1314].into_i32().copied().unwrap(),
                    self.row.columns[1554].into_i32().copied().unwrap(),
                    self.row.columns[1614].into_i32().copied().unwrap(),
                    self.row.columns[1734].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1794].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1914].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[714].into_u16().copied().unwrap(),
                    self.row.columns[954].into_u16().copied().unwrap(),
                    self.row.columns[1194].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2034].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[654].into_u8().copied().unwrap(),
                    self.row.columns[894].into_u8().copied().unwrap(),
                    self.row.columns[1134].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1374].into_u8().copied().unwrap(),
                    self.row.columns[1434].into_u8().copied().unwrap(),
                    self.row.columns[1494].into_u8().copied().unwrap(),
                    self.row.columns[1674].into_u8().copied().unwrap(),
                    self.row.columns[1854].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1974].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[234].into_bool().copied().unwrap(),
                    self.row.columns[474].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[115].into_u32().copied().unwrap(),
                    self.row.columns[355].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[595].into_u32().copied().unwrap(),
                    self.row.columns[835].into_u32().copied().unwrap(),
                    self.row.columns[1075].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[55].into_i32().copied().unwrap(),
                    self.row.columns[295].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[175].into_i32().copied().unwrap(),
                    self.row.columns[415].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[535].into_i32().copied().unwrap(),
                    self.row.columns[775].into_i32().copied().unwrap(),
                    self.row.columns[1015].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1255].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1315].into_i32().copied().unwrap(),
                    self.row.columns[1555].into_i32().copied().unwrap(),
                    self.row.columns[1615].into_i32().copied().unwrap(),
                    self.row.columns[1735].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1795].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1915].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[715].into_u16().copied().unwrap(),
                    self.row.columns[955].into_u16().copied().unwrap(),
                    self.row.columns[1195].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2035].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[655].into_u8().copied().unwrap(),
                    self.row.columns[895].into_u8().copied().unwrap(),
                    self.row.columns[1135].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1375].into_u8().copied().unwrap(),
                    self.row.columns[1435].into_u8().copied().unwrap(),
                    self.row.columns[1495].into_u8().copied().unwrap(),
                    self.row.columns[1675].into_u8().copied().unwrap(),
                    self.row.columns[1855].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1975].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[235].into_bool().copied().unwrap(),
                    self.row.columns[475].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[116].into_u32().copied().unwrap(),
                    self.row.columns[356].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[596].into_u32().copied().unwrap(),
                    self.row.columns[836].into_u32().copied().unwrap(),
                    self.row.columns[1076].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[56].into_i32().copied().unwrap(),
                    self.row.columns[296].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[176].into_i32().copied().unwrap(),
                    self.row.columns[416].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[536].into_i32().copied().unwrap(),
                    self.row.columns[776].into_i32().copied().unwrap(),
                    self.row.columns[1016].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1256].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1316].into_i32().copied().unwrap(),
                    self.row.columns[1556].into_i32().copied().unwrap(),
                    self.row.columns[1616].into_i32().copied().unwrap(),
                    self.row.columns[1736].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1796].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1916].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[716].into_u16().copied().unwrap(),
                    self.row.columns[956].into_u16().copied().unwrap(),
                    self.row.columns[1196].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2036].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[656].into_u8().copied().unwrap(),
                    self.row.columns[896].into_u8().copied().unwrap(),
                    self.row.columns[1136].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1376].into_u8().copied().unwrap(),
                    self.row.columns[1436].into_u8().copied().unwrap(),
                    self.row.columns[1496].into_u8().copied().unwrap(),
                    self.row.columns[1676].into_u8().copied().unwrap(),
                    self.row.columns[1856].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1976].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[236].into_bool().copied().unwrap(),
                    self.row.columns[476].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[117].into_u32().copied().unwrap(),
                    self.row.columns[357].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[597].into_u32().copied().unwrap(),
                    self.row.columns[837].into_u32().copied().unwrap(),
                    self.row.columns[1077].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[57].into_i32().copied().unwrap(),
                    self.row.columns[297].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[177].into_i32().copied().unwrap(),
                    self.row.columns[417].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[537].into_i32().copied().unwrap(),
                    self.row.columns[777].into_i32().copied().unwrap(),
                    self.row.columns[1017].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1257].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1317].into_i32().copied().unwrap(),
                    self.row.columns[1557].into_i32().copied().unwrap(),
                    self.row.columns[1617].into_i32().copied().unwrap(),
                    self.row.columns[1737].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1797].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1917].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[717].into_u16().copied().unwrap(),
                    self.row.columns[957].into_u16().copied().unwrap(),
                    self.row.columns[1197].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2037].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[657].into_u8().copied().unwrap(),
                    self.row.columns[897].into_u8().copied().unwrap(),
                    self.row.columns[1137].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1377].into_u8().copied().unwrap(),
                    self.row.columns[1437].into_u8().copied().unwrap(),
                    self.row.columns[1497].into_u8().copied().unwrap(),
                    self.row.columns[1677].into_u8().copied().unwrap(),
                    self.row.columns[1857].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1977].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[237].into_bool().copied().unwrap(),
                    self.row.columns[477].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[118].into_u32().copied().unwrap(),
                    self.row.columns[358].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[598].into_u32().copied().unwrap(),
                    self.row.columns[838].into_u32().copied().unwrap(),
                    self.row.columns[1078].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[58].into_i32().copied().unwrap(),
                    self.row.columns[298].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[178].into_i32().copied().unwrap(),
                    self.row.columns[418].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[538].into_i32().copied().unwrap(),
                    self.row.columns[778].into_i32().copied().unwrap(),
                    self.row.columns[1018].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1258].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1318].into_i32().copied().unwrap(),
                    self.row.columns[1558].into_i32().copied().unwrap(),
                    self.row.columns[1618].into_i32().copied().unwrap(),
                    self.row.columns[1738].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1798].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1918].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[718].into_u16().copied().unwrap(),
                    self.row.columns[958].into_u16().copied().unwrap(),
                    self.row.columns[1198].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2038].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[658].into_u8().copied().unwrap(),
                    self.row.columns[898].into_u8().copied().unwrap(),
                    self.row.columns[1138].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1378].into_u8().copied().unwrap(),
                    self.row.columns[1438].into_u8().copied().unwrap(),
                    self.row.columns[1498].into_u8().copied().unwrap(),
                    self.row.columns[1678].into_u8().copied().unwrap(),
                    self.row.columns[1858].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1978].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[238].into_bool().copied().unwrap(),
                    self.row.columns[478].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[119].into_u32().copied().unwrap(),
                    self.row.columns[359].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[599].into_u32().copied().unwrap(),
                    self.row.columns[839].into_u32().copied().unwrap(),
                    self.row.columns[1079].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[59].into_i32().copied().unwrap(),
                    self.row.columns[299].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[179].into_i32().copied().unwrap(),
                    self.row.columns[419].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[539].into_i32().copied().unwrap(),
                    self.row.columns[779].into_i32().copied().unwrap(),
                    self.row.columns[1019].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1259].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1319].into_i32().copied().unwrap(),
                    self.row.columns[1559].into_i32().copied().unwrap(),
                    self.row.columns[1619].into_i32().copied().unwrap(),
                    self.row.columns[1739].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1799].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1919].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[719].into_u16().copied().unwrap(),
                    self.row.columns[959].into_u16().copied().unwrap(),
                    self.row.columns[1199].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2039].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[659].into_u8().copied().unwrap(),
                    self.row.columns[899].into_u8().copied().unwrap(),
                    self.row.columns[1139].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1379].into_u8().copied().unwrap(),
                    self.row.columns[1439].into_u8().copied().unwrap(),
                    self.row.columns[1499].into_u8().copied().unwrap(),
                    self.row.columns[1679].into_u8().copied().unwrap(),
                    self.row.columns[1859].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1979].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[239].into_bool().copied().unwrap(),
                    self.row.columns[479].into_bool().copied().unwrap(),
                ],
            },
            ItemElement {
                ReceiveCount: [
                    self.row.columns[120].into_u32().copied().unwrap(),
                    self.row.columns[360].into_u32().copied().unwrap(),
                ],
                CurrencyCost: [
                    self.row.columns[600].into_u32().copied().unwrap(),
                    self.row.columns[840].into_u32().copied().unwrap(),
                    self.row.columns[1080].into_u32().copied().unwrap(),
                ],
                Item: [
                    self.row.columns[60].into_i32().copied().unwrap(),
                    self.row.columns[300].into_i32().copied().unwrap(),
                ],
                Category: [
                    self.row.columns[180].into_i32().copied().unwrap(),
                    self.row.columns[420].into_i32().copied().unwrap(),
                ],
                ItemCost: [
                    self.row.columns[540].into_i32().copied().unwrap(),
                    self.row.columns[780].into_i32().copied().unwrap(),
                    self.row.columns[1020].into_i32().copied().unwrap(),
                ],
                Quest: self.row.columns[1260].into_i32().copied().unwrap(),
                Unknown0: [
                    self.row.columns[1320].into_i32().copied().unwrap(),
                    self.row.columns[1560].into_i32().copied().unwrap(),
                    self.row.columns[1620].into_i32().copied().unwrap(),
                    self.row.columns[1740].into_i32().copied().unwrap(),
                ],
                AchievementUnlock: self.row.columns[1800].into_i32().copied().unwrap(),
                Unknown2: self.row.columns[1920].into_i32().copied().unwrap(),
                CollectabilityCost: [
                    self.row.columns[720].into_u16().copied().unwrap(),
                    self.row.columns[960].into_u16().copied().unwrap(),
                    self.row.columns[1200].into_u16().copied().unwrap(),
                ],
                PatchNumber: self.row.columns[2040].into_u16().copied().unwrap(),
                HqCost: [
                    self.row.columns[660].into_u8().copied().unwrap(),
                    self.row.columns[900].into_u8().copied().unwrap(),
                    self.row.columns[1140].into_u8().copied().unwrap(),
                ],
                Unknown1: [
                    self.row.columns[1380].into_u8().copied().unwrap(),
                    self.row.columns[1440].into_u8().copied().unwrap(),
                    self.row.columns[1500].into_u8().copied().unwrap(),
                    self.row.columns[1680].into_u8().copied().unwrap(),
                    self.row.columns[1860].into_u8().copied().unwrap(),
                ],
                Order: self.row.columns[1980].into_u8().copied().unwrap(),
                ReceiveHq: [
                    self.row.columns[240].into_bool().copied().unwrap(),
                    self.row.columns[480].into_bool().copied().unwrap(),
                ],
            },
        ]
    }
    pub fn Quest(&'a self) -> u32 {
        self.row.columns[2042].into_u32().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> u32 {
        self.row.columns[2045].into_u32().copied().unwrap()
    }
    pub fn RequiredContentFinderCondition(&'a self) -> u32 {
        self.row.columns[2049].into_u32().copied().unwrap()
    }
    pub fn CompleteText(&'a self) -> i32 {
        self.row.columns[2043].into_i32().copied().unwrap()
    }
    pub fn NotCompleteText(&'a self) -> i32 {
        self.row.columns[2044].into_i32().copied().unwrap()
    }
    pub fn RequiredFestival(&'a self) -> u16 {
        self.row.columns[2047].into_u16().copied().unwrap()
    }
    pub fn RequiredFestivalPhase(&'a self) -> u16 {
        self.row.columns[2048].into_u16().copied().unwrap()
    }
    pub fn UseCurrencyType(&'a self) -> u8 {
        self.row.columns[2041].into_u8().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> bool {
        self.row.columns[2046].into_bool().copied().unwrap()
    }
    /// If this is true, then the CFC needs to be completed; If this is false, then the CFC just needs to be unlocked
    pub fn RequiredContentFinderConditionComplete(&'a self) -> bool {
        self.row.columns[2050].into_bool().copied().unwrap()
    }
    pub fn Unknown4(&'a self) -> bool {
        self.row.columns[2051].into_bool().copied().unwrap()
    }
}
