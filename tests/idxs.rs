use purpur::find_idxs;


#[test]
fn test_find_idxs() {
    let x = vec![1, 2, 6, 3, 1, 8, 3,
                         4, 4, 9, 3, 6, 3, 8,
                         7, 8, 4, 21, 3, 2, 1,  
    ];
    let max_x = vec![8,
                              9,
                              21,
    ];
    let rows = 3;
    let u = find_idxs(rows, &x, &max_x);
    assert_eq!(vec![5, 2, 3], u);
    
}