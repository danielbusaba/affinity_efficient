mod histogram;  //Imports histogram module
use histogram::Histogram;   //Imports Histogram from histogram module
use std::io;    //Used for user input
use std::io::Write; //Used for output
use std::env;   //Used for parameters in console input
use std::fs;    //Used for file output
use std::fs::File;  //Used for file input
use std::io::prelude::*;    //Used for file input
use std::collections::HashMap;  //Used for storing frequencies
use std::hash::{Hash, Hasher};  //Used for custom tuple hashing
use std::cmp::Ordering; //Used for affinity sorting
use std::collections::BinaryHeap;   //Stores Priority Queue for affinities
use std::char;

const SUBLOG_BITS: u64 = 8;

struct Pair <T> (T, T);  //Custom tuple struct for pair frequencies

impl Hash for Pair <usize> //Makes custom tuple communative
{
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        let a = self.0;
        let b = self.1;
        if a > b
        {
            a.hash(state);
            b.hash(state);
        }
        else
        {
            b.hash(state);
            a.hash(state);
        }
    }
}

impl PartialEq for Pair <usize>    //Defines equality for custom tuple
{
    fn eq(&self, other: &Pair <usize>) -> bool //Checks for symmetrical equality
    {
        let a = self.0;
        let b = self.1;
        let c = other.0;
        let d = other.1;
        if a > b
        {
            if c > d
            {
                a == c && b == d
            }
            else
            {
                a == d && b == c
            }
        }
        else
        {
            if c > d
            {
                b == c && a == d
            }
            else
            {
                b == d && a == c
            }
        }
    }
}

impl Eq for Pair <usize>
{

}

impl Clone for Pair <usize> //Defines copying for custom tuple
{
    fn clone(&self) -> Pair <usize> //Returns a copy of the custom tuple
    {
        Pair (self.0, self.1)
    }
}

impl Hash for Pair <String> //Makes custom tuple communative
{
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        let a = &self.0;
        let b = &self.1;
        if a > b
        {
            a.hash(state);
            b.hash(state);
        }
        else
        {
            b.hash(state);
            a.hash(state);
        }
    }
}

impl PartialEq for Pair <String>    //Defines equality for custom tuple
{
    fn eq(&self, other: &Pair <String>) -> bool //Checks for symmetrical equality
    {
        let a = &self.0;
        let b = &self.1;
        let c = &other.0;
        let d = &other.1;
        if a > b
        {
            if c > d
            {
                a == c && b == d
            }
            else
            {
                a == d && b == c
            }
        }
        else
        {
            if c > d
            {
                b == c && a == d
            }
            else
            {
                b == d && a == c
            }
        }
    }
}

impl Eq for Pair <String>
{

}

impl Clone for Pair <String> //Defines copying for custom tuple
{
    fn clone(&self) -> Pair <String> //Returns a copy of the custom tuple
    {
        Pair (self.0.clone(), self.1.clone())
    }
}

struct Node <T> //Stores each Pair with its affinity
{
    pair: Pair <T>, //Stores the pair of trace elements as a Pair
    affinity: f64,  //Stores the affinity ratio as a double
}

impl PartialEq for Node <usize> //Defines equality for the Node
{
    fn eq(&self, other: &Node <usize>) -> bool  //Compares affinity ratios
    {
        self.affinity == other.affinity
    }
}

impl Eq for Node <usize>
{

}

impl PartialOrd for Node <usize>    //Defines comparison for the Node
{
    fn partial_cmp(&self, other: &Node <usize>) -> Option<Ordering> //Compares affinity ratios
    {
        self.affinity.partial_cmp(&other.affinity)
    }
}

impl Ord for Node <usize>   //Defines comparison for the Node
{
    fn cmp(&self, other: &Node <usize>) -> Ordering //Compares affinity ratios
    {
        self.affinity.partial_cmp(&other.affinity).unwrap()
    }
}

impl PartialEq for Node <String> //Defines equality for the Node
{
    fn eq(&self, other: &Node <String>) -> bool  //Compares affinity ratios
    {
        self.affinity == other.affinity
    }
}

impl Eq for Node <String>
{

}

impl PartialOrd for Node <String>    //Defines comparison for the Node
{
    fn partial_cmp(&self, other: &Node <String>) -> Option<Ordering> //Compares affinity ratios
    {
        self.affinity.partial_cmp(&other.affinity)
    }
}

impl Ord for Node <String>   //Defines comparison for the Node
{
    fn cmp(&self, other: &Node <String>) -> Ordering //Compares affinity ratios
    {
        self.affinity.partial_cmp(&other.affinity).unwrap()
    }
}

fn get_trace_user() -> Vec<char> //Retrieves trace
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
            if c.is_alphabetic()
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

