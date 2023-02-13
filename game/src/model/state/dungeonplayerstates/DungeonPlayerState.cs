namespace DarkAdapters.model.state.dungeonplayerstates
{
    public enum DungeonPlayerState
    {
        Default, // the default state, when rhea is standing still 
        Walking, // when moving from one tile to another
        Running, // when running. the camera will probs be locked in this state
        Rotating, // when rotating the camera
        Scanning // when scanning 
    }
}