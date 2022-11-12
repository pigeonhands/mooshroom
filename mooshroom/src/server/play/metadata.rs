use mooshroom_core::{
    io::{MooshroomReadProto, MooshroomReadable, MooshroomWritable, MooshroomWriteProto},
    primitives::Identifier,
    varint::VarInt,
};
use mooshroom_macros::Mooshroom;

use super::crafting::Slot;
use crate::core::error::Result;

pub type Ingredient = Vec<Slot>;
pub type Ingredients = Vec<Ingredient>;

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct Node {
    pub flag: u8,
    pub children: Vec<VarInt>,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x0f)]
pub struct Commands {
    pub motd: Option<String>,
    pub icon: Option<String>,
    pub previews_chat: bool,
    pub enforce_secure_chat: bool,
    //TODO
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x20)]
pub struct KeepAlive(pub i64);

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x42)]
pub struct ServerData {
    pub motd: Option<String>,
    pub icon: Option<String>,
    pub previews_chat: bool,
    pub enforce_secure_chat: bool,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x50)]
pub struct SetEntityMetadata {
    pub entity_id: VarInt,
    // TODO
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct RecipeWithExp {
    pub group: String,
    pub ingredient: Ingredient,
    pub result: Slot,
    pub experience: f32,
    pub cooking_time: VarInt,
}

#[derive(Debug, Clone, Default)]
pub enum RecipeData {
    CraftingShapeless {
        group: String,
        ingredients: Ingredients,
        result: Slot,
    },
    CraftingShaped {
        width: VarInt,
        height: VarInt,
        group: String,
        ingredients: Ingredients,
        result: Slot,
    },
    CraftingSpecialArmorDye,
    CraftingSpecialBookCloning,
    CraftingSpecialMapCloning,
    CraftingSpecialMapExtended,
    CraftingSpecialFireworkRocket,
    CraftingSpecialFireworkStar,
    CraftingSpecialFireworkStarFade,
    CraftingSpecialRepairItem,
    CraftingSpecialTippedArrow,
    CraftingSpecialBannerDuplicate,
    CraftingSpecialBannerAddPattern,
    CraftingSpecialShieldECoration,
    CraftingSpecialShulkerBoxColoring,
    CraftingSpecialSuspiciousStew,
    Smelting(RecipeWithExp),
    Blasting(RecipeWithExp),
    Smoking(RecipeWithExp),
    CampfireCooking(RecipeWithExp),
    StoneCutting {
        group: String,
        ingredients: Ingredient,
        result: Slot,
    },
    Smithing {
        base: Ingredient,
        addition: Ingredient,
        result: Slot,
    },
    #[default]
    Unknown,
}

impl RecipeData {
    pub fn read_crafting_shapeless<const PV: usize>(
        reader: &mut impl std::io::Read,
    ) -> Result<Self> {
        Ok(Self::CraftingShapeless {
            group: String::read_proto::<PV>(reader)?,
            ingredients: Ingredients::read_proto::<PV>(reader)?,
            result: Slot::read_proto::<PV>(reader)?,
        })
    }
    pub fn read_crafting_shaped<const PV: usize>(
        mut reader: &mut impl std::io::Read,
    ) -> Result<Self> {
        let width = VarInt::read_proto::<PV>(reader)?;
        let height = VarInt::read_proto::<PV>(reader)?;
        let group = String::read_proto::<PV>(reader)?;
        let ingredients = {
            let mut ing = Ingredients::new();
            for _ in 0..(width.0 * height.0) {
                ing.push(Ingredient::read_proto::<PV>(reader)?);
            }
            ing
        };
        Ok(Self::CraftingShaped {
            width,
            height,
            group,
            ingredients,
            result: Slot::read_proto::<PV>(&mut reader)?,
        })
    }

