use nalgebra_glm::Vec2;
pub struct Player {
    pub pos: Vec2,
    pub a: f32,
    pub fov: f32,
    pub move_speed: f32,
    pub rotate_speed: f32,
}

impl Player {
    pub fn new(pos: Vec2, a: f32, fov: f32, move_speed: f32, rotate_speed: f32) -> Self {
        Player {
            pos,
            a,
            fov,
            move_speed,
            rotate_speed,
        }
    }

    pub fn can_move_to(&self, new_pos: Vec2, maze: &Vec<Vec<char>>, block_size: usize) -> bool {
        let maze_x = (new_pos.x / block_size as f32).floor() as usize;
        let maze_y = (new_pos.y / block_size as f32).floor() as usize;

        if maze_y < maze.len() && maze_x < maze[maze_y].len() {
            let cell = maze[maze_y][maze_x];
            return cell == ' ' || cell == 'g'; // Allow movement into empty space or goal
        }
        false
    }
}