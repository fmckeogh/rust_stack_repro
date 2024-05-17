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
pub fn replicate_bits_borealis_internal<T: Tracer>(
    state: &mut State,
    tracer: &T,
    bits: Bits,
    count: u64,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        local_count: u64,
        result: Bits,
        bits: Bits,
        count: u64,
    }
    let fn_state = FunctionState {
        bits,
        count,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_0_0: read-var count:u64
        let s_0_0: u64 = fn_state.count;
        // D s_0_1: write-var local_count <= s_0_0
        fn_state.local_count = s_0_0;
        // C s_0_2: const #0u : u128
        let s_0_2: u128 = 0;
        // D s_0_3: read-var bits:bv
        let s_0_3: Bits = fn_state.bits;
        // D s_0_4: size-of s_0_3
        let s_0_4: u16 = s_0_3.length();
        // D s_0_5: cast trunc s_0_0 -> u16
        let s_0_5: u16 = ((s_0_0 as u16) & 65535);
        // D s_0_6: cast trunc s_0_4 -> u16
        let s_0_6: u16 = ((s_0_4));
        // D s_0_7: mul s_0_5 s_0_6
        let s_0_7: u16 = ((s_0_5) * (s_0_6));
        // D s_0_8: create-bits s_0_2 s_0_7
        let s_0_8: Bits = Bits::new(s_0_2, s_0_7);
        // D s_0_9: write-var result <= s_0_8
        fn_state.result = s_0_8;
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_1_0: const #0u : u64
        let s_1_0: u64 = 0;
        // D s_1_1: read-var local_count:u64
        let s_1_1: u64 = fn_state.local_count;
        // D s_1_2: cmp-eq s_1_1 s_1_0
        let s_1_2: bool = ((s_1_1) == (s_1_0));
        // N s_1_3: branch s_1_2 b3 b2
        if s_1_2 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_2_0: read-var local_count:u64
        let s_2_0: u64 = fn_state.local_count;
        // C s_2_1: const #1u : u64
        let s_2_1: u64 = 1;
        // D s_2_2: sub s_2_0 s_2_1
        let s_2_2: u64 = ((s_2_0) - (s_2_1));
        // D s_2_3: write-var local_count <= s_2_2
        fn_state.local_count = s_2_2;
        // D s_2_4: read-var result:bv
        let s_2_4: Bits = fn_state.result;
        // D s_2_5: read-var bits:bv
        let s_2_5: Bits = fn_state.bits;
        // D s_2_6: size-of s_2_5
        let s_2_6: u16 = s_2_5.length();
        // D s_2_7: cast cvt s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 16u16);
        // D s_2_8: lsl s_2_4 s_2_7
        let s_2_8: Bits = s_2_4 << s_2_7;
        // D s_2_9: or s_2_8 s_2_5
        let s_2_9: Bits = ((s_2_8) | (s_2_5));
        // D s_2_10: write-var result <= s_2_9
        fn_state.result = s_2_9;
        // N s_2_11: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_3_0: read-var result:bv
        let s_3_0: Bits = fn_state.result;
        // N s_3_1: return s_3_0
        return s_3_0;
    }
}
