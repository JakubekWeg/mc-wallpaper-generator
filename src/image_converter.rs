use crate::{ProgramConfig, TerracottaBehaviour};
use std::fs::DirEntry;
use std::path::Path;
use sfml::graphics::{IntRect, Color, Sprite, Image, Texture, RenderTarget, Transformable, RenderStates, RenderTexture, View, FloatRect};
use sfml::system::{Vector2f, SfBox};
use crate::random_helper::RandomHelper;


pub fn convert_using_random_method(config: ProgramConfig) {
    if config.input_folder_name == "" {
        println!("Specify source files separated by comma!");
        return;
    }

    if config.output_folder_name == "" {
        println!("Specify output file!");
        return;
    }
    if config.probabilities_str == "" {
        println!("Specify probabilities separated by comma!");
        return;
    }

    let paths: Vec<&str> = config.input_folder_name
        .split(',')
        .collect();

    let probabilities: Vec<u32> = config.probabilities_str
        .split(',')
        .map(|s| s.parse::<u32>().expect("Parsing probability"))
        .collect();

    if paths.len() != probabilities.len() {
        println!("Every path must have its probability");
        return;
    }

    let mut randomizer = RandomHelper::new(probabilities);


    let mut render_texture = sfml::graphics::RenderTexture::new(config.image_width, config.image_height, false)
        .expect("Creating render buffer");

    let scale_vec = Vector2f::new(config.scale_factor, config.scale_factor);


    let textures: Vec<SfBox<Texture>> = paths.iter().map(|p| load_texture(p).expect(format!("Loading {} texture", p).as_str())).collect();

    let mut tmp_iterator = textures.iter();
    let texture_size = tmp_iterator.next().unwrap().size();
    for x in tmp_iterator {
        if x.size() != texture_size {
            println!("All source images must be the same size!");
            return;
        }
    }

    let mut sprite = Sprite::new();
    sprite.set_scale(scale_vec);

    let tmp_rect = IntRect::new(0, 0, texture_size.x as i32, texture_size.y as i32);
    sprite.set_texture_rect(&tmp_rect);

    let size = render_texture.size();
    let iterations_horizontal = size.x / texture_size.x / config.scale_factor as u32 + 1;
    let iterations_vertical = size.y / texture_size.y / config.scale_factor as u32 + 1;

    let rendered_size_x = iterations_horizontal as f32 * texture_size.x as f32 * config.scale_factor;
    let rendered_size_y = iterations_vertical as f32 * texture_size.y as f32 * config.scale_factor;
    let view_port = View::from_rect(&FloatRect::new(
        -(size.x as f32 - rendered_size_x) / 2.,
        -(size.y as f32 - rendered_size_y) / 2.,
        size.x as f32,
        size.y as f32));
    render_texture.set_view(&view_port);


    for y in 0..iterations_vertical {
        for x in 0..iterations_horizontal {
            let texture: &SfBox<Texture> = &textures.get(randomizer.next_index()).unwrap();
            sprite.set_texture(texture, false);

            sprite.set_position(Vector2f::new(x as f32 * texture_size.x as f32 * config.scale_factor,
                                              y as f32 * texture_size.y as f32 * config.scale_factor));

            render_texture.draw_sprite(&sprite, RenderStates::default());
        }
    }

    render_texture.display();
    let image = render_texture.texture().copy_to_image().expect("Converting to image");
    image.save_to_file(config.output_folder_name);

    ()
}

pub fn convert_one(config: ProgramConfig) {
    if config.input_folder_name == "" {
        println!("Specify source file!");
        return;
    }
    if config.output_folder_name == "" {
        println!("Specify output file!");
        return;
    }
    let input_path = Path::new(config.input_folder_name);
    let output_path = Path::new(config.output_folder_name);

    if !input_path.is_file() {
        println!("Input path is not a file!");
        return;
    }


    let mut render_texture = sfml::graphics::RenderTexture::new(config.image_width, config.image_height, false)
        .expect("Creating render buffer");

    let rect = IntRect::new(0, 0, config.image_width as i32, config.image_height as i32);
    let scale_vec = Vector2f::new(config.scale_factor, config.scale_factor);

    let file_name = String::from(input_path.file_name().unwrap().to_str().unwrap());
    println!("Rendering {:?}", file_name);
    if let Some(texture) = load_texture(input_path.as_os_str().to_str().unwrap()) {
        render_texture.clear(Color::TRANSPARENT);

        let mut sprite = Sprite::new();
        sprite.set_scale(scale_vec);
        sprite.set_texture(&texture, false);

        sprite.set_texture_rect(&rect);
        draw_sprite_and_handle_terracotta(match config.terracotta_behaviour {
            TerracottaBehaviour::Never => false,
            TerracottaBehaviour::Always => true,
            TerracottaBehaviour::Auto => file_name.contains("glazed_terracotta")
        }, config.scale_factor, &mut sprite, &texture, &mut render_texture);

        render_texture.display();
        let image = render_texture.texture().copy_to_image().expect("Converting to image");
        if should_be_saved(&image) {
            image.save_to_file(output_path.to_str().unwrap());
        }
    } else {
        println!("Refused to open the file");
    }
}

