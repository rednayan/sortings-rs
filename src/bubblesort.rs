use crate::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(slice: &mut [T])
        where
            T:Ord 
    {       
      let mut swapped = true;
      while swapped {
        swapped = false;
        for i in 0..slice.len() -1{
            if slice[i] > slice[i+1] {
                slice.swap(i,i+1);
                swapped = true;
            }
        }
      }
    }   
}

#[test]
fn bubblesort_test() {
    let mut things = vec![4,3,5,2,1];
    super::sort::<_,BubbleSort>(&mut things);
    assert_eq!(things,&[1,2,3,4,5]);
}