    pub fn read_stone_cutting<const PV: usize>(reader: &mut impl std::io::Read) -> Result<Self> {
        Ok(Self::StoneCutting {
            group: String::read_proto::<PV>(reader)?,
            ingredients: Ingredient::read_proto::<PV>(reader)?,
            result: Slot::read_proto::<PV>(reader)?,
        })
    }
    pub fn read_smithing<const PV: usize>(reader: &mut impl std::io::Read) -> Result<Self> {
        Ok(Self::Smithing {
            base: Ingredient::read_proto::<PV>(reader)?,
            addition: Ingredient::read_proto::<PV>(reader)?,
            result: Slot::read_proto::<PV>(reader)?,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Recipe {
    pub recipe_type: Identifier,
    pub recipe_id: Identifier,
    pub data: RecipeData,
}

impl<const PV: usize> MooshroomReadable<PV> for Recipe {
    fn read(reader: &mut impl std::io::Read) -> Result<Self> {
        let recipe_type = String::read_proto::<PV>(reader)?;
        let recipe_id = String::read_proto::<PV>(reader)?;

        let data = match recipe_type.as_str() {
            "minecraft:crafting_shapeless" => RecipeData::read_crafting_shapeless::<PV>(reader)?,
            "minecraft:crafting_shaped" => RecipeData::read_crafting_shaped::<PV>(reader)?,
            "minecraft:crafting_special_armordye" => RecipeData::CraftingSpecialArmorDye,
            "minecraft:crafting_special_bookcloning" => RecipeData::CraftingSpecialBookCloning,
            "minecraft:crafting_special_mapcloning" => RecipeData::CraftingSpecialMapCloning,
            "minecraft:crafting_special_mapextending" => RecipeData::CraftingSpecialMapExtended,
            "minecraft:crafting_special_firework_rocket" => {
                RecipeData::CraftingSpecialFireworkRocket
            }
            "minecraft:crafting_special_firework_star" => RecipeData::CraftingSpecialFireworkStar,
            "minecraft:crafting_special_firework_star_fade" => {
                RecipeData::CraftingSpecialFireworkStarFade
            }
            "minecraft:crafting_special_repairitem" => RecipeData::CraftingSpecialRepairItem,
            "minecraft:crafting_special_tippedarrow" => RecipeData::CraftingSpecialTippedArrow,
            "minecraft:crafting_special_bannerduplicate" => {
                RecipeData::CraftingSpecialBannerDuplicate
            }
            "minecraft:crafting_special_banneraddpattern" => {
                RecipeData::CraftingSpecialBannerAddPattern
            }
            "minecraft:crafting_special_shielddecoration" => {
                RecipeData::CraftingSpecialShieldECoration
            }
            "minecraft:crafting_special_shulkerboxcoloring" => {
                RecipeData::CraftingSpecialShulkerBoxColoring
            }
            "minecraft:crafting_special_suspiciousstew" => {
                RecipeData::CraftingSpecialSuspiciousStew
            }
            "minecraft:smelting" => RecipeData::Smelting(RecipeWithExp::read_proto::<PV>(reader)?),
            "minecraft:blasting" => RecipeData::Blasting(RecipeWithExp::read_proto::<PV>(reader)?),
            "minecraft:smoking" => RecipeData::Smoking(RecipeWithExp::read_proto::<PV>(reader)?),
            "minecraft:campfire_cooking" => {
                RecipeData::CampfireCooking(RecipeWithExp::read_proto::<PV>(reader)?)
            }
            "minecraft:stonecutting" => RecipeData::read_stone_cutting::<PV>(reader)?,
            "minecraft:smithing" => RecipeData::read_smithing::<PV>(reader)?,
            _ => RecipeData::Unknown,
        };

        Ok(Self {
            recipe_type,
            recipe_id,
            data,
        })
    }
}

impl<const PV: usize> MooshroomWritable<PV> for Recipe {
    fn write(&self, writer: &mut impl std::io::Write) -> Result<()> {
        self.recipe_type.write_proto::<PV>(writer)?;
        self.recipe_id.write_proto::<PV>(writer)?;
        match &self.data {
            RecipeData::CraftingShapeless {
                group,
                ingredients,
                result,
            } => {
                group.write_proto::<PV>(writer)?;
                ingredients.write_proto::<PV>(writer)?;
                result.write_proto::<PV>(writer)?;
            }
            RecipeData::CraftingShaped {
                width,
                height,
                group,
                ingredients,
                result,
            } => {
                width.write_proto::<PV>(writer)?;
                height.write_proto::<PV>(writer)?;
                group.write_proto::<PV>(writer)?;
                ingredients.write_proto::<PV>(writer)?;
                result.write_proto::<PV>(writer)?;
            }
            RecipeData::Smelting(d)
            | RecipeData::Blasting(d)
            | RecipeData::Smoking(d)
            | RecipeData::CampfireCooking(d) => d.write_proto::<PV>(writer)?,
            RecipeData::StoneCutting {
                group,
                ingredients,
                result,
            } => {
                group.write_proto::<PV>(writer)?;
                ingredients.write_proto::<PV>(writer)?;
                result.write_proto::<PV>(writer)?;
            }
            RecipeData::Smithing {
                base,
                addition,
                result,
            } => {
                base.write_proto::<PV>(writer)?;
                addition.write_proto::<PV>(writer)?;
                result.write_proto::<PV>(writer)?;
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x6A)]
pub struct UpdateRecipies(Vec<Recipe>);

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct Tag {
    pub name: Identifier,
    pub entries: Vec<VarInt>,
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct GroupedTag {
    pub tag_type: Identifier,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x6B)]
pub struct UpdateTags(Vec<GroupedTag>);

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x3a)]
pub struct UpdateRecipeBook {
    //TODO
}
