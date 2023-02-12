using System.Collections.Generic;

namespace DarkAdapters.model
{
    public enum ActionNames
    {
        DungeonPlayerForward,
        DungeonPlayerLeft,
        DungeonPlayerRight,
        DungeonPlayerBack
    }

    public static class InputUtils
    {
        public static readonly Dictionary<ActionNames, string> InputStringMap = new Dictionary<ActionNames, string>
        {
            { ActionNames.DungeonPlayerForward, "dungeon_player_forward" },
            { ActionNames.DungeonPlayerLeft, "dungeon_player_left" },
            { ActionNames.DungeonPlayerRight, "dungeon_player_right" },
            { ActionNames.DungeonPlayerBack, "dungeon_player_back" }
        };
    }
}