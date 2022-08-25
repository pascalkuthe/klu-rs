use std::collections::HashSet;
use std::rc::Rc;

use float_cmp::{ApproxEq, F64Margin};
use num_complex::Complex64;
use proptest::prelude::{Strategy, TestCaseError};
use proptest::strategy::Just;
use proptest::{collection, prop_assert, proptest};

use crate::raw::KluData;
use crate::{FixedKluMatrix, KluMatrixBuilder, KluMatrixSpec, KluSettings};

proptest! {
    #[test]
    fn solve_real_linear_system(system in real_linear_system()){
        system.test_klu()?;
    }

    #[test]
    fn solve_complex_linear_system(system in complex_linear_system()){
        system.test_klu()?;
    }
}

#[test]
fn smoke_test() {
    LinearSystem {
        matrix_shape: vec![vec![0], vec![1]],
        matrix_data: vec![314.2, 5.1],
        matrix_len: 2,
        rhs: vec![314.2, 10.2],
    }
    .test_klu()
    .expect("smoke test failed")
}

fn real_number() -> impl Strategy<Value = f64> + Clone {
    let vals_pos = 1e-4..1e4;
    let vals_neg = -1e4..-1e-4;
    vals_neg.prop_union(vals_pos)
}

fn real_linear_system() -> impl Strategy<Value = LinearSystem<f64>> {
    linear_system(real_number())
}

fn complex_number() -> impl Strategy<Value = Complex64> + Clone {
    let re = real_number();
    let imag = real_number();
    let complex = [re, imag];
    complex.prop_map(|[re, imag]| Complex64::new(re, imag))
}

fn complex_linear_system() -> impl Strategy<Value = LinearSystem<Complex64>> {
    linear_system(complex_number())
}

fn linear_system<S: Strategy + Clone>(elem: S) -> impl Strategy<Value = LinearSystem<S::Value>> {
    (5i32..100i32)
        .prop_flat_map(move |size| {
            let matrix = matrix(elem.clone(), size);
            let rhs = vec![elem.clone(); size as usize];
            (rhs, matrix)
        })
        .prop_map(
            |(rhs, (matrix_shape, num_matricies, matrix_data))| LinearSystem {
                matrix_shape,
                matrix_data,
                matrix_len: num_matricies,
                rhs,
            },
        )
}

fn matrix<S: Strategy + Clone>(
    elem: S,
    size: i32,
) -> impl Strategy<Value = (Vec<Vec<i32>>, u32, Vec<S::Value>)> {
    let matrix_shape = matrix_shape(size);
    let num_matrix = 1usize..3usize;

    (matrix_shape, num_matrix).prop_flat_map(move |(shape, num_matrix)| {
        let data_len: usize = shape.iter().map(|row_entries| row_entries.len()).sum();
        (
            Just(shape),
            Just(data_len as u32),
            vec![elem.clone(); data_len * num_matrix],
        )
    })
}

fn matrix_shape(size: i32) -> impl Strategy<Value = Vec<Vec<i32>>> {
    let mut dst = Vec::new();
    for _row in 0..size {
        let col_entries = collection::vec(0i32..size, 2usize..(size as usize));
        let col_entries = col_entries.prop_filter_map("vector size", |mut col_entries| {
            let mut set = HashSet::new();
            col_entries.retain(|val| set.insert(*val));
            if col_entries.len() > 2 {
                Some(col_entries)
            } else {
                None
            }
        });
        dst.push(col_entries)
    }
    dst
}

#[derive(Debug)]
struct LinearSystem<D> {
    matrix_shape: Vec<Vec<i32>>,
    matrix_data: Vec<D>,
    matrix_len: u32,
    rhs: Vec<D>,
}

impl<D: KluData> LinearSystem<D> {
    fn gen_klu_spec(&self) -> Rc<KluMatrixSpec<i32>> {
        let dim = self.rhs.len() as i32;
        let mut builder = KluMatrixBuilder::new(dim as i32);
        self.for_matirx_entry(0, |col, row, _| builder.add_entry(col, row));
        builder.finish(KluSettings::new())
    }

