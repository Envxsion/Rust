import algo  
import random
import timeit

def giganormiouslyhuge_list(size=10000, value_range=(0, 10)):
    return [random.randint(*value_range) for _ in range(size)]

def clock_rust():
    option = input("Choose sorting algorithm (merge/quick): ")
    data = giganormiouslyhuge_list()
    print(f"Generated list of {len(data)} integers.")

    if option == "merge":
        time = timeit.timeit(lambda: algo.merge_sort_verbose(data), number=5)
        print(f"Rust merge_sort completed in {time:.4f} seconds.")
    elif option == "quick":
        sorted_list, log = algo.quick_sort_verbose(data)
        time = timeit.timeit(lambda: algo.quick_sort_verbose(data), number=5)
        print(f"Rust quick_sort completed in {time:.4f} seconds.")
        print("Sorted result:", sorted_list)
        print("Steps:\n", log)


if __name__ == "__main__":
    clock_rust()
