use hecs::*;
use rand::{thread_rng, Rng};
use std::io;

/*
 Simple simulation
 Spawn multiple entities. They have health, damage, position.
 On every tick they: 
	1. move a little
 	2. Find closest entity to themself.
 	3. Fire at it.
 	4. Get damaged by other entities firing at them.
 	5. If health <= 0, the entities die.
 Use print to describe state after every tick
*/

#[derive(Debug)]
struct Position {
	x: i32,
	y: i32,
}

#[derive(Debug)]
struct Health {
	hp: i32
}

#[derive(Debug)]
struct Speed {
	speed: i32
}

#[derive(Debug)]
struct Damage {
	dmg: i32
}


fn manhattan_dist(x0: i32, x1: i32, y0: i32, y1: i32) -> i32{
	let dx = (x0 - x1).max(x1 - x0);
	let dy = (y0 - y1).max(y1 - y0);
	return dx + dy

}

fn spawn_enitites(world: &mut World, n: usize){

	let mut rng = thread_rng();

	for _ in 0..n{
		let pos = Position{
			x: rng.gen_range(-10, 10),
			y: rng.gen_range(-10, 10),
		};
		let s =  Speed{speed: rng.gen_range(1, 5)};
		let hp = Health{hp: rng.gen_range(30, 50)};
		let dmg = Damage{dmg: rng.gen_range(1, 10)};

		world.spawn((pos, s, hp, dmg));
	}
}

fn move_system(world: &mut World){

	let mut rng = thread_rng();

	for (id, (pos, s)) in &mut  world.query::<(&mut Position, &Speed)>() {
		let change = (
			rng.gen_range(-s.speed, s.speed),
			rng.gen_range(-s.speed, s.speed)
			);
		pos.x += change.0;
		pos.y += change.1;
		println!("Unit {:?} moved to {:?}",id, pos);
	}

}

fn fire_at_closest(world: &mut World){
	// In this system entities find the closest entity and fire at them
	for (id0, (pos0, dmg0)) in &mut  world.query::<With<Health, (&Position, &Damage)>>(){

		let mut min_dist: Option<i32> = None;
		let mut closest_id: Option<Entity> = None;

		// find closest:
		for (id1, pos1) in &mut  world.query::<With<Health, &Position>>(){
			if id0 != id1 {
				let dist = manhattan_dist(pos0.x, pos1.x, pos0.y, pos1.y);
				match min_dist {
					None => {min_dist = Some(dist);},
					Some(mut _dist0) => {
						_dist0 = _dist0.min(dist);
					},
				}
				if Some(dist) == min_dist {
					closest_id = Some(id1);
					}
			}

		}

		if !closest_id.is_some(){
			println!("{:?} is the last survivor!", id0);
			return;
		}

		// deal damage:
/*
		//like this
		let mut hp1 = world.query_one::<&mut Health>(closest_id.unwrap()).unwrap();
        let hp1 = hp1.get().unwrap();
*/
		//Or like this:
		let mut hp1 = world.get_mut::<Health>(closest_id.unwrap()).unwrap();
		hp1.hp = hp1.hp - dmg0.dmg;

		println!("{:?} health is now {:?}",closest_id ,hp1.hp);
	}
}

fn remove_dead(world: &mut World) {
	// Here we query entities with 0 or less hp and despawn them
	let mut to_remove: Vec<Entity> = Vec::new();
	for (id, hp) in &mut  world.query::<&Health>(){
		if hp.hp <= 0 {
			to_remove.push(id);
		}
	}

	for i in 0..to_remove.len(){
		world.despawn(to_remove[i]).unwrap();
		println!("Entity {:?} has died!", to_remove[i]);
	}
}

fn get_world_state(world: &mut World) {
	for (id, (hp, pos, dmg)) in &mut  world.query::<
		(&Health, 
		&Position, 
		&Damage)
		>() {
			println!("Entity stats:");
			println!("ID: {:?}, {:?}, {:?}, {:?}",id, hp, dmg, pos);
		}
}

fn main() {

	let mut world = World::new();

// Spawn entity without health.
	let mut rng = thread_rng();
	world.spawn((Position{
			x: rng.gen_range(-10, 10),
			y: rng.gen_range(-10, 10),
		},Speed{speed: rng.gen_range(1, 5)}));

	spawn_enitites(&mut world, 20);

	'running: loop{

		move_system(&mut world);
		fire_at_closest(&mut world);
		remove_dead(&mut world);

		println!("Enter to continue, '?' for enity list, 'q' to quit");
		let mut input = String::new();
    	io::stdin().read_line(&mut input).unwrap();
    	match input.trim() {
    		"q" => break 'running,
    		"?" => {
    			get_world_state(&mut world);
    		},
    		_ => {},
    	}
	}

}
