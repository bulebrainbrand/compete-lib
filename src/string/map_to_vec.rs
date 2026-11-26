pub trait MapToVec {
    fn map_to_vec<T, F: Fn(char) -> T>(&self, func: F) -> Vec<T>;
    fn map_to_bool_vec(&self) -> Option<Vec<bool>>;
}
impl MapToVec for String {
    /// 最悪計算量 `O(文字数 * コールバック関数の最悪計算量)`
    /// 文字列を渡された関数によって変換します。
    /// # Example
    /// ```rust
    /// # use crate::compete_lib::string::map_to_vec::MapToVec;
    /// assert_eq!(vec![1,2,3],"abc".to_string().map_to_vec(|x| match x {
    ///   'a' => 1,
    ///   'b' => 2,
    ///   'c' => 3,
    ///   _ => 4
    /// }))
    /// ```
    fn map_to_vec<T, F: Fn(char) -> T>(&self, func: F) -> Vec<T> {
        let mut vec = vec![];
        for char in self.chars() {
            vec.push(func(char));
        }
        vec
    }
    /// 最悪計算量 `O(文字数)`
    /// 文字列を
    /// `'x'` => `true`
    /// `'o'` => `false`
    /// で変換して、`Some(Vec<bool>)`にします。ただし、`'o'`でも`'x'`でもない場合、Noneを返します。
    /// # Example
    /// ```rust
    /// # use crate::compete_lib::string::map_to_vec::MapToVec;
    /// assert_eq!(Some(vec![true,false,true]), "xox".to_string().map_to_bool_vec());
    /// assert_eq!(None, "abc".to_string().map_to_bool_vec());
    /// ```
    fn map_to_bool_vec(&self) -> Option<Vec<bool>> {
        let mut vec = vec![];
        for char in self.chars() {
            match char {
                'x' => vec.push(true),
                'o' => vec.push(false),
                _ => {
                    return None;
                }
            }
        }
        Some(vec)
    }
}
impl MapToVec for &str {
    /// 最悪計算量 `O(文字数 * コールバック関数の最悪計算量)`
    /// 文字列を渡された関数によって変換します。
    /// # Example
    /// ```rust
    /// # use crate::compete_lib::string::map_to_vec::MapToVec;
    /// assert_eq!(vec![1,2,3],"abc".map_to_vec(|x| match x {
    ///   'a' => 1,
    ///   'b' => 2,
    ///   'c' => 3,
    ///   _ => 4
    /// }))
    /// ```
    fn map_to_vec<T, F: Fn(char) -> T>(&self, func: F) -> Vec<T> {
        let mut vec = vec![];
        for char in self.chars() {
            vec.push(func(char));
        }
        vec
    }
    /// 最悪計算量 `O(文字数)`
    /// 文字列を
    /// `'x'` => `true`
    /// `'o'` => `false`
    /// で変換して、`Some(Vec<bool>)`にします。ただし、`'o'`でも`'x'`でもない場合、Noneを返します。
    /// # Example
    /// ```rust
    /// # use crate::compete_lib::string::map_to_vec::MapToVec;
    /// assert_eq!(Some(vec![true,false,true]), "xox".map_to_bool_vec());
    /// assert_eq!(None, "abc".map_to_bool_vec());
    /// ```
    fn map_to_bool_vec(&self) -> Option<Vec<bool>> {
        let mut vec = vec![];
        for char in self.chars() {
            match char {
                'x' => vec.push(true),
                'o' => vec.push(false),
                _ => return None,
            }
        }
        Some(vec)
    }
}
pub trait MapNestStringToVec {
    fn map_to_vec<T, F: Fn(char) -> T>(&self, func: F) -> Vec<Vec<T>>;
    fn map_to_bool_vec(&self) -> Option<Vec<Vec<bool>>>;
}

impl<S: MapToVec> MapNestStringToVec for Vec<S> {
    /// 最悪計算量 `O(全体の文字数 * コールバック関数の最悪計算量)`
    /// 文字列を渡された関数によって変換します。
    /// # Example
    /// ```rust
    /// # use crate::compete_lib::string::map_to_vec::MapNestStringToVec;
    /// assert_eq!(vec![
    ///   vec![1,2,3],
    ///   vec![3,3,3],
    ///   vec![3,2,1]
    /// ],
    /// vec![
    ///   "abc",
    ///   "ccc",
    ///   "cba"
    /// ].map_to_vec(|x| match x {
    ///   'a' => 1,
    ///   'b' => 2,
    ///   'c' => 3,
    ///   _ => 4
    /// }))
    /// ```
    fn map_to_vec<T, F: Fn(char) -> T>(&self, func: F) -> Vec<Vec<T>> {
        let mut outer = vec![];
        for strs in self {
            outer.push(strs.map_to_vec(&func));
        }
        outer
    }
    /// 最悪計算量 `O(文字数)`
    /// 文字列を
    /// `'x'` => `true`
    /// `'o'` => `false`
    /// で変換して、`Some(Vec<bool>)`にします。ただし、`'o'`でも`'x'`でもない場合、Noneを返します。
    /// # Example
    /// ```rust
    /// # use crate::compete_lib::string::map_to_vec::MapNestStringToVec;
    /// assert_eq!(Some(vec![
    ///   vec![true,false,true],
    ///   vec![false,false,false],
    ///   vec![true,true,true]]
    /// ),
    /// vec![
    ///   "xox",
    ///   "ooo",
    ///   "xxx"
    /// ].map_to_bool_vec());
    ///
    /// assert_eq!(None, vec!["abc"].map_to_bool_vec());
    /// ```
    fn map_to_bool_vec(&self) -> Option<Vec<Vec<bool>>> {
        let mut outer = vec![];
        for strs in self {
            let inner = strs.map_to_bool_vec();
            if inner.is_none() {
                return None;
            }
            outer.push(inner.unwrap());
        }
        Some(outer)
    }
}
