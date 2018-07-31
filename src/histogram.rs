use std::collections::HashMap;  //Can be used with bucket to create sublog histograms
use std::hash::{Hash, Hasher};  //Used for custom bucket hashing

pub struct Histogram <T>    //Stores a Histogram
{
    sublog_bits: u64,   //Stores sublog bits
    max_reuse_time: u64,    //Stores largest reuse time for histogram
    values: Vec <T>,  //Stores frequency values for histogram
}

impl Histogram <u64>  //Implements methods for a histogram with numeric frequencies
{
    pub fn new_single(sb: u64, mrt: u64) -> Histogram <u64>  //Constructor that takes in the sublog bits and the maximum reuse time
    {
        Histogram {sublog_bits: sb, max_reuse_time: mrt, values: vec!(0; (convert_value_to_index(mrt, sb) + 1) as usize)}   //Creates a histogram with a vector of an appropriate fixed length
    }

    pub fn add(&mut self, reuse_time: u64)
    {
        if reuse_time <= self.max_reuse_time && reuse_time >= 1    //Makes sure the reuse time is in range
        {
            let frequency = self.values[convert_value_to_index(reuse_time, self.sublog_bits) as usize].clone(); //Retrieves old frequency
            self.values[convert_value_to_index(reuse_time, self.sublog_bits) as usize] = frequency + 1; //Sets bucket value to incremented frequency
        }
        else
        {
            panic!("reuse time {} out of bounds [{}, {}]", reuse_time, 1, self.max_reuse_time);
        }
    }

    pub fn insert(&mut self, reuse_time: u64, frequency: u64)   //Inserts a value into the histogram at a given reuse time
    {
        if reuse_time <= self.max_reuse_time && reuse_time >= 1    //Makes sure the reuse time is in range
        {
            self.values[convert_value_to_index(reuse_time, self.sublog_bits) as usize] = frequency; //Sets bucket value to frequency
        }
        else
        {
            panic!("reuse time {} out of bounds [{}, {}]", reuse_time, 1, self.max_reuse_time);
        }
    }

    pub fn get_frequency(&self, reuse_time: u64) -> u64   //Retreives the frequency value at a given reuse time
    {
        if reuse_time > self.max_reuse_time //Makes sure reuse time is in range
        {
            panic!("reuse time {} out of bounds [{}, {}]", reuse_time, 1, self.max_reuse_time);
        }
        self.values[convert_value_to_index(reuse_time, self.sublog_bits) as usize]  //Returns the frequency in the reuse time's bucket
    }

    pub fn get_values(&self) -> Vec <u64>  //Returns copy of internal vector
    {
        self.values.clone()
    }

    pub fn get_max_reuse_time(&self) -> u64 //Returns copy of max reuse time
    {
        self.max_reuse_time.clone()
    }
}

impl Histogram <(u64, u64, u64, u64)>  //Implements methods for the histogram
{
    pub fn new_tuple(sb: u64, mrt: u64) -> Histogram <(u64, u64, u64, u64)>  //Constructor that takes in the sublog bits and the maximum reuse time
    {
        let mut values = vec!((0, 0, 0, 0); (convert_value_to_index(mrt, sb) + 1) as usize);    //Creates a vector of empty tuples
        for i in 1 ..= mrt   //Iterates through all possibel reuse times
        {
            let bucket = convert_value_to_index(i, sb); //Indexes vector
            if values [bucket].0 == 0   //Checks if bucket hasn't been processed
            {
                values [bucket].0 = i;  //Sets min to current reuse time
            }
            values [bucket].1 = i;  //Sets max to current reuse time
        }
        Histogram {sublog_bits: sb, max_reuse_time: mrt, values}   //Creates a histogram with the appropriate vector
    }

