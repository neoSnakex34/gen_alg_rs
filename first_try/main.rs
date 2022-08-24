use rand::{Rng, seq::SliceRandom};


fn main() {

    let target: [u8; 10] = [1,2,3,4,5,6,7,8,9,10];
    let size: usize = 3;
    let half = size/2;
    let mut population = generate_population(size);
    population.sort_by_key(|m| fitness(*m, target));
    let mut best = population[0];

    while best != target{
        population.drain(half..);
        repopulate(&mut population, size-half);
        population.sort_by_key(|m| fitness(*m, target));
        best = population[0];
        println!("{:?}", fitness(best, target))
    }

    println!("{:?}", best);
}

fn generate_population(size: usize) -> Vec<[u8; 10]>{

    let mut rng = rand::thread_rng();
    let mut population = Vec::new();

    for i in 0..size{
        
        let mut member = [0;10];
        for j in 0..member.len(){
            member[j] = rng.gen();
        }
        population.push(member);

    }

    population

} 

fn fitness(member: [u8; 10], target: [u8; 10]) -> usize{

    let mut fitness = 0;
    

    for i in 0..member.len(){
        fitness += (target[i] as isize - member[i] as isize).abs() as usize;
    }

    fitness

}

fn repopulate(population: &mut Vec<[u8; 10]>, difference: usize) {

    let mut rng = rand::thread_rng();
    
    for _ in 0..difference{
        let m1 = population.choose(&mut rng).unwrap();
        let m2 = population.choose(&mut rng).unwrap();
        let mut new_member = breed(*m1, *m2);

        mutate(&mut new_member, 0.05);
        population.push(new_member);
    }

}

fn breed(m1: [u8;10], m2: [u8;10]) -> [u8; 10]{

    let mut child = [0;10];
    let mut rng = rand::thread_rng();

    for i in 0..child.len(){
        if rng.gen(){
            child[i] = m1[i]
        } else {
            child[i] = m2[i]
        }
    }


    child

}

fn mutate(member: &mut[u8; 10], mutation_rate: f32) {

    let mut rng = rand::thread_rng();
    for i in 0..member.len(){

        if rng.gen::<f32>() < mutation_rate{
            member[i] = rng.gen()
        }

    }

}