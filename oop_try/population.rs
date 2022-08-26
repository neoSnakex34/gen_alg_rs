use std::{ vec::Drain, ops::{Range, RangeFrom},};

use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};
use crate::member::{Member, self};

#[derive(Clone)]
pub struct Population{
    members: Vec<Member>,
}

impl Population {
    
    pub fn new(size: usize, mut rng: &mut ThreadRng) -> Self{
        Self {
            members: 
                (0..size)
                    .map(|_| { 
                        Member::new(&mut rng)
                    })
                    .collect::<Vec<_>>()
            }

    }

    pub fn members(&mut self) -> &mut Vec<Member>{
        &mut self.members
    }
    
    pub fn drain_pop(&mut self, range: RangeFrom<usize>) -> Drain<Member> {
        self.members.drain(range)
    }

    pub fn sort_members(&mut self, target: &Member){
        self.members.sort_by_key(|m| m.fitness(*target))    
    }
   
   

    pub fn breed(p1: Member, p2: Member, mut rng: &mut ThreadRng) -> Member{
        let mut child = Member::new(&mut rng);

        child
            .genes()
            .iter_mut()
            .zip(p1.genes().iter()
                    .zip(p2.genes().iter()))
            .for_each(|(c, (gm1, gm2))|{
                *c = if rng.gen() { *gm1 } else { *gm2 } 
            });
        
        child
    }

    pub fn repopulate(&mut self, difference: usize, mutation_rate: f32, mut rng: &mut ThreadRng) {
        (0..difference)
            .for_each(|_| {
                let m1 = self.members.choose(&mut rng).unwrap();
                let m2 = self.members.choose(&mut rng).unwrap();

                let mut new_member = Self::breed(*m1, *m2, rng);
                
                new_member.mutate(mutation_rate, rng);
                self.members.push(new_member);
            })

    }

}