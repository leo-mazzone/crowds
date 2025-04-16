# crowds

> [!IMPORTANT]  
> This is currently a personal project for learning and development. Everything on this page is aspirational. The code will be incomplete and unstable until this notice disappears.

crowds provides a suite of anonymization algorithms written in Rust, allowing to transform dataframes so that they satisfy k-anonymity or differential privacy. 

## Optimal Lattice Anonymization
This is an implementation of the algorithm described by El Emam, Khalet, et al. (2009) [1]. Given a dataframe, an information loss function, and a set of generalization strategies, it returns a k-anonymous version [2], obtained using the single-dimensional global recording model, i.e.: the same values will be mapped consistently to the same generalizations in the new dataset, and the generalization for each dimension will not overlap.

## Examples
For information on how to use, check out the examples, which transform the "Adult" dataset from the UCI Machine Learning Repository [3].

## References

[1] El Emam, Khaled, et al. "A globally optimal k-anonymity method for the de-identification of health data." Journal of the American Medical Informatics Association 16.5 (2009): 670-682.

[2] Sweeney, Latanya. "k-anonymity: A model for protecting privacy." International Journal of Uncertainty, Fuzziness and Knowledge-Based Systems 10.05 (2002): 557-570.

[3] Dua, D. and Graff, C. "UCI Machine Learning Repository." Irvine, CA: University of California, School of Information and Computer Science (2019).