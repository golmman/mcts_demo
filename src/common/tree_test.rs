use crate::common::tree::Tree;

#[derive(Default, Debug)]
struct TestStruct {
    a: i32,
    b: i32,
}

#[test]
fn it_zzz() {
    println!("zzz");

    let mut tree = Tree::<TestStruct>::new();

    let root_index = tree.root_index();

    tree.add_children(
        root_index,
        vec![
            TestStruct::default(),
            TestStruct::default(),
            TestStruct::default(),
            TestStruct::default(),
            TestStruct::default(),
        ],
    );

    tree.add_children(3, vec![TestStruct::default(), TestStruct::default()]);

    tree.add_children(
        7,
        vec![
            TestStruct::default(),
            TestStruct::default(),
            TestStruct::default(),
        ],
    );

    for a in tree.children(root_index) {
        println!("{}", a);

        if a == 5 {
            tree.set_data(3, TestStruct { a: 1, b: 2 });
        }

        if a == 3 {
            tree.set_data(3, TestStruct { a: 1, b: 2 });
        }
    }

    println!("{:?}", tree);
}
