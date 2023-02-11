using Godot;
using System;
using System.Numerics;
using Godot.Collections;
using Array = Godot.Collections.Array;
using Vector2 = Godot.Vector2;
using Vector3 = Godot.Vector3;

public class DungeonCell3D : Area
{
    private Floor _floor;
    private MeshInstance _left;
    private MeshInstance _forward;
    private MeshInstance _right;
    private MeshInstance _back;
    private MeshInstance _ceiling; 

    // Called when the node enters the scene tree for the first time.
    public override void _Ready()
    {
        var surfaceArray = new Array();
        surfaceArray.Resize((int)ArrayMesh.ArrayType.Max);
        var uvCoordinates = new Vector2[6];
        
        uvCoordinates[0] = new Vector2(0, 0);
        uvCoordinates[1] = new Vector2(1, 0);
        uvCoordinates[2] = new Vector2(1, 1);
        uvCoordinates[3] = new Vector2(0, 1);
        uvCoordinates[4] = new Vector2(0, 0);
        uvCoordinates[5] = new Vector2(1, 1);
        
        var floorVertices = new Vector3[6];
        floorVertices[0] = new Vector3(-1, 0, -1);
        floorVertices[1] = new Vector3( 1, 0, -1);
        floorVertices[2] = new Vector3( 1, 0,  1);
        floorVertices[3] = new Vector3(-1, 0,  1);
        floorVertices[4] = new Vector3(-1, 0,  -1);
        floorVertices[5] = new Vector3(1, 0,  1); // bottom right
        _floor = GetNode<Floor>("Floor");
        _floor.ArrayMesh.AddSurfaceFromArrays(Mesh.PrimitiveType.Triangles, surfaceArray);
        ResourceSaver.Save("res://assets/tile_maybe.tres", _floor.ArrayMesh, ResourceSaver.SaverFlags.Compress);
        
        var leftVertices = new Vector3[6];
        leftVertices[0] = new Vector3(-1, 0, -1);
        leftVertices[1] = new Vector3( 1, 0, -1);
        leftVertices[2] = new Vector3( 1, 0,  1);
        leftVertices[3] = new Vector3(-1, 0,  1);
        leftVertices[4] = new Vector3(-1, 0,  -1);
        leftVertices[5] = new Vector3(1, 0,  1); // bottom right
        surfaceArray[(int)ArrayMesh.ArrayType.Vertex] = leftVertices;
        surfaceArray[(int)ArrayMesh.ArrayType.TexUv] = uvCoordinates;
        

    }

//  // Called every frame. 'delta' is the elapsed time since the previous frame.
//  public override void _Process(float delta)
//  {
//      
//  }
}
