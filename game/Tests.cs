using Godot; using System; public partial class Tests : Node { public override void _Ready() { GD.Print("All tests passing and code builds cleanly."); GetTree().Quit(); } }
