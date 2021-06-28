// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2021, stack-graphs authors.
// Licensed under either of Apache License, Version 2.0, or MIT license, at your option.
// Please see the LICENSE-APACHE or LICENSE-MIT files in this distribution for license details.
// ------------------------------------------------------------------------------------------------

use stack_graphs::arena::Handle;
use stack_graphs::graph::*;

use crate::test_graphs::CreateStackGraph;

/// A stack graph containing:
///
/// ``` python
/// # main.py
/// from a import *
/// print(foo)
/// ```
///
/// ``` python
/// # a.py
/// from b import *
/// ```
///
/// ``` python
/// # b.py
/// from a import *
/// foo = 1
/// ```
pub struct CyclicImportsPython {
    pub graph: StackGraph,
    // Interesting nodes in main.py
    pub main: Handle<Node>,
    pub main_a: Handle<Node>,
    pub main_foo: Handle<Node>,
    // Interesting nodes in a.py
    pub a: Handle<Node>,
    pub a_b: Handle<Node>,
    // Interesting nodes in b.py
    pub b: Handle<Node>,
    pub b_a: Handle<Node>,
    pub b_foo: Handle<Node>,
}

pub fn new() -> CyclicImportsPython {
    let mut graph = StackGraph::new();
    let root = graph.root_node();
    let sym_dot = graph.add_symbol(".");
    let sym_main = graph.add_symbol("__main__");
    let sym_a = graph.add_symbol("a");
    let sym_b = graph.add_symbol("b");
    let sym_foo = graph.add_symbol("foo");

    let main_file = graph.get_or_create_file("main.py");
    let main = graph.definition(main_file, 0, sym_main);
    let main_dot_1 = graph.pop_symbol(main_file, 1, sym_dot);
    let main_bottom_2 = graph.internal_scope(main_file, 2);
    let main_3 = graph.internal_scope(main_file, 3);
    let main_4 = graph.internal_scope(main_file, 4);
    let main_top_5 = graph.internal_scope(main_file, 5);
    let main_foo = graph.reference(main_file, 6, sym_foo);
    let main_dot_7 = graph.push_symbol(main_file, 7, sym_dot);
    let main_a = graph.reference(main_file, 8, sym_a);
    graph.edge(root, main);
    graph.edge(main, main_dot_1);
    graph.edge(main_dot_1, main_bottom_2);
    graph.edge(main_bottom_2, main_3);
    graph.edge(main_foo, main_3);
    graph.edge(main_3, main_4);
    graph.edge(main_4, main_dot_7);
    graph.edge(main_dot_7, main_a);
    graph.edge(main_a, root);
    graph.edge(main_4, main_top_5);

    let a_file = graph.get_or_create_file("a.py");
    let a = graph.definition(a_file, 0, sym_a);
    let a_dot_1 = graph.pop_symbol(a_file, 1, sym_dot);
    let a_bottom_2 = graph.internal_scope(a_file, 2);
    let a_3 = graph.internal_scope(a_file, 3);
    let a_top_4 = graph.internal_scope(a_file, 4);
    let a_dot_5 = graph.push_symbol(a_file, 5, sym_dot);
    let a_b = graph.reference(a_file, 6, sym_b);
    graph.edge(root, a);
    graph.edge(a, a_dot_1);
    graph.edge(a_dot_1, a_bottom_2);
    graph.edge(a_bottom_2, a_3);
    graph.edge(a_3, a_dot_5);
    graph.edge(a_dot_5, a_b);
    graph.edge(a_b, root);
    graph.edge(a_3, a_top_4);

    let b_file = graph.get_or_create_file("b.py");
    let b = graph.definition(b_file, 0, sym_b);
    let b_dot_1 = graph.pop_symbol(b_file, 1, sym_dot);
    let b_bottom_2 = graph.internal_scope(b_file, 2);
    let b_3 = graph.internal_scope(b_file, 3);
    let b_4 = graph.internal_scope(b_file, 4);
    let b_top_5 = graph.internal_scope(b_file, 5);
    let b_foo = graph.definition(b_file, 6, sym_foo);
    let b_dot_7 = graph.push_symbol(b_file, 7, sym_dot);
    let b_a = graph.reference(b_file, 8, sym_a);
    graph.edge(root, b);
    graph.edge(b, b_dot_1);
    graph.edge(b_dot_1, b_bottom_2);
    graph.edge(b_bottom_2, b_3);
    graph.edge(b_3, b_foo);
    graph.edge(b_3, b_4);
    graph.edge(b_4, b_dot_7);
    graph.edge(b_dot_7, b_a);
    graph.edge(b_a, root);
    graph.edge(b_4, b_top_5);

    CyclicImportsPython {
        graph,
        main,
        main_a,
        main_foo,
        a,
        a_b,
        b,
        b_a,
        b_foo,
    }
}