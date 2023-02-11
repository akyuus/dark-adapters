using Godot;
using System;

public class Floor : MeshInstance
{
    // Declare member variables here. Examples:
    // private int a = 2;
    // private string b = "text";

    public ArrayMesh ArrayMesh => base.Mesh as ArrayMesh;

    // Called when the node enters the scene tree for the first time.
    public override void _Ready()
    {
        
    }

    public void InitializeVerts()
    {
        
    }
//  // Called every frame. 'delta' is the elapsed time since the previous frame.
//  public override void _Process(float delta)
//  {
//      
//  }
}
