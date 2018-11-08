import os
import matplotlib.pyplot as plt

files = []

for filename in os.listdir("results/"):

	lines = open("results/" + filename, "r").readlines()
	file = open("data/" + filename, "w")
	file.writelines(lines[2:])
	file.close()

	lines = open("data/" + filename, "r").readlines()
	lines.sort()
	file = open("data/" + filename, "w")
	file.writelines(lines)
	file.close()
	
	files.append(lines)

pairs = []
pairx = []
pairy = []

for line in open("data/" + os.listdir("data/") [0], "r").readlines():
	
	pairs.append(line [:6])
	pairx.append([])
	pairy.append([])

for i in xrange(0, len(pairs)):
	
	pairy [i] += [0]

	for filename in os.listdir("data/"):

		lines = open("data/" + filename, "r").readlines()
		pairy [i] += [float(lines [i] [8:])]
	
	pairx [i] += [n + 1 for n in range(len(os.listdir("data/")) + 1)]


for i in range (0, len(pairs)):

	plt.figure(i)
	plt.plot(pairx [i], pairy [i], "r-")
	plt.title(pairs [i])
	plt.ylim(0, 1)
	plt.yticks([n/10.0 for n in range(11)])
	plt.ylabel("Conditional Probablility")
	plt.xlim(1, 20)
	plt.xticks([n + 1 for n in range(len(os.listdir("data/")) + 1)])
	plt.xlabel("Timescale")
	plt.savefig("graphs/" + pairs [i] + ".pdf", bbox_inches="tight")
	print (pairs [i])