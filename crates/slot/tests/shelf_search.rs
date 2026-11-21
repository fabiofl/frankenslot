mod common;

use slot::app::Phase;
use slot::session::Session;
use slot_input::{Btn, RawEvent};
use slot_store::{
    write_carousel, write_slot_state, Binding, CarouselAction, CarouselKeys, Device, KidMode,
    Platform, Recent, ShelfMode, SlotState,
};
use slot_ui::QuickRow;
use tempfile::TempDir;

fn card() -> TempDir {
    let card = common::tmp_root_with_carts(&["Comet", "Super Orbit"]);
    common::write_gb_cart(&card, "Orbit", "ORBIT");
    let mut color_rom = [0; 0x150];
    color_rom[0x134..0x139].copy_from_slice(b"ORBIT");
    color_rom[0x143] = 0x80;
    std::fs::write(card.path().join("Games/GBC/Orbit.gbc"), color_rom).unwrap();
    card
}

struct Player {
    session: Session,
    now: u64,
}

impl Player {
    fn boot(card: &TempDir, state: SlotState) -> Self {
        write_slot_state(
            card.path(),
            &SlotState {
                clock_set: true,
                ..state
            },
        )
        .unwrap();
        Self {
            session: Session::boot(card.path().into()),
            now: 0,
        }
    }

    fn tap(&mut self, button: Btn) {
        self.now += 500;
        self.session
            .feed([RawEvent::Down(button), RawEvent::Up(button)], self.now);
    }

    fn open_search(&mut self) {
        self.tap(Btn::Menu);
        assert_eq!(self.session.app().quick_menu(), Some(QuickRow::General));
        self.tap(Btn::A);
        for _ in 0..QuickRow::Search.position(&self.session.app().hidden_rows()) {
            self.tap(Btn::Down);
        }
        assert_eq!(self.session.app().quick_menu(), Some(QuickRow::Search));
        self.tap(Btn::A);
        assert_eq!(self.session.app().keyboard().unwrap().title(), "Search");
    }

    fn enter(&mut self, text: &str) {
        for ch in text.chars() {
            let keyboard = self.session.app().keyboard().unwrap();
            let (row, col) = keyboard
                .rows()
                .iter()
                .enumerate()
                .find_map(|(r, keys)| {
                    keys.iter()
                        .position(|key| keyboard.label(*key) == ch.to_string())
                        .map(|c| (r, c))
                })
                .expect("the character belongs to the existing keyboard");
            while self.session.app().keyboard().unwrap().row() != row {
                self.tap(Btn::Down);
            }
            while self.session.app().keyboard().unwrap().col() != col {
                self.tap(Btn::Right);
            }
            self.tap(Btn::A);
        }
        self.tap(Btn::Start);
    }
}

#[test]
fn search_finds_three_shelves_and_focuses_the_exact_duplicate() {
    let card = card();
    let mut player = Player::boot(&card, SlotState::default());
    assert_eq!(player.session.app().shelf_key(), "gba");
    player.open_search();
    player.enter("orb");

    let Phase::Search { hits, cursor } = player.session.app().phase() else {
        panic!("search has no results screen");
    };
    let rows: Vec<_> = hits
        .iter()
        .map(|hit| (hit.title.as_str(), hit.platform))
        .collect();
    assert_eq!(
        rows,
        [
            ("Orbit", Platform::Gbc),
            ("Orbit", Platform::Gb),
            ("Super Orbit", Platform::Gba)
        ]
    );
    assert_eq!(*cursor, 0);
    player.tap(Btn::Down);
    player.tap(Btn::A);

    let app = player.session.app();
    assert!(matches!(app.phase(), Phase::Shelf));
    assert_eq!(app.shelf_key(), "gb");
    assert_eq!(app.selected_stem(), Some("Orbit"));
    assert_eq!(app.selected_cart().unwrap().platform, Platform::Gb);
}

#[test]
fn empty_results_and_cancelling_keyboard_return_to_the_same_cart() {
    let card = card();
    let mut player = Player::boot(&card, SlotState::default());
    let shelf = player.session.app().shelf_key();
    let stem = player.session.app().selected_stem().unwrap().to_owned();
    player.open_search();
    player.enter("qq");
    player.tap(Btn::A);
    assert!(matches!(player.session.app().phase(), Phase::Search { hits, .. } if hits.is_empty()));
    player.tap(Btn::B);
    assert!(matches!(player.session.app().phase(), Phase::Shelf));
    assert_eq!(player.session.app().shelf_key(), shelf);
    assert_eq!(player.session.app().selected_stem(), Some(stem.as_str()));

    player.open_search();
    player.tap(Btn::Y);
    assert!(matches!(player.session.app().phase(), Phase::Shelf));
    assert!(player.session.app().keyboard().is_none());
    assert_eq!(player.session.app().shelf_key(), shelf);
    assert_eq!(player.session.app().selected_stem(), Some(stem.as_str()));
}

#[test]
fn search_preserves_default_and_remapped_remove_recent() {
    let card = card();
    let state = SlotState {
        show_recents: true,
        recents: vec![Recent {
            platform: Platform::Gba,
            stem: "Comet".into(),
        }],
        ..SlotState::default()
    };
    let mut player = Player::boot(&card, state.clone());
    assert_eq!(player.session.app().shelf_key(), "recents");
    player.tap(Btn::X);
    assert_eq!(player.session.app().shelf_key(), "gba");

    let mut carousel = CarouselKeys::default();
    carousel.set(CarouselAction::ForgetRecent, Binding::plain(Device::B));
    write_carousel(card.path(), &carousel).unwrap();
    let mut player = Player::boot(&card, state);
    player.open_search();
    player.tap(Btn::Y);
    player.tap(Btn::X);
    assert_eq!(player.session.app().shelf_key(), "recents");
    player.tap(Btn::B);
    assert_eq!(player.session.app().shelf_key(), "gba");
}

#[test]
fn search_respects_shelf_access_restrictions() {
    let card = card();
    let mut kid = Player::boot(
        &card,
        SlotState {
            kid_mode: KidMode::OneShelf,
            ..SlotState::default()
        },
    );
    kid.tap(Btn::Menu);
    assert!(matches!(kid.session.app().phase(), Phase::Shelf));
    assert!(kid.session.app().keyboard().is_none());

    let mut collections = Player::boot(
        &card,
        SlotState {
            shelf_mode: ShelfMode::Collections,
            ..SlotState::default()
        },
    );
    collections.tap(Btn::Menu);
    collections.tap(Btn::A);
    assert!(!QuickRow::General
        .child()
        .unwrap()
        .shown(&collections.session.app().hidden_rows())
        .contains(&QuickRow::Search));
}