    pub fn add(&mut self, reuse_time: u64)   //Inserts a value into the histogram at a given reuse time
    {
        if reuse_time <= self.max_reuse_time && reuse_time >= 1    //Makes sure the reuse time is in range
        {
            let sum = self.values[convert_value_to_index(reuse_time, self.sublog_bits) as usize].2.clone(); //Retrieves old sum
            self.values[convert_value_to_index(reuse_time, self.sublog_bits) as usize].2 = sum + reuse_time;    //Inserts sum incremented by reuse time

            let frequency = self.values[convert_value_to_index(reuse_time, self.sublog_bits) as usize].3.clone();   //Retrieves old freuency
            self.values[convert_value_to_index(reuse_time, self.sublog_bits) as usize].3 = frequency + 1;   //Inserts incremented frequency
        }
        else
        {
            panic!("reuse time {} out of bounds [{}, {}]", reuse_time, 1, self.max_reuse_time);
        }
    }

    pub fn get_tuple(&self, reuse_time: u64) -> (u64, u64, u64, u64)   //Retreives the value tuple at a given reuse time
    {
        if reuse_time > self.max_reuse_time || reuse_time < 1 //Makes sure reuse time is in range
        {
            panic!("reuse time {} out of bounds [{}, {}]", reuse_time, 1, self.max_reuse_time);
        }
        self.values[convert_value_to_index(reuse_time, self.sublog_bits) as usize]  //Returns the tuple in the reuse time's bucket
    }

    pub fn get_frequency(&self, reuse_time: u64) -> u64   //Retreives the frequency value at a given reuse time
    {
        if reuse_time > self.max_reuse_time || reuse_time < 1 //Makes sure reuse time is in range
        {
            panic!("reuse time {} out of bounds [{}, {}]", reuse_time, 1, self.max_reuse_time);
        }
        self.values[convert_value_to_index(reuse_time, self.sublog_bits) as usize].3  //Returns the frequency in the reuse time's bucket
    }

    pub fn get_values(&self) -> Vec <(u64, u64, u64, u64)>  //Returns copy of internal vector
    {
        self.values.clone()
    }

    pub fn get_max_reuse_time(&self) -> u64 //Returns copy of max reuse time
    {
        self.max_reuse_time.clone()
    }
}

pub struct Bucket (u64, u64);  //Custom bucket struct to index reuse time (takes in value, sublog bits)

impl Hash for Bucket    //Turns raw indexes into bucket indexes
{
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        convert_value_to_index(self.0, self.1).hash(state);
    }
}

impl PartialEq for Bucket   //Defines equality for Bucket
{
    fn eq(&self, other: &Bucket) -> bool  //Checks for equality of bucket indexes
    {
        convert_value_to_index(self.0, self.1) == convert_value_to_index(other.0, other.1)
    }
}

impl Eq for Bucket
{

}

impl Clone for Bucket //Defines copying for custom bucket
{
    fn clone(&self) -> Bucket //Returns a copy of the custom bucket
    {
        Bucket(self.0, self.1)
    }
}

pub fn convert_value_to_index(value: u64, sublog_bits: u64) -> usize    //Taken from histo.h
{
    if value < (1 << sublog_bits)   //Ignores values too small to be bucketized
    {
        return value as usize;
    }

    let most_significant_bit = (63 - value.leading_zeros()) as u64;  //Find's value's most significant bit
    let shift = most_significant_bit - sublog_bits; //Defines shift as difference between most significant bit and sublog bits
    let mut index = value >> shift; //Sets index as value shifted by shift
    index = index & ((1 << sublog_bits) - 1);   //Does a bitwise and with sublog bits number of 1's

    (index + ((shift + 1) << sublog_bits)) as usize    //Adds the shift + 1 shfted by the number of sublogg bits and to the index
}

// pub fn convert_index_to_value(index: u64, sublog_bits: u64) -> u64     //Not working yet
// {
//     let shift = index >> sublog_bits;
//     let temp = index & ((1 << sublog_bits) - 1);
    
//     if shift == 0
//     {
//         return index;
//     }
    
//     ((1 << sublog_bits) + temp) << (shift - 1)
// }

