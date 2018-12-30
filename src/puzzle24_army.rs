#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Element {
    FIRE,
    COLD,
    SLASHING,
    BLUDGEONING,
    RADIATION,
}

impl Element {
    fn parse(s: &str) -> Element {
        match s {
            "radiation" => Element::RADIATION,
            "bludgeoning" => Element::BLUDGEONING,
            "fire" => Element::FIRE,
            "cold" => Element::COLD,
            "slashing" => Element::SLASHING,
            s => panic!("{:?} is not an element", s),
        }
    }
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
    fn parse(s: &str) -> Army {
        let tokens: Vec<&str> = s.split(" ").collect();
        let size = tokens[0].parse::<u64>().unwrap();
        let hit_points = tokens[4].parse::<u64>().unwrap();
        let initiative = tokens[tokens.len() - 1].parse::<u64>().unwrap();
        let attack = tokens[tokens.len() - 6].parse::<u64>().unwrap();
        let attack_type = Element::parse(tokens[tokens.len() - 5]);

        let mut immune = vec![];
        let mut weak = vec![];
        if s.contains('(') {
            let spec: String = s
                .chars()
                .skip_while(|&c| c != '(')
                .take_while(|&c| c != ')')
                .collect();
            for sub in spec.split("; ") {
                let elements: Vec<_> = sub
                    .split(" ")
                    .skip(2)
                    .map(|e| Element::parse(e.trim_end_matches(|x| ";),".contains(x))))
                    .collect();
                if sub.trim_start_matches("(").starts_with("weak") {
                    weak = elements;
                } else {
                    immune = elements;
                }
            }
        }

        Army {
            size,
            hit_points,
            immune,
            weak,
            attack,
            attack_type,
            initiative,
        }
    }

    fn parse_army(s: &str) -> Vec<Army> {
        s.split("\n")
            .filter(|l| l.len() > 0)
            .skip(1)
            .map(Army::parse)
            .collect()
    }

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

fn parse_armies(s: &str) -> (Vec<Army>, Vec<Army>) {
    let mut teams = s.split("\n\n");
    let s = Army::parse_army(teams.next().unwrap());
    let i = Army::parse_army(teams.next().unwrap());

    (s, i)
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
fn test_parse() {
    let s = std::fs::read_to_string("input/input24_debug.txt").expect("cannot read file");
    let (s, i) = parse_armies(&s);

    println!("s: {:?}", s);
    println!("i: {:?}", i);

    assert_eq!(2, s.len());
    assert_eq!(2, i.len());
}

#[test]
fn test_targeting() {
    let s = std::fs::read_to_string("input/input24_debug.txt").expect("cannot read file");
    let (mut s, mut i) = parse_armies(&s);

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
    let s = std::fs::read_to_string("input/input24_debug.txt").expect("cannot read file");
    let (mut s, mut i) = parse_armies(&s);

    combat_turn(&mut s, &mut i);

    assert_eq!(1, s.len());
    assert_eq!(2, i.len());
}

#[test]
fn test_combat_to_death() {
    let s = std::fs::read_to_string("input/input24_debug.txt").expect("cannot read file");
    let (mut s, mut i) = parse_armies(&s);

    assert_eq!(2, combat_to_death(&mut s, &mut i));

    assert_eq!(0, s.len());
    assert_eq!(2, i.len());
}