pub fn convert_every_from_directory(config: ProgramConfig) {
    if config.input_folder_name == "" {
        println!("Specify source folder!");
        return;
    }
    if config.output_folder_name == "" {
        println!("Specify output folder!");
        return;
    }

    let input_path = Path::new(config.input_folder_name);
    let output_path = Path::new(config.output_folder_name);

    if !input_path.is_dir() {
        println!("Input path is not a folder!");
        return;
    }

    if std::fs::metadata(output_path).is_ok() {
        if !output_path.is_dir() {
            println!("Output path exists and is not a folder!");
            return;
        }
    }

    std::fs::create_dir_all(output_path).unwrap();


    let file_names: Vec<DirEntry> = std::fs::read_dir(input_path)
        .expect("Read source folder")
        .map(|result| result.unwrap())
        .collect();


    let mut render_texture = sfml::graphics::RenderTexture::new(config.image_width, config.image_height, false)
        .expect("Creating render buffer");

    let rect = IntRect::new(0, 0, config.image_width as i32, config.image_height as i32);
    let scale_vec = Vector2f::new(config.scale_factor, config.scale_factor);

    for file in file_names {
        let file_name = String::from(file.file_name().to_str().unwrap());
        println!("Rendering {:?}", file_name);
        let read_from_file = input_path.join(file.file_name());
        if let Some(texture) = load_texture(read_from_file.to_str().unwrap()) {
            render_texture.clear(Color::TRANSPARENT);

            let mut sprite = Sprite::new();
            sprite.set_scale(scale_vec);
            sprite.set_texture(&texture, false);

            sprite.set_texture_rect(&rect);
            draw_sprite_and_handle_terracotta(match config.terracotta_behaviour {
                TerracottaBehaviour::Never => false,
                TerracottaBehaviour::Always => true,
                TerracottaBehaviour::Auto => file_name.contains("glazed_terracotta")
            }, config.scale_factor, &mut sprite, &texture, &mut render_texture);

            render_texture.display();
            let image = render_texture.texture().copy_to_image().expect("Converting to image");
            if should_be_saved(&image) {
                let save_to_file = output_path.join(file.file_name());
                image.save_to_file(save_to_file.to_str().unwrap());
            }
        }
    }
}

fn draw_sprite_and_handle_terracotta(is_terracotta: bool,
                                     scale_factor: f32,
                                     sprite: &mut Sprite,
                                     texture: &Texture,
                                     render_texture: &mut RenderTexture) {

    let texture_size = texture.size();
    let size = render_texture.size();
    let iterations_horizontal = size.x / texture_size.x / scale_factor as u32 + 1;
    let iterations_vertical = size.y / texture_size.y / scale_factor as u32 + 1;
    let mut is_even_y = false;

    let rendered_size_x = iterations_horizontal as f32 * texture_size.x as f32 * scale_factor;
    let rendered_size_y = iterations_vertical as f32 * texture_size.y as f32 * scale_factor;
    let view_port = View::from_rect(&FloatRect::new(
        -(size.x as f32 - rendered_size_x) / 2.,
        -(size.y as f32 - rendered_size_y) / 2.,
        size.x as f32,
        size.y as f32));
    render_texture.set_view(&view_port);

    if is_terracotta {
        let tmp_rect = IntRect::new(0, 0, texture_size.x as i32, texture_size.y as i32);
        sprite.set_texture_rect(&tmp_rect);

        let mut origin_vec = Vector2f::new(texture_size.x as f32 / 2., texture_size.y as f32 / 2.);
        sprite.set_origin(origin_vec);
        origin_vec *= scale_factor;


        for y in 0..iterations_vertical {
            let mut is_even_x = false;
            for x in 0..iterations_horizontal {
                sprite.set_rotation(90. * (match (is_even_x, is_even_y) {
                    (false, false) => 0,
                    (true, false) => 1,
                    (false, true) => 3,
                    (true, true) => 2,
                }) as f32);
                sprite.set_position(Vector2f::new(x as f32 * texture_size.x as f32 * scale_factor + origin_vec.x,
                                                  y as f32 * texture_size.y as f32 * scale_factor + origin_vec.y));

                render_texture.draw_sprite(&sprite, RenderStates::default());
                is_even_x = !is_even_x;
            }
            is_even_y = !is_even_y;
        }
    } else {
        render_texture.draw_sprite(&sprite, RenderStates::default());
    }
}

fn should_be_saved(img: &Image) -> bool {
    let size = img.size();
    img.pixel_at(0, 0).a == 255 && img.pixel_at(size.x - 1, size.y - 1).a == 255
}

fn load_texture(source_file_name: &str) -> Option<SfBox<Texture>> {
    if !source_file_name.ends_with(".png") {
        return None;
    }
    let mut texture = Texture::from_file(source_file_name).expect("Loading texture");
    let size = texture.size();
    if size.x != size.y {
        return None;
    }

    texture.set_smooth(false);
    texture.set_repeated(true);
    Some(texture)
}