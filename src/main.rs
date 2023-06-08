use anyhow::Result;
use constants::N_TURNS;
use population::Population;

mod constants;
mod entities;
mod population;
mod gui;

fn main() -> Result<()> {
    let mut history = vec![];

    (0..N_TURNS).fold(Population::new(), |pop, turn_idx| {
        history.push(pop.clone());
        println!("turn number {}: num of entities: {}", turn_idx, pop.entities.len());
        let g = pop.entities.iter().max_by(|e1, e2| e1.position.x.cmp(&e2.position.x));
        println!("{g:#?}");
        pop.advance()
    });

    history
        .iter()
        .for_each(|point| println!("{:?}\n", point.entities[0]));

    Ok(())
}
