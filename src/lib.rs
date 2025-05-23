/// A simple text-based maze game library.
/// 
/// This library provides functionality to create and play maze games
/// with procedurally generated mazes.

pub mod maze;
pub mod generator;
pub mod position;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_maze_creation() {
        let maze = maze::Maze::new();
        assert!(maze.is_some());
    }
    
    #[test]
    fn test_custom_size_maze_creation() {
        let maze = maze::Maze::with_size(21, 15);
        assert!(maze.is_some());
        
        if let Some(maze) = maze {
            assert_eq!(maze.get_width(), 21);
            assert_eq!(maze.get_height(), 15);
        }
    }
    
    #[test]
    fn test_player_movement() {
        if let Some(mut maze) = maze::Maze::new() {
            let directions = ["w", "a", "s", "d"];
            let mut any_move_successful = false;
            
            for dir in &directions {
                // 移動前の位置を保存
                let initial_pos = maze.get_state().player_position;
                
                // 移動を試みる
                maze.move_player(dir);
                
                // 移動後の位置を取得
                let new_pos = maze.get_state().player_position;
                
                // 位置が変わったか確認
                if initial_pos.x != new_pos.x || initial_pos.y != new_pos.y {
                    any_move_successful = true;
                    break;
                }
            }
            
            // 少なくとも1つの方向には移動できるはず
            assert!(any_move_successful, "Player could not move in any direction");
        } else {
            panic!("Failed to create maze for test");
        }
    }
}
