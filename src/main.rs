mod histogram;  //Imports histogram module
use histogram::Histogram;   //Imports Histogram from histogram module
use std::io;    //Used for input
use std::io::Write; //Used for output
use std::collections::HashMap;  //Used for storing frequencies
use std::hash::{Hash, Hasher};  //Used for custom tuple hashing

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

fn get_histograms(trace: &Vec<char>) -> (HashMap<char, Histogram>, HashMap<Pair, HashMap<usize, usize>>, HashMap<char, usize>, HashMap<char, usize>)    //Generates histograms to calculate frequencies
{
    let mut first_seen: HashMap<char, usize> = HashMap::new();  //Stores the first access time that a trace element is seen
    let mut last_seen: HashMap<char, usize> = HashMap::new();   //Stores the last access time that a trace element is seen
    let mut reuse_times: HashMap<char, Histogram> = HashMap::new(); //Stores the reuse times of each trace element
    let mut switch_times: HashMap<Pair, HashMap<usize, usize>> = HashMap::new();    //Stores the reuse times of each trace element

    for i in 0 .. trace.len()   //Iterates through trace
    {
        let c = trace [i];  //Gets trace element

        if !reuse_times.contains_key(&c)    //Checks if the element has a reuse time
        {
            if last_seen.contains_key(&c)   //Checks if the element has been seen before
            {
                reuse_times.insert(c.clone(), Histogram::new(8, trace.len() as u64));  //Creates a new HashMap for the element
            }
            else    //Inserts current access time into first seen HashMap otherwise
            {
                first_seen.insert(c.clone(), i + 1);
            }
        }

        if reuse_times.contains_key(&c) //Checks if the element has a resue time
        {
            let rt = (i + 1) - last_seen.get(&c).unwrap();  //Sets the current reuse time to be the difference between the current access time and the last access time this trace element was accessed
            let mut temp = 1;   //Sets temporary frequency to 1

            if reuse_times.get(&c).unwrap().get(rt as u64) != 0   //Checks if the element's reuse time has a frequency
            {
                temp = reuse_times.get(&c).unwrap().get(rt as u64).clone() + 1;  //Stores one more than the old frequency into the temporary frequency
            }
            
            reuse_times.get_mut(&c).unwrap().insert(rt as u64, temp);  //Writes the temporary frequency into the character's reuse time

        }

        for j in last_seen.keys()   //Iterates through the last seen elements
        {
            if *j != c  //Makes sure that an element is not compared with itelf
            {
                let st = (i + 1) - last_seen.get(j).unwrap();   //Sets the switch time to be the difference between the current access time and the last access time the other trace element was seen

                if !last_seen.contains_key(&c) || st < (i + 1) - *last_seen.get(&c).unwrap()    //Checks if the current trace element has not been seen before or if the switch time is smaller than the current reuse time
                {
                    let p = Pair(c, *j);    //Stores the current element pair
                    let mut temp = 1;   //Sets temporary frequency to 1

                    if switch_times.contains_key(&p)    //Checks if the pair has a switch time
                    {
                        if switch_times.get(&p).unwrap().contains_key(&st)  //Checks if the pair's switch time has a frequency
                        {
                            temp = switch_times.get(&p).unwrap().get(&st).unwrap().clone() + 1; //Stores one more than the old frequency into the temporary frequency
                        }
                    }
                    else    //Creates a new HashMap for the pairs otherwise
                    {
                        switch_times.insert(p.clone(), HashMap::new());
                    }

                    switch_times.get_mut(&p).unwrap().insert(st, temp); //Writes the temporary frequency into the pair's switch time
                }
            }
        }

        last_seen.insert(c, i + 1); //Updates last seen time for current trace element with current access time
    }

    (reuse_times, switch_times, first_seen, last_seen)  //Returns histograms
}

fn get_size(trace_length: usize, start: usize) -> usize   //Inputs time window size
{
    loop    //Makes sure size is valid for trace
    {
        let mut num = 0;
        loop    //Makes sure size is a valid usize
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
            num = input;

            break;
        }
        
        if !(num > (trace_length - start) || num <= 1)   //Makes sure window size is not larger than the trace or less than 2
        {
            return num;
        }
        else
        {
            println!("Invalid Input");
        }
    }
}


fn get_single_frequencies(reuse_times: HashMap<char, Histogram>, first_seen: HashMap<char, usize>, last_seen: HashMap<char, usize>, window_size: usize, trace_length: usize) -> HashMap<char, usize>    //Generates single frequencies using a reusr time histogram, first seen indexes, last seen indexes, time window size, and trace length
{
    let mut single_frequencies: HashMap<char, usize> = HashMap::new();  //Stores the single frequencies
    let total_windows = trace_length - window_size + 1; //Sets the total windows to one more than the difference between the trace length and time window size

    for i in first_seen.keys()  //Iterates through first seen access times
    {
        let rt = first_seen.get(&i).unwrap();   //Sets the reuse time to the trace element's first access time
        if rt > &window_size    //Checks if the reuse time is greater than the window size
        {
            single_frequencies.insert(*i, total_windows - (rt - window_size));    //Inserts the trace element with its corresponding window count
        }
        else    //Inserts total window count with the trace element otherwise
        {
            single_frequencies.insert(*i, total_windows);
        }

        let rt = trace_length - (last_seen.get(&i).unwrap() - 1);   //Sets the reuse time to the trace element's reverse last access time
        if rt > window_size //Checks if the reuse time is greater than the window size
        {
            let mut temp = single_frequencies.get(&i).unwrap().clone(); //Retrieves old window count
            temp = temp - (rt - window_size);   //Subtracts off the difference between the reuse time and window size
            single_frequencies.insert(*i, temp);    //Writes new window count
        }
    }

    for i in reuse_times.keys() //Iterates through reuse time histogram
    {
        for rt in reuse_times.get(i).unwrap().get_values()    //Iterates through reuse times of each trace character
        {
            let f = reuse_times.get(i).unwrap().get(*rt);   //Retrieves reuse time frequency
            if rt > &(window_size as u64)    //Checks if the reuse time is larger than the window size
            {
                let mut temp = single_frequencies.get(&i).unwrap().clone(); //Retrieves old window count
                temp = temp - (f * (rt - window_size as u64)) as usize;   //Subtraccts off the frequency multiplied by the difference between the reuse time and window size
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
            println!("{}: {}", s, times.0.get(c).unwrap().get(*s));
        }
    }

    println!("\nSwitch Time Histogram:");
    for c in times.1.keys()
    {
        println!("\n({}, {}):", c.0, c.1);
        for s in times.1.get(c).unwrap().keys()
        {
            println!("{}: {}", s, times.1.get(c).unwrap().get(s).unwrap());
        }
    }

    println!("\nFirst Seen:\n");
    for c in times.2.keys()
    {
        println!("{}: {}", c, times.2.get(c).unwrap());
    }

    println!("\nLast Seen:\n");
    for c in times.3.keys()
    {
        println!("{}: {}", c, times.3.get(c).unwrap());
    }

    println!();
    let window_size = get_size(trace.len(), 1); //Gets time window size from user

    let single_frequencies = get_single_frequencies(times.0, times.2, times.3, window_size, trace.len());   //Calculates single frequencies

    println!("\nSingle Frequencies:\n");
    for c in single_frequencies.keys()  //Prints single frequencies
    {
        println!("{}: {}", c, single_frequencies.get(c).unwrap());
    }
}