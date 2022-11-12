pub mod crafting;
pub mod metadata;
pub mod nbt;
pub mod player;
pub mod population;
pub mod world;
use mooshroom_macros::MooshroomCollection;

#[derive(Debug, Clone, MooshroomCollection)]
pub enum PlayStage {
    Login(world::LoginPlay),
    ChangeDifficulty(world::ChangeDifficulty),
    BlockUpdate(world::BlockUpdate),
    EntityEvent(world::EntityEvent),
    SetRenderDistance(world::SetRenderDistance),
    SetSimulationDistance(world::SetSimulationDistance),
    SetCenterChunk(world::SetCenterChunk),
    InitializeWorldBorder(world::InitializeWorldBorder),
    UpdateTime(world::UpdateTime),
    ChunkData(world::ChunkData),
    WorldEvent(world::WorldEvent),
    UpdateLight(world::UpdateLight),
    SoundEffect(world::SoundEffect),
    UpdateSectionBlocks(world::UpdateSectionBlocks),
    Particle(world::Particle),
    GameEvent(world::GameEvent),
    PlayerChatMessage(world::PlayerChatMessage),

    Disconnect(player::Disconnect),
    SetPlayerRotation(player::SetPlayerRotation),
    PlayerAbilities(player::PlayerAbilities),
    SetHeldItem(player::SetHeldItem),
    SynchronizePlayerPosition(player::SynchronizePlayerPosition),
    SetDefaultSpawnPosition(player::SetDefaultSpawnPosition),
    UpdateAdvancements(player::UpdateAdvancements),
    SetExperience(player::SetExperience),
    SetHealth(player::SetHealth),
    EndCombat(player::EndCombat),
    EnterCombat(player::EnterCombat),
    Respawn(player::Respawn),
    CombatDeath(player::CombatDeath),

    SpawnEntity(population::SpawnEntity),
    SpawnPlayer(population::SpawnPlayer),
    EntityAnimation(population::EntityAnimation),
    SystemChatMessage(population::SystemChatMessage),
    PlayerInfo(population::PlayerInfo),
    UpdateAttributes(population::UpdateAttributes),
    SetHeadRotation(population::SetHeadRotation),
    SetEquipment(population::SetEquipment),
    SetEntityVelocity(population::SetEntityVelocity),
    UpdateEntityPosition(population::UpdateEntityPosition),
    UpdateEntityPositionAndRotation(population::UpdateEntityPositionAndRotation),
    UpdateEntityRotation(population::UpdateEntityRotation),
    RemoveEntity(population::RemoveEntity),
    TeleportEntity(population::TeleportEntity),
    SetPassengers(population::SetPassengers),
    LinkEntities(population::LinkEntities),
    EntityEffect(population::EntityEffect),

    CloseContainer(crafting::CloseContainer),
    SetContainerContent(crafting::SetContainerContent),

    KeepAlive(metadata::KeepAlive),
    UpdateRecipies(metadata::UpdateRecipies),
    UpdateTags(metadata::UpdateTags),
    UpdateRecipeBook(metadata::UpdateRecipeBook),
    SetEntityMetadata(metadata::SetEntityMetadata),
    ServerData(metadata::ServerData),
    Commands(metadata::Commands),
}
