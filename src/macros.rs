// TODO: Hide it's implementation details from the generated rustdoc.
#[macro_export]
macro_rules! collect_commands {
    () => {
        $crate::internal::command(::tauri::generate_handler![], ::specta::function::collect_functions![])
    };
    ($i:ident) => {
        $crate::internal::command(::tauri::generate_handler![$i], ::specta::function::collect_functions![$i])
    };
    ($i:ident, $($rest:tt)*) => {
        $crate::internal::command($crate::collect_commands!(@internal; $i; $($rest)*), ::specta::function::collect_functions![$i, $($rest)*])
    };
    ($i:ident::<$g:path>) => {
        $crate::internal::command(::tauri::generate_handler![$i], ::specta::function::collect_functions![$i<$g>])
    };
    ($i:ident::<$g:path>, $($rest:tt)*) => {
        $crate::internal::command($crate::collect_commands!(@internal; $i; $($rest)*), ::specta::function::collect_functions![$i<$g>, $($rest)*])
    };
    //
    (@internal; $($b:path),*;) => {
        ::tauri::generate_handler![$($b),*]
    };
    (@internal; $($b:path),*; $i:ident) => {
        ::tauri::generate_handler![$($b),*, $i]
    };
    (@internal; $($b:path),*; $i:ident, $($rest:tt)*) => {
        $crate::collect_commands!(@internal; $($b),*, $i; $($rest)*)
    };
    (@internal; $($b:path),*; $i:ident::<$g:path>) => {
        ::tauri::generate_handler![$($b),*, $i]
    };
    (@internal; $($b:path),*; $i:ident::<$g:ident>, $($rest:tt)*) => {
        $crate::collect_commands!(@internal; $($b),*, $i; $($rest)*)
    };
}

// TODO: Hide it's implementation details from the generated rustdoc.
#[macro_export]
macro_rules! collect_events {
    ($($event:path),* $(,)?) => {{
        // TODO: Cleanup the internals of this

    	let mut collection: $crate::EventCollection = ::core::default::Default::default();

     	$(collection.register::<$event>();)*

      	let mut type_map = Default::default();

      	let event_data_types = [$(
	       $crate::EventDataType {
	       		name: <$event as $crate::Event>::NAME,
	       		typ: <$event as ::specta::Type>::reference(&mut type_map, &[]).inner
	       }
       	),*]
        .into_iter()
        .collect::<Vec<_>>();

        $crate::internal::events(collection, event_data_types, type_map)
    }};
}
