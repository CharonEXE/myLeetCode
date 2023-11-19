impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        
        strs.into_iter() // create an iterator over the Vec
            .reduce(|acc, cur| // call reduce on the iterator to reduce to one output only
            {
                acc.chars() // turn String into Chars
                    .zip(cur.chars()) // zip to combine multiple iterator into one iterator
                    .take_while(|(a, c)| a==c) // create iterator out of the zipped iterator take take the elements that fit the requirement, stop once one of the element doesnt fit and ignore all thre remaining elements
                    .map(|(c, _)| c) // create iterator from the iterator created from take_while and map into the zipped iterator format
                    .collect::<String>() // collect all the value into String
            }).unwrap() // unwrap the option return from the reduce

        
    }
}