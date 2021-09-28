mod draw;

use tcod::colors::*;
use tcod::console::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 24;

struct Tcod {
    root: Root,
}

fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Default)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("My roguelike like or something")
        .init();

    let mut tcod = Tcod { root };

    let mut player_x = SCREEN_WIDTH / 2;
    let mut player_y = SCREEN_HEIGHT / 2;

    let mut player_char = '[';

    while !tcod.root.window_closed() {
        tcod.root.set_default_background(Color {
            r: 216u8,
            g: 220u8,
            b: 185u8,
        });
        tcod.root.clear();
        tcod.root
            .put_char(player_x, player_y, player_char, BackgroundFlag::None);
        tcod.root.put_char(10, 10, '(', BackgroundFlag::None);
        tcod.root.put_char(11, 10, ')', BackgroundFlag::None);
        tcod.root.put_char(20, 20, 'G', BackgroundFlag::None);
        tcod.root.put_char(21, 20, 'g', BackgroundFlag::None);
        tcod.root.put_char(0, 0, '.', BackgroundFlag::Add);
        tcod.root.flush();

        let exit = handle_keys(&mut tcod, &mut player_x, &mut player_y, &mut player_char);
        if exit {
            break;
        }
    }
}

fn handle_keys(
    tcod: &mut Tcod,
    player_x: &mut i32,
    player_y: &mut i32,
    player_char: &mut char,
) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = tcod.root.wait_for_keypress(true);
    match key {
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            tcod.root.set_fullscreen(!tcod.root.is_fullscreen());
        }

        Key { code: Escape, .. } => return true,

        Key { code: Up, .. } => *player_y -= 1,
        Key { code: Down, .. } => *player_y += 1,
        Key { code: Left, .. } => {
            *player_x -= 1;
            *player_char = ']'
        }
        Key { code: Right, .. } => {
            *player_x += 1;
            *player_char = '['
        }
        _ => {}
    }

    false
}
