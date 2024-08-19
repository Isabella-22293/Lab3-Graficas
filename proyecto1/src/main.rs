mod framebuffer;
mod maze;
mod player;
mod caster;

use image::RgbaImage;
use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::Player;
use crate::caster::cast_ray;

const MINIMAP_SIZE: usize = 200;
const MINIMAP_OFFSET: usize = 10;

enum RenderMode {
    TwoD,
    ThreeD,
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    let color = match cell {
        '+' => 0x000080, // Intersecciones
        '-' => 0x000080, // Paredes horizontales
        '|' => 0x000080, // Paredes verticales
        ' ' => 0x87CEEB, // Color de fondo
        'g' => 0xFF0000, // Color para el objetivo
        _ => 0x000080,   // Color de fondo predeterminado
    };

    framebuffer.set_current_color(color);

    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            framebuffer.point(x, y);
        }
    }
}

fn draw_minimap(framebuffer: &mut Framebuffer, maze: &Vec<Vec<char>>, player: &Player) {
    let minimap_start_x = framebuffer.width - MINIMAP_SIZE - MINIMAP_OFFSET;
    let minimap_start_y = MINIMAP_OFFSET;
    let block_size = MINIMAP_SIZE / maze.len().max(1); // Calcula el tamaño de los bloques en el minimapa

    // Dibuja el laberinto en el minimapa
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            let cell = maze[row][col];
            let x = minimap_start_x + col * block_size;
            let y = minimap_start_y + row * block_size;
            draw_cell(framebuffer, x, y, block_size, cell);
        }
    }

    // Dibuja la posición del jugador en el minimapa
    framebuffer.set_current_color(0xFFFF00); // Amarillo para el jugador
    let player_x = minimap_start_x + (player.pos.x as usize / 10);
    let player_y = minimap_start_y + (player.pos.y as usize / 10);
    framebuffer.point(player_x, player_y);
}

fn render_2d(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./maze.txt");

    // Calcula el tamaño del bloque para que el laberinto completo quepa en la pantalla
    let block_size_width = framebuffer.width / maze[0].len();
    let block_size_height = framebuffer.height / maze.len();
    let block_size = block_size_width.min(block_size_height);

    // Dibuja el laberinto
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
        }
    }

    // Dibuja al jugador más grande y de color blanco
    framebuffer.set_current_color(0xFFFFFF); // Blanco para el jugador
    let player_size = block_size / 5; // Ajusta el tamaño del jugador basado en el tamaño del bloque

    let player_x = (player.pos.x / (block_size as f32 / 10.0)) as usize;
    let player_y = (player.pos.y / (block_size as f32 / 10.0)) as usize;

    for x in player_x.saturating_sub(player_size)..player_x.saturating_add(player_size) {
        for y in player_y.saturating_sub(player_size)..player_y.saturating_add(player_size) {
            framebuffer.point(x, y);
        }
    }

    // Lanza rayos
    let num_rays = 5;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);

        cast_ray(framebuffer, &maze, &player, a, block_size);
    }

    // Dibuja el minimapa
    draw_minimap(framebuffer, &maze, player);
}


fn load_texture(path: &str) -> RgbaImage {
    image::open(path)
        .expect("No se pudo cargar la textura")
        .to_rgba8()
}

fn load_textures() -> HashMap<char, RgbaImage> {
    let mut textures = HashMap::new();
    textures.insert('+', load_texture("D:/Documentos/Sexto semestre/Graficas/Lab3-Graficas/proyecto1/src/imagenes/textura.jpg"));
    textures.insert('-', load_texture("D:/Documentos/Sexto semestre/Graficas/Lab3-Graficas/proyecto1/src/imagenes/textura.jpg"));
    textures.insert('|', load_texture("D:/Documentos/Sexto semestre/Graficas/Lab3-Graficas/proyecto1/src/imagenes/textura.jpg"));
    textures
}

fn render_3d(framebuffer: &mut Framebuffer, player: &Player, textures: &HashMap<char, RgbaImage>) {
    let maze = load_maze("./maze.txt");
    let block_size = 100.0; 
    let framebuffer_width = framebuffer.width as f32;
    let framebuffer_height = framebuffer.height as f32;

    for col in 0..framebuffer_width as usize {
        let ray_angle = player.a - (player.fov / 2.0) + (player.fov * (col as f32 / framebuffer_width));
        let ray_dir = Vec2::new(ray_angle.cos(), ray_angle.sin());
        let mut ray_pos = player.pos;
        let mut distance = 0.0;

        while ray_pos.x >= 0.0 && ray_pos.x < (maze[0].len() as f32 * block_size) &&
              ray_pos.y >= 0.0 && ray_pos.y < (maze.len() as f32 * block_size) {
            let maze_x = (ray_pos.x / block_size).floor() as usize;
            let maze_y = (ray_pos.y / block_size).floor() as usize;

            if maze_y < maze.len() && maze_x < maze[maze_y].len() {
                if maze[maze_y][maze_x] != ' ' {
                    let corrected_distance = distance * ray_angle.cos(); // Corrección de la distorsión
                    let wall_height = (block_size * framebuffer_height / (corrected_distance + 0.0001)) as usize;
                    let top = (framebuffer_height / 2.0) - (wall_height as f32 / 2.0);
                    let bottom = (framebuffer_height / 2.0) + (wall_height as f32 / 2.0);
                    
                    if let Some(texture) = textures.get(&maze[maze_y][maze_x]) {
                        let texture_width = texture.width() as usize;
                        let texture_height = texture.height() as usize;
                        let texture_x = ((ray_pos.x % block_size) * texture_width as f32 / block_size) as usize;

                        for y in top as usize..bottom as usize {
                            let texture_y = ((y - top as usize) * texture_height / wall_height).min(texture_height - 1);
                            let pixel = texture.get_pixel(texture_x as u32, texture_y as u32);
                            let color = ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2] as u32);
                            framebuffer.set_current_color(color);
                            framebuffer.point(col as usize, y);
                        }
                    } else {
                        framebuffer.set_current_color(0x87CEEB); // Color de fondo
                        for y in top as usize..bottom as usize {
                            framebuffer.point(col as usize, y);
                        }
                    }
                    break;
                }
            }

            ray_pos += ray_dir;
            distance += 1.0;
        }
    }
}


