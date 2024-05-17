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
use NVMem_set__1::*;
use common::*;
pub fn NVMem_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    offset: i128,
    value_name: u64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        offset: i128,
        value_name: u64,
    }
    let fn_state = FunctionState {
        offset,
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0s : i
        let s_0_0: i128 = 0;
        // D s_0_1: read-var offset:i
        let s_0_1: i128 = fn_state.offset;
        // D s_0_2: cmp-gt s_0_1 s_0_0
        let s_0_2: bool = ((s_0_1) > (s_0_0));
        // N s_0_3: assert s_0_2
        let s_0_3: () = assert!(s_0_2);
        // C s_0_4: const #64s : i64
        let s_0_4: i64 = 64;
        // C s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // C s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: read-var value_name:u64
        let s_0_7: u64 = fn_state.value_name;
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 64u16);
        // D s_0_9: read-var offset:i
        let s_0_9: i128 = fn_state.offset;
        // D s_0_10: call NVMem_set__1(s_0_9, s_0_6, s_0_8)
        let s_0_10: () = NVMem_set__1(state, tracer, s_0_9, s_0_6, s_0_8);
        // N s_0_11: return
        return;
    }
}
