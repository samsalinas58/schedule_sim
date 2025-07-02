<h1> Project Description </h1>

This project aims to simulate four different scheduling algorithms. Those scheduling algorithms include: Highest Response Ratio Next (HRRN), Shortest Response Time (SRT), Round Robin (RR), and First Come First Serve (FCFS). The aforementioned acronyms are used throughout the source code and the resulting test data, so keep that in mind. The processes are designed such that the each one contains: 

__Arrival time__: The time t which a process arrives

__Service time__: How long the process "uses the CPU" 

__Disk I/O time__: How long it takes the process to perform an I/O request

__Disk I/O activity__: How many times the process makes an I/O request

<h1> Running the Project </h1>

To be able to run this project, you will need a system that has rust installed, along with cargo.

After cloning the project, simply run the project using 'cargo run'.

<h1> Program Output </h1>

The output of the program is two different data sets. As it is currently implemented, the program defaults to creating a random_results.txt file in the current working directory that the project is run in, showing the results of the randomly generated 30 sets of processes. For the given processes that were provided as a test case by my professor, those will default to writing to the standard output. This was a weird design decision at the time, but it's a trivial problem to solve if you want to send the data to a file, instead.

I have also provided a Report_data.pdf to see the results of the given processes by the algorithm, but in a nice table.

<h2> ... Some reflecting </h2>

If you delve into the source code you may notice one thing: it's messy lol. I did not clearly define the rules for the randomly generated processes, nor should I have used an unsafe block in this project (in finished_ps.rs). However, that means I have experience using unsafe blocks! There are many mistakes I made doing this project, and those are just the start. Let's not mention how there might be 43 warnings, but the input does not vary in a way that results in errors. With that being said, these are things to learn from which I will take into my next projects.