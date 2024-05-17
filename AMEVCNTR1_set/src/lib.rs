#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use common::*;
pub fn AMEVCNTR1_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i64,
    value_name: ProductType5c790c8ef59cc8b2,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        n: i64,
        value_name: ProductType5c790c8ef59cc8b2,
    }
    let fn_state = FunctionState {
        n,
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType5c790c8ef59cc8b2 = fn_state.value_name;
        // C s_0_1: const #103240u : u32
        let s_0_1: u32 = 103240;
        // D s_0_2: read-reg s_0_1:[struct; 16]
        let s_0_2: [ProductType5c790c8ef59cc8b2; 16usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 16usize]>(s_0_1 as isize);
            tracer.read_register(s_0_1 as isize, value);
            value
        };
        // D s_0_3: read-var n:i64
        let s_0_3: i64 = fn_state.n;
        // D s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // D s_0_5: read-element s_0_2[s_0_4]
        let s_0_5: ProductType5c790c8ef59cc8b2 = s_0_2[(s_0_4) as usize];
        // C s_0_6: const #103240u : u32
        let s_0_6: u32 = 103240;
        // D s_0_7: read-reg s_0_6:[struct; 16]
        let s_0_7: [ProductType5c790c8ef59cc8b2; 16usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 16usize]>(s_0_6 as isize);
            tracer.read_register(s_0_6 as isize, value);
            value
        };
        // D s_0_8: read-var n:i64
        let s_0_8: i64 = fn_state.n;
        // D s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_10: mutate-element s_0_7[s_0_9] <= s_0_5
        let s_0_10: [ProductType5c790c8ef59cc8b2; 16usize] = {
            let mut local = s_0_7.clone();
            local[(s_0_9) as usize] = s_0_5;
            local
        };
        // D s_0_11: cast cvt s_0_10 -> [struct; 0]
        let s_0_11: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_0_10,
        );
        // D s_0_12: cast cvt s_0_11 -> [struct; 16]
        let s_0_12: [ProductType5c790c8ef59cc8b2; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_0_11);
            buf
        };
        // C s_0_13: const #103240u : u32
        let s_0_13: u32 = 103240;
        // N s_0_14: write-reg s_0_13 <= s_0_12
        let s_0_14: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 16usize],
                >(s_0_13 as isize, s_0_12);
            tracer.write_register(s_0_13 as isize, s_0_12);
        };
        // C s_0_15: const #15080u : u32
        let s_0_15: u32 = 15080;
        // D s_0_16: read-reg s_0_15:[struct; 16]
        let s_0_16: [ProductType5c790c8ef59cc8b2; 16usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 16usize],
                >(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: read-var n:i64
        let s_0_17: i64 = fn_state.n;
        // D s_0_18: cast zx s_0_17 -> i
        let s_0_18: i128 = (i128::try_from(s_0_17).unwrap());
        // D s_0_19: mutate-element s_0_16[s_0_18] <= s_0_0
        let s_0_19: [ProductType5c790c8ef59cc8b2; 16usize] = {
            let mut local = s_0_16.clone();
            local[(s_0_18) as usize] = s_0_0;
            local
        };
        // D s_0_20: cast cvt s_0_19 -> [struct; 0]
        let s_0_20: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_0_19,
        );
        // D s_0_21: cast cvt s_0_20 -> [struct; 16]
        let s_0_21: [ProductType5c790c8ef59cc8b2; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_0_20);
            buf
        };
        // C s_0_22: const #15080u : u32
        let s_0_22: u32 = 15080;
        // N s_0_23: write-reg s_0_22 <= s_0_21
        let s_0_23: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 16usize],
                >(s_0_22 as isize, s_0_21);
            tracer.write_register(s_0_22 as isize, s_0_21);
        };
        // N s_0_24: return
        return;
    }
}
