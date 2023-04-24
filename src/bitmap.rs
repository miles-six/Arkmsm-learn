#[derive(Debug)]
pub struct Bitmap {
    size: usize,   //位图大小
    data: Vec<u32>, //位图数据
}
//Bitmap 通常用于高效地存储和查询集合中的元素。
//例如，在处理大量的 URL 或 IP 地址时，可以使用 Bitmap 存储这些地址的集合。
//在查询时，可以将查询的地址转换为一个整数，然后检查 Bitmap 中对应的位是否为 1，以判断该地址是否在集合中。




impl Bitmap {
    //实例化Bitmap
    pub fn new(size: usize) -> Bitmap {
        let data = vec![0; size];
        Bitmap { size, data }
    }

    //Bucket是测试的和设置的位的索引
    pub fn test_and_set(&mut self, bucket: u32) -> bool {
        let word = bucket >> 5; //word32位。 得到在 self.data 中对应的word的索引  

        // data[0]表示的范围：0~31 data[1]表示的范围：32~63 
        println!("word is {}", word);
        let bit = 1 << (bucket & 0x1F); // bucket 与 0x1F 进行按位与操作，相当于将 bucket 对 32 取模。这样可以得到在 self.data[word] 字中对应的位的索引。
                                             // 然后将 1 左移这个位的索引位，得到一个二进制数，其中只有对应位是 1，其他位都是 0。
       
        println!("{}",self.data[word as usize]);
        if (self.data[word as usize] & bit) != 0 {
            return true;
        }
        self.data[word as usize] |= bit;
        return false;
    }

    pub fn clear(&mut self) {
        for i in 0..self.size {
            self.data[i] = 0;
        }
    }
}


#[test]
fn test_bitmap() {

    let mut bitmap = Bitmap::new(1);
    println!("{:?}",bitmap); 
    let a = bitmap.test_and_set(2);
    println!("{a}");
    println!("{:?}",bitmap); 

}


