mod histogram;  //Imports histogram module
use histogram::Histogram;   //Imports Histogram from histogram module
use std::io;    //Used for input
use std::io::Write; //Used for output
use std::collections::HashMap;  //Used for storing frequencies
use std::hash::{Hash, Hasher};  //Used for custom tuple hashing

static SUBLOG_BITS: u64 = 8;

struct Pair (char, char);  //Custom tuple struct for pair frequencies

impl Hash for Pair //Makes custom tuple communative
{
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        ((self.0 as usize * self.1 as usize) | (self.0 as usize + self.1 as usize)).hash(state);    //Creates unique but symmetrical hash
    }
}

impl PartialEq for Pair    //Defines equality for custom tuple
{
    fn eq(&self, other: &Pair) -> bool //Checks for symmetrical equality
    {
        (self.0 as usize * self.1 as usize) | (self.0 as usize + self.1 as usize) == (other.0 as usize * other.1 as usize) | (other.0 as usize + other.1 as usize)
    }
}

impl Eq for Pair
{

}

impl Clone for Pair //Defines copying for custom tuple
{
    fn clone(&self) -> Pair //Returns a copy of the custom tuple
    {
        Pair(self.0, self.1)
    }
}

fn get_trace() -> Vec<char> //Retrieves trace
{
    let mut trace: Vec<char> = Vec::new();

    loop    //Loops until valid trace is inputed
    {
        print!("Please input a trace: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_string();

        let mut valid = true;
        for c in input.chars()  //Makes sure trace is alphabetic, ignores spaces
        {
            if c.is_ascii_alphabetic()
            {
                trace.push(c);
            }
            else if c != ' '
            {
                println!("Invalid Trace");
                valid = false;
                break;
            }
        }

        if trace.len() < 3  //Makes sure the trace has at least three elements
        {
            println!("Invalid Trace");
            valid = false;
        }

        if valid
        {
            break;
        }
    }

    trace
}

fn get_histograms(trace: &Vec<char>) -> (HashMap<char, Histogram <(u64, u64, u64, u64)>>, HashMap<Pair, Histogram <(u64, u64, u64, u64)>>, HashMap<Pair, Histogram <(u64, u64, u64, u64)>>)    //Generates histograms to calculate frequencies
{
    let mut last_seen_single: HashMap<char, usize> = HashMap::new();   //Stores the last access time that a trace element is seen
    let mut last_seen_pair: HashMap<Pair, usize> = HashMap::new();  //Stores the last access time of the first elment of a switch
    let mut reuse_times: HashMap<char, Histogram <(u64, u64, u64, u64)>> = HashMap::new(); //Stores the reuse times of each trace element
    let mut switch_times: HashMap<Pair, Histogram <(u64, u64, u64, u64)>> = HashMap::new();    //Stores the switch times of each trace pair
    let mut inter_switch_times: HashMap<Pair, Histogram <(u64, u64, u64, u64)>> = HashMap::new();    //Stores the inter-switch times of each trace pair

    for i in 0 .. trace.len()   //Iterates through trace
    {
        let c = trace [i];  //Gets trace element

        if !reuse_times.contains_key(&c)    //Checks if the element has a reuse time
        {
            reuse_times.insert(c, Histogram::new_tuple(SUBLOG_BITS, trace.len() as u64));  //Creates a new Histogram for the element
            reuse_times.get_mut(&c).unwrap().add((i + 1) as u64);   //Adds current time to character reuse time histogram
        }
        else
        {
            let rt: u64 = ((i + 1) - last_seen_single.get(&c).unwrap()) as u64;  //Sets the current reuse time to be the difference between the current access time and the last access time this trace element was accessed
            reuse_times.get_mut(&c).unwrap().add(rt);  //Adds reuse time to histogram
        }

        for j in last_seen_single.keys()   //Iterates through the last seen elements
        {
            if *j != c  //Makes sure that an element is not compared with itelf
            {
                let st = (i + 1) - last_seen_single.get(j).unwrap();   //Sets the switch time to be the difference between the current access time and the last access time the other trace element was seen

                if !last_seen_single.contains_key(&c) || st < (i + 1) - *last_seen_single.get(&c).unwrap()    //Checks if the current trace element has not been seen before or if the switch time is smaller than the current reuse time
                {
                    let p = Pair(c, *j);    //Stores the current element pair

                    if !switch_times.contains_key(&p)    //Checks if the pair has a switch time
                    {
                        switch_times.insert(p.clone(), Histogram::new_tuple(SUBLOG_BITS, trace.len() as u64));
                    }

                    switch_times.get_mut(&p).unwrap().add(st as u64); //Adds to the switch time's bucket's frequency
                }
            }
        }

        last_seen_single.insert(c, i + 1); //Updates last seen time for current trace element with current access time
    }

    for c in last_seen_single.keys()    //Adds time to end of trace to each character's reuse time
    {
        reuse_times.get_mut(&c).unwrap().add((trace.len() + 1 - *last_seen_single.get(c).unwrap()) as u64);
    }

    (reuse_times, switch_times, inter_switch_times)  //Returns histograms
}

fn get_size(trace_length: usize, start: usize) -> usize   //Inputs time window size
{
    loop    //Makes sure size is valid for trace
    {
        loop    //Makes sure size is a valid usize and is valid for trace
        {
            print!("Please input a window size: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .expect("Failed to read line");
            
            let input: usize = match input.trim().parse()
            {
                Ok(int) => int,
                Err(_) => 
                {
                    println!("Invalid Input");
                    continue;
                }
            };
            
            if !(input > (trace_length - start) || input <= 1)   //Makes sure window size is not larger than the trace or less than 2
            {
                return input;
            }
            println!("Invalid Input");
        }
    }
}

fn get_single_frequencies(reuse_times: HashMap<char, Histogram <(u64, u64, u64, u64)>>, window_size: usize, trace_length: usize) -> HashMap<char, usize>    //Generates single frequencies using a reusr time histogram, first seen indexes, last seen indexes, time window size, and trace length
{
    let mut single_frequencies: HashMap<char, usize> = HashMap::new();  //Stores the single frequencies
    let total_windows = trace_length - window_size + 1; //Sets the total windows to one more than the difference between the trace length and time window size

    for i in reuse_times.keys() //Iterates through reuse time histogram
    {
        single_frequencies.insert(*i, total_windows);   //Inserts total windows into each character frequency
        for j in reuse_times.get(i).unwrap().get_values()    //Iterates through reuse times of each trace character
        {
            let rt = j.0;

            if rt == 0
            {
                continue;
            }

            let f = reuse_times.get(i).unwrap().get_tuple(rt);   //Retrieves reuse time frequency
            if rt > window_size as u64    //Checks if the reuse time is larger than the window size
            {
                let mut temp = single_frequencies.get(&i).unwrap().clone(); //Retrieves old window count
                temp = temp - (j.2 - window_size as u64 * j.3) as usize;   //Subtraccts off the frequency multiplied by the difference between the reuse time and window size
                single_frequencies.insert(*i, temp);    //Writes new window count
            }
        }
    }

    single_frequencies  //Returns single frequencies
}

fn get_joint_frequenies()
{

}

fn main()
{
    println!("\nThis program calculates affinity for a given trace and window size.");

    let trace = get_trace();    //Gets trace from user

    let times = get_histograms(&trace); //Generates histograms

    //Prints histograms
    println!("\nReuse Time Histogram:");
    for c in times.0.keys()
    {
        println!("\n{}:", c);
        for s in times.0.get(c).unwrap().get_values()
        {
            if s != (0, 0, 0, 0)
            {
                println!("{} - {}: ({}, {})", s.0, s.1, s.2, s.3);
            }
        }
    }

    println!("\nSwitch Time Histogram:");
    for c in times.1.keys()
    {
        println!("\n({}, {}):", c.0, c.1);
        for s in times.1.get(c).unwrap().get_values()
        {
            if s != (0, 0, 0, 0)
            {
                println!("{} - {}: ({}, {})", s.0, s.1, s.2, s.3);
            }
        }
    }

    println!("\nInter-Switch Time Histogram:");
    for c in times.2.keys()
    {
        println!("\n({}, {}):", c.0, c.1);
        for s in times.2.get(c).unwrap().get_values()
        {
            if s != (0, 0, 0, 0)
            {
                println!("{} - {}: ({}, {})", s.0, s.1, s.2, s.3);
            }
        }
    }

    println!();
    let window_size = get_size(trace.len(), 1); //Gets time window size from user

    let single_frequencies = get_single_frequencies(times.0, window_size, trace.len());   //Calculates single frequencies

    println!("\nSingle Frequencies:\n");
    for c in single_frequencies.keys()  //Prints single frequencies
    {
        println!("{}: {}", c, single_frequencies.get(c).unwrap());
    }
}