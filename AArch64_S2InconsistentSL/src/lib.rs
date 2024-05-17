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
use AArch64_IASize::*;
use TGxGranuleBits::*;
use AArch64_S2StartLevel::*;
use common::*;
pub fn AArch64_S2InconsistentSL<T: Tracer>(
    state: &mut State,
    tracer: &T,
    walkparams: ProductTypeb05ce25a107f0c5e,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_18951: bool,
        sl_max_iasize: i128,
        iasize: i128,
        walkparams: ProductTypeb05ce25a107f0c5e,
    }
    let fn_state = FunctionState {
        walkparams,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var walkparams:struct
        let s_0_0: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_0_1: call AArch64_S2StartLevel(s_0_0)
        let s_0_1: i128 = AArch64_S2StartLevel(state, tracer, s_0_0);
        // C s_0_2: const #800u : u32
        let s_0_2: u32 = 800;
        // D s_0_3: read-reg s_0_2:i64
        let s_0_3: i64 = {
            let value = state.read_register::<i64>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // D s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // D s_0_5: sub s_0_4 s_0_1
        let s_0_5: i128 = ((s_0_4) - (s_0_1));
        // D s_0_6: read-var walkparams.26:struct
        let s_0_6: u32 = fn_state.walkparams._26;
        // D s_0_7: call TGxGranuleBits(s_0_6)
        let s_0_7: i128 = TGxGranuleBits(state, tracer, s_0_6);
        // C s_0_8: const #3s : i64
        let s_0_8: i64 = 3;
        // C s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_10: sub s_0_7 s_0_9
        let s_0_10: i128 = ((s_0_7) - (s_0_9));
        // D s_0_11: mul s_0_5 s_0_10
        let s_0_11: i128 = ((s_0_5) * (s_0_10));
        // D s_0_12: add s_0_11 s_0_7
        let s_0_12: i128 = (s_0_11 + s_0_7);
        // C s_0_13: const #1s : i
        let s_0_13: i128 = 1;
        // D s_0_14: add s_0_12 s_0_13
        let s_0_14: i128 = (s_0_12 + s_0_13);
        // C s_0_15: const #1s : i
        let s_0_15: i128 = 1;
        // D s_0_16: sub s_0_10 s_0_15
        let s_0_16: i128 = ((s_0_10) - (s_0_15));
        // D s_0_17: add s_0_14 s_0_16
        let s_0_17: i128 = (s_0_14 + s_0_16);
        // C s_0_18: const #4s : i
        let s_0_18: i128 = 4;
        // D s_0_19: add s_0_17 s_0_18
        let s_0_19: i128 = (s_0_17 + s_0_18);
        // D s_0_20: write-var sl_max_iasize <= s_0_19
        fn_state.sl_max_iasize = s_0_19;
        // D s_0_21: read-var walkparams.29:struct
        let s_0_21: u8 = fn_state.walkparams._29;
        // D s_0_22: call AArch64_IASize(s_0_21)
        let s_0_22: i128 = AArch64_IASize(state, tracer, s_0_21);
        // D s_0_23: write-var iasize <= s_0_22
        fn_state.iasize = s_0_22;
        // D s_0_24: read-var iasize:i
        let s_0_24: i128 = fn_state.iasize;
        // D s_0_25: cmp-lt s_0_24 s_0_14
        let s_0_25: bool = ((s_0_24) < (s_0_14));
        // N s_0_26: branch s_0_25 b3 b1
        if s_0_25 {
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
        // D s_1_0: read-var iasize:i
        let s_1_0: i128 = fn_state.iasize;
        // D s_1_1: read-var sl_max_iasize:i
        let s_1_1: i128 = fn_state.sl_max_iasize;
        // D s_1_2: cmp-gt s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) > (s_1_1));
        // D s_1_3: write-var gs#18951 <= s_1_2
        fn_state.gs_18951 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#18951:u8
        let s_2_0: bool = fn_state.gs_18951;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // D s_3_1: write-var gs#18951 <= s_3_0
        fn_state.gs_18951 = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
