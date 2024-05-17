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
use is_zero_subrange::*;
use AArch64_IASize::*;
use common::*;
pub fn AArch64_IPAIsOutOfRange<T: Tracer>(
    state: &mut State,
    tracer: &T,
    ipa: u64,
    walkparams: ProductTypeb05ce25a107f0c5e,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        iasize: i128,
        return_value: bool,
        ipa: u64,
        walkparams: ProductTypeb05ce25a107f0c5e,
    }
    let fn_state = FunctionState {
        ipa,
        walkparams,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var walkparams.29:struct
        let s_0_0: u8 = fn_state.walkparams._29;
        // D s_0_1: call AArch64_IASize(s_0_0)
        let s_0_1: i128 = AArch64_IASize(state, tracer, s_0_0);
        // D s_0_2: write-var iasize <= s_0_1
        fn_state.iasize = s_0_1;
        // C s_0_3: const #56s : i
        let s_0_3: i128 = 56;
        // D s_0_4: read-var iasize:i
        let s_0_4: i128 = fn_state.iasize;
        // D s_0_5: cmp-lt s_0_4 s_0_3
        let s_0_5: bool = ((s_0_4) < (s_0_3));
        // N s_0_6: branch s_0_5 b3 b1
        if s_0_5 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var return_value <= s_1_0
        fn_state.return_value = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var return_value:u8
        let s_2_0: bool = fn_state.return_value;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #55s : i
        let s_3_0: i128 = 55;
        // D s_3_1: read-var ipa:u56
        let s_3_1: u64 = fn_state.ipa;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 56u16);
        // D s_3_3: read-var iasize:i
        let s_3_3: i128 = fn_state.iasize;
        // D s_3_4: call is_zero_subrange(s_3_2, s_3_0, s_3_3)
        let s_3_4: bool = is_zero_subrange(state, tracer, s_3_2, s_3_0, s_3_3);
        // D s_3_5: not s_3_4
        let s_3_5: bool = !s_3_4;
        // D s_3_6: write-var return_value <= s_3_5
        fn_state.return_value = s_3_5;
        // N s_3_7: jump b2
        return block_2(state, tracer, fn_state);
    }
}