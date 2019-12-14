#[macro_export]
macro_rules! html_component {
    ( $c:ident { $($opt:expr),* } ) => {
        // let mut __component = $c::new();
        // let __boxed_component = Box::new(__component);
        // let __component_context = $crate::ComponentContext::new(__boxed_component.id(), 0, __boxed_component);

        // $crate::with_comp_tree_mut(|context| {
        //     context.insert(__component_context);
        // });

		//__component.render(ctx)
		
		"trst"
    }
}
