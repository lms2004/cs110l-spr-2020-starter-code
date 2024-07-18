// 实现为平铺向量的网格
pub struct Grid {
    num_rows: usize,
    num_cols: usize,
    elems: Vec<usize>,
}

impl Grid {
    /// 返回具有指定大小的网格，所有元素预先初始化为零。
    pub fn new(num_rows: usize, num_cols: usize) -> Grid {
        Grid {
            num_rows: num_rows,
            num_cols: num_cols,
            // 该语法使用 vec! 宏创建一个长度特定的零向量
            // https://stackoverflow.com/a/29530932
            elems: vec![0; num_rows * num_cols],
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.num_rows, self.num_cols)
    }

    /// 返回指定位置的元素。如果位置超出范围，则返回 None。
    ///
    /// 对学生的说明：这个函数也可以返回 Result。这是定义语义的一个品味问题；许多语言对超出范围的异常引发异常，
    /// 但其他人认为这使代码不必要地复杂。在这里，我们决定返回 Option 以便你更多地练习 Option :)，
    /// 并且因为这个类似的库返回 Option：
    /// https://docs.rs/array2d/0.2.1/array2d/struct.Array2D.html
    pub fn get(&self, row: usize, col: usize) -> Option<usize> {
        if row < self.num_rows && col < self.num_cols{
            Some(self.elems[row * self.num_cols + col])
        }
        else{
            None
        }
    }

    /// 将指定位置的元素设置为指定值。如果位置超出范围，则返回包含错误消息的 Err。
    pub fn set(&mut self, row: usize, col: usize, val: usize) -> Result<(), &'static str> {
        if row < self.num_rows && col < self.num_cols{
            self.elems[row * self.num_cols + col] = val;
            Ok(())
        }
        else{
            Err("Index out of bounds")
        }
    }

    /// 打印网格的可视化表示。你可以用它进行调试。
    pub fn display(&self) {
        for row in 0..self.num_rows {
            let mut line = String::new();
            for col in 0..self.num_cols {
                line.push_str(&format!("{}, ", self.get(row, col).unwrap()));
            }
            println!("{}", line);
        }
    }

    /// 将所有元素重置为零。
    pub fn clear(&mut self) {
        for i in self.elems.iter_mut() {
            *i = 0;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_grid() {
        let n_rows = 4;
        let n_cols = 3;
        let mut grid = Grid::new(n_rows, n_cols);

        // 初始化网格
        for r in 0..n_rows {
            for c in 0..n_cols {
                assert!(
                    grid.set(r, c, r * n_cols + c).is_ok(),
                    "Grid::set 返回 Err 即使提供的边界是有效的!"
                );
            }
        }

        // 注意：你需要运行 "cargo test  -- --nocapture" 以查看打印的输出
        println!("网格内容:");
        grid.display();

        // 确保值符合预期
        for r in 0..n_rows {
            for c in 0..n_cols {
                assert!(
                    grid.get(r, c).is_some(),
                    "Grid::get 返回 None 即使提供的边界是有效的!"
                );
                assert_eq!(grid.get(r, c).unwrap(), r * n_cols + c);
            }
        }
    }
}
