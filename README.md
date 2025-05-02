A. Project Overview

    Goal: What question are you answering?

        The goal of this project is to analyze the Amazon Product Co-Purchasing Network, specifically identifying important relationships and key points in the network. By analyzing this data, I am answering the following questions:

            1. Finding connected components: Identifying groups of products (nodes) that are closely related based on shared co-purchase information.

            2. Centrality measures: Determining the importance of individual nodes in the graph, focusing on degree centrality, which highlights nodes with the highest number of direct neighbors (or co-purchased products).

These analyses are relevant to Amazon in several ways:

    1. Product Recommendations: Understanding the co-purchase relationships between products can help in building better recommendation systems. By identifying which products are frequently bought together, Amazon can offer smarter product recommendations to customers.

    2. Targeted Marketing: Identifying key products in the network can help Amazon target advertisements and promotions more effectively.

    3. Inventory and Stock Management: By analyzing the network's central products, Amazon can anticipate demand patterns for related products, improving supply chain and inventory management.

Thus, the goal of this project is to provide insights into how products are interconnected in Amazon’s marketplace, which can lead to enhanced customer experience and operational efficiency.

Dataset: Source, size

    The dataset I am using is the Amazon Product Co-Purchasing Network. This dataset contains products as nodes and co-purchases as edges, where an edge exists between two products if they are often purchased together. 
    The files are:

        1. com-amazon.ungraph.txt: Contains the edge list of the network (about 400,000 nodes and 3 million edges).

        2. com-amazon.all.dedup.cmty.txt: Contains precomputed community assignments for the graph, which I’m not directly using but help to compare results.

Since the dataset is large, I’ve included it in my project folder (data/), since the raw data is too large for GitHub. This is a link to the dataset I have utilized: https://snap.stanford.edu/data/com-Amazon.html

B. Data Processing

    How I loaded it into Rust

    To load the dataset into Rust, I utilized a graph adjacency list representation using a HashMap<u32, HashSet<u32>>, where the keys are node IDs (products) and the values are sets of neighboring nodes (co-purchased products). 
    I read the edge list from the com-amazon.ungraph.txt file, processing each line and adding edges bidirectionally (since the graph is undirected).

    Any cleaning or transformations applied

    The dataset needed minimal cleaning. 
    I skipped comment lines in the file (which start with #) and ignored any malformed lines. The edges are read and added in both directions to account for the undirected nature of the graph (i.e., if product A and product B are co-purchased, both A -> B and B -> A are recorded).

C. Code Structure

    Modules

    I divided the project into four primary modules:

        1. graph.rs: Responsible for loading the graph from the edge list file.

        2. connected_components.rs: Contains the logic to find connected components using DFS (Depth-First Search).

        3. centrality.rs: Provides centrality measures, focusing on degree centrality.

        4. tests.rs: Contains unit tests for validating the functionality of the other modules.

    This structure was chosen to clearly separate the responsibilities of the program and make it modular. Each module handles a specific aspect of the analysis, making the code easier to maintain and extend (if necessary).

Key Functions & Types

    graph.rs

        Graph: Type alias for the graph structure, representing an adjacency list (HashMap<u32, HashSet<u32>>).
        load_graph: Function to load the graph from a file.
        Inputs: A file path (&str) to the edge list file.
        Outputs: A Graph (type alias for HashMap<u32, HashSet<u32>>).
        Logic: Reads the file line-by-line, parses the node pairs, and populates the adjacency list.

    connected_components.rs

        connected_components: Function to find all connected components using DFS.
        Inputs: A reference to the graph (&HashMap<u32, HashSet<u32>>).
        Outputs: A Vec<Vec<u32>> containing lists of connected nodes.
        Logic: Iterates through each node, performing DFS on unvisited nodes and grouping connected nodes into components.

    centrality.rs

        degree_centrality: Function to compute degree centrality for all nodes in the graph.
        Inputs: A reference to the graph (&HashMap<u32, HashSet<u32>>).
        Outputs: A Vec<(u32, usize)> of node IDs and their respective degrees, sorted by degree in descending order.
        Logic: Counts the neighbors for each node and sorts the nodes based on their degree.

Main Workflow

    Loading the Graph: The main function calls load_graph from graph.rs to load the graph.

    Finding Connected Components: The graph is passed to the connected_components function, which returns a list of connected components.

    Calculating Degree Centrality: The degree_centrality function is used to compute the degree of each node and return the top 5 nodes by degree.

    These modules work together by passing the graph between functions to process the data and compute the results.

D. Tests

Here is the output from running cargo test:

    running 4 tests
    test tests::tests::test_all_connected_nodes ... ok
    test tests::tests::test_connected_components ... ok
    test tests::tests::test_single_node ... ok
    test tests::tests::test_two_disconnected_nodes ... ok

    test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Test Cases

    1. test_connected_components: This test ensures that the algorithm correctly identifies two disconnected components in a small graph. This is important to verify that the DFS implementation works as expected for graphs with multiple connected components.

    2. test_single_node: Tests the case of a graph with only one node. It checks that the function can handle small edge cases where no edges exist.

    3. test_two_disconnected_nodes: This test checks if the algorithm correctly identifies two isolated nodes as separate components.

    4. test_all_connected_nodes: Verifies that the function can correctly identify a single connected component when all nodes are connected in a chain.

E. Results

    Program Outputs:

    Here’s the output of running the program:

        Loaded graph with 334863 nodes
        Found 1 connected components
        Largest connected component size: 334863
        Top 5 nodes by degree:
        Node 548091: degree 549
        Node 458358: degree 324
        Node 222074: degree 257
        Node 199628: degree 230
        Node 515301: degree 228

Interpretation in Project Context:  

    The output shows that there is 1 connected component that includes all 334,863 nodes. The largest connected component contains all the nodes in the graph, which suggests that the graph is one large connected network of co-purchased products. The top 5 nodes by degree centrality are products with the highest number of co-purchases, which can be considered highly influential in the network. These nodes are likely crucial for product recommendations, marketing strategies, and inventory management.

    This is useful for Amazon in improving product recommendations, targeted marketing, and inventory management by focusing on these highly connected products and the key groups of products that tend to be co-purchased together.

F. Usage Instructions

    How to Build and Run
    To build and run the project, follow these steps:

    1. Clone the repository:
    git clone <https://github.com/ColeMontanaro/ColeMontanaro-FinalProject>

    2. Change to the directory:
    cd ColeMontanaro-FinalProject/

    3. Run the program:
    To run the program and see the analysis results:
    cargo run

    4. The output will display the following information:
    Loaded graph with 334863 nodes
    Found 1 connected components
    Largest connected component size: 334863
    Top 5 nodes by degree:
    Node 548091: degree 549
    Node 458358: degree 324
    Node 222074: degree 257
    Node 199628: degree 230
    Node 515301: degree 228

Command-line Arguments:

    This project doesn’t require any user inputs during runtime. You can modify the file path in the main function if needed.

Expected Runtime:

    For my current dataset, the program typically runs within a few seconds depending on my system. Specifically when running cargo test my runtime was 0.00s, and cargo run took 4.08s.

