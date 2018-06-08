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
    let length = trace.len();

    for i in 0 .. length
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
            // add s(from start to first access), add current i
            times.insert(c, vec![i+1,i]);
        }
    }

    let copy = times.clone();
    let keys = copy.keys();

    for k in keys
    {
        let mut temp = times.get(k).unwrap().clone();
        let last_index = times.get(k).unwrap().len() - 1;
        //temp.remove(last_index);
        temp[last_index] = length-temp[last_index];
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


//function to calculate individual occurrence based on reuse times
fn get_ind_occ(length: usize, winSize: usize, times: &HashMap<char, Vec<usize>>, target: char) -> usize
{
    //get the number of total windows
    let totalWindows = length - winSize + 1;

    //get the number of windows NOT containing the char
    let reuseTimes = times.get(&target).unwrap(); //Unwrap?
    let mut emptyWindows = 0;

    for t in reuseTimes
    {
        if t > &winSize
        {
            emptyWindows = emptyWindows + (t - winSize);
        }

    }

    //individual occurrence = total - NOT
    let occ = totalWindows - emptyWindows;
    return occ;

}

//function to get the two chars
fn get_chars(trace: &HashMap<char, Vec<usize>>) -> Vec<char>
{
    let mut chars: Vec<char> = Vec::new();
    chars = insert_char(trace, chars, "first".to_string());
    chars = insert_char(trace, chars, "second".to_string());
    return chars;
}

fn insert_char(trace: &HashMap<char, Vec<usize>>,mut chars: Vec<char>, order: String) -> Vec<char>
{
    loop
    {
        print!("Please input the {} character for comparison: ", order);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let c = input.trim().to_string().chars().next().unwrap();
        // TODO: handle empty input None Value
        let mut valid = true;
        if c.is_ascii_alphabetic()
        {
            if trace.contains_key(&c)
            {
                chars.push(c);
            }
            else
            {
                println!("Character not in the original trace");
                valid = false;
            }
        }
        else
        {
            println!("Invalid Input");
            valid = false;
        }

        if valid
        {
            break;
        }
    }
    return chars;
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
    let chars = get_chars(&t);
    let char1 = chars[0];
    let char2 = chars[1];

    let temp_result1 = get_ind_occ(trace.len(),s,&t,char1);
    let temp_r2 = get_ind_occ(trace.len(),s,&t,char2);
    println!("Temp Result is {} and {}", temp_result1, temp_r2);

    //get_affinity(t, trace.len(), s);
}
