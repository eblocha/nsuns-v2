use cucumber::{given, then, when};
use hyper::StatusCode;
use nsuns_server::{
    program::model::ProgramSummary,
    program::{model::ReorderSets, router::REORDER_SETS_PATH},
    router::{PROGRAMS_PATH, SETS_PATH},
    sets::model::{CreateSet, Day, Set},
};

use crate::{
    util::{Auth, JsonBody},
    world::NsunsWorld,
};

fn day_from_str(day: &str) -> Day {
    match day {
        "Sunday" => Day::Sunday,
        "Monday" => Day::Monday,
        "Tuesday" => Day::Tuesday,
        "Wednesday" => Day::Wednesday,
        "Thursday" => Day::Thursday,
        "Friday" => Day::Friday,
        "Saturday" => Day::Saturday,
        _ => panic!(r#"day "{day}" not recognized"#),
    }
}

fn sets_for_day(program: &ProgramSummary, day: Day) -> &Vec<Set> {
    match day {
        Day::Sunday => &program.sets_sunday,
        Day::Monday => &program.sets_monday,
        Day::Tuesday => &program.sets_tuesday,
        Day::Wednesday => &program.sets_wednesday,
        Day::Thursday => &program.sets_thursday,
        Day::Friday => &program.sets_friday,
        Day::Saturday => &program.sets_saturday,
    }
}

#[when(regex = r#"I create a "(.*)" set for (\S+)"#)]
#[given(regex = r#"I have a "(.*)" set for (\S+)"#)]
pub async fn create_set(world: &mut NsunsWorld, movement_name: String, day: String) {
    let movement_id = world
        .movement_world
        .movement_by_name(&movement_name)
        .unwrap_or_else(|| panic!(r#"Movement "{movement_name}" not found"#))
        .id;

    let program_id = world.program_world.unwrap_program_meta().id;

    let day = day_from_str(&day);

    let create_set = CreateSet {
        amount: 70.0,
        day,
        description: None,
        movement_id,
        program_id,
        percentage_of_max: None,
        reps: None,
        reps_is_minimum: false,
    };

    let res = world
        .client
        .post(SETS_PATH)
        .json_body(&create_set)
        .authed(world)
        .send()
        .await;

    assert_eq!(StatusCode::CREATED, res.status());
}

#[then(regex = r"My program has (\[.*\]) on (\S+)")]
pub async fn test_sets_for_day(world: &mut NsunsWorld, movement_names: String, day: String) {
    let names: Vec<String> = serde_json::from_str(&movement_names)
        .unwrap_or_else(|_| panic!("Could not deserialize {movement_names} into array of strings"));

    let actual_names: Vec<_> = sets_for_day(
        world.program_world.unwrap_program_summary(),
        day_from_str(&day),
    )
    .iter()
    .filter_map(|set| world.movement_world.movement_by_id(set.movement_id))
    .map(|movement| movement.name.clone())
    .collect();

    assert_eq!(names, actual_names);
}

#[when(regex = r"I reorder (\S+) from (\d+) to (\d+)")]
pub async fn reorder_sets(world: &mut NsunsWorld, day: String, from: usize, to: usize) {
    let program_id = world.program_world.unwrap_program_meta().id;

    let res = world
        .client
        .post(&format!("{PROGRAMS_PATH}{REORDER_SETS_PATH}"))
        .json_body(&ReorderSets {
            day: day_from_str(&day),
            from,
            to,
            program_id,
        })
        .authed(world)
        .send()
        .await;

    assert_eq!(StatusCode::OK, res.status());
}
