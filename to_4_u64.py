def convert(i):
    h = '{0:0{1}X}'.format(i, 64)
    # print(h)
    n = 64/4
    p = [int(h[i:i+n], 16) for i in range(0, len(h), n) ]
    txt = ("[" + ", ".join("{}u64".format(i) for i in p[::-1]) + "]")
    # print(txt)
    # print("U256::new(BigInteger256::new(" + txt + ")), ")
    return txt

if __name__ == "__main__":
    import fileinput

    for line in fileinput.input():
        print(convert(int(line)))

    # g = 21888242871839275222246405745257275088548364400416034343698204186575808495617
    # for i in map(int, txt):
    #     convert(i)