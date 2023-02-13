using System;
using System.Collections.Generic;
using DarkAdapters.model.state;
using DarkAdapters.model.state.dungeonplayerstates;
using Godot;

namespace DarkAdapters.scripts
{
    public class DungeonPlayer : Spatial
    {
        // time it takes to rotate 90 degrees
        private readonly Dictionary<DungeonPlayerState, IState<DungeonPlayer>> _states;

        private IState<DungeonPlayer> _currentState;

        public DungeonPlayer()
        {
            _states = new Dictionary<DungeonPlayerState, IState<DungeonPlayer>>();
            foreach (DungeonPlayerState stateId in Enum.GetValues(typeof(DungeonPlayerState)))
            {
                IState<DungeonPlayer>? state;
                switch (stateId)
                {
                    case DungeonPlayerState.Default:
                        state = new DungeonPlayerDefaultState(this);
                        break;
                    case DungeonPlayerState.Rotating:
                        state = new DungeonPlayerRotatingState(this);
                        break;
                    default:
                        state = new DungeonPlayerDefaultState(this);
                        break;
                }

                if (_states != null) _states[stateId] = state;
            }

            _currentState = _states![DungeonPlayerState.Default];
        }

        public IReadOnlyDictionary<DungeonPlayerState, IState<DungeonPlayer>> States => _states;


        // Called when the node enters the scene tree for the first time.
        public override void _Ready()
        {
        }

        private void _handleInput(float delta)
        {
            var newState = _currentState.HandleInput(delta);
            if (newState != null)
            {
                _currentState.Exit();
                _currentState = newState;
                _currentState.Enter();
            }
        }

        public override void _Process(float delta)
        {
            _handleInput(delta);
            _currentState._Process(delta);
        }
    }
}