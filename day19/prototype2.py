def f(n):
    if n == 1:
        return 1
    k = (1 + n//2) % n
    return (f(n - 1) + k - 1) % n + 1

def main() -> None:
    for i in range(1, 10):
        l = list(range(1, i + 1))
        while len(l) > 1:
            print(l)
            del l[len(l) // 2]
            rotate = l.pop(0)
            l.append(rotate)
        print(l)
        expected = l[0]
        print(f"\x1b[{32 if expected == f(i) else 31}mf({i}) = {f(i)}\x1b[0m")
        print()


if __name__ == "__main__":
    main()
