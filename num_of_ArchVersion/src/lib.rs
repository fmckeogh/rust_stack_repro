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
pub fn num_of_ArchVersion<T: Tracer>(state: &mut State, tracer: &T, arg_: u32) -> i64 {
    #[derive(Default)]
    struct FunctionState {
        gs_746: i64,
        arg_: u32,
    }
    let fn_state = FunctionState {
        arg_,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_0_0: const #0u : u32
        let s_0_0: u32 = 0;
        // D s_0_1: read-var arg#:u32
        let s_0_1: u32 = fn_state.arg_;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // D s_0_3: not s_0_2
        let s_0_3: bool = !s_0_2;
        // N s_0_4: branch s_0_3 b3 b1
        if s_0_3 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_1_0: const #0s : i64
        let s_1_0: i64 = 0;
        // D s_1_1: write-var gs#746 <= s_1_0
        fn_state.gs_746 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // D s_2_0: read-var gs#746:i64
        let s_2_0: i64 = fn_state.gs_746;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_3_0: const #1u : u32
        let s_3_0: u32 = 1;
        // D s_3_1: read-var arg#:u32
        let s_3_1: u32 = fn_state.arg_;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: not s_3_2
        let s_3_3: bool = !s_3_2;
        // N s_3_4: branch s_3_3 b5 b4
        if s_3_3 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_4_0: const #1s : i64
        let s_4_0: i64 = 1;
        // D s_4_1: write-var gs#746 <= s_4_0
        fn_state.gs_746 = s_4_0;
        // N s_4_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_5_0: const #2u : u32
        let s_5_0: u32 = 2;
        // D s_5_1: read-var arg#:u32
        let s_5_1: u32 = fn_state.arg_;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // D s_5_3: not s_5_2
        let s_5_3: bool = !s_5_2;
        // N s_5_4: branch s_5_3 b7 b6
        if s_5_3 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_6_0: const #2s : i64
        let s_6_0: i64 = 2;
        // D s_6_1: write-var gs#746 <= s_6_0
        fn_state.gs_746 = s_6_0;
        // N s_6_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_7_0: const #3u : u32
        let s_7_0: u32 = 3;
        // D s_7_1: read-var arg#:u32
        let s_7_1: u32 = fn_state.arg_;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // D s_7_3: not s_7_2
        let s_7_3: bool = !s_7_2;
        // N s_7_4: branch s_7_3 b9 b8
        if s_7_3 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_8_0: const #3s : i64
        let s_8_0: i64 = 3;
        // D s_8_1: write-var gs#746 <= s_8_0
        fn_state.gs_746 = s_8_0;
        // N s_8_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_9_0: const #4u : u32
        let s_9_0: u32 = 4;
        // D s_9_1: read-var arg#:u32
        let s_9_1: u32 = fn_state.arg_;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // D s_9_3: not s_9_2
        let s_9_3: bool = !s_9_2;
        // N s_9_4: branch s_9_3 b11 b10
        if s_9_3 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_10_0: const #4s : i64
        let s_10_0: i64 = 4;
        // D s_10_1: write-var gs#746 <= s_10_0
        fn_state.gs_746 = s_10_0;
        // N s_10_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_11_0: const #5u : u32
        let s_11_0: u32 = 5;
        // D s_11_1: read-var arg#:u32
        let s_11_1: u32 = fn_state.arg_;
        // D s_11_2: cmp-eq s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) == (s_11_1));
        // D s_11_3: not s_11_2
        let s_11_3: bool = !s_11_2;
        // N s_11_4: branch s_11_3 b13 b12
        if s_11_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_12_0: const #5s : i64
        let s_12_0: i64 = 5;
        // D s_12_1: write-var gs#746 <= s_12_0
        fn_state.gs_746 = s_12_0;
        // N s_12_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_13_0: const #6u : u32
        let s_13_0: u32 = 6;
        // D s_13_1: read-var arg#:u32
        let s_13_1: u32 = fn_state.arg_;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // D s_13_3: not s_13_2
        let s_13_3: bool = !s_13_2;
        // N s_13_4: branch s_13_3 b15 b14
        if s_13_3 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_14_0: const #6s : i64
        let s_14_0: i64 = 6;
        // D s_14_1: write-var gs#746 <= s_14_0
        fn_state.gs_746 = s_14_0;
        // N s_14_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_15_0: const #7u : u32
        let s_15_0: u32 = 7;
        // D s_15_1: read-var arg#:u32
        let s_15_1: u32 = fn_state.arg_;
        // D s_15_2: cmp-eq s_15_0 s_15_1
        let s_15_2: bool = ((s_15_0) == (s_15_1));
        // D s_15_3: not s_15_2
        let s_15_3: bool = !s_15_2;
        // N s_15_4: branch s_15_3 b17 b16
        if s_15_3 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_16_0: const #7s : i64
        let s_16_0: i64 = 7;
        // D s_16_1: write-var gs#746 <= s_16_0
        fn_state.gs_746 = s_16_0;
        // N s_16_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_17_0: const #8u : u32
        let s_17_0: u32 = 8;
        // D s_17_1: read-var arg#:u32
        let s_17_1: u32 = fn_state.arg_;
        // D s_17_2: cmp-eq s_17_0 s_17_1
        let s_17_2: bool = ((s_17_0) == (s_17_1));
        // D s_17_3: not s_17_2
        let s_17_3: bool = !s_17_2;
        // N s_17_4: branch s_17_3 b19 b18
        if s_17_3 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_18_0: const #8s : i64
        let s_18_0: i64 = 8;
        // D s_18_1: write-var gs#746 <= s_18_0
        fn_state.gs_746 = s_18_0;
        // N s_18_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_19_0: const #9u : u32
        let s_19_0: u32 = 9;
        // D s_19_1: read-var arg#:u32
        let s_19_1: u32 = fn_state.arg_;
        // D s_19_2: cmp-eq s_19_0 s_19_1
        let s_19_2: bool = ((s_19_0) == (s_19_1));
        // D s_19_3: not s_19_2
        let s_19_3: bool = !s_19_2;
        // N s_19_4: branch s_19_3 b21 b20
        if s_19_3 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_20_0: const #9s : i64
        let s_20_0: i64 = 9;
        // D s_20_1: write-var gs#746 <= s_20_0
        fn_state.gs_746 = s_20_0;
        // N s_20_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_21_0: const #10u : u32
        let s_21_0: u32 = 10;
        // D s_21_1: read-var arg#:u32
        let s_21_1: u32 = fn_state.arg_;
        // D s_21_2: cmp-eq s_21_0 s_21_1
        let s_21_2: bool = ((s_21_0) == (s_21_1));
        // D s_21_3: not s_21_2
        let s_21_3: bool = !s_21_2;
        // N s_21_4: branch s_21_3 b23 b22
        if s_21_3 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_22_0: const #10s : i64
        let s_22_0: i64 = 10;
        // D s_22_1: write-var gs#746 <= s_22_0
        fn_state.gs_746 = s_22_0;
        // N s_22_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_23_0: const #11u : u32
        let s_23_0: u32 = 11;
        // D s_23_1: read-var arg#:u32
        let s_23_1: u32 = fn_state.arg_;
        // D s_23_2: cmp-eq s_23_0 s_23_1
        let s_23_2: bool = ((s_23_0) == (s_23_1));
        // D s_23_3: not s_23_2
        let s_23_3: bool = !s_23_2;
        // N s_23_4: branch s_23_3 b25 b24
        if s_23_3 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_24_0: const #11s : i64
        let s_24_0: i64 = 11;
        // D s_24_1: write-var gs#746 <= s_24_0
        fn_state.gs_746 = s_24_0;
        // N s_24_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_25_0: const #12u : u32
        let s_25_0: u32 = 12;
        // D s_25_1: read-var arg#:u32
        let s_25_1: u32 = fn_state.arg_;
        // D s_25_2: cmp-eq s_25_0 s_25_1
        let s_25_2: bool = ((s_25_0) == (s_25_1));
        // D s_25_3: not s_25_2
        let s_25_3: bool = !s_25_2;
        // N s_25_4: branch s_25_3 b27 b26
        if s_25_3 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_26_0: const #12s : i64
        let s_26_0: i64 = 12;
        // D s_26_1: write-var gs#746 <= s_26_0
        fn_state.gs_746 = s_26_0;
        // N s_26_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_27_0: const #13u : u32
        let s_27_0: u32 = 13;
        // D s_27_1: read-var arg#:u32
        let s_27_1: u32 = fn_state.arg_;
        // D s_27_2: cmp-eq s_27_0 s_27_1
        let s_27_2: bool = ((s_27_0) == (s_27_1));
        // D s_27_3: not s_27_2
        let s_27_3: bool = !s_27_2;
        // N s_27_4: branch s_27_3 b29 b28
        if s_27_3 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_28_0: const #13s : i64
        let s_28_0: i64 = 13;
        // D s_28_1: write-var gs#746 <= s_28_0
        fn_state.gs_746 = s_28_0;
        // N s_28_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_29_0: const #14u : u32
        let s_29_0: u32 = 14;
        // D s_29_1: read-var arg#:u32
        let s_29_1: u32 = fn_state.arg_;
        // D s_29_2: cmp-eq s_29_0 s_29_1
        let s_29_2: bool = ((s_29_0) == (s_29_1));
        // D s_29_3: not s_29_2
        let s_29_3: bool = !s_29_2;
        // N s_29_4: branch s_29_3 b31 b30
        if s_29_3 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_30_0: const #14s : i64
        let s_30_0: i64 = 14;
        // D s_30_1: write-var gs#746 <= s_30_0
        fn_state.gs_746 = s_30_0;
        // N s_30_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // N s_31_0: jump b2
        return block_2(state, tracer, fn_state);
    }
}
