use std::fmt::Display;

use egui::{ImageSource, Response, Widget};

#[derive(Clone, Copy)]
pub enum Cost {
    FixedCost(u8),
    VariableCost(u8, u8),
}

impl Default for Cost {
    fn default() -> Self {
        Self::FixedCost(0)
    }
}
impl Display for Cost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cost::FixedCost(amount) => write!(f, "{amount}"),
            Cost::VariableCost(min_amount, max_amount) => {
                write!(f, "{min_amount} to {max_amount}")
            }
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct SpellCost {
    pub ap: Cost,
    mp: u8,
    wp: u8,
}
impl Display for SpellCost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.ap, self.mp, self.wp)
    }
}

impl SpellCost {
    fn match_pattern(&self, cost_pattern: &Self) -> bool {
        if self.wp != cost_pattern.wp {
            return false;
        }
        if self.mp != cost_pattern.mp {
            return false;
        }
        match cost_pattern.ap {
            Cost::FixedCost(pattern_amount) => match self.ap {
                Cost::FixedCost(amount) => amount == pattern_amount,
                Cost::VariableCost(min_amount, max_amount) => {
                    pattern_amount >= min_amount && pattern_amount <= max_amount
                }
            },
            Cost::VariableCost(pattern_min_amount, pattern_max_amount) => match self.ap {
                Cost::FixedCost(amount) => {
                    amount >= pattern_min_amount && amount <= pattern_max_amount
                }
                Cost::VariableCost(min_amount, max_amount) => {
                    pattern_max_amount >= min_amount || pattern_min_amount <= max_amount
                }
            },
        }
    }
}

pub struct Spell<'a> {
    pub name: String,
    pub icon: ImageSource<'a>,
    pub cost: SpellCost,
    pub spell_id: u16,
}

impl<'a> Default for Spell<'a> {
    fn default() -> Self {
        Self {
            name: Default::default(),
            icon: ImageSource::Uri("file://../assets/air_spell.png".into()),
            cost: Default::default(),
            spell_id: Default::default(),
        }
    }
}

impl<'a> Display for Spell<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:?}, {}, {}, {})",
            self.icon, self.name, self.cost, self.spell_id
        )
    }
}

impl<'a> Widget for &Spell<'a> {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        ui.image(self.icon.clone())
            .on_hover_text_at_pointer(self.name.clone())
    }
}

pub struct SpellCombo {
    pub name: String,
    pub spell_chain: Vec<SpellCost>,
    pub effect: String,
}

pub struct SpellBook<'a> {
    spells: Vec<Spell<'a>>,
    deck: Vec<u16>,
}

impl<'a> SpellBook<'a> {
    pub fn get_matching_spell(&self, cost: &SpellCost) -> Vec<&Spell> {
        let mut matching_spells = vec![];
        for spell in self.spells.iter() {
            if !self.deck.contains(&spell.spell_id) {
                continue;
            }
            if spell.cost.match_pattern(cost) {
                matching_spells.push(spell);
            }
        }
        matching_spells
    }
}

