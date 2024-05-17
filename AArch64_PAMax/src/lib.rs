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
use u__IMPDEF_integer::*;
use common::*;
pub fn AArch64_PAMax<T: Tracer>(state: &mut State, tracer: &T, gs_12998: ()) -> i64 {
    #[derive(Default)]
    struct FunctionState {
        gs_13009: bool,
        gs_13008: bool,
        gs_13010: bool,
        pamax: i128,
        gs_13012: bool,
        gs_13011: bool,
        gs_13014: bool,
        gs_13013: bool,
        gs_12998: (),
    }
    let fn_state = FunctionState {
        gs_12998,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_0_0: const #"Maximum Physical Address Size" : str
        let s_0_0: &'static str = "Maximum Physical Address Size";
        // S s_0_1: call __IMPDEF_integer(s_0_0)
        let s_0_1: i128 = u__IMPDEF_integer(state, tracer, s_0_0);
        // D s_0_2: write-var pamax <= s_0_1
        fn_state.pamax = s_0_1;
        // C s_0_3: const #32s : i
        let s_0_3: i128 = 32;
        // D s_0_4: read-var pamax:i
        let s_0_4: i128 = fn_state.pamax;
        // D s_0_5: cmp-eq s_0_4 s_0_3
        let s_0_5: bool = ((s_0_4) == (s_0_3));
        // N s_0_6: branch s_0_5 b21 b1
        if s_0_5 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_1_0: const #36s : i
        let s_1_0: i128 = 36;
        // D s_1_1: read-var pamax:i
        let s_1_1: i128 = fn_state.pamax;
        // D s_1_2: cmp-eq s_1_1 s_1_0
        let s_1_2: bool = ((s_1_1) == (s_1_0));
        // N s_1_3: branch s_1_2 b20 b2
        if s_1_2 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_2_0: const #40s : i
        let s_2_0: i128 = 40;
        // D s_2_1: read-var pamax:i
        let s_2_1: i128 = fn_state.pamax;
        // D s_2_2: cmp-eq s_2_1 s_2_0
        let s_2_2: bool = ((s_2_1) == (s_2_0));
        // N s_2_3: branch s_2_2 b19 b3
        if s_2_2 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_3_0: const #42s : i
        let s_3_0: i128 = 42;
        // D s_3_1: read-var pamax:i
        let s_3_1: i128 = fn_state.pamax;
        // D s_3_2: cmp-eq s_3_1 s_3_0
        let s_3_2: bool = ((s_3_1) == (s_3_0));
        // N s_3_3: branch s_3_2 b18 b4
        if s_3_2 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_4_0: const #44s : i
        let s_4_0: i128 = 44;
        // D s_4_1: read-var pamax:i
        let s_4_1: i128 = fn_state.pamax;
        // D s_4_2: cmp-eq s_4_1 s_4_0
        let s_4_2: bool = ((s_4_1) == (s_4_0));
        // N s_4_3: branch s_4_2 b17 b5
        if s_4_2 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_5_0: const #48s : i
        let s_5_0: i128 = 48;
        // D s_5_1: read-var pamax:i
        let s_5_1: i128 = fn_state.pamax;
        // D s_5_2: cmp-eq s_5_1 s_5_0
        let s_5_2: bool = ((s_5_1) == (s_5_0));
        // N s_5_3: branch s_5_2 b16 b6
        if s_5_2 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_6_0: const #52s : i
        let s_6_0: i128 = 52;
        // D s_6_1: read-var pamax:i
        let s_6_1: i128 = fn_state.pamax;
        // D s_6_2: cmp-eq s_6_1 s_6_0
        let s_6_2: bool = ((s_6_1) == (s_6_0));
        // N s_6_3: branch s_6_2 b15 b7
        if s_6_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_7_0: const #56s : i
        let s_7_0: i128 = 56;
        // D s_7_1: read-var pamax:i
        let s_7_1: i128 = fn_state.pamax;
        // D s_7_2: cmp-eq s_7_1 s_7_0
        let s_7_2: bool = ((s_7_1) == (s_7_0));
        // D s_7_3: write-var gs#13008 <= s_7_2
        fn_state.gs_13008 = s_7_2;
        // N s_7_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // D s_8_0: read-var gs#13008:u8
        let s_8_0: bool = fn_state.gs_13008;
        // D s_8_1: write-var gs#13009 <= s_8_0
        fn_state.gs_13009 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // D s_9_0: read-var gs#13009:u8
        let s_9_0: bool = fn_state.gs_13009;
        // D s_9_1: write-var gs#13010 <= s_9_0
        fn_state.gs_13010 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // D s_10_0: read-var gs#13010:u8
        let s_10_0: bool = fn_state.gs_13010;
        // D s_10_1: write-var gs#13011 <= s_10_0
        fn_state.gs_13011 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // D s_11_0: read-var gs#13011:u8
        let s_11_0: bool = fn_state.gs_13011;
        // D s_11_1: write-var gs#13012 <= s_11_0
        fn_state.gs_13012 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // D s_12_0: read-var gs#13012:u8
        let s_12_0: bool = fn_state.gs_13012;
        // D s_12_1: write-var gs#13013 <= s_12_0
        fn_state.gs_13013 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // D s_13_0: read-var gs#13013:u8
        let s_13_0: bool = fn_state.gs_13013;
        // D s_13_1: write-var gs#13014 <= s_13_0
        fn_state.gs_13014 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // D s_14_0: read-var gs#13014:u8
        let s_14_0: bool = fn_state.gs_13014;
        // N s_14_1: assert s_14_0
        let s_14_1: () = assert!(s_14_0);
        // D s_14_2: read-var pamax:i
        let s_14_2: i128 = fn_state.pamax;
        // D s_14_3: cast reint s_14_2 -> i64
        let s_14_3: i64 = (s_14_2 as i64);
        // N s_14_4: return s_14_3
        return s_14_3;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#13008 <= s_15_0
        fn_state.gs_13008 = s_15_0;
        // N s_15_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#13009 <= s_16_0
        fn_state.gs_13009 = s_16_0;
        // N s_16_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#13010 <= s_17_0
        fn_state.gs_13010 = s_17_0;
        // N s_17_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#13011 <= s_18_0
        fn_state.gs_13011 = s_18_0;
        // N s_18_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#13012 <= s_19_0
        fn_state.gs_13012 = s_19_0;
        // N s_19_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#13013 <= s_20_0
        fn_state.gs_13013 = s_20_0;
        // N s_20_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#13014 <= s_21_0
        fn_state.gs_13014 = s_21_0;
        // N s_21_2: jump b14
        return block_14(state, tracer, fn_state);
    }
}
