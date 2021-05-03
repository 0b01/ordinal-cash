// https://github.com/iden3/snarkjs/blob/master/templates/verifier_groth16.sol
use ark_ec::{AffineCurve, PairingEngine, ProjectiveCurve};
use ark_ff::{fields::Field, BigInteger256, One, Zero};
use ark_bn254::{Bn254, Fq12, Fr, G1Affine, G2Affine, FqParameters, FrParameters, G1Projective, G2Projective};
use ark_groth16::{prepare_verifying_key, VerifyingKey};

pub type U256 = ark_ff::Fp256<ark_bn254::FqParameters>;
pub type FrU256 = ark_ff::Fp256<ark_bn254::FrParameters>;

fn verifying_key() -> VerifyingKey<Bn254> {
    let mut vk = VerifyingKey::default();
    vk.alpha_g1 = G1Affine::new(
        U256::new(BigInteger256::new([
            2320423973851029649u64,
            8916178567782527473u64,
            11796251668749875282u64,
            3296568872293101285u64,
        ])), // 20692898189092739278193869274495556617788530808486270118371701516666252877969
        U256::new(BigInteger256::new([
            14889541792691161055u64,
            4827721236820259407u64,
            11639101353050404710u64,
            1865998572599382637u64,
        ])), // 11713062878292653967971378194351968039596396853904572879488166084231740557279
        false,
    );

    vk.beta_g2 = G2Affine::new(
        ark_ff::Fp2::new(
            U256::new(BigInteger256::new([
                2802343415646674798u64,
                6436255260775964289u64,
                7771138277089019314u64,
                1938558481788197500u64,
            ])), // 12168528810181263706895252315640534818222943348193302139358377162645029937006
            U256::new(BigInteger256::new([
                5714556100039207638u64,
                13840956571565422495u64,
                5987349592421371165u64,
                44785091940186339u64,
            ])), // 281120578337195720357474965979947690431622127986816839208576358024608803542
        ),
        ark_ff::Fp2::new(
            U256::new(BigInteger256::new([
                18270623098688934377u64,
                7374529823324200399u64,
                7162334397420450950u64,
                2569526064041000562u64,
            ])), // 16129176515713072042442734839012966563817890688785805090011011570989315559913
            U256::new(BigInteger256::new([
                8912529107179052606u64,
                3521353661490560267u64,
                1878274539807868375u64,
                1435647187772860467u64,
            ])), // 9011703453772030375124466642203641636825223906145908770308724549646909480510
        ),
        false,
    );
    vk.gamma_g2 = G2Affine::new(
        ark_ff::Fp2::new(
            U256::new(BigInteger256::new([
                10945020018377822914u64,
                17413811393473931026u64,
                8241798111626485029u64,
                1841571559660931130u64,
            ])), // 11559732032986387107991004021392285783925812861821192530917403151452391805634
            U256::new(BigInteger256::new([
                5106727233969649389u64,
                7440829307424791261u64,
                4785637993704342649u64,
                1729627375292849782u64,
            ])), // 10857046999023057135944570762232829481370756359578518086990519993285655852781
        ),
        ark_ff::Fp2::new(
            U256::new(BigInteger256::new([
                6173549831154472795u64,
                13567992399387660019u64,
                17050234209342075797u64,
                650358724130500725u64,
            ])), // 4082367875863433681332203403145435568316851327593401208105741076214120093531
            U256::new(BigInteger256::new([
                5541340697920699818u64,
                16416156555105522555u64,
                5380518976772849807u64,
                1353435754470862315u64,
            ])), // 8495653923123431417604973247489272438418190587263600148770280649306958101930
        ),
        false,
    );
    vk.delta_g2 = G2Affine::new(
        ark_ff::Fp2::new(
            U256::new(BigInteger256::new([
                5213764371784450568u64,
                6501183050373073302u64,
                14566812404951809013u64,
                3390194367816482299u64,
            ])), // 21280594949518992153305586783242820682644996932183186320680800072133486887432
            U256::new(BigInteger256::new([
                10533399507885132422u64,
                8502066251625666885u64,
                1517065339218800882u64,
                24036433181161453u64,
            ])), // 150879136433974552800030963899771162647715069685890547489132178314736470662
        ),
        ark_ff::Fp2::new(
            U256::new(BigInteger256::new([
                9061981669440563599u64,
                9970460087211956522u64,
                6425071750582774905u64,
                172346419185440659u64,
            ])), // 1081836006956609894549771334721413187913047383331561601606260283167615953295
            U256::new(BigInteger256::new([
                15896780023867373562u64,
                16431530531166173360u64,
                3706243266024269393u64,
                1821555101122443735u64,
            ])), // 11434086686358152335540554643130007307617078324975981257823476472104616196090
        ),
        false,
    );
    vk.gamma_abc_g1 = vec![Default::default(); 7];
    vk.gamma_abc_g1[0] = G1Affine::new(
        U256::new(BigInteger256::new([
            16612350195003011646u64,
            16694472726483573245u64,
            11181241841677491472u64,
            2584815261611629596u64,
        ])), // 16225148364316337376768119297456868908427925829817748684139175309620217098814
        U256::new(BigInteger256::new([
            8542989561729874066u64,
            6086688765331839688u64,
            3305414017189897822u64,
            823193395181110779u64,
        ])), // 5167268689450204162046084442581051565997733233062478317813755636162413164690
        false,
    );

    vk.gamma_abc_g1[1] = G1Affine::new(
        U256::new(BigInteger256::new([
            12204370672090286004u64,
            7025176770105815553u64,
            16861487272848831192u64,
            2052281193635792586u64,
        ])), // 12882377842072682264979317445365303375159828272423495088911985689463022094260
        U256::new(BigInteger256::new([
            5860392755383281266u64,
            3457281379082106305u64,
            14936960457651748240u64,
            3104651904365648200u64,
        ])), // 19488215856665173565526758360510125932214252767275816329232454875804474844786
        false,
    );

    vk.gamma_abc_g1[2] = G1Affine::new(
        U256::new(BigInteger256::new([
            2412237337469479481u64,
            7101257042553009156u64,
            6505378200991192527u64,
            2084320632868867203u64,
        ])), // 13083492661683431044045992285476184182144099829507350352128615182516530014777
        U256::new(BigInteger256::new([
            17359025414632902913u64,
            17467136144854214864u64,
            2629340672204543825u64,
            95912302711637706u64,
        ])), // 602051281796153692392523702676782023472744522032670801091617246498551238913
        false,
    );

    vk.gamma_abc_g1[3] = G1Affine::new(
        U256::new(BigInteger256::new([
            5890356199836945626u64,
            90976302902759043u64,
            8283983010322231996u64,
            1550471281565233100u64,
        ])), // 9732465972180335629969421513785602934706096902316483580882842789662669212890
        U256::new(BigInteger256::new([
            3323012601845369022u64,
            15483114678579076473u64,
            12294806027057697037u64,
            442326222459392621u64,
        ])), // 2776526698606888434074200384264824461688198384989521091253289776235602495678
        false,
    );

    vk.gamma_abc_g1[4] = G1Affine::new(
        U256::new(BigInteger256::new([
            6024717448228493914u64,
            3324201148237316350u64,
            8908918589169136773u64,
            1367886747179770165u64,
        ])), // 8586364274534577154894611080234048648883781955345622578531233113180532234842
        U256::new(BigInteger256::new([
            13405775906222472799u64,
            11351761182979731792u64,
            8921574734511693836u64,
            3389483845695942495u64,
        ])), // 21276134929883121123323359450658320820075698490666870487450985603988214349407
        false,
    );

    vk.gamma_abc_g1[5] = G1Affine::new(
        U256::new(BigInteger256::new([
            11934087455856126012u64,
            14386424791387963807u64,
            3468552646876333559u64,
            782308259477189136u64,
        ])), // 4910628533171597675018724709631788948355422829499855033965018665300386637884
        U256::new(BigInteger256::new([
            15466433915116038126u64,
            6356970036249743257u64,
            11543294905883257111u64,
            3271011010427608988u64,
        ])), // 20532468890024084510431799098097081600480376127870299142189696620752500664302
        false,
    );

    vk.gamma_abc_g1[6] = G1Affine::new(
        U256::new(BigInteger256::new([
            3780248646755399265u64,
            5034584203229263010u64,
            3189555373153285973u64,
            2443143149303319549u64,
        ])), // 15335858102289947642505450692012116222827233918185150176888641903531542034017
        U256::new(BigInteger256::new([
            15736264559489144332u64,
            10723219374054513722u64,
            16968158167652998003u64,
            846186232369622035u64,
        ])), // 5311597067667671581646709998171703828965875677637292315055030353779531404812
        false,
    );

    vk
}

