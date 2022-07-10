use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{stdout, Write};
use std::time::Instant;

// this abstraction is absolutely necessary, if you remove it it will break EVERYTHING
struct Box {
    slip: u32,
}

impl Box {
    fn new(slip: u32) -> Self {
        Self { slip }
    }
}

// Rooms contains boxes, everyone knows thats how prisons work
struct Room {
    boxes: Vec<Box>,
}

impl Room {
    // shuffle the boxes, amazons warehouses use my code if you ever wondered why your packages are always delivered on time
    // this was actually from stack overflow https://stackoverflow.com/a/26035435/16106482
    fn new(count: u32) -> Self {
        let mut slips: Vec<u32> = (0..count).collect();
        slips.shuffle(&mut thread_rng());

        let mut boxes: Vec<Box> = vec![];

        for v in &slips {
            boxes.push(Box::new(*v));
        }

        Self { boxes }
    }
}

// prisoners are just numbers?                      unsigned numbers actually
struct Prisoner {
    id: u32,
}

impl Prisoner {
    fn new(id: u32) -> Self {
        Self { id }
    }
}

// we all know Prisons only need Rooms and prisoners, nothing else
struct Prison {
    room: Room, // just one room actually
    prisoners: Vec<Prisoner>,
}

impl Prison {
    // create a new instance of the prison with LOADS of prisoners
    fn new(count: u32) -> Self {
        let room = Room::new(count);
        let mut prisoners: Vec<Prisoner> = vec![];

        for i in 0..count {
            prisoners.push(Prisoner::new(i));
        }

        Self { room, prisoners }
    }

    // find the prisoners id by starting at the box their id corresponds to, then following the chain
    fn get_slip_depth(&self, box_id: u32, prisoner_id: u32, mut depth: u64) -> u64 {
        depth += 1;

        // get the id of the slip in the box
        let slip_id = self.room.boxes[box_id as usize].slip;

        // we are not allowed to open more than half of the boxes, prison rules am i right?
        if slip_id != prisoner_id && depth <= self.prisoners.len() as u64 / 2 {
            // now do it again, but recursively
            depth = self.get_slip_depth(slip_id, prisoner_id, depth);
        }

        // return the depth
        depth
    }

    fn solve(&self) -> bool {
        let mut results: Vec<bool> = vec![];

        // now simulate the prisoners doing their search for their slip
        for prisoner in &self.prisoners {
            // if the slip depth is less than or equal to half the prisons population then we were successful
            if self.get_slip_depth(prisoner.id, prisoner.id, 0) < self.prisoners.len() as u64 / 2 {
                results.push(true);
            } else {
                // otherwise cull the prisoners, this happens 69% of the time ;)
                results.push(false);
            }
        }

        // if the count of successful slip finds matches the prison population then the prisoners
        // managed to escape using a method only people with an IQ of 140+ manage to find
        results.iter().filter(|x| **x).count() == self.prisoners.len()
        //                                ^^^
        //                                 |
        // how about that double deref though, damn
    }
}

// this is black magic to me, credits to stack overflow... of course... https://stackoverflow.com/a/59890400/16106482
fn overwrite_stdout(message: String) {
    let mut stdout = stdout();

    stdout.execute(cursor::Hide).unwrap();
    stdout.queue(cursor::SavePosition).unwrap();
    stdout.write_all(message.as_bytes()).unwrap();
    stdout.queue(cursor::RestorePosition).unwrap();
    stdout.flush().unwrap();
    stdout.queue(cursor::RestorePosition).unwrap();
    stdout
        .queue(terminal::Clear(terminal::ClearType::FromCursorDown))
        .unwrap();
    drop(message);
}

fn main() {
    // the success rate after we have tried a few times, hopefully close to 30.5%
    let mut success_count: u32 = 0;
    let mut total_time_taken = 0;

    // try a couple of times...
    let iterations = 100_000;

    // we love prisons so lets create one hundred thousand prisons
    for i in 0..iterations {
        // we love prisoners so lets create one thousand of them
        let prison = Prison::new(1_000);

        // lets measure how long it took
        let now = Instant::now();

        // start the sick game
        let result = prison.solve();

        // obviously it was blazingly fast... its rust after all
        total_time_taken += now.elapsed().as_micros();

        // increment the success counter if they escaped
        if result {
            success_count += 1;
        }

        // well because rust is blazingly fast, we can afford to convert from u128 to f64
        #[allow(clippy::cast_precision_loss)]
        let average_iteration_time = total_time_taken as f64 / f64::from(i);
        let success_chance = f64::from(success_count) / f64::from(i) * 100.0;
        let message = format!("{success_chance:.3}% success chance\n{i}/{iterations} iterations\n{average_iteration_time:.3}μs average iteration time");

        // this is just to help overwrite stdout so we dont bombard the terminal
        overwrite_stdout(message);
    }

    #[rustfmt::skip]
    /*|--------------self documenting code by the way, and perfectly formatted too--------------|*/
    print!( "Final success chance: {}%", f64::from(success_count) / f64::from(iterations) * 100.0);
    /*|-----------------------------------------------------------------------------------------|*/
}
