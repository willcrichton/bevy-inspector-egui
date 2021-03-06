use crate::{utils, Context, Inspectable};
use bevy::{
    asset::{Asset, HandleId},
    prelude::*,
    render::texture::Texture,
};
use bevy_egui::egui;
use egui::TextureId;

macro_rules! expect_handle {
    ($ui:ident, $assets:ident, $method:ident $asset:ident) => {
        match $assets.$method($asset.clone()) {
            Some(val) => val,
            None => {
                return utils::error_label($ui, format!("No value for handle {:?}", $asset));
            }
        }
    };
}

impl<T: Asset + Inspectable> Inspectable for Handle<T> {
    type Attributes = T::Attributes;

    fn ui(&mut self, ui: &mut egui::Ui, options: Self::Attributes, context: &Context) {
        if self.id == HandleId::default::<T>() {
            ui.label("<default handle>");
            return;
        }

        let resources = expect_context!(ui, context.resources, "Handle<T>");
        let mut assets = expect_resource!(ui, resources, get_mut Assets<T>);

        let value = expect_handle!(ui, assets, get_mut self);

        value.ui(ui, options, context);
    }
}

impl Inspectable for Handle<Texture> {
    type Attributes = ();

    fn ui(&mut self, ui: &mut egui::Ui, _: Self::Attributes, context: &Context) {
        let resources = expect_context!(ui, context.resources, "Handle<Texture>");

        // let mut egui_context = resources.get_mut::<EguiContext>().unwrap();
        let textures = expect_resource!(ui, resources, get Assets<Texture>);
        let texture = expect_handle!(ui, textures, get self);

        let size = texture.size;
        let size = [size.width as f32, size.height as f32];

        let id = id_of_handle(self);
        let texture_id = TextureId::User(id);

        let max = size[0].max(size[1]);
        if max > 256.0 {
            ui.collapsing("Texture", |ui| ui.image(texture_id, size));
        } else {
            ui.image(texture_id, size);
        }
    }
}

pub(crate) fn id_of_handle(handle: &Handle<Texture>) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = bevy::utils::AHasher::default();
    handle.hash(&mut hasher);
    hasher.finish()
}
