
def part1():
	with open("depths.txt") as f:
		lines = f.readlines();

		pairs = [(int(lines[i]), int(lines[i+1])) for i in range(len(lines)-1)]
		incs = [1 for (k, l) in pairs if l > k]

		print(sum(incs))

def part2():
	with open("input2.txxt") as f:
		lines = f.readlines();

		windows = [(int(lines[i]), int(lines[i+1]), int(lines[i+2])) for i in range(len(lines)-2)]
		window_sums = [(k + l + m) for (k, l, m) in windows]
		incs = [1 for i in range(len(window_sums)-1) if window_sums[i+1] > window_sums[i] ]

		print(sum(incs))

part2();