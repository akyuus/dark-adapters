using DarkAdapters.scripts;
using Godot;

namespace DarkAdapters.model.state.dungeonplayerstates
{
    /// <summary>
    ///     This is the default state for Rhea when exploring the dungeon. She'll be in this state when she's just standing
    ///     still and not scanning.
    /// </summary>
    public class DungeonPlayerDefaultState : IState<DungeonPlayer>
    {
        public DungeonPlayerDefaultState(DungeonPlayer dungeonPlayer)
        {
            Node = dungeonPlayer;
        }

        public DungeonPlayer Node { get; }

        public IState<DungeonPlayer>? HandleInput(float delta)
        {
            // start rotating. note that we don't need any _isRotating flags anymore!
            if (Input.IsActionJustPressed(InputUtils.InputStringMap[ActionNames.DungeonPlayerLeft]))
            {
                // should never be null
                var rotatingState = Node.States[DungeonPlayerState.Rotating] as DungeonPlayerRotatingState;
                rotatingState!.TargetQuat = Node.Transform.basis.Rotated(Vector3.Up, Mathf.Tau / 4).Quat();
                rotatingState.OriginalQuat = Node.Transform.basis.Quat();
                return rotatingState;
            }

            if (Input.IsActionJustPressed(InputUtils.InputStringMap[ActionNames.DungeonPlayerRight]))
            {
                var rotatingState = Node.States[DungeonPlayerState.Rotating] as DungeonPlayerRotatingState;
                rotatingState!.TargetQuat = Node.Transform.basis.Rotated(Vector3.Up, -Mathf.Tau / 4).Quat();
                rotatingState.OriginalQuat = Node.Transform.basis.Quat();
                return rotatingState;
            }

            return null;
        }

        public void _Process(float delta)
        {
        }

        public void Enter()
        {
        }

        public void Exit()
        {
        }
    }
}