use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{stdout, Write};
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct Box {
    slip: u32,
}

impl Box {
    fn new(slip: u32) -> Self {
        Self { slip }
    }
}

#[derive(Debug, Clone)]
struct Room {
    boxes: Vec<Box>,
}

impl Room {
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

#[derive(Debug)]
struct Prisoner {
    id: u32,
}

impl Prisoner {
    fn new(id: u32) -> Self {
        Self { id }
    }
}

#[derive(Debug)]
struct Prison {
    room: Room,
    prisoners: Vec<Prisoner>,
}

impl Prison {
    fn new(count: u32) -> Self {
        let room = Room::new(count);
        let mut prisoners: Vec<Prisoner> = vec![];

        for i in 0..count {
            prisoners.push(Prisoner::new(i));
        }

        Self { room, prisoners }
    }

    fn get_slip_depth(&self, box_id: u32, prisoner_id: u32, mut depth: u64) -> u64 {
        depth += 1;
        let slip_id = self.room.boxes[box_id as usize].slip;

        if slip_id != prisoner_id && depth <= self.prisoners.len() as u64 / 2 {
            depth = self.get_slip_depth(slip_id, prisoner_id, depth);
        }

        depth
    }

    fn solve(&self) -> bool {
        let mut results: Vec<bool> = vec![];

        for prisoner in &self.prisoners {
            if self.get_slip_depth(prisoner.id, prisoner.id, 0) < self.prisoners.len() as u64 / 2 {
                results.push(true);
            } else {
                results.push(false);
            }
        }

        results.iter().filter(|x| **x).count() == self.prisoners.len()
    }
}

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
    let mut success_count: u32 = 0;
    let mut total_time_taken = 0;
    let iterations = 100_000;

    for i in 0..iterations {
        let prison = Prison::new(1_000);
        let now = Instant::now();
        let result = prison.solve();
        total_time_taken += now.elapsed().as_micros();

        if result {
            success_count += 1;
        }

        #[allow(clippy::cast_precision_loss)]
        let average_iteration_time = total_time_taken as f64 / f64::from(i);
        let success_chance = f64::from(success_count) / f64::from(i) * 100.0;
        let message = format!("{success_chance:.3}% success chance\n{i}/{iterations} iterations\n{average_iteration_time:.5}μs average iteration time");

        overwrite_stdout(message);
    }

    print!(
        "Final success chance: {}%",
        f64::from(success_count) / f64::from(iterations) * 100.0
    );
}
