use mooshroom_macros::Mooshroom;
use mooshroom_core::{
    varint::VarInt,
};
#[derive(Debug, Clone, Default, Mooshroom)]
#[repr(i32)]
#[value_type(VarInt)]
pub enum EntityType {
    #[default]    
    #[id("minecraft:allay")]
    Allay = 0,
    #[id("minecraft:area_effect_cloud")]
    AreaEffectCloud = 1,
    #[id("minecraft:armor_stand")]
    ArmorStand= 2,
    #[id("minecraft:arrow")]
    Arrow = 3,
    #[id("minecraft:axolotl")]
    Axolotl = 4,
    #[id("minecraft:bat")]
    Bat = 5,
    #[id("minecraft:bee")]
    Bee = 6,
    #[id("minecraft:blaze")]
    Blaze = 7,
    #[id("minecraft:boat")]
    Boat = 8,
    #[id("minecraft:chest_boat")]
    ChestBoat = 9,
    #[id("minecraft:cat")]
    Cat = 10,
    #[id("minecraft:cave_spider")]
    CaveSpider = 11,
    #[id("minecraft:chicken")]
    Chicken = 12,
    #[id("minecraft:cod")]
    Cod = 13,
    #[id("minecraft:cow")]
    Cow = 14,
    #[id("minecraft:creeper")]
    Creeper = 15,
    #[id("minecraft:dolphin")]
    Dolphin = 16,
    #[id("minecraft:donkey")]
    Donkey = 17,
    #[id("minecraft:dragon_fireball")]
    DragonFireball = 18,
    #[id("minecraft:drowned")]
    Drowned = 19,
    #[id("minecraft:elder_guardian")]
    ElderGuardian = 20,
    #[id("minecraft:end_crystal")]
    EndCrystal = 21,
    #[id("minecraft:ender_dragon")]
    EnderDragon = 22,
    #[id("minecraft:enderman")]
    Enderman = 23,
    #[id("minecraft:endermite")]
    Endermite = 24,
    #[id("minecraft:evoker")]
    Evoker = 25,
    #[id("minecraft:evoker_fangs")]
    EvokerFangs = 26,
    #[id("minecraft:experience_orb")]
    ExperienceOrb = 27,
    #[id("minecraft:eye_of_ender")]
    EyeofEnder = 28,
    #[id("minecraft:falling_block")]
    FallingBlock = 29,
    #[id("minecraft:firework_rocket")]
    FireworkRocketEntity = 30,
    #[id("minecraft:fox")]
    Fox = 31,
    #[id("minecraft:frog")]
    Frog = 32,
    #[id("minecraft:ghast")]
    Ghast = 33,
    #[id("minecraft:giant")]
    Giant = 34,
    #[id("minecraft:glow_item_frame")]
    GlowItemFrame = 35,
    #[id("minecraft:glow_squid")]
    GlowSquid = 36,
    #[id("minecraft:goat")]
    Goat = 37,
    #[id("minecraft:guardian")]
    Guardian = 38,
    #[id("minecraft:hoglin")]
    Hoglin = 39,
    #[id("minecraft:horse")]
    Horse = 40,
    #[id("minecraft:husk")]
    Husk = 41,
    #[id("minecraft:illusioner")]
    Illusioner = 42,
    #[id("minecraft:iron_golem")]
    IronGolem = 43,
    #[id("minecraft:item")]
    Item = 44,
    #[id("minecraft:item_frame")]
    ItemFrame = 45,
    #[id("minecraft:fireball")]
    Fireball = 46,
    #[id("minecraft:leash_knot")]
    LeashKnot = 47,
    #[id("minecraft:lightning_bolt")]
    LightningBolt = 48,
    #[id("minecraft:llama")]
    Llama = 49,
    #[id("minecraft:llama_spit")]
    LlamaSpit = 50,
    #[id("minecraft:magma_cube")]
    MagmaCube = 51,
    #[id("minecraft:marker")]
    Marker = 52,
    #[id("minecraft:minecart")]
    Minecart = 53,
    #[id("minecraft:chest_minecart")]
    MinecartChest = 54,
    #[id("minecraft:commandblock_minecart")]
    MinecartCommandBlock = 55,
    #[id("minecraft:furnace_minecart")]
    MinecartFurnace = 56,
    #[id("minecraft:hopper_minecart")]
    MinecartHopper = 57,
    #[id("minecraft:spawner_minecart")]
    MinecartSpawner = 58,
    #[id("minecraft:tnt_minecart")]
    MinecartTNT = 59,
    #[id("minecraft:mule")]
    Mule = 60,
    #[id("minecraft:mooshroom")]
    Mooshroom = 61,
    #[id("minecraft:ocelot")]
    Ocelot = 62,
    #[id("minecraft:painting")]
    Painting = 63,
    #[id("minecraft:panda")]
    Panda = 64,
    #[id("minecraft:parrot")]
    Parrot = 65,
    #[id("minecraft:phantom")]
    Phantom = 66,
    #[id("minecraft:pig")]
    Pig = 67,
    #[id("minecraft:piglin")]
    Piglin = 68,
    #[id("minecraft:piglin_brute")]
    PiglinBrute = 69,
    #[id("minecraft:pillager")]
    Pillager = 70,
    #[id("minecraft:polar_bear")]
    PolarBear = 71,
    #[id("minecraft:tnt")]
    PrimedTNT = 72,
    #[id("minecraft:pufferfish")]
    Pufferfish = 73,
    #[id("minecraft:rabbit")]
    Rabbit = 74,
    #[id("minecraft:ravager")]
    Ravager = 75,
    #[id("minecraft:salmon")]
    Salmon = 76,
    #[id("minecraft:sheep")]
    Sheep = 77,
    #[id("minecraft:shulker")]
    Shulker = 78,
    #[id("minecraft:shulker_bullet")]
    ShulkerBullet = 79,
    #[id("minecraft:silverfish")]
    Silverfish = 80,
    #[id("minecraft:skeleton")]
    Skeleton = 81,
    #[id("minecraft:skeleton_horse")]
    SkeletonHorse = 82,
    #[id("minecraft:slime")]
    Slime = 83,
    #[id("minecraft:small_fireball")]
    SmallFireball = 84,
    #[id("minecraft:snow_golem")]
    SnowGolem = 85,
    #[id("minecraft:snowball")]
    Snowball = 86,
    #[id("minecraft:spectral_arrow")]
    SpectralArrow = 87,
    #[id("minecraft:spider")]
    Spider = 88,
    #[id("minecraft:squid")]
    Squid = 89,
    #[id("minecraft:stray")]
    Stray = 90,
    #[id("minecraft:strider")]
    Strider = 91,
    #[id("minecraft:tadpole")]
    Tadpole = 92,
    #[id("minecraft:egg")]
    ThrownEgg = 93,
    #[id("minecraft:ender_pearl")]
    ThrownEnderPearl = 94,
    #[id("minecraft:experience_bottle")]
    ThrownExperienceBottle = 95,
    #[id("minecraft:potion")]
    ThrownPotion = 96,
    #[id("minecraft:trident")]
    ThrownTrident = 97,
    #[id("minecraft:trader_llama")]
    TraderLlama = 98,
    #[id("minecraft:tropical_fish")]
    TropicalFish = 99,
    #[id("minecraft:turtle")]
    Turtle = 100,
    #[id("minecraft:vex")]
    Vex = 101,
    #[id("minecraft:villager")]
    Villager = 102,
    #[id("minecraft:vindicator")]
    Vindicator = 103,
    #[id("minecraft:wandering_trader")]
    WanderingTrader = 104,
    #[id("minecraft:warden")]
    Warden = 105,
    #[id("minecraft:witch")]
    Witch = 106,
    #[id("minecraft:wither")]
    Wither = 107,
    #[id("minecraft:wither_skeleton")]
    WitherSkeleton = 108,
    #[id("minecraft:wither_skull")]
    WitherSkull = 109,
    #[id("minecraft:wolf")]
    Wolf = 110,
    #[id("minecraft:zoglin")]
    Zoglin = 111,
    #[id("minecraft:zombie")]
    Zombie = 112,
    #[id("minecraft:zombie_horse")]
    ZombieHorse = 113,
    #[id("minecraft:zombie_villager")]
    ZombieVillager = 114,
    #[id("minecraft:zombified_piglin")]
    ZombifiedPiglin = 115,
    #[id("minecraft:player")]
    Player = 116,
    #[id("minecraft:fishing_bobber")]
    FishingHook = 117,
}

