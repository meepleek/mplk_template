use bevy_asset_loader::prelude::*;

use crate::prelude::*;

pub(super) fn plugin(initial_screen: Screen) -> impl Fn(&mut App) {
    move |app| {
        app.add_loading_state(
            LoadingState::new(Screen::Loading).continue_to_state(initial_screen), // .load_collection::<SpriteAssets>()
                                                                                  // .load_collection::<FontAssets>()
                                                                                  // .load_collection::<SfxAssets>()
                                                                                  // .load_collection::<MusicAssets>()
        );
        // app.add_systems(Startup, setup_particles);
        //
        //

        // // default font
        // Assets::insert(
        //     &mut app.world_mut().resource_mut(),
        //     AssetId::default(),
        //     Font {
        //         data: include_bytes!("../../assets/fonts/AtkinsonHyperlegibleMono_medium.ttf")
        //             .to_vec()
        //             .into(),
        //         ..default()
        //     },
        // )
        // .expect("Failed to set default font");
    }
}
