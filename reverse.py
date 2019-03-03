from timeit import timeit


@timeit
def reverse_strings(data):
    rev = []
    for word in data:
        rev.append(''.join(reversed(word)))
    return rev


@timeit
def main():
    path = 'words.txt'

    with open(path) as f:
        data = [x.strip('\n') for x in f.readlines()]

    for x in range(20):
        rev = reverse_strings(data)
        # print(len(rev))


if __name__ == '__main__':
    main()
