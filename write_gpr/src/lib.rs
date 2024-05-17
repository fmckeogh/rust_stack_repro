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
pub fn write_gpr<T: Tracer>(state: &mut State, tracer: &T, n: i64, v: u64) -> () {
    #[derive(Default)]
    struct FunctionState {
        n: i64,
        v: u64,
    }
    let fn_state = FunctionState {
        n,
        v,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
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
        // N s_0_5: branch s_0_4 b2 b1
        if s_0_4 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var v:u64
        let s_1_0: u64 = fn_state.v;
        // C s_1_1: const #21216u : u32
        let s_1_1: u32 = 21216;
        // N s_1_2: write-reg s_1_1 <= s_1_0
        let s_1_2: () = {
            state.write_register::<u64>(s_1_1 as isize, s_1_0);
            tracer.write_register(s_1_1 as isize, s_1_0);
        };
        // N s_1_3: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var n:i64
        let s_2_0: i64 = fn_state.n;
        // C s_2_1: const #1s : i
        let s_2_1: i128 = 1;
        // D s_2_2: cast zx s_2_0 -> i
        let s_2_2: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_3: cmp-eq s_2_2 s_2_1
        let s_2_3: bool = ((s_2_2) == (s_2_1));
        // D s_2_4: not s_2_3
        let s_2_4: bool = !s_2_3;
        // N s_2_5: branch s_2_4 b4 b3
        if s_2_4 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var v:u64
        let s_3_0: u64 = fn_state.v;
        // C s_3_1: const #16480u : u32
        let s_3_1: u32 = 16480;
        // N s_3_2: write-reg s_3_1 <= s_3_0
        let s_3_2: () = {
            state.write_register::<u64>(s_3_1 as isize, s_3_0);
            tracer.write_register(s_3_1 as isize, s_3_0);
        };
        // N s_3_3: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var n:i64
        let s_4_0: i64 = fn_state.n;
        // C s_4_1: const #2s : i
        let s_4_1: i128 = 2;
        // D s_4_2: cast zx s_4_0 -> i
        let s_4_2: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_3: cmp-eq s_4_2 s_4_1
        let s_4_3: bool = ((s_4_2) == (s_4_1));
        // D s_4_4: not s_4_3
        let s_4_4: bool = !s_4_3;
        // N s_4_5: branch s_4_4 b6 b5
        if s_4_4 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var v:u64
        let s_5_0: u64 = fn_state.v;
        // C s_5_1: const #15672u : u32
        let s_5_1: u32 = 15672;
        // N s_5_2: write-reg s_5_1 <= s_5_0
        let s_5_2: () = {
            state.write_register::<u64>(s_5_1 as isize, s_5_0);
            tracer.write_register(s_5_1 as isize, s_5_0);
        };
        // N s_5_3: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var n:i64
        let s_6_0: i64 = fn_state.n;
        // C s_6_1: const #3s : i
        let s_6_1: i128 = 3;
        // D s_6_2: cast zx s_6_0 -> i
        let s_6_2: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_3: cmp-eq s_6_2 s_6_1
        let s_6_3: bool = ((s_6_2) == (s_6_1));
        // D s_6_4: not s_6_3
        let s_6_4: bool = !s_6_3;
        // N s_6_5: branch s_6_4 b8 b7
        if s_6_4 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var v:u64
        let s_7_0: u64 = fn_state.v;
        // C s_7_1: const #22984u : u32
        let s_7_1: u32 = 22984;
        // N s_7_2: write-reg s_7_1 <= s_7_0
        let s_7_2: () = {
            state.write_register::<u64>(s_7_1 as isize, s_7_0);
            tracer.write_register(s_7_1 as isize, s_7_0);
        };
        // N s_7_3: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var n:i64
        let s_8_0: i64 = fn_state.n;
        // C s_8_1: const #4s : i
        let s_8_1: i128 = 4;
        // D s_8_2: cast zx s_8_0 -> i
        let s_8_2: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_3: cmp-eq s_8_2 s_8_1
        let s_8_3: bool = ((s_8_2) == (s_8_1));
        // D s_8_4: not s_8_3
        let s_8_4: bool = !s_8_3;
        // N s_8_5: branch s_8_4 b10 b9
        if s_8_4 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var v:u64
        let s_9_0: u64 = fn_state.v;
        // C s_9_1: const #18320u : u32
        let s_9_1: u32 = 18320;
        // N s_9_2: write-reg s_9_1 <= s_9_0
        let s_9_2: () = {
            state.write_register::<u64>(s_9_1 as isize, s_9_0);
            tracer.write_register(s_9_1 as isize, s_9_0);
        };
        // N s_9_3: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var n:i64
        let s_10_0: i64 = fn_state.n;
        // C s_10_1: const #5s : i
        let s_10_1: i128 = 5;
        // D s_10_2: cast zx s_10_0 -> i
        let s_10_2: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_3: cmp-eq s_10_2 s_10_1
        let s_10_3: bool = ((s_10_2) == (s_10_1));
        // D s_10_4: not s_10_3
        let s_10_4: bool = !s_10_3;
        // N s_10_5: branch s_10_4 b12 b11
        if s_10_4 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var v:u64
        let s_11_0: u64 = fn_state.v;
        // C s_11_1: const #22928u : u32
        let s_11_1: u32 = 22928;
        // N s_11_2: write-reg s_11_1 <= s_11_0
        let s_11_2: () = {
            state.write_register::<u64>(s_11_1 as isize, s_11_0);
            tracer.write_register(s_11_1 as isize, s_11_0);
        };
        // N s_11_3: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var n:i64
        let s_12_0: i64 = fn_state.n;
        // C s_12_1: const #6s : i
        let s_12_1: i128 = 6;
        // D s_12_2: cast zx s_12_0 -> i
        let s_12_2: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_3: cmp-eq s_12_2 s_12_1
        let s_12_3: bool = ((s_12_2) == (s_12_1));
        // D s_12_4: not s_12_3
        let s_12_4: bool = !s_12_3;
        // N s_12_5: branch s_12_4 b14 b13
        if s_12_4 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var v:u64
        let s_13_0: u64 = fn_state.v;
        // C s_13_1: const #12016u : u32
        let s_13_1: u32 = 12016;
        // N s_13_2: write-reg s_13_1 <= s_13_0
        let s_13_2: () = {
            state.write_register::<u64>(s_13_1 as isize, s_13_0);
            tracer.write_register(s_13_1 as isize, s_13_0);
        };
        // N s_13_3: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var n:i64
        let s_14_0: i64 = fn_state.n;
        // C s_14_1: const #7s : i
        let s_14_1: i128 = 7;
        // D s_14_2: cast zx s_14_0 -> i
        let s_14_2: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_3: cmp-eq s_14_2 s_14_1
        let s_14_3: bool = ((s_14_2) == (s_14_1));
        // D s_14_4: not s_14_3
        let s_14_4: bool = !s_14_3;
        // N s_14_5: branch s_14_4 b16 b15
        if s_14_4 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var v:u64
        let s_15_0: u64 = fn_state.v;
        // C s_15_1: const #101120u : u32
        let s_15_1: u32 = 101120;
        // N s_15_2: write-reg s_15_1 <= s_15_0
        let s_15_2: () = {
            state.write_register::<u64>(s_15_1 as isize, s_15_0);
            tracer.write_register(s_15_1 as isize, s_15_0);
        };
        // N s_15_3: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var n:i64
        let s_16_0: i64 = fn_state.n;
        // C s_16_1: const #8s : i
        let s_16_1: i128 = 8;
        // D s_16_2: cast zx s_16_0 -> i
        let s_16_2: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_3: cmp-eq s_16_2 s_16_1
        let s_16_3: bool = ((s_16_2) == (s_16_1));
        // D s_16_4: not s_16_3
        let s_16_4: bool = !s_16_3;
        // N s_16_5: branch s_16_4 b18 b17
        if s_16_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var v:u64
        let s_17_0: u64 = fn_state.v;
        // C s_17_1: const #100952u : u32
        let s_17_1: u32 = 100952;
        // N s_17_2: write-reg s_17_1 <= s_17_0
        let s_17_2: () = {
            state.write_register::<u64>(s_17_1 as isize, s_17_0);
            tracer.write_register(s_17_1 as isize, s_17_0);
        };
        // N s_17_3: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var n:i64
        let s_18_0: i64 = fn_state.n;
        // C s_18_1: const #9s : i
        let s_18_1: i128 = 9;
        // D s_18_2: cast zx s_18_0 -> i
        let s_18_2: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_3: cmp-eq s_18_2 s_18_1
        let s_18_3: bool = ((s_18_2) == (s_18_1));
        // D s_18_4: not s_18_3
        let s_18_4: bool = !s_18_3;
        // N s_18_5: branch s_18_4 b20 b19
        if s_18_4 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var v:u64
        let s_19_0: u64 = fn_state.v;
        // C s_19_1: const #19032u : u32
        let s_19_1: u32 = 19032;
        // N s_19_2: write-reg s_19_1 <= s_19_0
        let s_19_2: () = {
            state.write_register::<u64>(s_19_1 as isize, s_19_0);
            tracer.write_register(s_19_1 as isize, s_19_0);
        };
        // N s_19_3: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var n:i64
        let s_20_0: i64 = fn_state.n;
        // C s_20_1: const #10s : i
        let s_20_1: i128 = 10;
        // D s_20_2: cast zx s_20_0 -> i
        let s_20_2: i128 = (i128::try_from(s_20_0).unwrap());
        // D s_20_3: cmp-eq s_20_2 s_20_1
        let s_20_3: bool = ((s_20_2) == (s_20_1));
        // D s_20_4: not s_20_3
        let s_20_4: bool = !s_20_3;
        // N s_20_5: branch s_20_4 b22 b21
        if s_20_4 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var v:u64
        let s_21_0: u64 = fn_state.v;
        // C s_21_1: const #23152u : u32
        let s_21_1: u32 = 23152;
        // N s_21_2: write-reg s_21_1 <= s_21_0
        let s_21_2: () = {
            state.write_register::<u64>(s_21_1 as isize, s_21_0);
            tracer.write_register(s_21_1 as isize, s_21_0);
        };
        // N s_21_3: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var n:i64
        let s_22_0: i64 = fn_state.n;
        // C s_22_1: const #11s : i
        let s_22_1: i128 = 11;
        // D s_22_2: cast zx s_22_0 -> i
        let s_22_2: i128 = (i128::try_from(s_22_0).unwrap());
        // D s_22_3: cmp-eq s_22_2 s_22_1
        let s_22_3: bool = ((s_22_2) == (s_22_1));
        // D s_22_4: not s_22_3
        let s_22_4: bool = !s_22_3;
        // N s_22_5: branch s_22_4 b24 b23
        if s_22_4 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var v:u64
        let s_23_0: u64 = fn_state.v;
        // C s_23_1: const #102768u : u32
        let s_23_1: u32 = 102768;
        // N s_23_2: write-reg s_23_1 <= s_23_0
        let s_23_2: () = {
            state.write_register::<u64>(s_23_1 as isize, s_23_0);
            tracer.write_register(s_23_1 as isize, s_23_0);
        };
        // N s_23_3: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var n:i64
        let s_24_0: i64 = fn_state.n;
        // C s_24_1: const #12s : i
        let s_24_1: i128 = 12;
        // D s_24_2: cast zx s_24_0 -> i
        let s_24_2: i128 = (i128::try_from(s_24_0).unwrap());
        // D s_24_3: cmp-eq s_24_2 s_24_1
        let s_24_3: bool = ((s_24_2) == (s_24_1));
        // D s_24_4: not s_24_3
        let s_24_4: bool = !s_24_3;
        // N s_24_5: branch s_24_4 b26 b25
        if s_24_4 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var v:u64
        let s_25_0: u64 = fn_state.v;
        // C s_25_1: const #102696u : u32
        let s_25_1: u32 = 102696;
        // N s_25_2: write-reg s_25_1 <= s_25_0
        let s_25_2: () = {
            state.write_register::<u64>(s_25_1 as isize, s_25_0);
            tracer.write_register(s_25_1 as isize, s_25_0);
        };
        // N s_25_3: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var n:i64
        let s_26_0: i64 = fn_state.n;
        // C s_26_1: const #13s : i
        let s_26_1: i128 = 13;
        // D s_26_2: cast zx s_26_0 -> i
        let s_26_2: i128 = (i128::try_from(s_26_0).unwrap());
        // D s_26_3: cmp-eq s_26_2 s_26_1
        let s_26_3: bool = ((s_26_2) == (s_26_1));
        // D s_26_4: not s_26_3
        let s_26_4: bool = !s_26_3;
        // N s_26_5: branch s_26_4 b28 b27
        if s_26_4 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var v:u64
        let s_27_0: u64 = fn_state.v;
        // C s_27_1: const #20408u : u32
        let s_27_1: u32 = 20408;
        // N s_27_2: write-reg s_27_1 <= s_27_0
        let s_27_2: () = {
            state.write_register::<u64>(s_27_1 as isize, s_27_0);
            tracer.write_register(s_27_1 as isize, s_27_0);
        };
        // N s_27_3: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var n:i64
        let s_28_0: i64 = fn_state.n;
        // C s_28_1: const #14s : i
        let s_28_1: i128 = 14;
        // D s_28_2: cast zx s_28_0 -> i
        let s_28_2: i128 = (i128::try_from(s_28_0).unwrap());
        // D s_28_3: cmp-eq s_28_2 s_28_1
        let s_28_3: bool = ((s_28_2) == (s_28_1));
        // D s_28_4: not s_28_3
        let s_28_4: bool = !s_28_3;
        // N s_28_5: branch s_28_4 b30 b29
        if s_28_4 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var v:u64
        let s_29_0: u64 = fn_state.v;
        // C s_29_1: const #19112u : u32
        let s_29_1: u32 = 19112;
        // N s_29_2: write-reg s_29_1 <= s_29_0
        let s_29_2: () = {
            state.write_register::<u64>(s_29_1 as isize, s_29_0);
            tracer.write_register(s_29_1 as isize, s_29_0);
        };
        // N s_29_3: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var n:i64
        let s_30_0: i64 = fn_state.n;
        // C s_30_1: const #15s : i
        let s_30_1: i128 = 15;
        // D s_30_2: cast zx s_30_0 -> i
        let s_30_2: i128 = (i128::try_from(s_30_0).unwrap());
        // D s_30_3: cmp-eq s_30_2 s_30_1
        let s_30_3: bool = ((s_30_2) == (s_30_1));
        // D s_30_4: not s_30_3
        let s_30_4: bool = !s_30_3;
        // N s_30_5: branch s_30_4 b32 b31
        if s_30_4 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var v:u64
        let s_31_0: u64 = fn_state.v;
        // C s_31_1: const #16944u : u32
        let s_31_1: u32 = 16944;
        // N s_31_2: write-reg s_31_1 <= s_31_0
        let s_31_2: () = {
            state.write_register::<u64>(s_31_1 as isize, s_31_0);
            tracer.write_register(s_31_1 as isize, s_31_0);
        };
        // N s_31_3: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var n:i64
        let s_32_0: i64 = fn_state.n;
        // C s_32_1: const #16s : i
        let s_32_1: i128 = 16;
        // D s_32_2: cast zx s_32_0 -> i
        let s_32_2: i128 = (i128::try_from(s_32_0).unwrap());
        // D s_32_3: cmp-eq s_32_2 s_32_1
        let s_32_3: bool = ((s_32_2) == (s_32_1));
        // D s_32_4: not s_32_3
        let s_32_4: bool = !s_32_3;
        // N s_32_5: branch s_32_4 b34 b33
        if s_32_4 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var v:u64
        let s_33_0: u64 = fn_state.v;
        // C s_33_1: const #23192u : u32
        let s_33_1: u32 = 23192;
        // N s_33_2: write-reg s_33_1 <= s_33_0
        let s_33_2: () = {
            state.write_register::<u64>(s_33_1 as isize, s_33_0);
            tracer.write_register(s_33_1 as isize, s_33_0);
        };
        // N s_33_3: return
        return;
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var n:i64
        let s_34_0: i64 = fn_state.n;
        // C s_34_1: const #17s : i
        let s_34_1: i128 = 17;
        // D s_34_2: cast zx s_34_0 -> i
        let s_34_2: i128 = (i128::try_from(s_34_0).unwrap());
        // D s_34_3: cmp-eq s_34_2 s_34_1
        let s_34_3: bool = ((s_34_2) == (s_34_1));
        // D s_34_4: not s_34_3
        let s_34_4: bool = !s_34_3;
        // N s_34_5: branch s_34_4 b36 b35
        if s_34_4 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var v:u64
        let s_35_0: u64 = fn_state.v;
        // C s_35_1: const #105000u : u32
        let s_35_1: u32 = 105000;
        // N s_35_2: write-reg s_35_1 <= s_35_0
        let s_35_2: () = {
            state.write_register::<u64>(s_35_1 as isize, s_35_0);
            tracer.write_register(s_35_1 as isize, s_35_0);
        };
        // N s_35_3: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var n:i64
        let s_36_0: i64 = fn_state.n;
        // C s_36_1: const #18s : i
        let s_36_1: i128 = 18;
        // D s_36_2: cast zx s_36_0 -> i
        let s_36_2: i128 = (i128::try_from(s_36_0).unwrap());
        // D s_36_3: cmp-eq s_36_2 s_36_1
        let s_36_3: bool = ((s_36_2) == (s_36_1));
        // D s_36_4: not s_36_3
        let s_36_4: bool = !s_36_3;
        // N s_36_5: branch s_36_4 b38 b37
        if s_36_4 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var v:u64
        let s_37_0: u64 = fn_state.v;
        // C s_37_1: const #21200u : u32
        let s_37_1: u32 = 21200;
        // N s_37_2: write-reg s_37_1 <= s_37_0
        let s_37_2: () = {
            state.write_register::<u64>(s_37_1 as isize, s_37_0);
            tracer.write_register(s_37_1 as isize, s_37_0);
        };
        // N s_37_3: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var n:i64
        let s_38_0: i64 = fn_state.n;
        // C s_38_1: const #19s : i
        let s_38_1: i128 = 19;
        // D s_38_2: cast zx s_38_0 -> i
        let s_38_2: i128 = (i128::try_from(s_38_0).unwrap());
        // D s_38_3: cmp-eq s_38_2 s_38_1
        let s_38_3: bool = ((s_38_2) == (s_38_1));
        // D s_38_4: not s_38_3
        let s_38_4: bool = !s_38_3;
        // N s_38_5: branch s_38_4 b40 b39
        if s_38_4 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var v:u64
        let s_39_0: u64 = fn_state.v;
        // C s_39_1: const #17104u : u32
        let s_39_1: u32 = 17104;
        // N s_39_2: write-reg s_39_1 <= s_39_0
        let s_39_2: () = {
            state.write_register::<u64>(s_39_1 as isize, s_39_0);
            tracer.write_register(s_39_1 as isize, s_39_0);
        };
        // N s_39_3: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var n:i64
        let s_40_0: i64 = fn_state.n;
        // C s_40_1: const #20s : i
        let s_40_1: i128 = 20;
        // D s_40_2: cast zx s_40_0 -> i
        let s_40_2: i128 = (i128::try_from(s_40_0).unwrap());
        // D s_40_3: cmp-eq s_40_2 s_40_1
        let s_40_3: bool = ((s_40_2) == (s_40_1));
        // D s_40_4: not s_40_3
        let s_40_4: bool = !s_40_3;
        // N s_40_5: branch s_40_4 b42 b41
        if s_40_4 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var v:u64
        let s_41_0: u64 = fn_state.v;
        // C s_41_1: const #20368u : u32
        let s_41_1: u32 = 20368;
        // N s_41_2: write-reg s_41_1 <= s_41_0
        let s_41_2: () = {
            state.write_register::<u64>(s_41_1 as isize, s_41_0);
            tracer.write_register(s_41_1 as isize, s_41_0);
        };
        // N s_41_3: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var n:i64
        let s_42_0: i64 = fn_state.n;
        // C s_42_1: const #21s : i
        let s_42_1: i128 = 21;
        // D s_42_2: cast zx s_42_0 -> i
        let s_42_2: i128 = (i128::try_from(s_42_0).unwrap());
        // D s_42_3: cmp-eq s_42_2 s_42_1
        let s_42_3: bool = ((s_42_2) == (s_42_1));
        // D s_42_4: not s_42_3
        let s_42_4: bool = !s_42_3;
        // N s_42_5: branch s_42_4 b44 b43
        if s_42_4 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var v:u64
        let s_43_0: u64 = fn_state.v;
        // C s_43_1: const #20064u : u32
        let s_43_1: u32 = 20064;
        // N s_43_2: write-reg s_43_1 <= s_43_0
        let s_43_2: () = {
            state.write_register::<u64>(s_43_1 as isize, s_43_0);
            tracer.write_register(s_43_1 as isize, s_43_0);
        };
        // N s_43_3: return
        return;
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var n:i64
        let s_44_0: i64 = fn_state.n;
        // C s_44_1: const #22s : i
        let s_44_1: i128 = 22;
        // D s_44_2: cast zx s_44_0 -> i
        let s_44_2: i128 = (i128::try_from(s_44_0).unwrap());
        // D s_44_3: cmp-eq s_44_2 s_44_1
        let s_44_3: bool = ((s_44_2) == (s_44_1));
        // D s_44_4: not s_44_3
        let s_44_4: bool = !s_44_3;
        // N s_44_5: branch s_44_4 b46 b45
        if s_44_4 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var v:u64
        let s_45_0: u64 = fn_state.v;
        // C s_45_1: const #102712u : u32
        let s_45_1: u32 = 102712;
        // N s_45_2: write-reg s_45_1 <= s_45_0
        let s_45_2: () = {
            state.write_register::<u64>(s_45_1 as isize, s_45_0);
            tracer.write_register(s_45_1 as isize, s_45_0);
        };
        // N s_45_3: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var n:i64
        let s_46_0: i64 = fn_state.n;
        // C s_46_1: const #23s : i
        let s_46_1: i128 = 23;
        // D s_46_2: cast zx s_46_0 -> i
        let s_46_2: i128 = (i128::try_from(s_46_0).unwrap());
        // D s_46_3: cmp-eq s_46_2 s_46_1
        let s_46_3: bool = ((s_46_2) == (s_46_1));
        // D s_46_4: not s_46_3
        let s_46_4: bool = !s_46_3;
        // N s_46_5: branch s_46_4 b48 b47
        if s_46_4 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var v:u64
        let s_47_0: u64 = fn_state.v;
        // C s_47_1: const #11488u : u32
        let s_47_1: u32 = 11488;
        // N s_47_2: write-reg s_47_1 <= s_47_0
        let s_47_2: () = {
            state.write_register::<u64>(s_47_1 as isize, s_47_0);
            tracer.write_register(s_47_1 as isize, s_47_0);
        };
        // N s_47_3: return
        return;
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var n:i64
        let s_48_0: i64 = fn_state.n;
        // C s_48_1: const #24s : i
        let s_48_1: i128 = 24;
        // D s_48_2: cast zx s_48_0 -> i
        let s_48_2: i128 = (i128::try_from(s_48_0).unwrap());
        // D s_48_3: cmp-eq s_48_2 s_48_1
        let s_48_3: bool = ((s_48_2) == (s_48_1));
        // D s_48_4: not s_48_3
        let s_48_4: bool = !s_48_3;
        // N s_48_5: branch s_48_4 b50 b49
        if s_48_4 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var v:u64
        let s_49_0: u64 = fn_state.v;
        // C s_49_1: const #23912u : u32
        let s_49_1: u32 = 23912;
        // N s_49_2: write-reg s_49_1 <= s_49_0
        let s_49_2: () = {
            state.write_register::<u64>(s_49_1 as isize, s_49_0);
            tracer.write_register(s_49_1 as isize, s_49_0);
        };
        // N s_49_3: return
        return;
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var n:i64
        let s_50_0: i64 = fn_state.n;
        // C s_50_1: const #25s : i
        let s_50_1: i128 = 25;
        // D s_50_2: cast zx s_50_0 -> i
        let s_50_2: i128 = (i128::try_from(s_50_0).unwrap());
        // D s_50_3: cmp-eq s_50_2 s_50_1
        let s_50_3: bool = ((s_50_2) == (s_50_1));
        // D s_50_4: not s_50_3
        let s_50_4: bool = !s_50_3;
        // N s_50_5: branch s_50_4 b52 b51
        if s_50_4 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var v:u64
        let s_51_0: u64 = fn_state.v;
        // C s_51_1: const #10592u : u32
        let s_51_1: u32 = 10592;
        // N s_51_2: write-reg s_51_1 <= s_51_0
        let s_51_2: () = {
            state.write_register::<u64>(s_51_1 as isize, s_51_0);
            tracer.write_register(s_51_1 as isize, s_51_0);
        };
        // N s_51_3: return
        return;
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var n:i64
        let s_52_0: i64 = fn_state.n;
        // C s_52_1: const #26s : i
        let s_52_1: i128 = 26;
        // D s_52_2: cast zx s_52_0 -> i
        let s_52_2: i128 = (i128::try_from(s_52_0).unwrap());
        // D s_52_3: cmp-eq s_52_2 s_52_1
        let s_52_3: bool = ((s_52_2) == (s_52_1));
        // D s_52_4: not s_52_3
        let s_52_4: bool = !s_52_3;
        // N s_52_5: branch s_52_4 b54 b53
        if s_52_4 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var v:u64
        let s_53_0: u64 = fn_state.v;
        // C s_53_1: const #23096u : u32
        let s_53_1: u32 = 23096;
        // N s_53_2: write-reg s_53_1 <= s_53_0
        let s_53_2: () = {
            state.write_register::<u64>(s_53_1 as isize, s_53_0);
            tracer.write_register(s_53_1 as isize, s_53_0);
        };
        // N s_53_3: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var n:i64
        let s_54_0: i64 = fn_state.n;
        // C s_54_1: const #27s : i
        let s_54_1: i128 = 27;
        // D s_54_2: cast zx s_54_0 -> i
        let s_54_2: i128 = (i128::try_from(s_54_0).unwrap());
        // D s_54_3: cmp-eq s_54_2 s_54_1
        let s_54_3: bool = ((s_54_2) == (s_54_1));
        // D s_54_4: not s_54_3
        let s_54_4: bool = !s_54_3;
        // N s_54_5: branch s_54_4 b56 b55
        if s_54_4 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var v:u64
        let s_55_0: u64 = fn_state.v;
        // C s_55_1: const #21840u : u32
        let s_55_1: u32 = 21840;
        // N s_55_2: write-reg s_55_1 <= s_55_0
        let s_55_2: () = {
            state.write_register::<u64>(s_55_1 as isize, s_55_0);
            tracer.write_register(s_55_1 as isize, s_55_0);
        };
        // N s_55_3: return
        return;
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var n:i64
        let s_56_0: i64 = fn_state.n;
        // C s_56_1: const #28s : i
        let s_56_1: i128 = 28;
        // D s_56_2: cast zx s_56_0 -> i
        let s_56_2: i128 = (i128::try_from(s_56_0).unwrap());
        // D s_56_3: cmp-eq s_56_2 s_56_1
        let s_56_3: bool = ((s_56_2) == (s_56_1));
        // D s_56_4: not s_56_3
        let s_56_4: bool = !s_56_3;
        // N s_56_5: branch s_56_4 b58 b57
        if s_56_4 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var v:u64
        let s_57_0: u64 = fn_state.v;
        // C s_57_1: const #11656u : u32
        let s_57_1: u32 = 11656;
        // N s_57_2: write-reg s_57_1 <= s_57_0
        let s_57_2: () = {
            state.write_register::<u64>(s_57_1 as isize, s_57_0);
            tracer.write_register(s_57_1 as isize, s_57_0);
        };
        // N s_57_3: return
        return;
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var n:i64
        let s_58_0: i64 = fn_state.n;
        // C s_58_1: const #29s : i
        let s_58_1: i128 = 29;
        // D s_58_2: cast zx s_58_0 -> i
        let s_58_2: i128 = (i128::try_from(s_58_0).unwrap());
        // D s_58_3: cmp-eq s_58_2 s_58_1
        let s_58_3: bool = ((s_58_2) == (s_58_1));
        // D s_58_4: not s_58_3
        let s_58_4: bool = !s_58_3;
        // N s_58_5: branch s_58_4 b60 b59
        if s_58_4 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var v:u64
        let s_59_0: u64 = fn_state.v;
        // C s_59_1: const #14768u : u32
        let s_59_1: u32 = 14768;
        // N s_59_2: write-reg s_59_1 <= s_59_0
        let s_59_2: () = {
            state.write_register::<u64>(s_59_1 as isize, s_59_0);
            tracer.write_register(s_59_1 as isize, s_59_0);
        };
        // N s_59_3: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var v:u64
        let s_60_0: u64 = fn_state.v;
        // C s_60_1: const #20472u : u32
        let s_60_1: u32 = 20472;
        // N s_60_2: write-reg s_60_1 <= s_60_0
        let s_60_2: () = {
            state.write_register::<u64>(s_60_1 as isize, s_60_0);
            tracer.write_register(s_60_1 as isize, s_60_0);
        };
        // N s_60_3: return
        return;
    }
}
