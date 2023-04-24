use crate::{
    bucket_msm::BucketMSM,
    glv::{decompose},
    types::{G1BigInt, G1Projective, G1_SCALAR_SIZE_GLV},
};
//从不同的 crate 中导入一些模块和类型


use ark_bls12_381::{G1Affine, Fr};
//使用 use 关键字从 ark_bls12_381 crate 中引入了两种类型：

/*G1Affine，
用于表示 BLS12-381 pairing-based 加密方案中使用的椭圆曲线上的点，使用仿射坐标表示。
仿射坐标是一种表示椭圆曲线上点的方式，用两个有理数来表示一个点
在 BLS12-381 曲线上，一个 G1Affine 点可以被表示为 P = (x, y)，
其中 x 和 y 是有理数，满足 y^2 = x^3 + 4。这个方程定义了 BLS12-381 曲线上的所有点。*/

//Fr，用于表示 BLS12-381 曲线上的有限域中的元素
use ark_ff::BigInteger;
//引入了 BigInteger trait，它提供了处理任意精度整数的方法

pub struct MSMRun {
    pub points: Vec<G1Affine>,
    pub scalars: Vec<G1BigInt>,
    pub window_bits: u32,
    pub max_batch: u32,
    pub max_collisions: u32,
}

fn msm_slice(scalar: G1BigInt, slices: &mut Vec<u32>, window_bits: u32) {
    assert!(window_bits <= 31); // reserve one bit for marking signed slices
    let mut temp = scalar;
    println!("temp: {}", temp);
    for i in 0..slices.len() {
        println!("temp.as_ref()[0] is: {}", temp.as_ref()[0]);
        println!("temp.as_ref() is: {:?}", temp.as_ref());
        slices[i] = (temp.as_ref()[0] % (1 << window_bits)) as u32;
        //as.ref()定义了一种将类型转换为引用的方式，可以在不改变类型的情况下，将一个类型作为另一个类型的参数或者成员变量传递。
        //比如对于字符串类型s， r = s.as_ref() 返回一个指向字符串的引用r;
        println!("slices{}", slices[i]);
         //将 temp 右移 window_bits 位，相当于将二进制表示中的第一个块去掉，进而将下一个块变成了第一个块。
        temp.divn(window_bits);
       //将scalar二进制表示 然后按窗口大小分解成多个部分，每一部分放在slices[i]中存着。

    }
    

    let mut carry = 0;
    let total = 1 << window_bits; //桶的数量
    let half = total >> 1; //桶数量的一半

    //为了之后使用signed-bucket做准备。
    //c=3 7个桶 -- signed bucket--> 4个桶   桶的index对应slice[i]
    //假设slice[i]=7 7>4  所以7变为-1   即将{0...2^c-1}集合变为{-2^(c-1)...2^(c-1)}集合
    for i in 0..slices.len() {
        // println!("slices,{}", slices[i]);
        // println!("carry,{}", carry);
        slices[i] += carry;
        if slices[i] > half {
            // slices[i] == half is okay, since (slice[i]-1) will be used for bucket_id
            slices[i] = total - slices[i];
            carry = 1;
            slices[i] |= 1 << 31; // mark the highest bit for later
        } else {
            carry = 0;
        }
    }

    assert!(
        carry == 0,
        "msm_slice overflows when apply signed-bucket-index"
    );
}

pub fn quick_msm(run: &MSMRun) -> G1Projective {
    let mut bucket_msm: BucketMSM<G1Affine> = BucketMSM::new(
        G1_SCALAR_SIZE_GLV,
        run.window_bits,
        run.max_batch,
        run.max_collisions,
    );
    let num_slices: u32 = (G1_SCALAR_SIZE_GLV + run.window_bits - 1) / run.window_bits;
    // scalar = phi * lambda + normal
    let mut phi_slices: Vec<u32> = vec![0; num_slices as usize];
    let mut normal_slices: Vec<u32> = vec![0; num_slices as usize];

    let scalars_and_bases_iter = run
        .scalars.iter() //scalars的迭代器 
        .zip(&run.points) //zip方法将scalars 与 points 组合在一起，每一个元素都是元组，包含scalar与points相应位置的元素
        .filter(|(s, _)| !s.is_zero()); //将系数Scalar为0的项过滤掉   
    //msm：求sum(a_i*G_i) 即 sum(scalar_i * points_i) 

    

    scalars_and_bases_iter.for_each(|(&scalar, point)| {
        //利用glv计算scalar * points   将scalar decompose成 scalar = phi + λ*nomal  
        let (phi, normal, is_neg_scalar, is_neg_normal) = decompose(&Fr::from(scalar), run.window_bits);
        msm_slice(phi.into(), &mut phi_slices, run.window_bits);
        msm_slice(normal.into(), &mut normal_slices, run.window_bits);
        bucket_msm.process_point_and_slices_glv(&point, &normal_slices, &phi_slices, is_neg_scalar, is_neg_normal);
    });

    bucket_msm.process_complete();
    return bucket_msm.batch_reduce();
}

#[cfg(test)]
mod collision_method_pippenger_tests {
    use super::*;

    #[test]
    fn test_msm_slice_window_size_1() {
        let scalar = G1BigInt::from(0b101);
        println!("scalar: {}", scalar);
        let mut slices: Vec<u32> = vec![0; 3];
        println!("slices:{:?}",slices);
        msm_slice(scalar, &mut slices, 1);
        print!("slices {:?}\n", slices);
        assert_eq!(slices.iter().eq([1, 0, 1].iter()), true);
    }
    #[test]
    fn test_msm_slice_window_size_2() {
        let scalar = G1BigInt::from(0b000110);
        let mut slices: Vec<u32> = vec![0; 3];
        msm_slice(scalar, &mut slices, 2);
        assert_eq!(slices.iter().eq([2, 1, 0].iter()), true);
    }

    #[test]

    fn test_msm_slice_window_size_3() {
        let scalar = G1BigInt::from(0b010111000);
        let mut slices: Vec<u32> = vec![0; 3];
        msm_slice(scalar, &mut slices, 3);
        assert_eq!(slices.iter().eq([0, 0x80000001, 3].iter()), true);
        /* scalar二进制表示为 010111000 取分组大小为3  则为(010 111 000) 高位->低位 
           则slice[0]=0  slice[1]=7 slice[2]=2  */
        
    }

    #[test]
    fn test_msm_slice_window_size_16() {
        let scalar = G1BigInt::from(0x123400007FFF);
        let mut slices: Vec<u32> = vec![0; 3];
        msm_slice(scalar, &mut slices, 16);
        assert_eq!(slices.iter().eq([0x7FFF, 0, 0x1234].iter()), true);
    }
}
