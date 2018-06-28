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

impl Clone for Pair
{
    fn clone(&self) -> Pair
    {
        Pair(self.0, self.1)
    }
}

fn get_trace() -> Vec<char>
{
    let mut trace: Vec<char> = Vec::new();

    loop
    {
        print!("Please input a trace: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_string();

        let mut valid = true;
        for c in input.chars()
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

        if valid
        {
            break;
        }
    }

    return trace;
}

fn get_histograms(trace: &Vec<char>) -> (HashMap<char, HashMap<usize, usize>>, HashMap<Pair, HashMap<usize, usize>>, HashMap<char, usize>, HashMap<char, usize>)
{
    let mut first_seen: HashMap<char, usize> = HashMap::new();
    let mut last_seen: HashMap<char, usize> = HashMap::new();
    let mut reuse_times: HashMap<char, HashMap<usize, usize>> = HashMap::new();
    let mut switch_times: HashMap<Pair, HashMap<usize, usize>> = HashMap::new();

    for i in 0 .. trace.len()
    {
        let c = trace [i];

        if !reuse_times.contains_key(&c)
        {
            if last_seen.contains_key(&c)
            {
                reuse_times.insert(c.clone(), HashMap::new());
            }
            else
            {
                first_seen.insert(c.clone(), i + 1);
            }
        }

        if reuse_times.contains_key(&c)
        {
            let rt = (i + 1) - last_seen.get(&c).unwrap();
            let mut temp = 1;

            if reuse_times.get(&c).unwrap().contains_key(&rt)
            {
                temp = reuse_times.get(&c).unwrap().get(&rt).unwrap().clone() + 1;
            }
            
            reuse_times.get_mut(&c).unwrap().insert(rt, temp);
        }

        for j in last_seen.keys()
        {
            if *j != c
            {
                let t = Pair(c, *j);
                let st = (i + 1) - last_seen.get(j).unwrap();
                let mut temp = 1;

                if switch_times.contains_key(&t)
                {
                    if switch_times.get(&t).unwrap().contains_key(&st)
                    {
                        temp = switch_times.get(&t).unwrap().get(&st).unwrap().clone() + 1;
                    }
                }
                else
                {
                    switch_times.insert(t.clone(), HashMap::new());
                }

                switch_times.get_mut(&t).unwrap().insert(st, temp);
            }
        }

        last_seen.insert(c, i + 1);
    }

    (reuse_times, switch_times, first_seen, last_seen)
}

fn get_size(trace: usize, start: usize) -> usize
{
    loop
    {
        let mut num = 0;
        loop
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
        
        if !(num > (trace - start) || num <= 1)
        {
            return num;
        }
        else
        {
            println!("Invalid Input");
        }
    }
}

fn get_single_frequencies(reuse_times: HashMap<char, HashMap<usize, usize>>, first_seen: HashMap<char, usize>, last_seen: HashMap<char, usize>, window_size: usize, trace_length: usize) -> HashMap<char, usize>
{
    let mut single_frequencies: HashMap<char, usize> = HashMap::new();
    let total_windows = trace_length - window_size + 1;

    for i in first_seen.keys()
    {
        let rt = first_seen.get(&i).unwrap();
        if rt > &window_size
        {
            single_frequencies.insert(*i, rt - window_size);
        }
    }

    for i in last_seen.keys()
    {
        let rt = trace_length - (last_seen.get(&i).unwrap() - 1);
        if rt > window_size
        {
            if single_frequencies.contains_key(&i)
            {
                let mut temp = single_frequencies.get(&i).unwrap().clone();
                temp = temp + (rt - window_size);
                single_frequencies.insert(*i, temp);
            }
            else
            {
                single_frequencies.insert(*i, rt - window_size);
            }
        }
    }

    for i in reuse_times.keys()
    {
        for rt in reuse_times.get(i).unwrap().keys()
        {
            let f = reuse_times.get(i).unwrap().get(rt).unwrap();
            if rt > &window_size
            {
                if single_frequencies.contains_key(&i)
                {
                    let mut temp = single_frequencies.get(&i).unwrap().clone();
                    temp = temp + f * (rt - window_size);
                    single_frequencies.insert(*i, temp);
                }
                else
                {
                    single_frequencies.insert(*i, f * (rt - window_size));
                }
            } 
        }
    }

    for i in first_seen.keys()
    {
        if single_frequencies.contains_key(&i)
        {
            let mut temp = single_frequencies.get(&i).unwrap().clone();
            temp = total_windows - temp;
            single_frequencies.insert(*i, temp);
        }
        else
        {
            single_frequencies.insert(*i, total_windows);
        }
    }

    single_frequencies
}

fn main()
{
    println!("\nThis program calculates affinity for a given trace and window size.");

    let trace = get_trace();
    let times = get_histograms(&trace);

    println!("\nReuse Time Histogram:");
    for c in times.0.keys()
    {
        println!("\n{}:", c);
        for s in times.0.get(c).unwrap().keys()
        {
            println!("{}: {}", s, times.0.get(c).unwrap().get(s).unwrap());
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
    let window_size = get_size(trace.len(), 1);

    let single_frequencies = get_single_frequencies(times.0, times.2, times.3, window_size, trace.len());

    println!("\nSingle Frequencies:\n");
    for c in single_frequencies.keys()
    {
        println!("{}: {}", c, single_frequencies.get(c).unwrap());
    }
}