fn handle_player_movement(player: &mut Player, window: &Window, maze: &Vec<Vec<char>>, block_size: usize, render_mode: &RenderMode) {
    let forward = Vec2::new(player.a.cos(), player.a.sin()) * player.move_speed;
    let right = Vec2::new(-player.a.sin(), player.a.cos()) * player.move_speed;

    match render_mode {
        RenderMode::ThreeD => {
            if window.is_key_down(Key::W) || window.is_key_down(Key::Up) {
                let new_pos = player.pos + forward;
                if player.can_move_to(new_pos, maze, block_size) {
                    player.pos = new_pos;
                }
            }

            if window.is_key_down(Key::S) || window.is_key_down(Key::Down) {
                let new_pos = player.pos - forward;
                if player.can_move_to(new_pos, maze, block_size) {
                    player.pos = new_pos;
                }
            }

            if window.is_key_down(Key::D) || window.is_key_down(Key::Right) {
                let new_pos = player.pos + right;
                if player.can_move_to(new_pos, maze, block_size) {
                    player.pos = new_pos;
                }
            }

            if window.is_key_down(Key::A) || window.is_key_down(Key::Left) {
                let new_pos = player.pos - right;
                if player.can_move_to(new_pos, maze, block_size) {
                    player.pos = new_pos;
                }
            }
        },
        RenderMode::TwoD => {
            if window.is_key_down(Key::W) || window.is_key_down(Key::Up) {
                let new_pos = player.pos + Vec2::new(0.0, -player.move_speed);
                if player.can_move_to(new_pos, maze, block_size) {
                    player.pos = new_pos;
                }
            }

            if window.is_key_down(Key::S) || window.is_key_down(Key::Down) {
                let new_pos = player.pos + Vec2::new(0.0, player.move_speed);
                if player.can_move_to(new_pos, maze, block_size) {
                    player.pos = new_pos;
                }
            }

            if window.is_key_down(Key::D) || window.is_key_down(Key::Right) {
                let new_pos = player.pos + Vec2::new(player.move_speed, 0.0);
                if player.can_move_to(new_pos, maze, block_size) {
                    player.pos = new_pos;
                }
            }

            if window.is_key_down(Key::A) || window.is_key_down(Key::Left) {
                let new_pos = player.pos + Vec2::new(-player.move_speed, 0.0);
                if player.can_move_to(new_pos, maze, block_size) {
                    player.pos = new_pos;
                }
            }
        }
    }

    if window.is_key_down(Key::A) || window.is_key_down(Key::Left) {
        player.a -= player.rotate_speed;
    }

    if window.is_key_down(Key::D) || window.is_key_down(Key::Right) {
        player.a += player.rotate_speed;
    }
}



fn main() {
    // Carga las texturas
    let textures = load_textures();

    let window_width = 1500;
    let window_height = 700;
    let framebuffer_width = 1500;
    let framebuffer_height = 700;
    let target_fps = 15;
    let frame_delay = Duration::from_millis(1000 / target_fps as u64);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Maze Runner",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    framebuffer.set_current_color(0x333355);
    framebuffer.clear();

    let mut player = Player::new(
        Vec2::new(150.0, 150.0),
        PI / 3.0,
        PI / 3.0,
        2.0,
        0.05
    );

    // Cargar el laberinto y calcular el tamaño del bloque
    let maze = load_maze("./maze.txt");
    let block_size = 100.0; // Asigna un valor adecuado para el tamaño del bloque

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    let file = std::fs::File::open("D:/Documentos/Sexto semestre/Graficas/Lab3-Graficas/proyecto1/src/sounds/fondo.mp3").unwrap();
    let source = rodio::Decoder::new(std::io::BufReader::new(file)).unwrap();
    sink.append(source);

    let mut last_time = Instant::now();
    let mut frame_count = 0;
    let mut render_mode = RenderMode::TwoD;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let start_time = Instant::now();

        // Llama a la función handle_player_movement para manejar el movimiento del jugador
        handle_player_movement(&mut player, &window, &maze, block_size as usize, &render_mode);

        // Cambio de modo de renderizado
        if window.is_key_down(Key::X) {
            render_mode = match render_mode {
                RenderMode::TwoD => RenderMode::ThreeD,
                RenderMode::ThreeD => RenderMode::TwoD,
            };
            std::thread::sleep(Duration::from_millis(200));
        }

        framebuffer.clear();

        match render_mode {
            RenderMode::TwoD => {
                render_2d(&mut framebuffer, &player);
            },
            RenderMode::ThreeD => {
                render_3d(&mut framebuffer, &player, &textures);
                // Dibuja el minimapa sobre la vista 3D
                draw_minimap(&mut framebuffer, &maze, &player);
            },
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        frame_count += 1;
        if last_time.elapsed() >= Duration::from_secs(1) {
            let fps = frame_count;
            frame_count = 0;
            last_time = Instant::now();
            window.set_title(&format!("Maze Runner - FPS: {}", fps));
        }

        let elapsed = Instant::now() - start_time;
        if elapsed < frame_delay {
            std::thread::sleep(frame_delay - elapsed);
        }
    }
}