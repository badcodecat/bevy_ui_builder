// ! I think development on this will be frozen until the change to Event based callbacks is complete.
/*
	This dropdown widget is going to be a bit more complicated than the others.
	It will need to use Z indices to make sure that the dropdown is always on top of everything else.
	It will also need to use a container to hold the dropdown items.
	The dropdown items will be a list of buttons, and the buttons will need to be able to change the dropdown's value.
	TODO: The dropdown will also need to be able to close itself when the user clicks outside of it.
*/

use super::*;

#[derive(Component)]
pub struct DropdownOption
{
	pub value: String,
}

#[derive(Component)]
pub struct DropdownSelection
{
	pub value: String,
}

pub struct Dropdown<U, M>
	where U: Component + Default, M: Component + Default
{
	/// The button that opens the dropdown and displays the current selection.
	pub dropdown_base: TextButton<U, M>,
	pub options: Vec<String>,
	pub selection: String,
}