impl EntityType {
    pub fn bounding_box(&self) -> (f32, f32) { //(xy, z)
        match self {
            Self::Allay => (0.6, 0.35),
            Self::AreaEffectCloud => (2.0, 0.5), //2.0* => (Radius, 0.5)
            Self::ArmorStand => (0.5, 1.975), //(normal: 0.5 marker: 0.0 small: 0.25), (normal: 1.975 marker: 0.0 small: 0.9875	)
            Self::Arrow => (0.5, 0.5),
            Self::Axolotl => (1.3, 0.6),
            Self::Bat => (0.5, 0.9),
            Self::Bee => (0.7, 0.6),
            Self::Blaze => (0.6, 1.8),
            Self::Boat => (1.375, 0.5625),
            Self::ChestBoat => (1.375, 0.5625),
            Self::Cat => (0.6, 0.7),
            Self::CaveSpider => (0.7, 0.5),
            Self::Chicken => (0.4, 0.7),
            Self::Cod => (0.5, 0.3),
            Self::Cow => (0.9, 1.4),
            Self::Creeper => (0.6, 1.7),
            Self::Dolphin => (0.9, 0.6),
            Self::Donkey => (1.5, 1.39648),
            Self::DragonFireball => (1.0, 1.0),
            Self::Drowned => (0.6, 1.95),
            Self::ElderGuardian => (1.9975, 1.9975), //1.9975 (2.35 * guardian), 1.9975 (2.35 * guardian)	
            Self::EndCrystal => (2.0, 2.0),
            Self::EnderDragon => (16.0, 8.0),
            Self::Enderman => (0.6, 2.9),
            Self::Endermite => (0.4, 0.3),
            Self::Evoker => (0.6, 1.95),
            Self::EvokerFangs => (0.5, 0.8),
            Self::ExperienceOrb => (0.5, 0.5),
            Self::EyeofEnder => (0.25, 0.25),
            Self::FallingBlock => (0.98, 0.98),
            Self::FireworkRocketEntity => (0.25, 0.25),
            Self::Fox => (0.6, 0.7),
            Self::Frog => (0.5, 0.5),
            Self::Ghast => (4.0, 4.0),
            Self::Giant => (3.6, 12.0),
            Self::GlowItemFrame => (0.75,0.75), //0.75 or 0.0625 (depth), 0.75	
            Self::GlowSquid => (0.8, 0.8),
            Self::Goat => (1.3, 0.9), //1.3 (adult) 0.65 (baby), 0.9 (adult), 0.45 (baby)	
            Self::Guardian => (0.85, 0.85),
            Self::Hoglin => (1.39648, 1.4),
            Self::Horse => (1.39648, 1.6),
            Self::Husk => (0.6, 1.95),
            Self::Illusioner => (0.6, 1.95),
            Self::IronGolem => (1.4, 2.7),
            Self::Item => (0.25, 0.25),
            Self::ItemFrame=> (0.75,0.75), //0.75 or 0.0625 (depth), 0.75	
            Self::Fireball => (1.0, 1.0),
            Self::LeashKnot => (0.375, 0.5),
            Self::LightningBolt => (0.0, 0.0),
            Self::Llama => (0.9, 1.87),
            Self::LlamaSpit => (0.25, 0.25),
            Self::MagmaCube => (0.51000005, 0.51000005), //*size, *size
            Self::Marker => (0.0, 0.0),
            Self::Minecart => (0.98, 0.7),
            Self::MinecartChest => (0.98, 0.7),
            Self::MinecartCommandBlock => (0.98, 0.7),
            Self::MinecartFurnace => (0.98, 0.7),
            Self::MinecartHopper => (0.98, 0.7),
            Self::MinecartSpawner => (0.98, 0.7),
            Self::MinecartTNT => (0.98, 0.7),
            Self::Mule => (1.39648, 1.6),
            Self::Mooshroom => (0.9, 1.4),
            Self::Ocelot => (0.6, 0.7),
            Self::Painting => (1.,1.), //type width or 0.0625 (depth), type height	
            Self::Panda => (1.3, 1.25),
            Self::Parrot => (0.5, 0.9),
            Self::Phantom => (0.9, 0.5),
            Self::Pig => (0.9, 0.9),
            Self::Piglin => (0.6, 1.95),
            Self::PiglinBrute => (0.6, 1.95),
            Self::Pillager => (0.6, 1.95),
            Self::PolarBear => (1.4, 1.4),
            Self::PrimedTNT => (0.98, 0.98),
            Self::Pufferfish => (0.7, 0.7),
            Self::Rabbit => (0.4, 0.5),
            Self::Ravager => (1.95, 2.2),
            Self::Salmon => (0.7, 0.4),
            Self::Sheep => (0.9, 1.3),
            Self::Shulker => (1., 1.),
            Self::ShulkerBullet => (0.3125, 0.3125),
            Self::Silverfish => (0.4, 0.3),
            Self::Skeleton => (0.6, 1.99),
            Self::SkeletonHorse => (1.39648, 1.6),
            Self::Slime => (0.51000005, 0.51000005), // *size, *size
            Self::SmallFireball => (0.3125, 0.3125),
            Self::SnowGolem => (0.7, 1.9),
            Self::Snowball => (0.25, 0.25),
            Self::SpectralArrow => (0.5, 0.5),
            Self::Spider => (1.4, 0.9),
            Self::Squid => (0.8, 0.8),
            Self::Stray => (0.6, 1.99),
            Self::Strider => (0.9, 1.7),
            Self::Tadpole => (0.4, 0.3),
            Self::ThrownEgg => (0.25, 0.25),
            Self::ThrownEnderPearl => (0.25, 0.25),
            Self::ThrownExperienceBottle => (0.25, 0.25),
            Self::ThrownPotion => (0.25, 0.25),
            Self::ThrownTrident => (0.5, 0.5),
            Self::TraderLlama => (0.9, 1.87),
            Self::TropicalFish => (0.5, 0.4),
            Self::Turtle => (1.2, 0.4),
            Self::Vex => (0.4, 0.8),
            Self::Villager => (0.6, 1.95),
            Self::Vindicator => (0.6, 1.95),
            Self::WanderingTrader => (0.6, 1.95),
            Self::Warden => (0.9, 2.9),
            Self::Witch => (0.6, 1.95),
            Self::Wither => (0.9, 3.5),
            Self::WitherSkeleton => (0.7, 2.4),
            Self::WitherSkull => (0.3125, 0.3125),
            Self::Wolf => (0.6, 0.85),
            Self::Zoglin => (1.39648, 1.4),
            Self::Zombie => (0.6, 1.95),
            Self::ZombieHorse => (1.39648, 1.6),
            Self::ZombieVillager => (0.6, 1.95),
            Self::ZombifiedPiglin => (0.6, 1.95),
            Self::Player => (0.6, 1.8),
            Self::FishingHook => (0.25, 0.25),
        }
    }
}