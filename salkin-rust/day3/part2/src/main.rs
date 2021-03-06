#[macro_use]
extern crate clap;
use clap::App;

use std::ops::Add;
use std::io::prelude::*;
use std::fs::File;
use std::io::{self, BufReader};

extern crate input;
use input::filereader;



fn main() {


    let mut v = filereader::read_file("input.txt");
    let mut count = 0;
    while v.len() > 0 {
        let line1 = v.pop().unwrap();
        let lin1: Vec<&str> = line1.split_whitespace().collect();

        let line2 = v.pop().unwrap();
        let lin2: Vec<&str> = line2.split_whitespace().collect();
        let line3 = v.pop().unwrap();
        let lin3: Vec<&str> = line3.split_whitespace().collect();
        for i in 0..3 {
            let mut st = String::new();
            st.push_str(lin1[i]);
            st.push_str(" ");
            st.push_str(lin2[i]);
            st.push_str(" ");
            st.push_str(lin3[i]);
            let is = is_triangle(st);
        //println!("possible triangle {}", is);
             if is {
                count += 1;
            }
    
        }
    }   
    println!("Amount of possible triangles: {}", count)
}

fn is_triangle(s: String) -> bool {

    let sides: Vec<&str> = s.split_whitespace().collect();

    if sides.len() != 3 {
        println!("Faulty line");
        return false;
    }
    
    let mut v: Vec<u32> = Vec::new();
       for s in sides  {
           let i = match s.parse::<u32>() {
               Ok(val) => val,
               Err(..) => 0,
           };
           v.push(i);
       }
       for n in 0..3 {
           let side = v[n];
           for i in 0..3 {
               if i == n {
                   continue;
               }
               let comp;
               if i+1 == n {
                  comp = i+2;
               } else {
                   comp = i+1;
               }
               if comp >= v.len() {
                  break;
               }
               if side >= (v[i] + v[comp]) {
                   return false;
               }
               
           }
       }
       return true;

}
