use crate::framebuffer::Framebuffer;
use crate::player::Player;
use nalgebra_glm::Vec2;

pub fn cast_ray(framebuffer: &mut Framebuffer, maze: &[Vec<char>], player: &Player, angle: f32, block_size: usize) {
    let ray_dir = Vec2::new(angle.cos(), angle.sin());
    let mut ray_pos = player.pos;
    let mut _distance = 0.0;

    while ray_pos.x >= 0.0 && ray_pos.x < (maze[0].len() * block_size) as f32 &&
          ray_pos.y >= 0.0 && ray_pos.y < (maze.len() * block_size) as f32 {
        // Calcula la posición en el laberinto
        let maze_x = (ray_pos.x / block_size as f32).floor() as usize;
        let maze_y = (ray_pos.y / block_size as f32).floor() as usize;

        if maze_y < maze.len() && maze_x < maze[maze_y].len() {
            match maze[maze_y][maze_x] {
                '+' | '-' | '|' => {
                    framebuffer.set_current_color(0xFF0000); // Color de impacto
                    // Dibuja la línea del rayo
                    framebuffer.line(player.pos.x as usize, player.pos.y as usize, ray_pos.x as usize, ray_pos.y as usize);
                    return;
                },
                _ => {},
            }
        }

        // Avanza el rayo
        ray_pos += ray_dir * 1.0; // Incrementa la posición del rayo
        _distance += 1.0;
    }
}