#[test]
fn test_clone() //Tests clone equality
{
    let b1 = Bucket(0, 0);
    let b2 = b1.clone();
    assert_eq!(b1.0, b2.0);
    assert_eq!(b1.1, b2.1);
}

#[test]
fn test_value_to_index()    //Demonstrates how the two adjacent values index the same bucket
{
    let b1 = Bucket(512, 8);
    let b2 = Bucket(513, 8);
    assert_eq!(convert_value_to_index(b1.0, b1.1), convert_value_to_index(b2.0, b2.1));
}

// #[test]  //Fails
// fn test_index_to_value()
// {
//     assert_eq!(convert_index_to_value(514, 8), 514);
// }

#[test]
fn test_hash()  //Demonstrates bucket usage in HashMap
{
    let b1 = Bucket(512, 8);
    let b2 = Bucket(513, 8);

    let mut histogram: HashMap<Bucket, usize> = HashMap::new();
    histogram.insert(b1.clone(), 1);

    if histogram.contains_key(&b2)  //Finds ame bucket as b1
    {
        let temp = histogram.get(&b2).unwrap().clone();
        histogram.insert(b2, temp + 1); //Overwrites b1
    }

    assert_eq!(*histogram.get(&b1).unwrap(), 2);
}

#[test]
fn test_histogram_insertion() //Tests histogram buckets
{
    let mut h = Histogram::new_single(8, 513); //Creates a new histogram for given sublog bits and maximum reuse time
    h.insert(512, 2);   //Inserts a value into the same bucket
    assert_eq!(h.get_frequency(513), 2); //Checks the bucket value
}

#[test]
fn test_histogram_addition() //Tests frequency incrementation
{
    let mut h = Histogram::new_single(8, 512); //Creates a new histogram for given sublog bits and maximum reuse time
    h.add(512); //Adds to 512 bucket
    assert_eq!(h.get_frequency(512), 1);   //Checks frequency at 512
}

#[test]
fn test_histogram_values()  //Tests iterating through histogram
{
    let sublog_bits = 1;
    let mut h1 = Histogram::new_single(sublog_bits, 7); //Creates a new histogram for given sublog bits and maximum reuse time
    for i in 1 ..= 7    //Fills each bucket with its bucket size
    {
        let temp = h1.get_frequency(i);
        h1.insert(i, temp + 1);
    }

    let values = h1.get_values();   //Retrieves histogram values
    for i in 1 ..= 7    //Iterates through histogram
    {
        let mut exponent = (63 - (i as u64).leading_zeros()) as i64 - sublog_bits as i64; //Defines exponent in 2^(bits - sublog_bits) formula
        if exponent < 0 //Sets exponent to 0 if less than 0
        {
            exponent = 0;
        }
        assert_eq!(values [convert_value_to_index(i, sublog_bits) as usize], u64::pow(2, exponent as u32));   //Checks if bucket size matches theory
    }
}

#[test]
fn test_histogram_max_reuse_time()  //Tests max reuse time storage
{
    let sublog_bits = 2;    //Sets sublog bits to 2
    let max_reuse_time = 10;    //Sets max reuse time to 10
    let h = Histogram::new_single(sublog_bits, max_reuse_time);    //Creates a histogram with 2 sublog bits and a max reuse time of 10
    assert_eq!(h.get_max_reuse_time(), max_reuse_time);    //Checks to see if the stored max reuse time is 10
}

#[test]
fn test_histogram_tuple_add()   //Tests tuple values when addding to bucket
{
    let mut h = Histogram::new_tuple(8, 513);   //Creates a new histogram for given sublog bits and maximum reuse time
    h.add(512); //Adds to 512's bucket
    h.add(513); //Adds again to same bucket
    assert_eq!(h.get_tuple(512), (512, 513, 1025, 2));  //Checks tuple value
    assert_eq!(h.get_frequency(513), 2);    //Checks frequency value
}