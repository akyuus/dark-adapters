using System;
using DarkAdapters.model;
using Godot;

namespace DarkAdapters.scripts
{
    public class DungeonPlayer : Spatial
    {
        // time it takes to rotate 90 degrees
        private const float RotationTime = 0.3f;

        private bool _isRotating;

        // represents the original rotation quaternion
        private Quat _originalQuat;

        // how much time has elapsed, if the player is currently rotating
        private float _rotationTimeElapsed;

        // represents the target quaternion. this is used when the player hits left or right
        private Quat _targetQuat;

        // Called when the node enters the scene tree for the first time.
        public override void _Ready()
        {
        }

        public override void _Process(float delta)
        {
            _handleInput(delta);
            if (_isRotating)
            {
                // if the player is currently rotating 
                _interpolateRotationQuat(delta);
            }
        }

        private void _handleInput(float delta)
        {
            
            if (Input.IsActionJustPressed(InputUtils.InputStringMap[ActionNames.DungeonPlayerLeft]))
            {
                if (!_isRotating)
                {
                    // start rotating
                    _isRotating = true;
                    _targetQuat = Transform.basis.Rotated(Vector3.Up, Mathf.Tau / 4).Quat();
                    _originalQuat = Transform.basis.Quat();
                }
            }
            if (Input.IsActionJustPressed(InputUtils.InputStringMap[ActionNames.DungeonPlayerRight]))
            {
                if (!_isRotating)
                {
                    _isRotating = true;
                    _targetQuat = Transform.basis.Rotated(Vector3.Up, -Mathf.Tau / 4).Quat();
                    _originalQuat = Transform.basis.Quat();
                }
            }
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
            var transform = Transform;
            var newQuat = _originalQuat.Slerp(_targetQuat, t);
            transform.basis = new Basis(newQuat);
            Transform = transform;
            if (Math.Abs(1f - t) < 0.0000001)
            {
                _isRotating = false;
                _rotationTimeElapsed = 0;
            }
        }
    }
}