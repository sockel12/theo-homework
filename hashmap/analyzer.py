# import matplotlib.pyplot as plt
import numpy as np
import matplotlib.pyplot as plt


# hashmap_size, percentage, runs, test_cases, test_time, collision, avg_collsion, collision_rate

# make a function that reads a file called file "test1" in data directory and returns a list of lists
def read_file(file):
    with open(file, 'r') as f:
        data = f.readlines()
    data = [line.strip().split(',') for line in data]
    return data

data = read_file('./data/test4')

#[hashmap_size, percentage, runs, test_cases, test_time, collision, avg_collsion, collision_rate]

# print(data)

# make the x axis scale between 0 and 10000 (hashmap_size) and the y axis scale between 0 and 100 (collision_rate)
xAxisScale = np.arange(100, 11000, step=1000)
yAxisScale = np.arange(50, 110, dtype=np.float32, step=10)




x = []
y = []

for i in range(len(data)):
    if i % 25 == 0:
        x.append(int(data[i][0]))
        y.append(float(data[i][7]))


# title = 'Collision Rate on a 50% filled Hashmap'
plt.title("Collision Rate on a 100% filled Hashmap")

# label "hashmap_size" on the x axis and "collision_rate" on the y axis
plt.xlabel('Hashmap size')
plt.ylabel('Collision rate')

plt.plot(x, y)

plt.xticks(xAxisScale)
plt.yticks(yAxisScale)

plt.show()



