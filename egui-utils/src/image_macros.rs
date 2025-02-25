#[allow(unused_imports)]
use nih_plug_egui::egui;

/// Create [`egui::TextureHandle`] from raw bytes data imported by `include_bytes`
///
/// `path` is the path of the image file.
///
/// `name` is the name that [`egui::Context::load_texture`] uses.
///
/// `scale` is a multiplier applied to the the image resolution. It needs to be `f64`
///
/// # Examples
///
/// Simple example. In actual code you should precompute `handle_from_bytes` in a build closure, and save the
/// [`egui::TextureHandle`] in the `user_state`, whether it be in a map, or a struct
///
/// ```
/// // Initialize central panel
/// egui::CentralPanel::default()
///     .frame(egui::Frame::none())
///     .show(cx, |ui| {
///         let image_rect = ui.available_rect_before_wrap();
///
///         egui::Image::from_texture(egui::load::SizedTexture::from_handle(&handle_from_bytes!(
///             cx,
///             "../image.png",
///             "image",
///             1_f64
///         )))
///         .paint_at(ui, image_rect);
///     });
/// ```
#[macro_export]
macro_rules! handle_from_bytes {
    ($cx:ident, $path:expr, $name:expr, $scale:expr) => {{
        let image_data = image::load_from_memory(include_bytes!($path)).unwrap();
        let rescaled_size = [
            (image_data.width() as f64 * $scale) as usize,
            (image_data.height() as f64 * $scale) as usize,
        ];
        let color_image = egui::ColorImage::from_rgba_unmultiplied(
            rescaled_size,
            image_data
                .resize_to_fill(
                    rescaled_size[0] as u32,
                    rescaled_size[1] as u32,
                    image::imageops::FilterType::CatmullRom,
                )
                .into_rgba8()
                .as_bytes(),
        );

        $cx.load_texture($name, color_image, egui::TextureOptions::LINEAR)
    }};
}

/// Insert a [`egui::TextureHandle`] created in [`handle_from_bytes`] into a map, using `name` as a key value
///
/// `map` needs to be `Map<&'static str, egui::TextureHandle>`
///
/// Other requirements are the same as for the [`handle_from_bytes`] macro
///
/// # Examples
///
/// Simple example. In actual code you should precompute [`handle_from_bytes`] in a build closure, and save the
/// map in the `user_state`, whether it be in a map, or a struct
///
/// ```
/// let mut map: HashMap<&'static str, egui::TextureHandle> = HashMap::new();
/// // Insert the handle into the map
/// insert_handle_to_map_from_bytes!(map, cx, "../image.png", "image", 1_f64);
/// egui::CentralPanel::default()
///    .frame(egui::Frame::none())
///    .show(cx, |ui| {
///        let image_rect = ui.available_rect_before_wrap();
///
///        egui::Image::from_texture(egui::load::SizedTexture::from_handle(
///            // Get the image handle out
///            map.get("image").unwrap(),
///        ))
///        .paint_at(ui, image_rect);
/// });
/// ```
#[macro_export]
macro_rules! insert_handle_to_map_from_bytes {
    ($map:ident, $cx:ident, $path:expr, $name:expr, $scale:expr) => {
        $map.insert($name, handle_from_bytes!($cx, $path, $name, $scale));
    };
}