fn get_histograms_user(trace: &Vec<char>) -> (HashMap<usize, Histogram <(u64, u64, u64, u64)>>, HashMap<Pair <usize>, Histogram <(u64, u64, u64, u64)>>, HashMap<Pair <usize>, Histogram <(u64, u64, u64, u64)>>, HashMap<Pair <usize>, Histogram <(u64, u64, u64, u64)>>)    //Generates histograms to calculate frequencies
{
    let mut last_seen_single: HashMap<usize, usize> = HashMap::new();   //Stores the last access time that a trace element is seen
    let mut last_seen_joint: HashMap<Pair <usize>, usize> = HashMap::new(); //Stores the last access time the beginning of a pair is seen
    let mut reuse_times: HashMap<usize, Histogram <(u64, u64, u64, u64)>> = HashMap::new(); //Stores the reuse times of each trace element
    let mut switch_times: HashMap<Pair <usize>, Histogram <(u64, u64, u64, u64)>> = HashMap::new();    //Stores the switch times of each trace pair
    let mut inter_switch_times: HashMap<Pair <usize>, Histogram <(u64, u64, u64, u64)>> = HashMap::new();   //Stores the inter-switch times of each pair
    let mut joint_times: HashMap<Pair <usize>, Histogram <(u64, u64, u64, u64)>> = HashMap::new();   //Stores the sum of switch time and correspondong inter-switch time

    for i in 0 .. trace.len()   //Iterates through trace
    {
        let c = trace [i] as usize;  //Gets trace element

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
                    let p = Pair (c, *j);    //Stores the current element pair

                    if !switch_times.contains_key(&p)    //Checks if the pair has a switch time
                    {
                        switch_times.insert(p.clone(), Histogram::new_tuple(SUBLOG_BITS, trace.len() as u64));
                    }

                    switch_times.get_mut(&p).unwrap().add(st as u64); //Adds to the switch time's bucket's frequency

                    let mut ist = *last_seen_single.get(&j).unwrap() as u64;
                    if last_seen_joint.contains_key(&p)
                    {
                        ist = ist - *last_seen_joint.get(&p).unwrap() as u64;
                    }
                    else    //Populates inter-switch times and joint-switch times with histograms otherwise
                    {
                        inter_switch_times.insert(p.clone(), Histogram::new_tuple(SUBLOG_BITS, trace.len() as u64));
                        joint_times.insert(p.clone(), Histogram::new_tuple(SUBLOG_BITS, trace.len() as u64));
                    }
                    inter_switch_times.get_mut(&p).unwrap().add(ist);   //Adds inter-switch time to histogram
                    joint_times.get_mut(&p).unwrap().add(ist + st as u64);  //Adds sum of inter-switch time and switch time to joint-switch histogram

                    println!("{}: ({}, {}) -> {}", i, char::from_u32(p.0 as u32).unwrap(), char::from_u32(p.1 as u32).unwrap(), last_seen_single.get(&j).unwrap());
                    last_seen_joint.insert(p, *last_seen_single.get(&j).unwrap());  //Adds index of first element of switch to the last seen joint HashMap
                }
            }
        }

        last_seen_single.insert(c, i + 1); //Updates last seen time for current trace element with current access time
    }

    for c in last_seen_single.keys()    //Adds time to end of trace to each character's reuse time
    {
        reuse_times.get_mut(&c).unwrap().add((trace.len() + 1 - *last_seen_single.get(c).unwrap()) as u64);
    }

    for p in switch_times.keys()    //Adds time to end of trace to each character's reuse time
    {
        let ist = (trace.len() + 1 - *last_seen_joint.get(p).unwrap()) as u64;
        inter_switch_times.get_mut(&p).unwrap().add(ist);
        joint_times.get_mut(&p).unwrap().add(ist);
    }

    (reuse_times, switch_times, inter_switch_times, joint_times)  //Returns histograms
}

fn get_size_user(trace_length: usize, start: usize) -> usize   //Inputs time window size
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

fn get_trace_file(split: Vec<&str>) -> Vec<(usize, usize)>
{
    let mut trace: Vec <(usize, usize)> = Vec::with_capacity(split.len());

    for i in 0 .. split.len()
    {
        let line = split [i].split(" ").collect::<Vec<&str>>();

        if line.len() != 2
        {
            panic!("Incorrect File Format: Line ".to_owned() + &(i + 1).to_string() + " has " + &line.len().to_string() + " Elements");
        }

        let time: usize = line [0].to_string().parse().unwrap();
        let element = line [1].to_string().parse().unwrap();

        trace.insert(i, (time, element));
    }

    trace
}

fn get_trace_words(split: Vec<&str>) -> Vec<(usize, String)>
{
    let mut trace: Vec <(usize, String)> = Vec::with_capacity(split.len());

    for i in 0 .. split.len()
    {
        let line = split [i].split(" ").collect::<Vec<&str>>();

        if line.len() != 2
        {
            panic!("Incorrect File Format: Line ".to_owned() + &(i + 1).to_string() + " has " + &line.len().to_string() + " Elements");
        }

        let time: usize = line [0].to_string().parse().unwrap();
        let element = line [1].to_string().to_lowercase();

        trace.insert(i, (time, element));
    }

    trace
}

