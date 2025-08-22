pub fn binary_search<T: PartialEq + PartialOrd>(list: &[T], el: &T) -> Option<usize> {
    let mut low= 0;
    let mut hight = list.len() - 1;

    while low <= hight {
        
        let mid = (low + hight) / 2;
        let guess = list.get(mid).unwrap();
        dbg!(mid);
        if guess == el {
            return Some(mid);
        }
        if guess > el {
            if mid == 0{
                return None;
            }
            hight = mid - 1;
        }
        if guess < el {
            low = mid +1 ;
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use crate::binsearch::binary_search;

    const LIST_INT: [i32; 8] = [2,3,4,5,6,7,8,9];
    const LIST_NAME: [&'static str; 20] = [
        "Алексей", "Алина", "Андрей", "Валентина", "Виктор", 
        "Галина", "Георгий", "Дарья", "Евгений", "Ирина", 
        "Кирилл", "Лариса", "Михаил", "Наталья", "Олег", 
        "Павел", "Роман", "Светлана", "Татьяна", "Юлия"
    ];

    #[test]
    fn fined_int_none(){
        let ans = binary_search(&LIST_INT, &1);
        assert!(ans.is_none());
    }

    #[test]
    fn fined_name_none(){
        let ans = binary_search(&LIST_NAME, &"Ярослав");
        assert!(ans.is_none());
    }
    
    #[test]
    fn fined_int(){
        let ans = binary_search(&LIST_INT, &8).unwrap();
        assert_eq!(ans, 6);
    }

    #[test]
    fn fined_name(){
        let ans = binary_search(&LIST_NAME, &"Алексей").unwrap();
        assert_eq!(ans, 0);
    }

}
