using System;
using DarkAdapters.scripts;
using Godot;

namespace DarkAdapters.model.state.dungeonplayerstates
{
    /// <summary>
    ///     Rhea is rotating the camera in this state
    /// </summary>
    public class DungeonPlayerRotatingState : IState<DungeonPlayer>
    {
        private const float RotationTime = 0.3f;

        private bool _done;

        private float _rotationTimeElapsed;

        // vvv ONLY MODIFY THESE WHEN ENTERING THE STATE! vvv //
        public Quat OriginalQuat;

        public Quat TargetQuat;
        // ^^^ ONLY MODIFY THESE WHEN ENTERING THE STATE! ^^^ //

        public DungeonPlayerRotatingState(DungeonPlayer node)
        {
            Node = node;
        }

        public DungeonPlayer Node { get; }

        public IState<DungeonPlayer>? HandleInput(float delta)
        {
            if (_done) return Node.States[DungeonPlayerState.Default];

            return null;
        }

        public void _Process(float delta)
        {
            _interpolateRotationQuat(delta);
        }

        public void Enter()
        {
            _rotationTimeElapsed = 0;
            _done = false;
        }

        public void Exit()
        {
        }

        private void _interpolateRotationQuat(float delta)
        {
            /*
             * basic idea: interpolate between our original and target quats using the time elapsed and the total
             * rotation time. then set our transform to a new transform with its basis constructed using that interpolated
             * quat. we set _isRotating and _rotationTimeElapsed back to their defaults if t is very close to 1; i.e.
             * the interpolation is complete
             */
            _rotationTimeElapsed += delta;
            var t = Mathf.Clamp(_rotationTimeElapsed / RotationTime, 0, 1f);
            var transform = Node.Transform;
            var newQuat = OriginalQuat.Slerp(TargetQuat, t);
            transform.basis = new Basis(newQuat);
            Node.Transform = transform;

            if (Math.Abs(t - 1f) < Utils.Epsilon)
            {
                _rotationTimeElapsed = 0;
                _done = true;
            }
        }
    }
}