    fn data(&self, matrix: u32) -> &[D] {
        let start = self.matrix_len * matrix;
        let end = self.matrix_len * (matrix + 1);
        &self.matrix_data[start as usize..end as usize]
    }

    fn for_matirx_entry(&self, matrix: u32, mut f: impl FnMut(i32, i32, D)) {
        let mut i = 0;
        let data = self.data(matrix);
        for (col, col_data) in self.matrix_shape.iter().enumerate() {
            for &row in col_data {
                f(col as i32, row, data[i]);
                i += 1;
            }
        }
    }

    fn test_klu(&self) -> Result<(), TestCaseError> {
        let spec = self.gen_klu_spec();
        let mut matrix = spec.create_matrix().expect("matrix is not empty");
        let num_matricies = self.matrix_data.len() as u32 / self.matrix_len;
        for i in 0..num_matricies {
            self.test_klu_solve(i as u32, &mut matrix)?
        }
        Ok(())
    }

    fn test_klu_solve(
        &self,
        matrix: u32,
        dst: &mut FixedKluMatrix<i32, D>,
    ) -> Result<(), TestCaseError> {
        self.for_matirx_entry(matrix, |col, row, val| {
            dst[(col, row)].set(val);
        });
        let is_singluar = dst.lu_factorize(Some(1e-12));
        if is_singluar {
            // singular matrix... assume this is correct
            return Ok(());
        }
        let mut solv = self.rhs.clone();
        dst.solve_linear_system(&mut solv);
        let mut check = vec![D::zero(); solv.len()];

        self.for_matirx_entry(matrix, |col, row, val| {
            check[row as usize] += val * solv[col as usize]
        });

        let margin = F64Margin::default().epsilon(20f64 * f64::EPSILON).ulps(8);
        for (refval, check) in self.rhs.iter().zip(check) {
            prop_assert!(
                f64::approx_eq(refval.re(), check.re(), margin)
                    || (refval.re() - check.re()).abs() / refval.re() <= 0.005,
                "{matrix}: {} != {}",
                refval.re(),
                check.re(),
            );

            prop_assert!(
                f64::approx_eq(refval.im(), check.im(), margin)
                    || (refval.im() - check.im()).abs() / refval.im() <= 0.005,
                "{matrix}: {} != {}",
                refval.im(),
                check.im()
            )
        }

        Ok(())
    }

    // fn test_klu(&self) -> Result<(), TestCaseError> {
    //     let matrix = spec.create_matrix::<f64>().expect("matrix can't be empty");

    //     let reference = self.rhs.clone();

    //     matrix.lu_factorize(None);
    //     let mut rhs: Vec<f64> = Vec::with_capacity(size as usize);
    //     for i in 0..size {
    //         rhs.push(rand::random());
    //         unsafe { *matrix[i as usize].get() = rand::random() };
    //     }
    //     let reference = rhs.clone();
    //     matrix.factorize().solve(&mut rhs);

    //     Ok(())
    // }

    // fn test_matrix<D: KluData>(
    //     matrix: &FixedKluMatrix<i32, D>,
    //     reference: &[D],
    //     rhs: &mut [D],
    // ) -> Result<(), TestCaseError> {
    //     reference.copy_from_slice(rhs);
    //     matrix.lu_factorize(Some(1e-12));
    //     matrix.solve_linear_system(rhs);
    //     for ((diagonal, solution), &val) in matrix.data().into_iter().zip(reference).zip(&*rhs) {
    //         let refval = diagonal.get() * val;
    //         prop_assert!(f64::approx_eq(
    //             refval.re(),
    //             solution.re(),
    //             F64Margin::default()
    //         ));

    //         prop_assert!(f64::approx_eq(
    //             refval.im(),
    //             solution.im(),
    //             F64Margin::default()
    //         ))
    //     }
    //     Ok(())
    // }
}
