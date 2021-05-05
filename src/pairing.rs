// https://github.com/iden3/snarkjs/blob/master/templates/verifier_groth16.sol
use ark_bn254::{
    Bn254, Fq, Fq12, Fq2, FqParameters, Fr, FrParameters, G1Affine, G1Projective, G2Affine,
    G2Projective,
};
use ark_ec::{AffineCurve, PairingEngine, ProjectiveCurve};
use ark_ff::{field_new, fields::Field, BigInteger, BigInteger256, One, PrimeField, Zero};
use ark_groth16::{prepare_verifying_key, VerifyingKey};

pub struct Proof {
    pub a: G1Affine,
    pub b: G2Affine,
    pub c: G1Affine,
}

// #[derive(Default)]
// pub struct VerifyingKey {
//     pub alpha_g1: G1Affine,
//     pub beta_g2: G2Affine,
//     pub gamma_g2: G2Affine,
//     pub delta_g2: G2Affine,
//     pub gamma_abc_g1: Vec<G1Affine>,
// }

fn verifying_key() -> VerifyingKey<Bn254> {
    let mut vk = VerifyingKey::default();
    vk.alpha_g1 = G1Affine::new(
        field_new!(
            Fq,
            "20692898189092739278193869274495556617788530808486270118371701516666252877969"
        ),
        field_new!(
            Fq,
            "11713062878292653967971378194351968039596396853904572879488166084231740557279"
        ),
        false,
    );

    vk.beta_g2 = G2Affine::new(
        field_new!(
            Fq2,
            field_new!(
                Fq,
                "281120578337195720357474965979947690431622127986816839208576358024608803542"
            ),
            field_new!(
                Fq,
                "12168528810181263706895252315640534818222943348193302139358377162645029937006"
            ),
        ),
        field_new!(
            Fq2,
            field_new!(
                Fq,
                "9011703453772030375124466642203641636825223906145908770308724549646909480510"
            ),
            field_new!(
                Fq,
                "16129176515713072042442734839012966563817890688785805090011011570989315559913"
            ),
        ),
        false,
    );
    vk.gamma_g2 = G2Affine::new(
        field_new!(
            Fq2,
            field_new!(
                Fq,
                "10857046999023057135944570762232829481370756359578518086990519993285655852781"
            ),
            field_new!(
                Fq,
                "11559732032986387107991004021392285783925812861821192530917403151452391805634"
            ),
        ),
        field_new!(
            Fq2,
            field_new!(
                Fq,
                "8495653923123431417604973247489272438418190587263600148770280649306958101930"
            ),
            field_new!(
                Fq,
                "4082367875863433681332203403145435568316851327593401208105741076214120093531"
            ),
        ),
        false,
    );
    vk.delta_g2 = G2Affine::new(
        field_new!(
            Fq2,
            field_new!(
                Fq,
                "150879136433974552800030963899771162647715069685890547489132178314736470662"
            ),
            field_new!(
                Fq,
                "21280594949518992153305586783242820682644996932183186320680800072133486887432"
            ),
        ),
        field_new!(
            Fq2,
            field_new!(
                Fq,
                "11434086686358152335540554643130007307617078324975981257823476472104616196090"
            ),
            field_new!(
                Fq,
                "1081836006956609894549771334721413187913047383331561601606260283167615953295"
            ),
        ),
        false,
    );
    vk.gamma_abc_g1 = vec![Default::default(); 7];
    vk.gamma_abc_g1[0] = G1Affine::new(
        field_new!(
            Fq,
            "16225148364316337376768119297456868908427925829817748684139175309620217098814"
        ),
        field_new!(
            Fq,
            "5167268689450204162046084442581051565997733233062478317813755636162413164690"
        ),
        false,
    );

    vk.gamma_abc_g1[1] = G1Affine::new(
        field_new!(
            Fq,
            "12882377842072682264979317445365303375159828272423495088911985689463022094260"
        ),
        field_new!(
            Fq,
            "19488215856665173565526758360510125932214252767275816329232454875804474844786"
        ),
        false,
    );

    vk.gamma_abc_g1[2] = G1Affine::new(
        field_new!(
            Fq,
            "13083492661683431044045992285476184182144099829507350352128615182516530014777"
        ),
        field_new!(
            Fq,
            "602051281796153692392523702676782023472744522032670801091617246498551238913"
        ),
        false,
    );

    vk.gamma_abc_g1[3] = G1Affine::new(
        field_new!(
            Fq,
            "9732465972180335629969421513785602934706096902316483580882842789662669212890"
        ),
        field_new!(
            Fq,
            "2776526698606888434074200384264824461688198384989521091253289776235602495678"
        ),
        false,
    );

    vk.gamma_abc_g1[4] = G1Affine::new(
        field_new!(
            Fq,
            "8586364274534577154894611080234048648883781955345622578531233113180532234842"
        ),
        field_new!(
            Fq,
            "21276134929883121123323359450658320820075698490666870487450985603988214349407"
        ),
        false,
    );

    vk.gamma_abc_g1[5] = G1Affine::new(
        field_new!(
            Fq,
            "4910628533171597675018724709631788948355422829499855033965018665300386637884"
        ),
        field_new!(
            Fq,
            "20532468890024084510431799098097081600480376127870299142189696620752500664302"
        ),
        false,
    );

    vk.gamma_abc_g1[6] = G1Affine::new(
        field_new!(
            Fq,
            "15335858102289947642505450692012116222827233918185150176888641903531542034017"
        ),
        field_new!(
            Fq,
            "5311597067667671581646709998171703828965875677637292315055030353779531404812"
        ),
        false,
    );

    vk
}

