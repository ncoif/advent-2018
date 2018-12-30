use std::collections::HashMap;

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
    id: u64,
    infection: bool,
    size: u64,
    hit_points: u64,
    immune: Vec<Element>,
    weak: Vec<Element>,
    attack: u64,
    attack_type: Element,
    initiative: u64,
    is_targeted: bool,
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
        other.size -= unit_killed;
        println!("{:?} deals {} damage to {:?}", self, damage, other);
    }
}

fn combat_to_death(armies: &mut Vec<Army>) {
    loop {
        combat_turn(armies);

        let immune_count = armies.iter().filter(|a| !a.infection).count();
        let infection_count = armies.iter().filter(|a| a.infection).count();

        if immune_count == 0 || infection_count == 0 {
            break;
        }
    }
}

fn combat_turn(armies: &mut Vec<Army>) {
    // targeting order
    let mut targets = HashMap::new();
    armies.sort_by_key(|a| (a.effective_power(), a.initiative));

    //FIXME:
    for army in armies {
        let candidate = armies
            .iter()
            .filter(|a| a.infection != army.infection)
            .filter(|a| army.damage_to(&a) > 0)
            .filter(|a| a.is_targeted == true)
            .max_by_key(|a| army.damage_to(&a));

        println!("{:?} will target {:?}", army, candidate);
        if candidate.is_some() {
            let target = candidate.unwrap();
            targets.insert(army.id, target.id);
            target.is_targeted = true;
        }
    }

    // combat order
    armies.sort_by_key(|a| a.initiative);
    for army in armies {
        let target_id = targets.get(&army.id);
        if target_id.is_some() {
            let mut target = armies.iter().find(|a| a.id == *target_id.unwrap()).unwrap();
            army.deal_damage(&mut target);
            // reset target
            target.is_targeted = false;
        }
    }

    armies.retain(|a| a.size > 0);
}

fn debug_army() -> Vec<Army> {
    use self::Element::*;
    let mut armies = vec![];

    armies.push(Army {
        id: 1,
        infection: false,
        size: 17,
        hit_points: 5390,
        immune: vec![],
        weak: vec![RADIATION, BLUDGEONING],
        attack: 4507,
        attack_type: FIRE,
        initiative: 2,
        is_targeted: false,
    });

    armies.push(Army {
        id: 2,
        infection: false,
        size: 989,
        hit_points: 1274,
        immune: vec![FIRE],
        weak: vec![BLUDGEONING, SLASHING],
        attack: 25,
        attack_type: SLASHING,
        initiative: 3,
        is_targeted: false,
    });

    armies.push(Army {
        id: 3,
        infection: true,
        size: 801,
        hit_points: 4706,
        immune: vec![],
        weak: vec![RADIATION],
        attack: 116,
        attack_type: BLUDGEONING,
        initiative: 1,
        is_targeted: false,
    });

    armies.push(Army {
        id: 4,
        infection: true,
        size: 4485,
        hit_points: 2961,
        immune: vec![RADIATION],
        weak: vec![FIRE, COLD],
        attack: 12,
        attack_type: SLASHING,
        initiative: 4,
        is_targeted: false,
    });

    armies
}

pub fn answer1() {
    let armies = debug_army();

    println!("Immune System Simulator 20XX (1/2): {:?}", 0);
}

#[test]
fn test_combat() {
    let mut armies = debug_army();
    combat_to_death(&mut armies);

    assert_eq!(true, false);
}
