from bench import measure

print("Python tests")
measure(1, "python tests.py", 0, 1000)
print("Rust tests")
measure(1, "./tests.exe", 0, 1000)

print("Python add")
measure(1, "python add.py", 0, 1000)
print("Rust add")
measure(1, "./add.exe", 0, 1000)

print("Python mul")
measure(1, "python add.py", 0, 1000)
print("Rust mul")
measure(1, "./add.exe", 0, 1000)

print("Python mem")
measure(1, "python mem.py", 0, 1000)
print("Rust mem")
measure(1, "./mem.exe", 0, 1000)