fn pcs(ps: &[G1Affine], qs: &[G2Affine]) -> bool {
    let mut acc = Fq12::one();
    for (p, q) in ps.iter().zip(qs.iter()) {
        acc *= Bn254::pairing(*p, *q);
    }
    // dbg!(acc.is_one());
    acc.is_one()
}

#[deprecated = "using ark-groth17 for now"]
pub fn verify_proof(proof: Proof, input: &[Fr]) -> bool {
    assert!(input.len() == 6);
    let vk = verifying_key();

    let mut vk_x = G1Affine::zero();
    vk_x += &vk.gamma_abc_g1[0];
    for i in 0..6 {
        vk_x += &(vk.gamma_abc_g1[i + 1]
            .mul(input[i].into_repr())
            .into_affine());
    }

    pcs(
        &[-proof.a, vk.alpha_g1, vk_x, proof.c],
        &[proof.b, vk.beta_g2, vk.gamma_g2, vk.delta_g2],
    )
}

#[cfg(test)]
mod tests {
    use ark_bn254::G1Projective;
    use std::ops::MulAssign;

    use super::*;

    #[test]
    fn test_curve() {
        let a = G1Affine::prime_subgroup_generator();
        let b = G2Affine::prime_subgroup_generator();

        assert_eq!(Bn254::pairing(a, b.mul(2)), Bn254::pairing(a, b).pow([2]));
        assert_eq!(Bn254::pairing(a, b.mul(2)), Bn254::pairing(a.mul(2), b));
        assert_eq!(Bn254::pairing(a, -b), Bn254::pairing(-a, b));
        assert!(pcs(&[a, -a], &[b, b]));
    }

    #[test]
    fn test_verifier() {
        /*
        let proofData = {
            pi_a: [
                '6160569254972695033541751837268483084366769085559190351203251273104161418400',
                '20752637131085270521468616099378777282140643373828587571739440568509124276056',
                '1'
            ],
            pi_b: [
                [
                '13491861502261516983101645835022566763609289651768567310911403618761646949192',
                '3945102381244376369012265840723718630151501000113196168586847393173187043791'
                ],
                [
                '2085938618828745684062388752928952586729586420152554769557897926110027529638',
                '6905794850663567498189336560369926542004763485239983052761138480096642452652'
                ],
                [ '1', '0' ]
            ],
            pi_c: [
                '13305831737468453023847968374851929911131764663101012129143182418423488429101',
                '8142609777444797171303864593507761446494142436560247433779193992616167019550',
                '1'
            ],
            publicSignals: [
                '18141211044530898481780712096785380507009040886197825359491225784587697908689',
                '17369391381428457005685637744737812745147294406289952788882032335952086150537',
                '617288482572789990873151114501867268774234674064',
                '827641930419614124039720421795580660909102123457',
                '50000000000000000',
                '0'
            ]
        };
        */

        let a = G1Affine::new(
            field_new!(
                Fq,
                "6160569254972695033541751837268483084366769085559190351203251273104161418400"
            ),
            field_new!(
                Fq,
                "20752637131085270521468616099378777282140643373828587571739440568509124276056"
            ),
            false,
        );

        let b = G2Affine::new(
            field_new!(
                Fq2,
                field_new!(
                    Fq,
                    "13491861502261516983101645835022566763609289651768567310911403618761646949192"
                ),
                field_new!(
                    Fq,
                    "3945102381244376369012265840723718630151501000113196168586847393173187043791"
                ),
            ),
            field_new!(
                Fq2,
                field_new!(
                    Fq,
                    "2085938618828745684062388752928952586729586420152554769557897926110027529638"
                ),
                field_new!(
                    Fq,
                    "6905794850663567498189336560369926542004763485239983052761138480096642452652"
                ),
            ),
            false,
        );

        let c = G1Affine::new(
            field_new!(
                Fq,
                "13305831737468453023847968374851929911131764663101012129143182418423488429101"
            ),
            field_new!(
                Fq,
                "8142609777444797171303864593507761446494142436560247433779193992616167019550"
            ),
            false,
        );

        let mut public_inputs = vec![
            field_new!(
                Fr,
                "18141211044530898481780712096785380507009040886197825359491225784587697908689"
            ),
            field_new!(
                Fr,
                "17369391381428457005685637744737812745147294406289952788882032335952086150537"
            ),
            field_new!(Fr, "617288482572789990873151114501867268774234674064"),
            field_new!(Fr, "827641930419614124039720421795580660909102123457"),
            field_new!(Fr, "50000000000000000"),
            field_new!(Fr, "0"),
        ];

        assert!(verify_proof(Proof { a, b, c }, &public_inputs));

        let pvk = prepare_verifying_key(&verifying_key());
        let ret = ark_groth16::verify_proof(
            &pvk,
            &ark_groth16::Proof { a, b, c },
            public_inputs.as_slice(),
        );
        assert_eq!(ret, Ok(true));

        public_inputs[1] = field_new!(Fr, "1337");

        assert!(!verify_proof(Proof { a, b, c }, &public_inputs));
        let pvk = prepare_verifying_key(&verifying_key());
        let ret = ark_groth16::verify_proof(
            &pvk,
            &ark_groth16::Proof { a, b, c },
            public_inputs.as_slice(),
        );
        assert_eq!(ret, Ok(false));
    }
}