fn get_histograms_file(trace: &Vec<(usize, usize)>) -> (HashMap<usize, Histogram <(u64, u64, u64, u64)>>, HashMap<Pair <usize>, Histogram <(u64, u64, u64, u64)>>, HashMap<Pair <usize>, Histogram <(u64, u64, u64, u64)>>, HashMap<Pair <usize>, Histogram <(u64, u64, u64, u64)>>)    //Generates histograms to calculate frequencies
{
    let mut last_seen_single: HashMap<usize, usize> = HashMap::new();   //Stores the last access time that a trace element is seen
    let mut last_seen_joint: HashMap<Pair <usize>, usize> = HashMap::new(); //Stores the last access time the beginning of a pair is seen
    let mut reuse_times: HashMap<usize, Histogram <(u64, u64, u64, u64)>> = HashMap::new(); //Stores the reuse times of each trace element
    let mut switch_times: HashMap<Pair <usize>, Histogram <(u64, u64, u64, u64)>> = HashMap::new();    //Stores the switch times of each trace pair
    let mut inter_switch_times: HashMap<Pair <usize>, Histogram <(u64, u64, u64, u64)>> = HashMap::new();   //Stores the inter-switch times of each pair
    let mut joint_times: HashMap<Pair <usize>, Histogram <(u64, u64, u64, u64)>> = HashMap::new();   //Stores the sum of switch time and correspondong inter-switch time

    for i in 0 .. trace.len()   //Iterates through trace
    {
        let c = trace [i];  //Gets trace element

        if !reuse_times.contains_key(&c.1)    //Checks if the element has a reuse time
        {
            reuse_times.insert(c.1 as usize, Histogram::new_tuple(SUBLOG_BITS, trace.len() as u64));  //Creates a new Histogram for the element
            reuse_times.get_mut(&c.1).unwrap().add((c.0) as u64);   //Adds current time to character reuse time histogram
        }
        else
        {
            let rt: u64 = (c.0 - last_seen_single.get(&c.1).unwrap()) as u64;  //Sets the current reuse time to be the difference between the current access time and the last access time this trace element was accessed
            reuse_times.get_mut(&c.1).unwrap().add(rt);  //Adds reuse time to histogram
        }

        for j in last_seen_single.keys()   //Iterates through the last seen elements
        {
            if *j != c.1  //Makes sure that an element is not compared with itelf
            {
                let st = c.0 - last_seen_single.get(j).unwrap();   //Sets the switch time to be the difference between the current access time and the last access time the other trace element was seen

                if !last_seen_single.contains_key(&c.1) || st < c.0 - *last_seen_single.get(&c.1).unwrap()    //Checks if the current trace element has not been seen before or if the switch time is smaller than the current reuse time
                {
                    let p = Pair (c.1, *j);    //Stores the current element pair

                    if !switch_times.contains_key(&p)    //Checks if the pair has a switch time
                    {
                        switch_times.insert(p.clone(), Histogram::new_tuple(SUBLOG_BITS, trace.len() as u64));
                    }

                    switch_times.get_mut(&p).unwrap().add(st as u64); //Adds to the switch time's bucket's frequency

                    let mut ist = *last_seen_single.get(&j).unwrap() as u64;
                    if last_seen_joint.contains_key(&p)
                    {
                        ist = ist - *last_seen_joint.get(&p).unwrap() as u64;
                    }
                    else    //Populates inter-switch times and joint-switch times with histograms otherwise
                    {
                        inter_switch_times.insert(p.clone(), Histogram::new_tuple(SUBLOG_BITS, trace.len() as u64));
                        joint_times.insert(p.clone(), Histogram::new_tuple(SUBLOG_BITS, trace.len() as u64));
                    }
                    inter_switch_times.get_mut(&p).unwrap().add(ist);   //Adds inter-switch time to histogram
                    joint_times.get_mut(&p).unwrap().add(ist + st as u64);  //Adds sum of inter-switch time and switch time to joint-switch histogram

                    last_seen_joint.insert(p, *last_seen_single.get(&j).unwrap());  //Adds index of first element of switch to the last seen joint HashMap
                }
            }
        }

        last_seen_single.insert(c.1, c.0); //Updates last seen time for current trace element with current access time
    }

    for c in last_seen_single.keys()    //Adds time to end of trace to each character's reuse time
    {
        reuse_times.get_mut(&c).unwrap().add((trace.len() + 1 - *last_seen_single.get(c).unwrap()) as u64);
    }

    for p in switch_times.keys()    //Adds time to end of trace to each character's reuse time
    {
        let ist = (trace.len() + 1 - *last_seen_joint.get(p).unwrap()) as u64;
        inter_switch_times.get_mut(&p).unwrap().add(ist);
        joint_times.get_mut(&p).unwrap().add(ist);
    }

    (reuse_times, switch_times, inter_switch_times, joint_times)  //Returns histograms
}

