fn total(list: &[u32])->Option<u32>{
    let mut  x =    list.iter();
    x.try_fold(0u32,|acc,&x| acc.checked_add(x))


}
fn main() {
    let list1 = [1,2,3,4,5,4000000000,4000000000,888888];
    let list2 = [1,2,3,4,5,6,7];
    let k = total(&list1);
    let k1 = total(&list2);
    match k {
        Some(c)=>println!("{c}"),
        None =>{
            println!("溢出错误None")
        }
    }
    match k1 {
        Some(c)=>println!("和为{c}"),
        None =>{
            println!("None")
        }
    }
}