pub fn get_iop_spells(deck_string: &str) -> SpellBook {
    let mut spell_book = SpellBook {
        spells: Vec::new(),
        deck: vec![0],
    };

    for spell_id_str in deck_string.split("-").collect::<Vec<&str>>().iter() {
        spell_book.deck.push(spell_id_str.parse().unwrap_or(0));
    }

    // Sorts neutres
    spell_book.spells.push(Spell {
        name: "Eventrail".to_owned(),
        icon: egui::include_image!("../assets/eventrail.png"),
        cost: SpellCost {
            ap: Cost::FixedCost(0),
            mp: 1,
            wp: 0,
        },
        spell_id: 0,
    });
    spell_book.spells.push(Spell {
        name: "Uppercut".to_owned(),
        icon: egui::include_image!("../assets/uppercut.png"),
        cost: SpellCost {
            ap: Cost::FixedCost(0),
            mp: 0,
            wp: 1,
        },
        spell_id: 0,
    });

    // Sorts feu
    spell_book.spells.push(Spell {
        name: "Epée céleste".to_owned(),
        icon: egui::include_image!("../assets/epee_celeste.png"),
        cost: SpellCost {
            ap: Cost::FixedCost(2),
            mp: 0,
            wp: 0,
        },
        spell_id: 4778,
    });
    spell_book.spells.push(Spell {
        name: "Fulgur".to_owned(),
        icon: egui::include_image!("../assets/fulgur.png"),
        cost: SpellCost {
            ap: Cost::FixedCost(3),
            mp: 0,
            wp: 0,
        },
        spell_id: 4775,
    });
    spell_book.spells.push(Spell {
        name: "Super Iop Punch".to_owned(),
        icon: egui::include_image!("../assets/fire_spell.png"),
        cost: SpellCost {
            ap: Cost::FixedCost(4),
            mp: 0,
            wp: 0,
        },
        spell_id: 4777,
    });
    spell_book.spells.push(Spell {
        name: "Jugement".to_owned(),
        icon: egui::include_image!("../assets/jugement.png"),
        cost: SpellCost {
            ap: Cost::FixedCost(1),
            mp: 0,
            wp: 0,
        },
        spell_id: 4776,
    });
    spell_book.spells.push(Spell {
        name: "Colère de Iop".to_owned(),
        icon: egui::include_image!("../assets/fire_spell.png"),
        cost: SpellCost {
            ap: Cost::FixedCost(6),
            mp: 0,
            wp: 1,
        },
        spell_id: 4779,
    });

    // Sorts terre
    spell_book.spells.push(Spell {
        name: "Ebranler".to_owned(),
        icon: egui::include_image!("../assets/ebranler.png"),
        cost: SpellCost {
            ap: Cost::FixedCost(2),
            mp: 0,
            wp: 0,
        },
        spell_id: 4780,
    });
    spell_book.spells.push(Spell {
        name: "Roknocerok".to_owned(),
        icon: egui::include_image!("../assets/rocknocerok.png"),
        cost: SpellCost {
            ap: Cost::FixedCost(4),
            mp: 0,
            wp: 0,
        },
        spell_id: 4781,
    });
    spell_book.spells.push(Spell {
        name: "Fendoir".to_owned(),
        icon: egui::include_image!("../assets/fendoir.png"),
        cost: SpellCost {
            ap: Cost::FixedCost(3),
            mp: 0,
            wp: 0,
        },
        spell_id: 4782,
    });
    spell_book.spells.push(Spell {
        name: "Charge".to_owned(),
        icon: egui::include_image!("../assets/charge.png"),
        cost: SpellCost {
            ap: Cost::VariableCost(1, 4),
            mp: 0,
            wp: 0,
        },
        spell_id: 4783,
    });
    spell_book.spells.push(Spell {
        name: "Ravage".to_owned(),
        icon: egui::include_image!("../assets/earth_spell.png"),
        cost: SpellCost {
            ap: Cost::FixedCost(5),
            mp: 0,
            wp: 0,
        },
        spell_id: 4784,
    });

    // Sorts air
    spell_book.spells.push(Spell {
        name: "Jabs".to_owned(),
        icon: egui::include_image!("../assets/jabs.png"),
        cost: SpellCost {
            ap: Cost::FixedCost(3),
            mp: 0,
            wp: 0,
        },
        spell_id: 4785,
    });
    spell_book.spells.push(Spell {
        name: "Rafale".to_owned(),
        icon: egui::include_image!("../assets/rafale.png"),
        cost: SpellCost {
            ap: Cost::FixedCost(1),
            mp: 0,
            wp: 0,
        },
        spell_id: 4786,
    });
    spell_book.spells.push(Spell {
        name: "Torgnole".to_owned(),
        icon: egui::include_image!("../assets/torgnole.png"),
        cost: SpellCost {
            ap: Cost::FixedCost(2),
            mp: 0,
            wp: 0,
        },
        spell_id: 4787,
    });
    spell_book.spells.push(Spell {
        name: "Tannée".to_owned(),
        icon: egui::include_image!("../assets/air_spell.png"),
        cost: SpellCost {
            ap: Cost::FixedCost(4),
            mp: 0,
            wp: 0,
        },
        spell_id: 4788,
    });
    spell_book.spells.push(Spell {
        name: "Epée de Iop".to_owned(),
        icon: egui::include_image!("../assets/air_spell.png"),
        cost: SpellCost {
            ap: Cost::FixedCost(3),
            mp: 0,
            wp: 0,
        },
        spell_id: 4789,
    });

    spell_book
}

pub fn get_iop_combos() -> Vec<SpellCombo> {
    let spell_cost_1_ap = SpellCost {
        ap: Cost::FixedCost(1),
        ..Default::default()
    };
    let spell_cost_2_ap = SpellCost {
        ap: Cost::FixedCost(2),
        ..Default::default()
    };
    let spell_cost_3_ap = SpellCost {
        ap: Cost::FixedCost(3),
        ..Default::default()
    };
    let spell_cost_1_mp = SpellCost {
        mp: 1,
        ..Default::default()
    };
    let spell_cost_1_wp = SpellCost {
        wp: 1,
        ..Default::default()
    };

    vec![
        SpellCombo {
            name: "Combo vol de vie".to_owned(),
            spell_chain: vec![spell_cost_1_mp, spell_cost_3_ap, spell_cost_3_ap],
            effect: "Vole 150% des dégats infligés".to_owned(),
        },
        SpellCombo {
            name: "Combo PA".to_owned(),
            spell_chain: vec![
                spell_cost_1_wp,
                spell_cost_3_ap,
                spell_cost_1_wp,
                spell_cost_1_ap,
            ],
            effect: "2 PA autour du Iop".to_owned(),
        },
        SpellCombo {
            name: "Combo poussé".to_owned(),
            spell_chain: vec![spell_cost_1_ap, spell_cost_1_ap, spell_cost_2_ap],
            effect: "Pousse de 2 cases".to_owned(),
        },
        SpellCombo {
            name: "Combo dégat".to_owned(),
            spell_chain: vec![spell_cost_2_ap, spell_cost_1_ap, spell_cost_1_mp],
            effect: "Dégat bonus".to_owned(),
        },
        SpellCombo {
            name: "Combo préparation".to_owned(),
            spell_chain: vec![spell_cost_1_mp, spell_cost_1_mp, spell_cost_1_wp],
            effect: "Préparation +20".to_owned(),
        },
    ]
}
