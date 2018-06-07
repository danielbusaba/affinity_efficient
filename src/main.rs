use std::io;
use std::io::Write;
use std::collections::HashMap;

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

fn get_reuse_time(trace: &Vec<char>) -> HashMap<char, Vec<usize>>
{
    let mut times: HashMap<char, Vec<usize>> = HashMap::new();
    for i in 0 .. trace.len()
    {
        let c = trace [i];
        if times.contains_key(&c)
        {
            let mut temp = times.get(&c).unwrap().clone();
            let len = times.get(&c).unwrap().len() - 1;
            temp[len] = i - temp[len];
            temp.push(i);
            times.insert(c, temp.to_vec());
        }
        else
        {
            times.insert(c, vec!(i));
        }
    }

    let copy = times.clone();
    let keys = copy.keys();

    for k in keys
    {
        let mut temp = times.get(k).unwrap().clone();
        let last_index = times.get(k).unwrap().len() - 1;
        temp.remove(last_index);
        times.insert(*k, temp.to_vec());
    }

    for k in times.keys()
    {
        print!("{}: ", k);
        for t in times.get(k).unwrap()
        {
            print!("{} ", t);
        }
        println!("");
    }

    return times;
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

fn get_affinity(times: HashMap<char, Vec<usize>>, length: usize, size: usize)
{
    let windows = length - size + 1;
}

fn main()
{
    println!("\nThis program calculates affinity for a given trace and window size.");

    let trace = get_trace();
    let t = get_reuse_time(&trace);
    let s = get_size(trace.len(), 1);

    get_affinity(t, trace.len(), s);
}