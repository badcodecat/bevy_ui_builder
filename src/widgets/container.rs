use bevy::prelude::*;

use super::WidgetBuilder;

// A container is just a NodeBundle with extra steps. You should use other widgets (Column, Row, etc.) instead of this.
pub struct Container
{
	pub children: Vec<Box<dyn WidgetBuilder>>,
    pub node_bundle: NodeBundle,
}

impl Container
{
	pub fn new() -> Self
	{
		Self
		{
			children: Vec::new(),
			node_bundle: NodeBundle
			{
				style: Style
				{
					width: Val::Percent(100.0),
					height: Val::Percent(100.0),
					justify_content: JustifyContent::Center,
					align_items: AlignItems::Center,
					..Default::default()
				},
				..Default::default()
			},
		}
	}

	pub fn push(mut self, child: impl Into<Box<dyn WidgetBuilder>>) -> Self
	{
		self.children.push(child.into());
		self
	}

	pub fn with_size(mut self, width: Val, height: Val) -> Self
	{
		self.node_bundle.style.width = width;
		self.node_bundle.style.height = height;
		self
	}
}

impl super::Widget for Container
{
	fn with_colour(mut self, colour: Color) -> Self
	{
		self.node_bundle.background_color = colour.into();
		self
	}

	fn with_direction(mut self, direction: FlexDirection) -> Self
	{
		self.node_bundle.style.flex_direction = direction;
		self
	}

	fn with_wrap(mut self, wrap: FlexWrap) -> Self
	{
		self.node_bundle.style.flex_wrap = wrap;
		self
	}

	// Sets the alignment of this element, WARNING: this will override the alignment options of the parent.
	fn with_align_self(mut self, align_self: AlignSelf) -> Self
	{
		self.node_bundle.style.align_self = align_self;
		self
	}

	fn with_align_content(mut self, align_content: AlignContent) -> Self
	{
		self.node_bundle.style.align_content = align_content;
		self
	}

	fn with_fill_portion(mut self, fill_portion: f32) -> Self
	{
		self.node_bundle.style.flex_basis = Val::Percent(fill_portion * 100.0);
		self
	}

}

impl super::WidgetBuilder for Container
{
	fn build(&self, theme: &crate::theme::ThemePallete, commands: &mut Commands) -> Entity
	{
		let root = commands.spawn(self.node_bundle.clone()).id(); // TODO: See if we can avoid cloning the node bundle.
		let children: Vec<Entity> = self.children.iter().map(|child| child.build(theme, commands)).collect();
		commands.entity(root).push_children(&children);
		root
	}
}

impl From<Container> for Box<dyn WidgetBuilder>
{
	fn from(container: Container) -> Self
	{
		Box::new(container)
	}
}