fn pcs(ps: &[G1Affine], qs: &[G2Affine]) -> bool {
    let mut acc = Fq12::one();
    for (p,q) in ps.iter().zip(qs.iter()) {
        acc *= Bn254::pairing(*p, *q);
    }
    acc.is_one()
}

struct Proof {
    a: G1Affine,
    b: G2Affine,
    c: G1Affine,
}

fn verify_proof(proof: Proof, input: [U256; 6]) -> bool {
    let vk = verifying_key();
    // Compute the linear combination vk_x
    let mut vk_x = G1Affine::zero();
    for i in 0..6 {
        vk_x += &(vk.gamma_abc_g1[i + 1].mul(input[i])).into_affine();
    }
    vk_x += &vk.gamma_abc_g1[0];
    !pcs(&[-proof.a, vk.alpha_g1, vk_x, proof.c], &[proof.b, vk.beta_g2,vk.gamma_g2,vk.delta_g2])
}


#[cfg(test)]
mod tests {
    use ark_bn254::G1Projective;
    use ark_ff::{BigInteger, Fp2};
    use std::ops::MulAssign;

    use super::*;

    #[test]
    fn test_curve() {
        use ark_ff::FpParameters;
        let a = G1Affine::prime_subgroup_generator();
        let b = G2Affine::prime_subgroup_generator();

        assert_eq!(Bn254::pairing(a, b.mul(2)), Bn254::pairing(a, b).pow([2]));
        assert_eq!(Bn254::pairing(a, b.mul(2)), Bn254::pairing(a.mul(2), b));
        assert_eq!(Bn254::pairing(a, -b), Bn254::pairing(-a, b));
        assert!(pcs(&[a, -a], &[b, b]));

        // let a = G1Affine::new(U256::new(BigInteger256::new([1,0,0,0])), U256::new(BigInteger256::new([2,0,0,0])), false);
        // let b = G2Affine::prime_subgroup_generator();
        // new(
        //     Fp2::new(
        //         U256::new(BigInteger256::new([10945020018377822914u64, 17413811393473931026u64, 8241798111626485029u64, 1841571559660931130u64])),
        //         U256::new(BigInteger256::new([5106727233969649389u64, 7440829307424791261u64, 4785637993704342649u64, 1729627375292849782u64])),
        //     ),
        //     Fp2::new(
        //         U256::new(BigInteger256::new([6173549831154472795u64, 13567992399387660019u64, 17050234209342075797u64, 650358724130500725u64])),
        //         U256::new(BigInteger256::new([5541340697920699818u64, 16416156555105522555u64, 5380518976772849807u64, 1353435754470862315u64])),
        //     ),
        //     false
        // );
        // assert!(pairing_check(a, b));
    }
}

//     #[test]
//     fn test_verify_badkey() {
//         let mut vk = VerifyingKey::default();
//         vk.alpha_g1 = G1Affine::new(
//             U256::new(BigInteger256::new([1, 0, 0, 0])),
//             U256::new(BigInteger256::new([2, 0, 0, 0])),
//             false,
//         );
//         let pvk = prepare_verifying_key::<Bn254>(&vk);
//         let proof = Proof::default();
//         let result = verify_proof(&pvk, &proof, &[]);
//         dbg!(result);
//         assert!(result.is_err());
//     }
// }
