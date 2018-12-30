enum Element {
    FIRE,
    COLD,
    SLASHING,
    BLUDGEONING,
    RADIATION,
}

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

fn debug_army() -> Vec<Army> {
    use self::Element::*;
    let mut armies = vec![];

    armies.push(Army {
        infection: false,
        size: 17,
        hit_points: 5390,
        immune: vec![],
        weak: vec![RADIATION, BLUDGEONING],
        attack: 4507,
        attack_type: FIRE,
        initiative: 2,
    });

    armies.push(Army {
        infection: false,
        size: 989,
        hit_points: 1274,
        immune: vec![FIRE],
        weak: vec![BLUDGEONING, SLASHING],
        attack: 25,
        attack_type: SLASHING,
        initiative: 3,
    });

    armies.push(Army {
        infection: true,
        size: 801,
        hit_points: 4706,
        immune: vec![],
        weak: vec![RADIATION],
        attack: 116,
        attack_type: BLUDGEONING,
        initiative: 1,
    });

    armies.push(Army {
        infection: true,
        size: 4485,
        hit_points: 2961,
        immune: vec![RADIATION],
        weak: vec![FIRE, COLD],
        attack: 12,
        attack_type: SLASHING,
        initiative: 4,
    });

    armies
}

pub fn answer1() {
    let armies = debug_army();

    println!("Immune System Simulator 20XX (1/2): {:?}", 0);
}
