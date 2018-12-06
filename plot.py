import os
import sys
import csv
import matplotlib.pyplot as plt

start = int(sys.argv[2])
end = int(sys.argv[3])
num_times = end - start + 1

if (len(sys.argv) != 4 or start >= end) and start >= 2:

	print("Invalid Arguments")
	exit()

start_dir = "results/" + sys.argv [1] + "/"

try:
	os.mkdir("graphs/")
	print("Created Folder graphs/")
except OSError:
	print("Folder graphs/ Already Exists")

graph_dir = "graphs/" + sys.argv [1] + "/"

pairs = []
pairx = []
pairy = []

with open(start_dir + os.listdir(start_dir) [0], "r") as tempfile:

	lines = csv.reader(tempfile, delimiter = ";")
	next(lines, None)
	sorted_lines = sorted(lines, key = lambda row: row [0], reverse = False)

	row_count = 0
	for row in sorted_lines:

		pairs.append(row [0])
		row_count = row_count + 1
	
	pairx = [] * row_count
	pairy = [] * row_count

	for i in range(0, row_count):

		pairx.append([0] * num_times)
		pairy.append([0] * num_times)

for filename in os.listdir(start_dir):

	index = int(filename [:filename.find(".")])

	if (index >= start and index <= end):

		with open(start_dir + filename, "r") as csvfile:

			lines = csv.reader(csvfile, delimiter = ";")
			next(lines, None)
			sorted_lines = sorted(lines, key = lambda row: row [0], reverse = False)

			for i in range(0, len(sorted_lines)):

				pairx [i] [index - start] = index
				pairy [i] [index - start] = float(sorted_lines [i] [1])

try:
	os.mkdir(graph_dir)
	print("Created Folder " + graph_dir)
except OSError:
	print("Folder " + graph_dir + " Already Exists")

for i in range (0, len(pairs)):

	plt.figure(i)
	#print (str(i) + ": " + str(pairs [i]) + "\nX: " + str(pairx [i]) + "\nY: " + str(pairy [i]) + "\n")
	plt.plot(pairx [i], pairy [i], "r-")
	
	try:
		pairs [i].decode("ascii")
		plt.title(pairs [i])
	except UnicodeDecodeError:
		plt.title("")
	
	plt.ylim(0, 1)
	plt.yticks([n/10.0 for n in range(11)])
	plt.ylabel("Conditional Probablility")
	plt.xlim(start - 1, end + 1)
	plt.xticks([n + 1 for n in range(num_times + 2)])
	plt.xlabel("Timescale")
	#plt.show()
	plt.savefig(graph_dir + pairs [i] + ".pdf", bbox_inches="tight")
	plt.close()