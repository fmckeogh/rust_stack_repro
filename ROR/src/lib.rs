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
pub fn ROR<T: Tracer>(state: &mut State, tracer: &T, x: Bits, shift: i128) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        return_value: Bits,
        x: Bits,
        shift: i128,
    }
    let fn_state = FunctionState {
        x,
        shift,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_0_0: const #0s : i
        let s_0_0: i128 = 0;
        // D s_0_1: read-var shift:i
        let s_0_1: i128 = fn_state.shift;
        // D s_0_2: cmp-eq s_0_1 s_0_0
        let s_0_2: bool = ((s_0_1) == (s_0_0));
        // N s_0_3: branch s_0_2 b3 b1
        if s_0_2 {
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
        // D s_1_0: read-var x:bv
        let s_1_0: Bits = fn_state.x;
        // D s_1_1: size-of s_1_0
        let s_1_1: u16 = s_1_0.length();
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: read-var shift:i
        let s_1_3: i128 = fn_state.shift;
        // D s_1_4: mod s_1_3 s_1_2
        let s_1_4: i128 = ((s_1_3) % (s_1_2));
        // D s_1_5: read-var x:bv
        let s_1_5: Bits = fn_state.x;
        // D s_1_6: lsr s_1_5 s_1_4
        let s_1_6: Bits = s_1_5 >> s_1_4;
        // D s_1_7: read-var x:bv
        let s_1_7: Bits = fn_state.x;
        // D s_1_8: size-of s_1_7
        let s_1_8: u16 = s_1_7.length();
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: sub s_1_9 s_1_4
        let s_1_10: i128 = ((s_1_9) - (s_1_4));
        // D s_1_11: read-var x:bv
        let s_1_11: Bits = fn_state.x;
        // D s_1_12: lsl s_1_11 s_1_10
        let s_1_12: Bits = s_1_11 << s_1_10;
        // D s_1_13: or s_1_6 s_1_12
        let s_1_13: Bits = ((s_1_6) | (s_1_12));
        // D s_1_14: write-var return_value <= s_1_13
        fn_state.return_value = s_1_13;
        // N s_1_15: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_2_0: read-var return_value:bv
        let s_2_0: Bits = fn_state.return_value;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_3_0: read-var x:bv
        let s_3_0: Bits = fn_state.x;
        // D s_3_1: write-var return_value <= s_3_0
        fn_state.return_value = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
