#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Element {
    FIRE,
    COLD,
    SLASHING,
    BLUDGEONING,
    RADIATION,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Army {
    size: u64,
    hit_points: u64,
    immune: Vec<Element>,
    weak: Vec<Element>,
    attack: u64,
    attack_type: Element,
    initiative: u64,
}

impl Army {
    fn effective_power(&self) -> u64 {
        self.size * self.attack
    }

    fn damage_to(&self, other: &Army) -> u64 {
        if other.immune.contains(&self.attack_type) {
            return 0;
        }

        if other.weak.contains(&self.attack_type) {
            return 2 * self.effective_power();
        }

        self.effective_power()
    }

    fn deal_damage(&self, other: &mut Army) {
        let damage = self.damage_to(&other);
        let unit_killed: u64 = damage / other.hit_points;
        other.size = other.size.saturating_sub(unit_killed);
        println!("{:?} deals {} damage to {:?}", self, damage, other);
    }
}

fn combat_to_death(s: &mut Vec<Army>, i: &mut Vec<Army>) -> usize {
    loop {
        combat_turn(s, i);
        if s.len() == 0 || i.len() == 0 {
            break;
        }
    }

    s.len().max(i.len())
}

fn combat_target(from: &Vec<Army>, to: &Vec<Army>) -> Vec<Option<usize>> {
    let mut attack_targets = vec![None; from.len()];

    for (i, army) in from.iter().enumerate() {
        let candidate_idx = to
            .iter()
            .enumerate()
            .filter(|(_, a)| army.damage_to(&a) > 0)
            // don't attack the same target twice
            .filter(|(idx, _)| !attack_targets.contains(&Some(*idx)))
            .max_by_key(|(_, a)| army.damage_to(&a))
            .map(|(idx, _)| idx);

        println!("{:?} will target {:?}", army, candidate_idx);
        attack_targets[i] = candidate_idx;
    }

    attack_targets
}

fn combat_turn(s: &mut Vec<Army>, i: &mut Vec<Army>) {
    // targeting order
    s.sort_by_key(|a| (a.effective_power(), a.initiative));
    i.sort_by_key(|a| (a.effective_power(), a.initiative));

    let attack_target_s = combat_target(s, i);
    let attack_target_i = combat_target(i, s);

    let mut attacks: Vec<(bool, usize, usize, u64)> = attack_target_s
        .iter()
        .enumerate()
        .filter(|(_, t)| t.is_some())
        .map(|(idx, a)| (true, idx, a.unwrap(), s[idx].initiative))
        .chain(
            attack_target_i
                .iter()
                .enumerate()
                .filter(|(_, t)| t.is_some())
                .map(|(idx, a)| (false, idx, a.unwrap(), i[idx].initiative)),
        )
        .collect();

    // combat order
    attacks.sort_by_key(|&(_is_system, _at, _def, ini)| ini);

    for attack in attacks {
        let (at, def) = if attack.0 == true {
            (&mut s[attack.1], &mut i[attack.2])
        } else {
            (&mut i[attack.1], &mut s[attack.2])
        };

        at.deal_damage(def);
    }

    s.retain(|a| a.size > 0);
    i.retain(|a| a.size > 0);

    println!("system: {:?}", s);
    println!("infection: {:?}", i);
}

fn debug_army() -> (Vec<Army>, Vec<Army>) {
    use self::Element::*;
    let mut system = vec![];
    let mut infection = vec![];

    system.push(Army {
        size: 17,
        hit_points: 5390,
        immune: vec![],
        weak: vec![RADIATION, BLUDGEONING],
        attack: 4507,
        attack_type: FIRE,
        initiative: 2,
    });

    system.push(Army {
        size: 989,
        hit_points: 1274,
        immune: vec![FIRE],
        weak: vec![BLUDGEONING, SLASHING],
        attack: 25,
        attack_type: SLASHING,
        initiative: 3,
    });

    infection.push(Army {
        size: 801,
        hit_points: 4706,
        immune: vec![],
        weak: vec![RADIATION],
        attack: 116,
        attack_type: BLUDGEONING,
        initiative: 1,
    });

    infection.push(Army {
        size: 4485,
        hit_points: 2961,
        immune: vec![RADIATION],
        weak: vec![FIRE, COLD],
        attack: 12,
        attack_type: SLASHING,
        initiative: 4,
    });

    (system, infection)
}

pub fn answer1() {
    println!("Immune System Simulator 20XX (1/2): {:?}", 0);
}

#[test]
fn test_targeting() {
    let (mut s, mut i) = debug_army();
    s.sort_by_key(|a| (a.effective_power(), a.initiative));
    i.sort_by_key(|a| (a.effective_power(), a.initiative));

    let target_s = combat_target(&mut s, &mut i);
    assert_eq!(Some(1), target_s[0]);
    assert_eq!(Some(0), target_s[1]);

    let target_i = combat_target(&mut i, &mut s);
    assert_eq!(Some(0), target_i[0]);
    assert_eq!(Some(1), target_i[1]);
}

#[test]
fn test_combat_turn() {
    let (mut s, mut i) = debug_army();

    combat_turn(&mut s, &mut i);

    assert_eq!(1, s.len());
    assert_eq!(2, i.len());
}

#[test]
fn test_combat_to_death() {
    let (mut s, mut i) = debug_army();

    assert_eq!(2, combat_to_death(&mut s, &mut i));

    assert_eq!(0, s.len());
    assert_eq!(2, i.len());
}
