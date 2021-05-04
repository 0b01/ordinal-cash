import json
def convert(i):
    h = '{0:0{1}X}'.format(i, 64)
    # print(h)
    n = 64/4
    p = [int(h[i:i+n], 16) for i in range(0, len(h), n) ]
    txt = ("[" + ", ".join("{}u64".format(i) for i in p[::-1]) + "]")
    # print(txt)
    # print("U256::new(BigInteger256::new(" + txt + ")), ")
    return txt

with open("../tornado-core/build/circuits/withdraw_verification_key.json") as f:
    txt = f.read()

d = json.loads(txt)

def p1(property, alpha):
    alpha_txt = '''vk.{property} = G1Affine::new(
        field_new!(Fq, "{c0}"),
        field_new!(Fq, "{c1}"),
        false);
    '''.format(
        convert(int(alpha[0])),
        convert(int(alpha[1])),
        property=property,
        c0 = alpha[0],
        c1 = alpha[1],
        )
    # print(alpha_txt)
    return alpha_txt

def p2(property, beta):
    beta_txt = """vk.{property} = G2Affine::new(
        field_new!(
            Fq2,
            field_new!(Fq, "{c0}"),
            field_new!(Fq, "{c1}")
        ),
        field_new!(
            Fq2,
            field_new!(Fq, "{c2}"),
            field_new!(Fq, "{c3}")
        ),
        false,
    );""".format(
        convert(int(beta[0][1])),
        convert(int(beta[0][0])),
        convert(int(beta[1][1])),
        convert(int(beta[1][0])),
        property=property,
        c0 = beta[0][1],
        c1 = beta[0][0],
        c2 = beta[1][1],
        c3 = beta[1][0],
        )
    return beta_txt
print("fn verifying_key() -> VerifyingKey<Bn254> {")
print("let mut vk = VerifyingKey::default();")
print(p1("alpha_g1", d['vk_alfa_1']))
print(p2("beta_g2", d['vk_beta_2']))
print(p2("gamma_g2", d['vk_gamma_2']))
print(p2("delta_g2", d['vk_delta_2']))
print("vk.gamma_abc_g1 = vec![Default::default(); {0}];".format(len(d['IC'])))
for i, pt in enumerate(d['IC']):
    print(p1("gamma_abc_g1[{i}]".format(i=i), pt))
print("vk")
print("}")