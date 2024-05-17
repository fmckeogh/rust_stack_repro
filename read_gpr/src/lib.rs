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
pub fn read_gpr<T: Tracer>(state: &mut State, tracer: &T, n: i64) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        gs_1199: u64,
        n: i64,
    }
    let fn_state = FunctionState {
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_0_0: read-var n:i64
        let s_0_0: i64 = fn_state.n;
        // C s_0_1: const #0s : i
        let s_0_1: i128 = 0;
        // D s_0_2: cast zx s_0_0 -> i
        let s_0_2: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_3: cmp-eq s_0_2 s_0_1
        let s_0_3: bool = ((s_0_2) == (s_0_1));
        // D s_0_4: not s_0_3
        let s_0_4: bool = !s_0_3;
        // N s_0_5: branch s_0_4 b3 b1
        if s_0_4 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_1_0: const #21216u : u32
        let s_1_0: u32 = 21216;
        // D s_1_1: read-reg s_1_0:u64
        let s_1_1: u64 = {
            let value = state.read_register::<u64>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: write-var gs#1199 <= s_1_1
        fn_state.gs_1199 = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_2_0: read-var gs#1199:u64
        let s_2_0: u64 = fn_state.gs_1199;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_3_0: read-var n:i64
        let s_3_0: i64 = fn_state.n;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: cast zx s_3_0 -> i
        let s_3_2: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_1
        let s_3_3: bool = ((s_3_2) == (s_3_1));
        // D s_3_4: not s_3_3
        let s_3_4: bool = !s_3_3;
        // N s_3_5: branch s_3_4 b5 b4
        if s_3_4 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_4_0: const #16480u : u32
        let s_4_0: u32 = 16480;
        // D s_4_1: read-reg s_4_0:u64
        let s_4_1: u64 = {
            let value = state.read_register::<u64>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: write-var gs#1199 <= s_4_1
        fn_state.gs_1199 = s_4_1;
        // N s_4_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_5_0: read-var n:i64
        let s_5_0: i64 = fn_state.n;
        // C s_5_1: const #2s : i
        let s_5_1: i128 = 2;
        // D s_5_2: cast zx s_5_0 -> i
        let s_5_2: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_1
        let s_5_3: bool = ((s_5_2) == (s_5_1));
        // D s_5_4: not s_5_3
        let s_5_4: bool = !s_5_3;
        // N s_5_5: branch s_5_4 b7 b6
        if s_5_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_6_0: const #15672u : u32
        let s_6_0: u32 = 15672;
        // D s_6_1: read-reg s_6_0:u64
        let s_6_1: u64 = {
            let value = state.read_register::<u64>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: write-var gs#1199 <= s_6_1
        fn_state.gs_1199 = s_6_1;
        // N s_6_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_7_0: read-var n:i64
        let s_7_0: i64 = fn_state.n;
        // C s_7_1: const #3s : i
        let s_7_1: i128 = 3;
        // D s_7_2: cast zx s_7_0 -> i
        let s_7_2: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_3: cmp-eq s_7_2 s_7_1
        let s_7_3: bool = ((s_7_2) == (s_7_1));
        // D s_7_4: not s_7_3
        let s_7_4: bool = !s_7_3;
        // N s_7_5: branch s_7_4 b9 b8
        if s_7_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_8_0: const #22984u : u32
        let s_8_0: u32 = 22984;
        // D s_8_1: read-reg s_8_0:u64
        let s_8_1: u64 = {
            let value = state.read_register::<u64>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: write-var gs#1199 <= s_8_1
        fn_state.gs_1199 = s_8_1;
        // N s_8_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_9_0: read-var n:i64
        let s_9_0: i64 = fn_state.n;
        // C s_9_1: const #4s : i
        let s_9_1: i128 = 4;
        // D s_9_2: cast zx s_9_0 -> i
        let s_9_2: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_3: cmp-eq s_9_2 s_9_1
        let s_9_3: bool = ((s_9_2) == (s_9_1));
        // D s_9_4: not s_9_3
        let s_9_4: bool = !s_9_3;
        // N s_9_5: branch s_9_4 b11 b10
        if s_9_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_10_0: const #18320u : u32
        let s_10_0: u32 = 18320;
        // D s_10_1: read-reg s_10_0:u64
        let s_10_1: u64 = {
            let value = state.read_register::<u64>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: write-var gs#1199 <= s_10_1
        fn_state.gs_1199 = s_10_1;
        // N s_10_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_11_0: read-var n:i64
        let s_11_0: i64 = fn_state.n;
        // C s_11_1: const #5s : i
        let s_11_1: i128 = 5;
        // D s_11_2: cast zx s_11_0 -> i
        let s_11_2: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_3: cmp-eq s_11_2 s_11_1
        let s_11_3: bool = ((s_11_2) == (s_11_1));
        // D s_11_4: not s_11_3
        let s_11_4: bool = !s_11_3;
        // N s_11_5: branch s_11_4 b13 b12
        if s_11_4 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_12_0: const #22928u : u32
        let s_12_0: u32 = 22928;
        // D s_12_1: read-reg s_12_0:u64
        let s_12_1: u64 = {
            let value = state.read_register::<u64>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: write-var gs#1199 <= s_12_1
        fn_state.gs_1199 = s_12_1;
        // N s_12_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_13_0: read-var n:i64
        let s_13_0: i64 = fn_state.n;
        // C s_13_1: const #6s : i
        let s_13_1: i128 = 6;
        // D s_13_2: cast zx s_13_0 -> i
        let s_13_2: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_3: cmp-eq s_13_2 s_13_1
        let s_13_3: bool = ((s_13_2) == (s_13_1));
        // D s_13_4: not s_13_3
        let s_13_4: bool = !s_13_3;
        // N s_13_5: branch s_13_4 b15 b14
        if s_13_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_14_0: const #12016u : u32
        let s_14_0: u32 = 12016;
        // D s_14_1: read-reg s_14_0:u64
        let s_14_1: u64 = {
            let value = state.read_register::<u64>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: write-var gs#1199 <= s_14_1
        fn_state.gs_1199 = s_14_1;
        // N s_14_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_15_0: read-var n:i64
        let s_15_0: i64 = fn_state.n;
        // C s_15_1: const #7s : i
        let s_15_1: i128 = 7;
        // D s_15_2: cast zx s_15_0 -> i
        let s_15_2: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_3: cmp-eq s_15_2 s_15_1
        let s_15_3: bool = ((s_15_2) == (s_15_1));
        // D s_15_4: not s_15_3
        let s_15_4: bool = !s_15_3;
        // N s_15_5: branch s_15_4 b17 b16
        if s_15_4 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_16_0: const #101120u : u32
        let s_16_0: u32 = 101120;
        // D s_16_1: read-reg s_16_0:u64
        let s_16_1: u64 = {
            let value = state.read_register::<u64>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: write-var gs#1199 <= s_16_1
        fn_state.gs_1199 = s_16_1;
        // N s_16_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_17_0: read-var n:i64
        let s_17_0: i64 = fn_state.n;
        // C s_17_1: const #8s : i
        let s_17_1: i128 = 8;
        // D s_17_2: cast zx s_17_0 -> i
        let s_17_2: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_3: cmp-eq s_17_2 s_17_1
        let s_17_3: bool = ((s_17_2) == (s_17_1));
        // D s_17_4: not s_17_3
        let s_17_4: bool = !s_17_3;
        // N s_17_5: branch s_17_4 b19 b18
        if s_17_4 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_18_0: const #100952u : u32
        let s_18_0: u32 = 100952;
        // D s_18_1: read-reg s_18_0:u64
        let s_18_1: u64 = {
            let value = state.read_register::<u64>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: write-var gs#1199 <= s_18_1
        fn_state.gs_1199 = s_18_1;
        // N s_18_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_19_0: read-var n:i64
        let s_19_0: i64 = fn_state.n;
        // C s_19_1: const #9s : i
        let s_19_1: i128 = 9;
        // D s_19_2: cast zx s_19_0 -> i
        let s_19_2: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_3: cmp-eq s_19_2 s_19_1
        let s_19_3: bool = ((s_19_2) == (s_19_1));
        // D s_19_4: not s_19_3
        let s_19_4: bool = !s_19_3;
        // N s_19_5: branch s_19_4 b21 b20
        if s_19_4 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_20_0: const #19032u : u32
        let s_20_0: u32 = 19032;
        // D s_20_1: read-reg s_20_0:u64
        let s_20_1: u64 = {
            let value = state.read_register::<u64>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // D s_20_2: write-var gs#1199 <= s_20_1
        fn_state.gs_1199 = s_20_1;
        // N s_20_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_21_0: read-var n:i64
        let s_21_0: i64 = fn_state.n;
        // C s_21_1: const #10s : i
        let s_21_1: i128 = 10;
        // D s_21_2: cast zx s_21_0 -> i
        let s_21_2: i128 = (i128::try_from(s_21_0).unwrap());
        // D s_21_3: cmp-eq s_21_2 s_21_1
        let s_21_3: bool = ((s_21_2) == (s_21_1));
        // D s_21_4: not s_21_3
        let s_21_4: bool = !s_21_3;
        // N s_21_5: branch s_21_4 b23 b22
        if s_21_4 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_22_0: const #23152u : u32
        let s_22_0: u32 = 23152;
        // D s_22_1: read-reg s_22_0:u64
        let s_22_1: u64 = {
            let value = state.read_register::<u64>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: write-var gs#1199 <= s_22_1
        fn_state.gs_1199 = s_22_1;
        // N s_22_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_23_0: read-var n:i64
        let s_23_0: i64 = fn_state.n;
        // C s_23_1: const #11s : i
        let s_23_1: i128 = 11;
        // D s_23_2: cast zx s_23_0 -> i
        let s_23_2: i128 = (i128::try_from(s_23_0).unwrap());
        // D s_23_3: cmp-eq s_23_2 s_23_1
        let s_23_3: bool = ((s_23_2) == (s_23_1));
        // D s_23_4: not s_23_3
        let s_23_4: bool = !s_23_3;
        // N s_23_5: branch s_23_4 b25 b24
        if s_23_4 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_24_0: const #102768u : u32
        let s_24_0: u32 = 102768;
        // D s_24_1: read-reg s_24_0:u64
        let s_24_1: u64 = {
            let value = state.read_register::<u64>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // D s_24_2: write-var gs#1199 <= s_24_1
        fn_state.gs_1199 = s_24_1;
        // N s_24_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_25_0: read-var n:i64
        let s_25_0: i64 = fn_state.n;
        // C s_25_1: const #12s : i
        let s_25_1: i128 = 12;
        // D s_25_2: cast zx s_25_0 -> i
        let s_25_2: i128 = (i128::try_from(s_25_0).unwrap());
        // D s_25_3: cmp-eq s_25_2 s_25_1
        let s_25_3: bool = ((s_25_2) == (s_25_1));
        // D s_25_4: not s_25_3
        let s_25_4: bool = !s_25_3;
        // N s_25_5: branch s_25_4 b27 b26
        if s_25_4 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_26_0: const #102696u : u32
        let s_26_0: u32 = 102696;
        // D s_26_1: read-reg s_26_0:u64
        let s_26_1: u64 = {
            let value = state.read_register::<u64>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: write-var gs#1199 <= s_26_1
        fn_state.gs_1199 = s_26_1;
        // N s_26_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_27_0: read-var n:i64
        let s_27_0: i64 = fn_state.n;
        // C s_27_1: const #13s : i
        let s_27_1: i128 = 13;
        // D s_27_2: cast zx s_27_0 -> i
        let s_27_2: i128 = (i128::try_from(s_27_0).unwrap());
        // D s_27_3: cmp-eq s_27_2 s_27_1
        let s_27_3: bool = ((s_27_2) == (s_27_1));
        // D s_27_4: not s_27_3
        let s_27_4: bool = !s_27_3;
        // N s_27_5: branch s_27_4 b29 b28
        if s_27_4 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_28_0: const #20408u : u32
        let s_28_0: u32 = 20408;
        // D s_28_1: read-reg s_28_0:u64
        let s_28_1: u64 = {
            let value = state.read_register::<u64>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: write-var gs#1199 <= s_28_1
        fn_state.gs_1199 = s_28_1;
        // N s_28_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_29_0: read-var n:i64
        let s_29_0: i64 = fn_state.n;
        // C s_29_1: const #14s : i
        let s_29_1: i128 = 14;
        // D s_29_2: cast zx s_29_0 -> i
        let s_29_2: i128 = (i128::try_from(s_29_0).unwrap());
        // D s_29_3: cmp-eq s_29_2 s_29_1
        let s_29_3: bool = ((s_29_2) == (s_29_1));
        // D s_29_4: not s_29_3
        let s_29_4: bool = !s_29_3;
        // N s_29_5: branch s_29_4 b31 b30
        if s_29_4 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_30_0: const #19112u : u32
        let s_30_0: u32 = 19112;
        // D s_30_1: read-reg s_30_0:u64
        let s_30_1: u64 = {
            let value = state.read_register::<u64>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // D s_30_2: write-var gs#1199 <= s_30_1
        fn_state.gs_1199 = s_30_1;
        // N s_30_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_31_0: read-var n:i64
        let s_31_0: i64 = fn_state.n;
        // C s_31_1: const #15s : i
        let s_31_1: i128 = 15;
        // D s_31_2: cast zx s_31_0 -> i
        let s_31_2: i128 = (i128::try_from(s_31_0).unwrap());
        // D s_31_3: cmp-eq s_31_2 s_31_1
        let s_31_3: bool = ((s_31_2) == (s_31_1));
        // D s_31_4: not s_31_3
        let s_31_4: bool = !s_31_3;
        // N s_31_5: branch s_31_4 b33 b32
        if s_31_4 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_32_0: const #16944u : u32
        let s_32_0: u32 = 16944;
        // D s_32_1: read-reg s_32_0:u64
        let s_32_1: u64 = {
            let value = state.read_register::<u64>(s_32_0 as isize);
            tracer.read_register(s_32_0 as isize, value);
            value
        };
        // D s_32_2: write-var gs#1199 <= s_32_1
        fn_state.gs_1199 = s_32_1;
        // N s_32_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_33_0: read-var n:i64
        let s_33_0: i64 = fn_state.n;
        // C s_33_1: const #16s : i
        let s_33_1: i128 = 16;
        // D s_33_2: cast zx s_33_0 -> i
        let s_33_2: i128 = (i128::try_from(s_33_0).unwrap());
        // D s_33_3: cmp-eq s_33_2 s_33_1
        let s_33_3: bool = ((s_33_2) == (s_33_1));
        // D s_33_4: not s_33_3
        let s_33_4: bool = !s_33_3;
        // N s_33_5: branch s_33_4 b35 b34
        if s_33_4 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_34_0: const #23192u : u32
        let s_34_0: u32 = 23192;
        // D s_34_1: read-reg s_34_0:u64
        let s_34_1: u64 = {
            let value = state.read_register::<u64>(s_34_0 as isize);
            tracer.read_register(s_34_0 as isize, value);
            value
        };
        // D s_34_2: write-var gs#1199 <= s_34_1
        fn_state.gs_1199 = s_34_1;
        // N s_34_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_35_0: read-var n:i64
        let s_35_0: i64 = fn_state.n;
        // C s_35_1: const #17s : i
        let s_35_1: i128 = 17;
        // D s_35_2: cast zx s_35_0 -> i
        let s_35_2: i128 = (i128::try_from(s_35_0).unwrap());
        // D s_35_3: cmp-eq s_35_2 s_35_1
        let s_35_3: bool = ((s_35_2) == (s_35_1));
        // D s_35_4: not s_35_3
        let s_35_4: bool = !s_35_3;
        // N s_35_5: branch s_35_4 b37 b36
        if s_35_4 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_36_0: const #105000u : u32
        let s_36_0: u32 = 105000;
        // D s_36_1: read-reg s_36_0:u64
        let s_36_1: u64 = {
            let value = state.read_register::<u64>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: write-var gs#1199 <= s_36_1
        fn_state.gs_1199 = s_36_1;
        // N s_36_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_37_0: read-var n:i64
        let s_37_0: i64 = fn_state.n;
        // C s_37_1: const #18s : i
        let s_37_1: i128 = 18;
        // D s_37_2: cast zx s_37_0 -> i
        let s_37_2: i128 = (i128::try_from(s_37_0).unwrap());
        // D s_37_3: cmp-eq s_37_2 s_37_1
        let s_37_3: bool = ((s_37_2) == (s_37_1));
        // D s_37_4: not s_37_3
        let s_37_4: bool = !s_37_3;
        // N s_37_5: branch s_37_4 b39 b38
        if s_37_4 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_38_0: const #21200u : u32
        let s_38_0: u32 = 21200;
        // D s_38_1: read-reg s_38_0:u64
        let s_38_1: u64 = {
            let value = state.read_register::<u64>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // D s_38_2: write-var gs#1199 <= s_38_1
        fn_state.gs_1199 = s_38_1;
        // N s_38_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_39_0: read-var n:i64
        let s_39_0: i64 = fn_state.n;
        // C s_39_1: const #19s : i
        let s_39_1: i128 = 19;
        // D s_39_2: cast zx s_39_0 -> i
        let s_39_2: i128 = (i128::try_from(s_39_0).unwrap());
        // D s_39_3: cmp-eq s_39_2 s_39_1
        let s_39_3: bool = ((s_39_2) == (s_39_1));
        // D s_39_4: not s_39_3
        let s_39_4: bool = !s_39_3;
        // N s_39_5: branch s_39_4 b41 b40
        if s_39_4 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_40_0: const #17104u : u32
        let s_40_0: u32 = 17104;
        // D s_40_1: read-reg s_40_0:u64
        let s_40_1: u64 = {
            let value = state.read_register::<u64>(s_40_0 as isize);
            tracer.read_register(s_40_0 as isize, value);
            value
        };
        // D s_40_2: write-var gs#1199 <= s_40_1
        fn_state.gs_1199 = s_40_1;
        // N s_40_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_41_0: read-var n:i64
        let s_41_0: i64 = fn_state.n;
        // C s_41_1: const #20s : i
        let s_41_1: i128 = 20;
        // D s_41_2: cast zx s_41_0 -> i
        let s_41_2: i128 = (i128::try_from(s_41_0).unwrap());
        // D s_41_3: cmp-eq s_41_2 s_41_1
        let s_41_3: bool = ((s_41_2) == (s_41_1));
        // D s_41_4: not s_41_3
        let s_41_4: bool = !s_41_3;
        // N s_41_5: branch s_41_4 b43 b42
        if s_41_4 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_42_0: const #20368u : u32
        let s_42_0: u32 = 20368;
        // D s_42_1: read-reg s_42_0:u64
        let s_42_1: u64 = {
            let value = state.read_register::<u64>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // D s_42_2: write-var gs#1199 <= s_42_1
        fn_state.gs_1199 = s_42_1;
        // N s_42_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_43_0: read-var n:i64
        let s_43_0: i64 = fn_state.n;
        // C s_43_1: const #21s : i
        let s_43_1: i128 = 21;
        // D s_43_2: cast zx s_43_0 -> i
        let s_43_2: i128 = (i128::try_from(s_43_0).unwrap());
        // D s_43_3: cmp-eq s_43_2 s_43_1
        let s_43_3: bool = ((s_43_2) == (s_43_1));
        // D s_43_4: not s_43_3
        let s_43_4: bool = !s_43_3;
        // N s_43_5: branch s_43_4 b45 b44
        if s_43_4 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_44_0: const #20064u : u32
        let s_44_0: u32 = 20064;
        // D s_44_1: read-reg s_44_0:u64
        let s_44_1: u64 = {
            let value = state.read_register::<u64>(s_44_0 as isize);
            tracer.read_register(s_44_0 as isize, value);
            value
        };
        // D s_44_2: write-var gs#1199 <= s_44_1
        fn_state.gs_1199 = s_44_1;
        // N s_44_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_45_0: read-var n:i64
        let s_45_0: i64 = fn_state.n;
        // C s_45_1: const #22s : i
        let s_45_1: i128 = 22;
        // D s_45_2: cast zx s_45_0 -> i
        let s_45_2: i128 = (i128::try_from(s_45_0).unwrap());
        // D s_45_3: cmp-eq s_45_2 s_45_1
        let s_45_3: bool = ((s_45_2) == (s_45_1));
        // D s_45_4: not s_45_3
        let s_45_4: bool = !s_45_3;
        // N s_45_5: branch s_45_4 b47 b46
        if s_45_4 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_46_0: const #102712u : u32
        let s_46_0: u32 = 102712;
        // D s_46_1: read-reg s_46_0:u64
        let s_46_1: u64 = {
            let value = state.read_register::<u64>(s_46_0 as isize);
            tracer.read_register(s_46_0 as isize, value);
            value
        };
        // D s_46_2: write-var gs#1199 <= s_46_1
        fn_state.gs_1199 = s_46_1;
        // N s_46_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_47_0: read-var n:i64
        let s_47_0: i64 = fn_state.n;
        // C s_47_1: const #23s : i
        let s_47_1: i128 = 23;
        // D s_47_2: cast zx s_47_0 -> i
        let s_47_2: i128 = (i128::try_from(s_47_0).unwrap());
        // D s_47_3: cmp-eq s_47_2 s_47_1
        let s_47_3: bool = ((s_47_2) == (s_47_1));
        // D s_47_4: not s_47_3
        let s_47_4: bool = !s_47_3;
        // N s_47_5: branch s_47_4 b49 b48
        if s_47_4 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_48_0: const #11488u : u32
        let s_48_0: u32 = 11488;
        // D s_48_1: read-reg s_48_0:u64
        let s_48_1: u64 = {
            let value = state.read_register::<u64>(s_48_0 as isize);
            tracer.read_register(s_48_0 as isize, value);
            value
        };
        // D s_48_2: write-var gs#1199 <= s_48_1
        fn_state.gs_1199 = s_48_1;
        // N s_48_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_49_0: read-var n:i64
        let s_49_0: i64 = fn_state.n;
        // C s_49_1: const #24s : i
        let s_49_1: i128 = 24;
        // D s_49_2: cast zx s_49_0 -> i
        let s_49_2: i128 = (i128::try_from(s_49_0).unwrap());
        // D s_49_3: cmp-eq s_49_2 s_49_1
        let s_49_3: bool = ((s_49_2) == (s_49_1));
        // D s_49_4: not s_49_3
        let s_49_4: bool = !s_49_3;
        // N s_49_5: branch s_49_4 b51 b50
        if s_49_4 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_50_0: const #23912u : u32
        let s_50_0: u32 = 23912;
        // D s_50_1: read-reg s_50_0:u64
        let s_50_1: u64 = {
            let value = state.read_register::<u64>(s_50_0 as isize);
            tracer.read_register(s_50_0 as isize, value);
            value
        };
        // D s_50_2: write-var gs#1199 <= s_50_1
        fn_state.gs_1199 = s_50_1;
        // N s_50_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_51_0: read-var n:i64
        let s_51_0: i64 = fn_state.n;
        // C s_51_1: const #25s : i
        let s_51_1: i128 = 25;
        // D s_51_2: cast zx s_51_0 -> i
        let s_51_2: i128 = (i128::try_from(s_51_0).unwrap());
        // D s_51_3: cmp-eq s_51_2 s_51_1
        let s_51_3: bool = ((s_51_2) == (s_51_1));
        // D s_51_4: not s_51_3
        let s_51_4: bool = !s_51_3;
        // N s_51_5: branch s_51_4 b53 b52
        if s_51_4 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_52_0: const #10592u : u32
        let s_52_0: u32 = 10592;
        // D s_52_1: read-reg s_52_0:u64
        let s_52_1: u64 = {
            let value = state.read_register::<u64>(s_52_0 as isize);
            tracer.read_register(s_52_0 as isize, value);
            value
        };
        // D s_52_2: write-var gs#1199 <= s_52_1
        fn_state.gs_1199 = s_52_1;
        // N s_52_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_53_0: read-var n:i64
        let s_53_0: i64 = fn_state.n;
        // C s_53_1: const #26s : i
        let s_53_1: i128 = 26;
        // D s_53_2: cast zx s_53_0 -> i
        let s_53_2: i128 = (i128::try_from(s_53_0).unwrap());
        // D s_53_3: cmp-eq s_53_2 s_53_1
        let s_53_3: bool = ((s_53_2) == (s_53_1));
        // D s_53_4: not s_53_3
        let s_53_4: bool = !s_53_3;
        // N s_53_5: branch s_53_4 b55 b54
        if s_53_4 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_54_0: const #23096u : u32
        let s_54_0: u32 = 23096;
        // D s_54_1: read-reg s_54_0:u64
        let s_54_1: u64 = {
            let value = state.read_register::<u64>(s_54_0 as isize);
            tracer.read_register(s_54_0 as isize, value);
            value
        };
        // D s_54_2: write-var gs#1199 <= s_54_1
        fn_state.gs_1199 = s_54_1;
        // N s_54_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_55_0: read-var n:i64
        let s_55_0: i64 = fn_state.n;
        // C s_55_1: const #27s : i
        let s_55_1: i128 = 27;
        // D s_55_2: cast zx s_55_0 -> i
        let s_55_2: i128 = (i128::try_from(s_55_0).unwrap());
        // D s_55_3: cmp-eq s_55_2 s_55_1
        let s_55_3: bool = ((s_55_2) == (s_55_1));
        // D s_55_4: not s_55_3
        let s_55_4: bool = !s_55_3;
        // N s_55_5: branch s_55_4 b57 b56
        if s_55_4 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_56_0: const #21840u : u32
        let s_56_0: u32 = 21840;
        // D s_56_1: read-reg s_56_0:u64
        let s_56_1: u64 = {
            let value = state.read_register::<u64>(s_56_0 as isize);
            tracer.read_register(s_56_0 as isize, value);
            value
        };
        // D s_56_2: write-var gs#1199 <= s_56_1
        fn_state.gs_1199 = s_56_1;
        // N s_56_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_57_0: read-var n:i64
        let s_57_0: i64 = fn_state.n;
        // C s_57_1: const #28s : i
        let s_57_1: i128 = 28;
        // D s_57_2: cast zx s_57_0 -> i
        let s_57_2: i128 = (i128::try_from(s_57_0).unwrap());
        // D s_57_3: cmp-eq s_57_2 s_57_1
        let s_57_3: bool = ((s_57_2) == (s_57_1));
        // D s_57_4: not s_57_3
        let s_57_4: bool = !s_57_3;
        // N s_57_5: branch s_57_4 b59 b58
        if s_57_4 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_58_0: const #11656u : u32
        let s_58_0: u32 = 11656;
        // D s_58_1: read-reg s_58_0:u64
        let s_58_1: u64 = {
            let value = state.read_register::<u64>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // D s_58_2: write-var gs#1199 <= s_58_1
        fn_state.gs_1199 = s_58_1;
        // N s_58_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_59_0: read-var n:i64
        let s_59_0: i64 = fn_state.n;
        // C s_59_1: const #29s : i
        let s_59_1: i128 = 29;
        // D s_59_2: cast zx s_59_0 -> i
        let s_59_2: i128 = (i128::try_from(s_59_0).unwrap());
        // D s_59_3: cmp-eq s_59_2 s_59_1
        let s_59_3: bool = ((s_59_2) == (s_59_1));
        // D s_59_4: not s_59_3
        let s_59_4: bool = !s_59_3;
        // N s_59_5: branch s_59_4 b61 b60
        if s_59_4 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_60_0: const #14768u : u32
        let s_60_0: u32 = 14768;
        // D s_60_1: read-reg s_60_0:u64
        let s_60_1: u64 = {
            let value = state.read_register::<u64>(s_60_0 as isize);
            tracer.read_register(s_60_0 as isize, value);
            value
        };
        // D s_60_2: write-var gs#1199 <= s_60_1
        fn_state.gs_1199 = s_60_1;
        // N s_60_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_61_0: const #20472u : u32
        let s_61_0: u32 = 20472;
        // D s_61_1: read-reg s_61_0:u64
        let s_61_1: u64 = {
            let value = state.read_register::<u64>(s_61_0 as isize);
            tracer.read_register(s_61_0 as isize, value);
            value
        };
        // D s_61_2: write-var gs#1199 <= s_61_1
        fn_state.gs_1199 = s_61_1;
        // N s_61_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
