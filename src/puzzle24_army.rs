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
    infection: bool,
    size: u64,
    hit_points: u64,
    immune: Vec<Element>,
    weak: Vec<Element>,
    attack: u64,
    attack_type: Element,
    initiative: u64,
}

impl Army {
    fn parse(s: &str, infection: bool) -> Army {
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
            infection,
            size,
            hit_points,
            immune,
            weak,
            attack,
            attack_type,
            initiative,
        }
    }

    fn parse_army(s: &str, is_infection: bool) -> Vec<Army> {
        s.split("\n")
            .filter(|l| l.len() > 0)
            .skip(1)
            .map(|l| Army::parse(l, is_infection))
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
        if self.size > 0 {
            let damage = self.damage_to(&other);
            let unit_killed: u64 = damage / other.hit_points;
            // println!(
            //     "{:?} deals {} damage (kill {:?} units) to {:?}",
            //     self, damage, unit_killed, other
            // );
            other.size = other.size.saturating_sub(unit_killed);
        }
    }

    fn boost_damage(&mut self, boost: u64) {
        self.attack += boost;
    }
}

fn parse_armies(s: &str) -> Vec<Army> {
    let mut teams = s.split("\n\n");
    let mut armies = Army::parse_army(teams.next().unwrap(), false);
    armies.extend(Army::parse_army(teams.next().unwrap(), true).into_iter());

    armies
}

fn combat_to_death(armies: &mut Vec<Army>) -> (u64, u64) {
    let mut previous_state = None;
    loop {
        combat_turn(armies);

        let score_s: u64 = armies.iter().filter(|a| !a.infection).map(|a| a.size).sum();
        let score_i: u64 = armies.iter().filter(|a| a.infection).map(|a| a.size).sum();

        let current_state = (score_s, score_i);

        if Some(current_state) == previous_state || score_s == 0 || score_i == 0 {
            return (score_s, score_i); //break if dead lock, or if there is a winner
        }

        previous_state = Some(current_state);
    }
}

fn combat_target(armies: &Vec<Army>) -> Vec<Option<usize>> {
    let mut attack_targets = vec![None; armies.len()];

    let mut orders: Vec<usize> = (0..=armies.len() - 1).collect();
    orders.sort_by_key(|&idx| {
        let a = &armies[idx];
        (-(a.effective_power() as i64), -(a.initiative as i64))
    });

    for o in orders {
        let candidate_idx = armies
            .iter()
            .enumerate()
            // don't attach the same side
            .filter(|(idx, _)| armies[o].infection != armies[*idx].infection)
            // don't attack the same target twice
            .filter(|(idx, _)| !attack_targets.contains(&Some(*idx)))
            .filter(|(_, a)| armies[o].damage_to(&a) > 0)
            .max_by_key(|(_, a)| (armies[o].damage_to(&a), a.effective_power(), a.initiative))
            .map(|(idx, _)| idx);

        //println!("{:?} will target {:?}", armies[o], candidate_idx);
        attack_targets[o] = candidate_idx;
    }

    attack_targets
}

fn combat_turn(armies: &mut Vec<Army>) {
    let attack_targets = combat_target(armies);

    let mut attacks: Vec<(usize, usize, u64)> = attack_targets
        .iter()
        .enumerate()
        .filter(|(_, t)| t.is_some())
        .map(|(idx, a)| (idx, a.unwrap(), armies[idx].initiative))
        .collect();

    // combat order
    attacks.sort_by_key(|&(_at, _def, ini)| -(ini as i64));
    attacks.iter_mut().for_each(|&mut (at, def, _)| {
        let at = armies[at].clone(); // seems dirty...
        let def = &mut armies[def];
        at.deal_damage(def);
    });

    armies.retain(|a| a.size > 0);
}

fn boost_immune_system(armies: &mut Vec<Army>, boost: u64) {
    armies
        .iter_mut()
        .filter(|a| !a.infection)
        .for_each(|a| a.boost_damage(boost));
}

pub fn answer1() {
    let s = std::fs::read_to_string("input/input24.txt").expect("cannot read file");
    let mut armies = parse_armies(&s);

    let (score_s, score_i) = combat_to_death(&mut armies);

    println!(
        "Day 24: Immune System Simulator 20XX (1/2): {:?}",
        score_s.max(score_i)
    );
}

pub fn answer2() {
    let s = std::fs::read_to_string("input/input24.txt").expect("cannot read file");
    let armies = parse_armies(&s);

    for boost in 0.. {
        let mut boostest_armies = armies.clone();
        //println!("trying boost {}", boost);
        boost_immune_system(&mut boostest_armies, boost);

        let (score_s, score_i) = combat_to_death(&mut boostest_armies);
        if score_i == 0 {
            println!("Day 24: Immune System Simulator 20XX (2/2): {:?}", score_s);
            return;
        }
    }
    unreachable!()
}

#[test]
fn test_parse() {
    let s = std::fs::read_to_string("input/input24_debug.txt").expect("cannot read file");

    let armies = parse_armies(&s);
    println!("armies: {:?}", armies);

    assert_eq!(4, armies.len());
}

#[test]
fn test_targeting() {
    let s = std::fs::read_to_string("input/input24_debug.txt").expect("cannot read file");
    let mut armies = parse_armies(&s);

    let targets = combat_target(&mut armies);
    println!("targets: {:?}", targets);

    assert_eq!(Some(3), targets[0]);
    assert_eq!(Some(2), targets[1]);
    assert_eq!(Some(0), targets[2]);
    assert_eq!(Some(1), targets[3]);
}

#[test]
fn test_combat_turn() {
    let s = std::fs::read_to_string("input/input24_debug.txt").expect("cannot read file");
    let mut armies = parse_armies(&s);

    combat_turn(&mut armies);

    assert_eq!(3, armies.len());
}

#[test]
fn test_combat_to_death() {
    let s = std::fs::read_to_string("input/input24_debug.txt").expect("cannot read file");
    let mut armies = parse_armies(&s);

    assert_eq!((0, 782 + 4434), combat_to_death(&mut armies));
    assert_eq!(2, armies.len());
}
