pub mod crafting;
pub mod npt;
pub mod player;
pub mod world;
pub mod metadata;
pub mod population;
use mooshroom_macros::MooshroomCollection;


#[derive(Debug, Clone, MooshroomCollection)]
pub enum PlayStage {
    Login(world::LoginPlay),
    SetDifficulty(world::ChangeDifficulty),
    EntityEvent(world::EntityEvent),
    SetPlayerRotation(player::SetPlayerRotation),
    PlayerAbilities(player::PlayerAbilities),
    SetHeldItem(player::SetHeldItem),
    SynchronizePlayerPosition(player::SynchronizePlayerPosition),
    SystemChatMessage(population::SystemChatMessage),
    PlayerInfo(population::PlayerInfo),
    SetContainerContent(crafting::SetContainerContent),
    UpdateRecipies(metadata::UpdateRecipies),
    UpdateTags(metadata::UpdateTags),
    UpdateRecipeBook(metadata::UpdateRecipeBook)
}
