
This tool could be used by a lot of persons if it was more user friendly.

Let's add a graph output (console at first ? or but could be any other output)
- 1 : we'll have to pick an in-memory graph crate to store all the nodes (yaml, template files, output files) and interactions : (a yaml load other yamls, the goes throught multiple templates, that output multiple files each) : perhaps something like : https://crates.io/crates/petgraph
- 3 : we can encapsulate the graph behind some really easy visitor api-ed struct that we will pass into each functions of the process
- 4 : so in the end the codegenr command can output something like :
file.yaml
  --loaded -- > other.yaml
  --ran through--> rust_template.hbs
     -- output --> file_one.rs
     -- output --> file_two.rs
  --ran through--> documentation_template.hbs
    -- outpout --> this_struct.md
    -- outpout --> other_struct.md

file2.yaml 
   can be handled by same template
   and output other files 
  ** that is why A TREE VIEW LIKE THIS IS NOT OK FOR THE NEED

Seems like petgraph have some stuff to output of .dot file that is a graph file (https://docs.rs/petgraph/latest/petgraph/dot/index.html)
