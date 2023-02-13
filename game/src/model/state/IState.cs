using Godot;

namespace DarkAdapters.model.state
{
    /// <summary>
    ///     fancy state interface that extends to basically anything we'll need!
    /// </summary>
    /// <typeparam name="T">
    ///     T needs to be any node. Basically, anything that'll be stateful and respond to input. though,
    ///     not necessarily *player* input.
    /// </typeparam>
    /// something with a well-defined "enumerable" number of states.
    public interface IState<out T> where T : Node
    {
        // ReSharper disable once UnusedMemberInSuper.Global
        T Node { get; }

        /// <summary>
        ///     What it says on the tin.
        /// </summary>
        /// <param name="delta"></param>
        /// <returns>New state to transition to, potentially.</returns>
        IState<T>? HandleInput(float delta);

        /// <summary>
        /// 
        /// </summary>
        /// <param name="delta"></param>
        void _Process(float delta);

        /// <summary>
        ///     Called when the state is initially entered
        /// </summary>
        void Enter();

        /// <summary>
        ///     Called when exiting the state
        /// </summary>
        void Exit();
    }
}