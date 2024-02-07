pub mod crafting;
pub mod entity;
pub mod metadata;
pub mod nbt;
pub mod player;
pub mod population;
pub mod world;
use mooshroom_macros::MooshroomCollection;

#[derive(Debug, Clone, MooshroomCollection)]
pub enum PlayStage {
    #[id(0x00)]
    SpawnEntity(population::SpawnEntity),
    #[id(0x01)]
    SpawnExperienceOrb(entity::SpawnExperienceOrb),
    #[id(0x02)]
    SpawnPlayer(population::SpawnPlayer),
    #[id(0x03)]
    EntityAnimation(population::EntityAnimation),
    //#[id(0x04)]
    //AwardStatistics(player::AwardStatistics),
    //#[id(0x05)]
    //AcknowledgeBlockChange(world::AcknowledgeBlockChange),
    //#[id(0x06)]
    //SetBlockDestroyStage(world::SetBlockDestroyStage),
    //#[id(0x07)]
    //BlockEntityData(world::BlockEntityData),
    //#[id(0x08)]
    //BlockAction(world::BlockAction),
    #[id(0x09)]
    BlockUpdate(world::BlockUpdate),
    //#[id(0x0A)]
    //BossBar(population::BossBar),
    #[id(0x0b)]
    ChangeDifficulty(world::ChangeDifficulty),
    //#[id(0x0C)]
    //ChatPreview(world::ChatPreview),
    //#[id(0x0D)]
    //ClearTitles(world::ClearTitles),
    // #[id(0x0E)]
    // CommandSuggestionsResponse(world::ClearTitles),
    #[id(0x0F)]
    Commands(world::Commands),
    #[id(0x10)]
    CloseContainer(crafting::CloseContainer),
    #[id(0x11)]
    SetContainerContent(crafting::SetContainerContent),
    // #[id(0x12)]
    // SetContainerProperty(crafting::SetContainerProperty),
    #[id(0x13)]
    SetContainerSlot(crafting::SetContainerSlot),
    // #[id(0x14)]
    // SetCooldown(crafting::SetCooldown),
    // #[id(0x15)]
    // ChatSuggestion(crafting::ChatSuggestion),
    #[id(0x16)]
    PluginMessage(metadata::PluginMessage),
    // #[id(0x17)]
    // CustomSoundEffect(world::CustomSoundEffect),
    // #[id(0x18)]
    // HideMessage(world::HideMessage),
    #[id(0x19)]
    Disconnect(player::Disconnect),
    #[id(0x1A)]
    EntityEvent(population::EntityEvent),
    //#[id(0x1B)]
    //Explosion(world::Explosion),
    #[id(0x1B)]
    UnloadChunk(world::UnloadChunk),
    //#[id(0x1C)]
    //GameEvent(world::GameEvent),
    //#[id(0x1d)] // Server bound packet
    //PlayerCommand(player::PlayerCommand),
    #[id(0x1D)]
    OpenHorseScreen(crafting::OpenHorseScreen),
    #[id(0x1F)]
    InitializeWorldBorder(world::InitializeWorldBorder),
    #[id(0x20)]
    KeepAlive(metadata::KeepAlive),
    #[id(0x21)]
    ChunkData(world::ChunkData),
    #[id(0x22)]
    WorldEvent(world::WorldEvent),
    #[id(0x23)]
    Particle(world::Particle),
    #[id(0x24)]
    UpdateLight(world::UpdateLight),
    #[id(0x25)]
    Login(world::LoginPlay),
    //#[id(0x26)]
    //MapData(crafting::MapData),
    //#[id(0x27)]
    //MerchantOffers(crafting::MerchantOffers),
    #[id(0x28)]
    UpdateEntityPosition(population::UpdateEntityPosition),
    #[id(0x29)]
    UpdateEntityPositionAndRotation(population::UpdateEntityPositionAndRotation),
    #[id(0x2A)]
    UpdateEntityRotation(population::UpdateEntityRotation),
    //v#[id(0x2B)]
    //MoveVehicle(world::MoveVehicle),
    //#[id(0x2C)]
    //OpenBook(crafting::OpenBook),
    //#[id(0x2D)]
    //OpenScreen(crafting::OpenScreen),
    //#[id(0x2E)]
    //OpenSignEditor(crafting::OpenSignEditor),
    //#[id(0x2F)]
    //Ping(metadata::Ping),
    //#[id(0x30)]
    //PlaceGhostRecipe(crafting::PlaceGhostRecipe),
    #[id(0x31)]
    PlayerAbilities(player::PlayerAbilities),
    //#[id(0x32)]
    //MessageHeader(metadata::MessageHeader),
    #[id(0x33)]
    PlayerChatMessage(world::PlayerChatMessage),
    #[id(0x34)]
    EndCombat(player::EndCombat),
    #[id(0x35)]
    EnterCombat(player::EnterCombat),
    #[id(0x36)]
    CombatDeath(player::CombatDeath),
    #[id(0x37)]
    PlayerInfo(population::PlayerInfo),
    //#[id(0x38)]
    //LookAt(player::LookAt),
    #[id(0x39)]
    SynchronizePlayerPosition(player::SynchronizePlayerPosition),
    #[id(0x3A)]
    UpdateRecipeBook(metadata::UpdateRecipeBook),
    #[id(0x3B)]
    RemoveEntities(population::RemoveEntities),
    //#[id(0x3C)]
    //RemoveEntityEffect(population::RemoveEntityEffect),
    //#[id(0x3D)]
    //ResourcePack(metadata::ResourcePack),
    #[id(0x3E)]
    Respawn(player::Respawn),
    #[id(0x3F)]
    SetHeadRotation(population::SetHeadRotation),
    #[id(0x40)]
    UpdateSectionBlocks(world::UpdateSectionBlocks),
    //#[id(0x41)]
    //SelectAdvancementsTabs(crafting::SelectAdvancementsTabs),
    #[id(0x42)]
    ServerData(metadata::ServerData),
    //#[id(0x43)]
    //SetActionBarText(metadata::SetActionBarText),
    //#[id(0x44)]
    //SetBorderCenter(world::SetBorderCenter),
    //#[id(0x45)]
    //SetBorderLerpSize(world::SetBorderLerpSize),
    //#[id(0x46)]
    //SetBorderSize(world::SetBorderSize),
    //#[id(0x47)]
    //SetBorderWarningDelay(world::SetBorderWarningDelay),
    //#[id(0x48)]
    //SetBorderWarningDistance(world::SetBorderWarningDistance),
    //#[id(0x49)]
    //SetCamera(player::SetCamera),
    #[id(0x4A)]
    SetHeldItem(player::SetHeldItem),
    #[id(0x4B)]
    SetCenterChunk(world::SetCenterChunk),
    #[id(0x4C)]
    SetRenderDistance(world::SetRenderDistance),
    #[id(0x4D)]
    SetDefaultSpawnPosition(player::SetDefaultSpawnPosition),
    //#[id(0x4E)]
    //SetDisplayChatPreview(world::SetDisplayChatPreview),
    //#[id(0x4F)]
    //DisplayObjective(world::DisplayObjective),
    #[id(0x50)]
    SetEntityMetadata(metadata::SetEntityMetadata),
    #[id(0x51)]
    LinkEntities(population::LinkEntities),
    #[id(0x52)]
    SetEntityVelocity(population::SetEntityVelocity),
    #[id(0x53)]
    SetEquipment(population::SetEquipment),
    #[id(0x54)]
    SetExperience(player::SetExperience),
    #[id(0x55)]
    SetHealth(player::SetHealth),
    //#[id(0x56)]
    //UpdateObjectives(player::UpdateObjectives),
    #[id(0x57)]
    SetPassengers(population::SetPassengers),
    //#[id(0x58)]
    //UpdateTeams(metadata::UpdateTeams),
    //#[id(0x59)]
    //UpdateScore(metadata::UpdateScore),
    #[id(0x5A)]
    SetSimulationDistance(world::SetSimulationDistance),
    //#[id(0x5B)]
    //SetSubtitleText(metadata::SetSimulationDistance),
    #[id(0x5C)]
    UpdateTime(world::UpdateTime),
    //#[id(0x5D)]
    //SetTitleText(metadata::SetTitleText),
    //#[id(0x5E)]
    //SetTitleAnimationTimes(metadata::SetTitleAnimationTimes),
    //#[id(0x5F)]
    //EntitySoundEffect(population::EntitySoundEffect),
    #[id(0x60)]
    SoundEffect(world::SoundEffect),
    //#[id(0x61)]
    //StopSound(world::StopSound),
    #[id(0x62)]
    SystemChatMessage(world::SystemChatMessage),
    //#[id(0x63)]
    //SetTabListHeaderAndFooter(metadata::SetTabListHeaderAndFooter),
    //#[id(0x64)]
    //TagQueryResponse(metadata::TagQueryResponse),
    //#[id(0x65)]
    //PickupItem(population::PickupItem),
    #[id(0x65)]
     // UpdateAdvancements(player::UpdateAdvancements),
    PickUpItem(population::PickUpItem),
    #[id(0x66)]
    TeleportEntity(population::TeleportEntity),
    #[id(0x67)]
    FeatureFlags(world::FeatureFlags),
    
    #[id(0x68)]
    UpdateAttributes(population::UpdateAttributes),
    #[id(0x69)]
    EntityEffect(population::EntityEffect),
    #[id(0x6A)]
    UpdateRecipies(metadata::UpdateRecipies),
    #[id(0x6B)]
    UpdateTags(metadata::UpdateTags),
    //SetPlayerRotation(player::SetPlayerRotation),
    //
    //EntityAnimation(population::EntityAnimation),
    //RemoveEntity(population::RemoveEntity),
}