fn get_histograms_words(trace: &Vec<(usize, String)>) -> (HashMap<String, Histogram <(u64, u64, u64, u64)>>, HashMap<Pair <String>, Histogram <(u64, u64, u64, u64)>>, HashMap<Pair <String>, Histogram <(u64, u64, u64, u64)>>, HashMap<Pair <String>, Histogram <(u64, u64, u64, u64)>>)    //Generates histograms to calculate frequencies
{
    let mut last_seen_single: HashMap<String, usize> = HashMap::new();   //Stores the last access time that a trace element is seen
    let mut last_seen_joint: HashMap<Pair <String>, usize> = HashMap::new(); //Stores the last access time the beginning of a pair is seen
    let mut reuse_times: HashMap<String, Histogram <(u64, u64, u64, u64)>> = HashMap::new(); //Stores the reuse times of each trace element
    let mut switch_times: HashMap<Pair <String>, Histogram <(u64, u64, u64, u64)>> = HashMap::new();    //Stores the switch times of each trace pair
    let mut inter_switch_times: HashMap<Pair <String>, Histogram <(u64, u64, u64, u64)>> = HashMap::new();   //Stores the inter-switch times of each pair
    let mut joint_times: HashMap<Pair <String>, Histogram <(u64, u64, u64, u64)>> = HashMap::new();   //Stores the sum of switch time and correspondong inter-switch time

    for i in 0 .. trace.len()   //Iterates through trace
    {
        let c = trace [i].clone();  //Gets trace element

        if !reuse_times.contains_key(&c.1)    //Checks if the element has a reuse time
        {
            reuse_times.insert(c.1.clone(), Histogram::new_tuple(SUBLOG_BITS, trace.len() as u64));  //Creates a new Histogram for the element
            reuse_times.get_mut(&c.1).unwrap().add((c.0) as u64);   //Adds current time to character reuse time histogram
        }
        else
        {
            let rt: u64 = (c.0 - last_seen_single.get(&c.1).unwrap()) as u64;  //Sets the current reuse time to be the difference between the current access time and the last access time this trace element was accessed
            reuse_times.get_mut(&c.1).unwrap().add(rt);  //Adds reuse time to histogram
        }

        for j in last_seen_single.keys()   //Iterates through the last seen elements
        {
            if *j != c.1.clone()  //Makes sure that an element is not compared with itelf
            {
                let st = c.0 - last_seen_single.get(j).unwrap();   //Sets the switch time to be the difference between the current access time and the last access time the other trace element was seen

                if !last_seen_single.contains_key(&c.1) || st < c.0 - *last_seen_single.get(&c.1).unwrap()    //Checks if the current trace element has not been seen before or if the switch time is smaller than the current reuse time
                {
                    let p = Pair (c.1.clone(), j.clone());    //Stores the current element pair

                    if !switch_times.contains_key(&p)    //Checks if the pair has a switch time
                    {
                        switch_times.insert(p.clone(), Histogram::new_tuple(SUBLOG_BITS, trace.len() as u64));
                    }

                    switch_times.get_mut(&p).unwrap().add(st as u64); //Adds to the switch time's bucket's frequency

                    let mut ist = *last_seen_single.get(&j.clone()).unwrap() as u64;
                    if last_seen_joint.contains_key(&p)
                    {
                        ist = ist - *last_seen_joint.get(&p).unwrap() as u64;
                    }
                    else    //Populates inter-switch times and joint-switch times with histograms otherwise
                    {
                        inter_switch_times.insert(p.clone(), Histogram::new_tuple(SUBLOG_BITS, trace.len() as u64));
                        joint_times.insert(p.clone(), Histogram::new_tuple(SUBLOG_BITS, trace.len() as u64));
                    }
                    inter_switch_times.get_mut(&p).unwrap().add(ist);   //Adds inter-switch time to histogram
                    joint_times.get_mut(&p).unwrap().add(ist + st as u64);  //Adds sum of inter-switch time and switch time to joint-switch histogram

                    last_seen_joint.insert(p, *last_seen_single.get(&j.clone()).unwrap());  //Adds index of first element of switch to the last seen joint HashMap
                }
            }
        }

        last_seen_single.insert(c.1, c.0); //Updates last seen time for current trace element with current access time
    }

    for c in last_seen_single.keys()    //Adds time to end of trace to each character's reuse time
    {
        reuse_times.get_mut(c).unwrap().add((trace.len() + 1 - *last_seen_single.get(c).unwrap()) as u64);
    }

    for p in switch_times.keys()    //Adds time to end of trace to each character's reuse time
    {
        let ist = (trace.len() + 1 - *last_seen_joint.get(p).unwrap()) as u64;
        inter_switch_times.get_mut(&p).unwrap().add(ist);
        joint_times.get_mut(&p).unwrap().add(ist);
    }

    (reuse_times, switch_times, inter_switch_times, joint_times)  //Returns histograms
}

