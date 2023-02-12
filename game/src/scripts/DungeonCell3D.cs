using Godot;

/// <summary>
///     Base class for a single cubic "cell" in a dungeon. Each one is 2 x 2 x 2 units.
/// </summary>
public class DungeonCell3D : Area
{
    private MeshInstance _back;
    private MeshInstance _ceiling;
    private Floor _floor;
    private MeshInstance _forward;
    private MeshInstance _left;
    private MeshInstance _right;

    // Called when the node enters the scene tree for the first time.
    public override void _Ready()
    {
    }

//  // Called every frame. 'delta' is the elapsed time since the previous frame.
//  public override void _Process(float delta)
//  {
//      
//  }
}