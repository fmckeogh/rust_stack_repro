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
pub fn Elem_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    vector_name__arg: Bits,
    e: i128,
    size: i128,
    value_name: Bits,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        gs_220: bool,
        sizeshadow_16: i128,
        vector_name: Bits,
        vector_name__arg: Bits,
        e: i128,
        size: i128,
        value_name: Bits,
    }
    let fn_state = FunctionState {
        vector_name__arg,
        e,
        size,
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_0_0: read-var size:i
        let s_0_0: i128 = fn_state.size;
        // D s_0_1: write-var sizeshadow#16 <= s_0_0
        fn_state.sizeshadow_16 = s_0_0;
        // D s_0_2: read-var vector_name__arg:bv
        let s_0_2: Bits = fn_state.vector_name__arg;
        // D s_0_3: write-var vector_name <= s_0_2
        fn_state.vector_name = s_0_2;
        // C s_0_4: const #0s : i
        let s_0_4: i128 = 0;
        // D s_0_5: read-var e:i
        let s_0_5: i128 = fn_state.e;
        // D s_0_6: cmp-ge s_0_5 s_0_4
        let s_0_6: bool = ((s_0_5) >= (s_0_4));
        // N s_0_7: branch s_0_6 b3 b1
        if s_0_6 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#220 <= s_1_0
        fn_state.gs_220 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_2_0: read-var gs#220:u8
        let s_2_0: bool = fn_state.gs_220;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #1s : i
        let s_2_2: i128 = 1;
        // D s_2_3: read-var e:i
        let s_2_3: i128 = fn_state.e;
        // D s_2_4: add s_2_3 s_2_2
        let s_2_4: i128 = (s_2_3 + s_2_2);
        // D s_2_5: read-var sizeshadow#16:i
        let s_2_5: i128 = fn_state.sizeshadow_16;
        // D s_2_6: mul s_2_4 s_2_5
        let s_2_6: i128 = ((s_2_4) * (s_2_5));
        // C s_2_7: const #1s : i
        let s_2_7: i128 = 1;
        // D s_2_8: sub s_2_6 s_2_7
        let s_2_8: i128 = ((s_2_6) - (s_2_7));
        // D s_2_9: read-var e:i
        let s_2_9: i128 = fn_state.e;
        // D s_2_10: read-var sizeshadow#16:i
        let s_2_10: i128 = fn_state.sizeshadow_16;
        // D s_2_11: mul s_2_9 s_2_10
        let s_2_11: i128 = ((s_2_9) * (s_2_10));
        // D s_2_12: read-var vector_name:bv
        let s_2_12: Bits = fn_state.vector_name;
        // D s_2_13: read-var value_name:bv
        let s_2_13: Bits = fn_state.value_name;
        // D s_2_14: sub s_2_8 s_2_11
        let s_2_14: i128 = ((s_2_8) - (s_2_11));
        // C s_2_15: const #1u : u64
        let s_2_15: u64 = 1;
        // C s_2_16: cast zx s_2_15 -> bv
        let s_2_16: Bits = Bits::new(s_2_15 as u128, 64u16);
        // D s_2_17: lsl s_2_16 s_2_14
        let s_2_17: Bits = s_2_16 << s_2_14;
        // D s_2_18: sub s_2_17 s_2_16
        let s_2_18: Bits = ((s_2_17) - (s_2_16));
        // D s_2_19: and s_2_13 s_2_18
        let s_2_19: Bits = ((s_2_13) & (s_2_18));
        // D s_2_20: lsl s_2_19 s_2_11
        let s_2_20: Bits = s_2_19 << s_2_11;
        // D s_2_21: lsl s_2_18 s_2_11
        let s_2_21: Bits = s_2_18 << s_2_11;
        // D s_2_22: cmpl s_2_21
        let s_2_22: Bits = !s_2_21;
        // D s_2_23: and s_2_12 s_2_22
        let s_2_23: Bits = ((s_2_12) & (s_2_22));
        // D s_2_24: or s_2_23 s_2_20
        let s_2_24: Bits = ((s_2_23) | (s_2_20));
        // D s_2_25: write-var vector_name <= s_2_24
        fn_state.vector_name = s_2_24;
        // D s_2_26: read-var vector_name:bv
        let s_2_26: Bits = fn_state.vector_name;
        // N s_2_27: return s_2_26
        return s_2_26;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_3_0: const #1s : i
        let s_3_0: i128 = 1;
        // D s_3_1: read-var e:i
        let s_3_1: i128 = fn_state.e;
        // D s_3_2: add s_3_1 s_3_0
        let s_3_2: i128 = (s_3_1 + s_3_0);
        // D s_3_3: read-var sizeshadow#16:i
        let s_3_3: i128 = fn_state.sizeshadow_16;
        // D s_3_4: mul s_3_2 s_3_3
        let s_3_4: i128 = ((s_3_2) * (s_3_3));
        // D s_3_5: read-var vector_name__arg:bv
        let s_3_5: Bits = fn_state.vector_name__arg;
        // D s_3_6: size-of s_3_5
        let s_3_6: u16 = s_3_5.length();
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: cmp-le s_3_4 s_3_7
        let s_3_8: bool = ((s_3_4) <= (s_3_7));
        // D s_3_9: write-var gs#220 <= s_3_8
        fn_state.gs_220 = s_3_8;
        // N s_3_10: jump b2
        return block_2(state, tracer, fn_state);
    }
}