fn get_single_frequencies(reuse_times: &HashMap<usize, Histogram <(u64, u64, u64, u64)>>, window_size: usize, total_windows: usize) -> HashMap<usize, usize>    //Generates single frequencies using a reuse time histogram, first seen indexes, last seen indexes, time window size, and total windows
{
    let mut single_frequencies: HashMap<usize, usize> = HashMap::new();  //Stores the single frequencies

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

fn get_single_frequencies_words(reuse_times: &HashMap<String, Histogram <(u64, u64, u64, u64)>>, window_size: usize, total_windows: usize) -> HashMap<String, usize>    //Generates single frequencies using a reuse time histogram, first seen indexes, last seen indexes, time window size, and total windows
{
    let mut single_frequencies: HashMap<String, usize> = HashMap::new();  //Stores the single frequencies

    for i in reuse_times.keys() //Iterates through reuse time histogram
    {
        single_frequencies.insert(i.clone(), total_windows);   //Inserts total windows into each character frequency
        for j in reuse_times.get(i).unwrap().get_values()    //Iterates through reuse times of each trace character
        {
            let rt = j.0;

            if rt == 0
            {
                continue;
            }

            if rt > window_size as u64    //Checks if the reuse time is larger than the window size
            {
                let mut temp = single_frequencies.get(&i.clone()).unwrap().clone(); //Retrieves old window count
                temp = temp - (j.2 - window_size as u64 * j.3) as usize;   //Subtraccts off the frequency multiplied by the difference between the reuse time and window size
                single_frequencies.insert(i.clone(), temp);    //Writes new window count
            }
        }
    }

    single_frequencies  //Returns single frequencies
}

fn get_joint_frequencies (switch_times: &HashMap<Pair <usize>, Histogram <(u64, u64, u64, u64)>>, joint_times: &HashMap<Pair <usize>, Histogram <(u64, u64, u64, u64)>>, window_size: usize, total_windows: usize) -> HashMap<Pair <usize>, usize>   //Generates joint frequencies using a switch time histogram, a joint switch time histogram, time window size, and total windows
{
    let mut joint_frequencies: HashMap<Pair <usize>, usize> = HashMap::new();   //Stores joint frequencies
    for p in switch_times.keys()    //Iterates through all pairs in switch time histogram
    {
        let total = total_windows + window_size;    //Finds one more than the trace length
        let mut switch_sum = 0; //Stores the sum of all the switch sums
        let mut switch_count = 0;   //Stores the switch count (including a 0 switch) multiplied by the window size
        let mut switch_adjust = 0;  //Adjusts number of switches where switch time is larger than the window size
        let mut joint_adjust = 0;

        for st in switch_times.get(&p).unwrap().get_values()    //Iterates through pair's histogram values
        {
            switch_sum = switch_sum + st.2; //Adds sum to switch sum
            switch_count = switch_count + st.3; //Adds frequency to switch count
            
            if st.0 > window_size as u64    //Checks if the minimum switch in the bucket is larger than the window size
            {
                switch_adjust = switch_adjust + (st.2 - (st.3 * window_size as u64));   //Adds the difference between the switch sum and the switch frequency multiplied by the window size
            }
        }
        switch_count = (switch_count + 1) * window_size as u64; //Adds one to the switch count and multiplies it by the window size

        for jst in joint_times.get(&p).unwrap().get_values()    //Iterates through the joint swith times
        {
            if jst.0 < window_size as u64   //Makes sure the joint switch time is stricly less than the window size
            {
                joint_adjust = joint_adjust + (window_size as u64 * jst.3) - jst.2; //Adds the difference between the window size multiplied by the frequency and the joint sum
            }
        }

        let absence_windows = total as u64 + switch_sum + joint_adjust - switch_count - switch_adjust; //Calculates the nuber of absence windows for the pair

        joint_frequencies.insert(p.clone(), total_windows - absence_windows as usize);  //Inserts the pair with its window count into the joint frequency HashMap
    }

    joint_frequencies
}

fn get_joint_frequencies_words (switch_times: &HashMap<Pair <String>, Histogram <(u64, u64, u64, u64)>>, joint_times: &HashMap<Pair <String>, Histogram <(u64, u64, u64, u64)>>, window_size: usize, total_windows: usize) -> HashMap<Pair <String>, usize>   //Generates joint frequencies using a switch time histogram, a joint switch time histogram, time window size, and total windows
{
    let mut joint_frequencies: HashMap<Pair <String>, usize> = HashMap::new();   //Stores joint frequencies
    for p in switch_times.keys()    //Iterates through all pairs in switch time histogram
    {
        let total = total_windows + window_size;    //Finds one more than the trace length
        let mut switch_sum = 0; //Stores the sum of all the switch sums
        let mut switch_count = 0;   //Stores the switch count (including a 0 switch) multiplied by the window size
        let mut switch_adjust = 0;  //Adjusts number of switches where switch time is larger than the window size
        let mut joint_adjust = 0;

        for st in switch_times.get(&p).unwrap().get_values()    //Iterates through pair's histogram values
        {
            switch_sum = switch_sum + st.2; //Adds sum to switch sum
            switch_count = switch_count + st.3; //Adds frequency to switch count
            
            if st.0 > window_size as u64    //Checks if the minimum switch in the bucket is larger than the window size
            {
                switch_adjust = switch_adjust + (st.2 - (st.3 * window_size as u64));   //Adds the difference between the switch sum and the switch frequency multiplied by the window size
            }
        }
        switch_count = (switch_count + 1) * window_size as u64; //Adds one to the switch count and multiplies it by the window size

        for jst in joint_times.get(&p).unwrap().get_values()    //Iterates through the joint swith times
        {
            if jst.0 < window_size as u64   //Makes sure the joint switch time is stricly less than the window size
            {
                joint_adjust = joint_adjust + (window_size as u64 * jst.3) - jst.2; //Adds the difference between the window size multiplied by the frequency and the joint sum
            }
        }

        let absence_windows = total as u64 + switch_sum + joint_adjust - switch_count - switch_adjust; //Calculates the nuber of absence windows for the pair

        joint_frequencies.insert(p.clone(), total_windows - absence_windows as usize);  //Inserts the pair with its window count into the joint frequency HashMap
    }

    joint_frequencies
}

fn get_affinities(single_frequencies: HashMap<usize, usize>, joint_frequencies: HashMap<Pair <usize>, usize>) -> BinaryHeap<Node <usize>>    //Takes single frequencies and joint frequencies to create a Priority Queue of affinities
{
    let mut affinities: BinaryHeap<Node <usize>> = BinaryHeap::new();   //Stores affinities
    
    for p in joint_frequencies.keys()   //Iterates through pairs
    {
        affinities.push(Node{pair: p.clone(), affinity: (*joint_frequencies.get(&p).unwrap() as f64) / (*single_frequencies.get(&p.0).unwrap() as f64)});
        affinities.push(Node{pair: Pair(p.1, p.0), affinity: (*joint_frequencies.get(&p).unwrap() as f64) / (*single_frequencies.get(&p.1).unwrap() as f64)});
    }

    affinities
}

fn get_affinities_words(single_frequencies: HashMap<String, usize>, joint_frequencies: HashMap<Pair <String>, usize>) -> BinaryHeap<Node <String>>    //Takes single frequencies and joint frequencies to create a Priority Queue of affinities
{
    let mut affinities: BinaryHeap<Node <String>> = BinaryHeap::new();   //Stores affinities
    
    for p in joint_frequencies.keys()   //Iterates through pairs
    {
        affinities.push(Node{pair: p.clone(), affinity: (*joint_frequencies.get(&p).unwrap() as f64) / (*single_frequencies.get(&p.0).unwrap() as f64)});
        affinities.push(Node{pair: Pair(p.1.clone(), p.0.clone()), affinity: (*joint_frequencies.get(&p).unwrap() as f64) / (*single_frequencies.get(&p.1).unwrap() as f64)});
    }

    affinities
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2
    {
        user_input();
    }
    else
    {
        if args [1].eq("u")
        {
            user_input();
        }
        else if args [1].eq("f")
        {
            if args.len() == 5
            {
                file_input(args);
            }
            else if args.len() < 5
            {
                panic!("Too Few Parameters: ".to_owned() + &(args.len().to_string()));
            }
            else
            {
                panic!("Too Many Parameters: ".to_owned() + &(args.len().to_string()));
            }
        }
        else if args [1].eq("c")
        {
            if args.len() == 5
            {
                text_input(args);
            }
            else if args.len() < 5
            {
                panic!("Too Few Parameters: ".to_owned() + &(args.len().to_string()));
            }
            else
            {
                panic!("Too Many Parameters: ".to_owned() + &(args.len().to_string()));
            }
        }
        else if args [1].eq("w")
        {
            if args.len() == 5
            {
                word_input(args);
            }
            else if args.len() < 5
            {
                panic!("Too Few Parameters: ".to_owned() + &(args.len().to_string()));
            }
            else
            {
                panic!("Too Many Parameters: ".to_owned() + &(args.len().to_string()));
            }
        }
        else if args [1].eq("h")
        {
            println!("Help:\t\t\th");
            println!("User input:\t\tu");
            println!("File input:\t\tf file_in_trace_folder start_timescale end_timescale");
            println!("Character input:\tc file_in_trace_folder start_timescale end_timescale");
            println!("Word input:\t\tw file_in_trace_folder start_timescale end_timescale");
            println!("*Run plot.py with python and file_in_trace_folder start_timescale end_timescale arguments for graphing");
        }
        else
        {
            panic!("Not a valid argument: ".to_owned() + &args [1]);
        }
    }
}

fn user_input()
{
    println!("\nThis program calculates affinity for a given trace and window size.");

    let trace = get_trace_user();    //Gets trace from user

    let times = get_histograms_user(&trace); //Generates histograms

    //Prints histograms
    println!("\nReuse Time Histogram:");
    for c in times.0.keys() //Prints reuse times
    {
        println!("\n{}:", c);
        for s in times.0.get(c).unwrap().get_values()
        {
            if s != (0, 0, 0, 0)
            {
                println!("min: {}, max: {}, sum: {}, frequency: {}", s.0, s.1, s.2, s.3);
            }
        }
    }

    println!("\nSwitch Time Histogram:");
    for c in times.1.keys() //Prints switch times
    {
        println!("\n({}, {}):", c.1, c.0);
        for s in times.1.get(c).unwrap().get_values()
        {
            if s != (0, 0, 0, 0)
            {
                println!("min: {}, max: {}, sum: {}, frequency: {}", s.0, s.1, s.2, s.3);
            }
        }
    }

    println!("\nInter-Switch Time Histogram:");
    for c in times.2.keys() //Prints inter-switch times
    {
        println!("\n({}, {}):", c.1, c.0);
        for s in times.2.get(c).unwrap().get_values()
        {
            if s != (0, 0, 0, 0)
            {
                println!("min: {}, max: {}, sum: {}, frequency: {}", s.0, s.1, s.2, s.3);
            }
        }
    }

    println!("\nJoint-Switch Time Histogram:");
    for c in times.3.keys() //Prints joint-switch times
    {
        println!("\n({}, {}):", c.1, c.0);
        for s in times.3.get(c).unwrap().get_values()
        {
            if s != (0, 0, 0, 0)
            {
                println!("min: {}, max: {}, sum: {}, frequency: {}", s.0, s.1, s.2, s.3);
            }
        }
    }

    println!();
    let window_size = get_size_user(trace.len(), 1); //Gets time window size from user

    let total_windows = trace.len() - window_size + 1; //Sets the total windows to one more than the difference between the trace length and time window size

    let single_frequencies = get_single_frequencies(&times.0, window_size, total_windows);   //Calculates single frequencies

    let joint_frequencies = get_joint_frequencies(&times.1, &times.3, window_size, total_windows);    //Calculates joint frequencies

    println!("\nSingle Frequencies:\n");
    for c in single_frequencies.keys()  //Prints single frequencies
    {
        println!("{}: {}", c, single_frequencies.get(c).unwrap());
    }

    println!("\nJoint Frequencies:\n");
    for c in joint_frequencies.keys()  //Prints joint frequencies
    {
        println!("({}, {}): {}", c.1, c.0, joint_frequencies.get(c).unwrap());
    }

    let mut affinities = get_affinities(single_frequencies, joint_frequencies);

    println!("\nAffinities:\n");
    while !affinities.is_empty()    //Prints affinities in descending order
    {
        let node = affinities.pop().unwrap();
        println!("({}, {}): {}", node.pair.0, node.pair.1, node.affinity);
    }
}

fn file_input(args: Vec <String>)
{
    let file_name = "traces/".to_owned() + &args [2];
    let mut file = File::open(&file_name).expect(&("File not Found: ".to_owned() + &file_name));

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(&("File Read Error: ".to_owned() + &file_name));

    let split = contents.split("\n").collect::<Vec<&str>>();

    let start = args [3].parse().unwrap();
    let end = args [4].parse().unwrap();

    let trace = get_trace_file(split);

    let times = get_histograms_file(&trace);

    // //Prints histograms
    // println!("\nReuse Time Histogram:");
    // for c in times.0.keys() //Prints reuse times
    // {
    //     println!("\n{}:", c);
    //     for s in times.0.get(c).unwrap().get_values()
    //     {
    //         if s != (0, 0, 0, 0)
    //         {
    //             println!("min: {}, max: {}, sum: {}, frequency: {}", s.0, s.1, s.2, s.3);
    //         }
    //     }
    // }

    // println!("\nSwitch Time Histogram:");
    // for c in times.1.keys() //Prints switch times
    // {
    //     println!("\n({}, {}):", c.1, c.0);
    //     for s in times.1.get(c).unwrap().get_values()
    //     {
    //         if s != (0, 0, 0, 0)
    //         {
    //             println!("min: {}, max: {}, sum: {}, frequency: {}", s.0, s.1, s.2, s.3);
    //         }
    //     }
    // }

    // println!("\nInter-Switch Time Histogram:");
    // for c in times.2.keys() //Prints inter-switch times
    // {
    //     println!("\n({}, {}):", c.1, c.0);
    //     for s in times.2.get(c).unwrap().get_values()
    //     {
    //         if s != (0, 0, 0, 0)
    //         {
    //             println!("min: {}, max: {}, sum: {}, frequency: {}", s.0, s.1, s.2, s.3);
    //         }
    //     }
    // }

    // println!("\nJoint-Switch Time Histogram:");
    // for c in times.3.keys() //Prints joint-switch times
    // {
    //     println!("\n({}, {}):", c.1, c.0);
    //     for s in times.3.get(c).unwrap().get_values()
    //     {
    //         if s != (0, 0, 0, 0)
    //         {
    //             println!("min: {}, max: {}, sum: {}, frequency: {}", s.0, s.1, s.2, s.3);
    //         }
    //     }
    // }

    for window_size in start ..= end
    {
        let total_windows = trace.len() - window_size + 1; //Sets the total windows to one more than the difference between the trace length and time window size

        let single_frequencies = get_single_frequencies(&times.0, window_size, total_windows);   //Calculates single frequencies

        let joint_frequencies = get_joint_frequencies(&times.1, &times.3, window_size, total_windows);    //Calculates joint frequencies

        println!("\nSingle Frequencies:\n");
        for c in single_frequencies.keys()  //Prints single frequencies
        {
            println!("{}: {}", c, single_frequencies.get(c).unwrap());
        }

        println!("\nJoint Frequencies:\n");
        for c in joint_frequencies.keys()  //Prints joint frequencies
        {
            println!("({}, {}): {}", c.1, c.0, joint_frequencies.get(c).unwrap());
        }

        let mut affinities = get_affinities(single_frequencies, joint_frequencies);

        println!("\nAffinities:\n");
        while !affinities.is_empty()    //Prints affinities in descending order
        {
            let node = affinities.pop().unwrap();
            println!("({}, {}): {}", node.pair.0, node.pair.1, node.affinity);
        }
    }
}

fn text_input(args: Vec <String>)
{
    let file_name = "traces/".to_owned() + &args [2];
    let mut file = File::open(&file_name).expect(&("File not Found: ".to_owned() + &file_name));

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(&("File Read Error: ".to_owned() + &file_name));

    let mut data: Vec<String> = Vec::with_capacity(contents.len());
    let mut time = 1;
    for c in contents.chars()
    {
        if c.is_alphabetic()
        {
            let character = c.to_lowercase().collect::<Vec<char>>() [0];
            let line = time.to_string() + " " + &(character as usize).to_string();
            data.push(line);
            time = time + 1;
        }
    }

    let split: Vec<&str> = data.iter().map(|s| &**s).collect();

    let start = args [3].parse().unwrap();
    let end = args [4].parse().unwrap();

    let trace = get_trace_file(split);

    let times = get_histograms_file(&trace);

    // //Prints histograms
    // println!("\nReuse Time Histogram:");
    // for c in times.0.keys() //Prints reuse times
    // {
    //     println!("\n{}:", *c as u8 as char);
    //     for s in times.0.get(c).unwrap().get_values()
    //     {
    //         if s != (0, 0, 0, 0)
    //         {
    //             println!("min: {}, max: {}, sum: {}, frequency: {}", s.0, s.1, s.2, s.3);
    //         }
    //     }
    // }

    // println!("\nSwitch Time Histogram:");
    // for c in times.1.keys() //Prints switch times
    // {
    //     println!("\n({}, {}):", c.1 as u8 as char, c.0 as u8 as char);
    //     for s in times.1.get(c).unwrap().get_values()
    //     {
    //         if s != (0, 0, 0, 0)
    //         {
    //             println!("min: {}, max: {}, sum: {}, frequency: {}", s.0, s.1, s.2, s.3);
    //         }
    //     }
    // }

    // println!("\nInter-Switch Time Histogram:");
    // for c in times.2.keys() //Prints inter-switch times
    // {
    //     println!("\n({}, {}):", c.1 as u8 as char, c.0 as u8 as char);
    //     for s in times.2.get(c).unwrap().get_values()
    //     {
    //         if s != (0, 0, 0, 0)
    //         {
    //             println!("min: {}, max: {}, sum: {}, frequency: {}", s.0, s.1, s.2, s.3);
    //         }
    //     }
    // }

    // println!("\nJoint-Switch Time Histogram:");
    // for c in times.3.keys() //Prints joint-switch times
    // {
    //     println!("\n({}, {}):", c.1 as u8 as char, c.0 as u8 as char);
    //     for s in times.3.get(c).unwrap().get_values()
    //     {
    //         if s != (0, 0, 0, 0)
    //         {
    //             println!("min: {}, max: {}, sum: {}, frequency: {}", s.0, s.1, s.2, s.3);
    //         }
    //     }
    // }

    for window_size in start ..= end
    {
        let total_windows = trace.len() - window_size + 1; //Sets the total windows to one more than the difference between the trace length and time window size

        let single_frequencies = get_single_frequencies(&times.0, window_size, total_windows);   //Calculates single frequencies

        let joint_frequencies = get_joint_frequencies(&times.1, &times.3, window_size, total_windows);    //Calculates joint frequencies

        // println!("\nSingle Frequencies:\n");
        // for c in single_frequencies.keys()  //Prints single frequencies
        // {
        //     println!("{}: {}", c, single_frequencies.get(c).unwrap());
        // }

        // println!("\nJoint Frequencies:\n");
        // for c in joint_frequencies.keys()  //Prints joint frequencies
        // {
        //     println!("({}, {}): {}", c.1, c.0, joint_frequencies.get(c).unwrap());
        // }

        let mut affinities = get_affinities(single_frequencies, joint_frequencies);

        match fs::create_dir("results/".to_owned() + &args [2].clone())
        {
            _ => (),
        }

        let mut file = File::create("results/".to_owned() + &args [2].clone() + "/" + &window_size.to_string() + ".csv").unwrap();
        //println!("\nAffinities for {} and window size {}:\n", &args [2].clone(), window_size);
        //file.write(("Affinities for window size ".to_owned() + &window_size.to_string() + ":\n\n").as_bytes()).expect("Header Write Error");
        file.write_all(("Pair;Affinity\n".to_owned()).as_bytes()).expect("Affinity Write Error");
        while !affinities.is_empty()    //Prints affinities in descending order
        {
            let node = affinities.pop().unwrap();
            let line = "(".to_owned() + &(char::from_u32(node.pair.0 as u32).unwrap()).to_string() + ", " + &(char::from_u32(node.pair.1 as u32).unwrap()).to_string() + ");" + &node.affinity.to_string() + "\n";
            //println!("{}", line);
            file.write_all(line.as_bytes()).expect("Affinity Write Error");
        }
    }
}

fn word_input(args: Vec <String>)
{
    let file_name = "traces/".to_owned() + &args [2];
    let mut file = File::open(&file_name).expect(&("File not Found: ".to_owned() + &file_name));

    let mut raw_contents = String::new();
    file.read_to_string(&mut raw_contents).expect(&("File Read Error: ".to_owned() + &file_name));

    let mut contents = raw_contents.split_whitespace().collect::<Vec<&str>>();

    let mut data: Vec<String> = Vec::with_capacity(contents.len());
    let mut time = 1;
    for s in contents
    {
        let mut string = String::new();
        for c in s.chars()
        {
            if c.is_alphabetic()
            {
                string.push(c);
            }
        }
        
        let line = time.to_string() + " " + &string.to_string();
        data.push(line);
        time = time + 1;
    }

    let split: Vec<&str> = data.iter().map(|s| &**s).collect();

    let start = args [3].parse().unwrap();
    let end = args [4].parse().unwrap();

    let trace = get_trace_words(split);

    let times = get_histograms_words(&trace);

    // //Prints histograms
    // println!("\nReuse Time Histogram:");
    // for c in times.0.keys() //Prints reuse times
    // {
    //     println!("\n{}:", *c as u8 as char);
    //     for s in times.0.get(c).unwrap().get_values()
    //     {
    //         if s != (0, 0, 0, 0)
    //         {
    //             println!("min: {}, max: {}, sum: {}, frequency: {}", s.0, s.1, s.2, s.3);
    //         }
    //     }
    // }

    // println!("\nSwitch Time Histogram:");
    // for c in times.1.keys() //Prints switch times
    // {
    //     println!("\n({}, {}):", c.1 as u8 as char, c.0 as u8 as char);
    //     for s in times.1.get(c).unwrap().get_values()
    //     {
    //         if s != (0, 0, 0, 0)
    //         {
    //             println!("min: {}, max: {}, sum: {}, frequency: {}", s.0, s.1, s.2, s.3);
    //         }
    //     }
    // }

    // println!("\nInter-Switch Time Histogram:");
    // for c in times.2.keys() //Prints inter-switch times
    // {
    //     println!("\n({}, {}):", c.1 as u8 as char, c.0 as u8 as char);
    //     for s in times.2.get(c).unwrap().get_values()
    //     {
    //         if s != (0, 0, 0, 0)
    //         {
    //             println!("min: {}, max: {}, sum: {}, frequency: {}", s.0, s.1, s.2, s.3);
    //         }
    //     }
    // }

    // println!("\nJoint-Switch Time Histogram:");
    // for c in times.3.keys() //Prints joint-switch times
    // {
    //     println!("\n({}, {}):", c.1 as u8 as char, c.0 as u8 as char);
    //     for s in times.3.get(c).unwrap().get_values()
    //     {
    //         if s != (0, 0, 0, 0)
    //         {
    //             println!("min: {}, max: {}, sum: {}, frequency: {}", s.0, s.1, s.2, s.3);
    //         }
    //     }
    // }

    for window_size in start ..= end
    {
        let total_windows = trace.len() - window_size + 1; //Sets the total windows to one more than the difference between the trace length and time window size

        let single_frequencies = get_single_frequencies_words(&times.0, window_size, total_windows);   //Calculates single frequencies

        let joint_frequencies = get_joint_frequencies_words(&times.1, &times.3, window_size, total_windows);    //Calculates joint frequencies

        // println!("\nSingle Frequencies:\n");
        // for c in single_frequencies.keys()  //Prints single frequencies
        // {
        //     println!("{}: {}", c, single_frequencies.get(c).unwrap());
        // }

        // println!("\nJoint Frequencies:\n");
        // for c in joint_frequencies.keys()  //Prints joint frequencies
        // {
        //     println!("({}, {}): {}", c.1, c.0, joint_frequencies.get(c).unwrap());
        // }

        let mut affinities = get_affinities_words(single_frequencies, joint_frequencies);

        match fs::create_dir("results/".to_owned() + &args [2].clone())
        {
            _ => (),
        }

        let mut file = File::create("results/".to_owned() + &args [2].clone() + "/" + &window_size.to_string() + ".csv").unwrap();
        //println!("\nAffinities for {} and window size {}:\n", &args [2].clone(), window_size);
        //file.write(("Affinities for window size ".to_owned() + &window_size.to_string() + ":\n\n").as_bytes()).expect("Header Write Error");
        file.write_all(("Pair;Affinity\n".to_owned()).as_bytes()).expect("Affinity Write Error");
        while !affinities.is_empty()    //Prints affinities in descending order
        {
            let node = affinities.pop().unwrap();
            let line = "(".to_owned() + &node.pair.0 + ", " + &node.pair.1 + ");" + &node.affinity.to_string() + "\n";
            //println!("{}", line);
            file.write_all(line.as_bytes()).expect("Affinity Write Error");
        }
    }
}