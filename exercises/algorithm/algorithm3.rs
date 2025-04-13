/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM DONE

fn sort<T: Ord>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }
    quick_sort(array, 0, array.len() - 1);
}

fn quick_sort<T: Ord>(array: &mut [T], low: usize, high: usize) {
    if low < high {
        let pivot = partition(array, low, high);

        // 如果 pivot > 0，才对左子数组进行排序
        if pivot > 0 {
            quick_sort(array, low, pivot - 1);
        }

        // 对右子数组进行排序
        quick_sort(array, pivot + 1, high);
    }
}

fn partition<T: Ord>(array: &mut [T], low: usize, high: usize) -> usize {
    let pivot = high;
    let mut i = low;

    // 将所有小于等于 pivot 的元素移到左侧
    for j in low..high {
        if array[j] <= array[pivot] {
            array.swap(i, j);
            i += 1;
        }
    }

    // 将 pivot 放到正确的位置
    array.swap(i, pivot);
    i
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}