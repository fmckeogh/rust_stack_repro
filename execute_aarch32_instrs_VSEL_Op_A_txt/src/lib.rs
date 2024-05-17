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
use CheckVFPEnabled::*;
use ConditionHolds::*;
use D_set::*;
use Zeros::*;
use S_set::*;
use S_read::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VSEL_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    d: i64,
    esize: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_366145: u64,
        ga_366143: u32,
        ga_366138: u32,
        ga_366139: u16,
        cond: u8,
        d: i64,
        esize: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        cond,
        d,
        esize,
        m,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1u : u8
        let s_0_0: bool = true;
        // S s_0_1: call CheckVFPEnabled(s_0_0)
        let s_0_1: () = CheckVFPEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var esize:i64
        let s_1_0: i64 = fn_state.esize;
        // C s_1_1: const #16s : i
        let s_1_1: i128 = 16;
        // D s_1_2: cast zx s_1_0 -> i
        let s_1_2: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_3: cmp-eq s_1_2 s_1_1
        let s_1_3: bool = ((s_1_2) == (s_1_1));
        // D s_1_4: not s_1_3
        let s_1_4: bool = !s_1_3;
        // N s_1_5: branch s_1_4 b6 b2
        if s_1_4 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #16s : i
        let s_2_0: i128 = 16;
        // S s_2_1: call Zeros(s_2_0)
        let s_2_1: Bits = Zeros(state, tracer, s_2_0);
        // S s_2_2: cast reint s_2_1 -> u16
        let s_2_2: u16 = (s_2_1.value() as u16);
        // D s_2_3: write-var ga#366139 <= s_2_2
        fn_state.ga_366139 = s_2_2;
        // D s_2_4: read-var cond:u8
        let s_2_4: u8 = fn_state.cond;
        // D s_2_5: call ConditionHolds(s_2_4)
        let s_2_5: bool = ConditionHolds(state, tracer, s_2_4);
        // N s_2_6: branch s_2_5 b5 b3
        if s_2_5 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var m:i64
        let s_3_0: i64 = fn_state.m;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: call S_read(s_3_1)
        let s_3_2: u32 = S_read(state, tracer, s_3_1);
        // D s_3_3: write-var ga#366138 <= s_3_2
        fn_state.ga_366138 = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0s : i
        let s_4_0: i128 = 0;
        // D s_4_1: read-var ga#366138:u32
        let s_4_1: u32 = fn_state.ga_366138;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 32u16);
        // C s_4_3: const #1s : i64
        let s_4_3: i64 = 1;
        // C s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // C s_4_5: const #15s : i
        let s_4_5: i128 = 15;
        // C s_4_6: add s_4_5 s_4_4
        let s_4_6: i128 = (s_4_5 + s_4_4);
        // D s_4_7: bit-extract s_4_2 s_4_0 s_4_6
        let s_4_7: Bits = (Bits::new(
            ((s_4_2) >> (s_4_0)).value(),
            u16::try_from(s_4_6).unwrap(),
        ));
        // D s_4_8: cast reint s_4_7 -> u16
        let s_4_8: u16 = (s_4_7.value() as u16);
        // D s_4_9: read-var ga#366139:u16
        let s_4_9: u16 = fn_state.ga_366139;
        // D s_4_10: cast zx s_4_9 -> bv
        let s_4_10: Bits = Bits::new(s_4_9 as u128, 16u16);
        // D s_4_11: cast zx s_4_8 -> bv
        let s_4_11: Bits = Bits::new(s_4_8 as u128, 16u16);
        // D s_4_12: cast reint s_4_10 -> u128
        let s_4_12: u128 = (s_4_10.value() as u128);
        // D s_4_13: size-of s_4_10
        let s_4_13: u16 = s_4_10.length();
        // D s_4_14: cast reint s_4_11 -> u128
        let s_4_14: u128 = (s_4_11.value() as u128);
        // D s_4_15: size-of s_4_11
        let s_4_15: u16 = s_4_11.length();
        // D s_4_16: lsl s_4_12 s_4_15
        let s_4_16: u128 = s_4_12 << s_4_15;
        // D s_4_17: or s_4_16 s_4_14
        let s_4_17: u128 = ((s_4_16) | (s_4_14));
        // D s_4_18: add s_4_13 s_4_15
        let s_4_18: u16 = (s_4_13 + s_4_15);
        // D s_4_19: create-bits s_4_17 s_4_18
        let s_4_19: Bits = Bits::new(s_4_17, s_4_18);
        // D s_4_20: cast reint s_4_19 -> u32
        let s_4_20: u32 = (s_4_19.value() as u32);
        // D s_4_21: read-var d:i64
        let s_4_21: i64 = fn_state.d;
        // D s_4_22: cast zx s_4_21 -> i
        let s_4_22: i128 = (i128::try_from(s_4_21).unwrap());
        // D s_4_23: call S_set(s_4_22, s_4_20)
        let s_4_23: () = S_set(state, tracer, s_4_22, s_4_20);
        // N s_4_24: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var n:i64
        let s_5_0: i64 = fn_state.n;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: call S_read(s_5_1)
        let s_5_2: u32 = S_read(state, tracer, s_5_1);
        // D s_5_3: write-var ga#366138 <= s_5_2
        fn_state.ga_366138 = s_5_2;
        // N s_5_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var esize:i64
        let s_6_0: i64 = fn_state.esize;
        // C s_6_1: const #32s : i
        let s_6_1: i128 = 32;
        // D s_6_2: cast zx s_6_0 -> i
        let s_6_2: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_3: cmp-eq s_6_2 s_6_1
        let s_6_3: bool = ((s_6_2) == (s_6_1));
        // D s_6_4: not s_6_3
        let s_6_4: bool = !s_6_3;
        // N s_6_5: branch s_6_4 b11 b7
        if s_6_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var cond:u8
        let s_7_0: u8 = fn_state.cond;
        // D s_7_1: call ConditionHolds(s_7_0)
        let s_7_1: bool = ConditionHolds(state, tracer, s_7_0);
        // N s_7_2: branch s_7_1 b10 b8
        if s_7_1 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var m:i64
        let s_8_0: i64 = fn_state.m;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: call S_read(s_8_1)
        let s_8_2: u32 = S_read(state, tracer, s_8_1);
        // D s_8_3: write-var ga#366143 <= s_8_2
        fn_state.ga_366143 = s_8_2;
        // N s_8_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var d:i64
        let s_9_0: i64 = fn_state.d;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: read-var ga#366143:u32
        let s_9_2: u32 = fn_state.ga_366143;
        // D s_9_3: call S_set(s_9_1, s_9_2)
        let s_9_3: () = S_set(state, tracer, s_9_1, s_9_2);
        // N s_9_4: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var n:i64
        let s_10_0: i64 = fn_state.n;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: call S_read(s_10_1)
        let s_10_2: u32 = S_read(state, tracer, s_10_1);
        // D s_10_3: write-var ga#366143 <= s_10_2
        fn_state.ga_366143 = s_10_2;
        // N s_10_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var esize:i64
        let s_11_0: i64 = fn_state.esize;
        // C s_11_1: const #64s : i
        let s_11_1: i128 = 64;
        // D s_11_2: cast zx s_11_0 -> i
        let s_11_2: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_3: cmp-eq s_11_2 s_11_1
        let s_11_3: bool = ((s_11_2) == (s_11_1));
        // D s_11_4: not s_11_3
        let s_11_4: bool = !s_11_3;
        // N s_11_5: branch s_11_4 b16 b12
        if s_11_4 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var cond:u8
        let s_12_0: u8 = fn_state.cond;
        // D s_12_1: call ConditionHolds(s_12_0)
        let s_12_1: bool = ConditionHolds(state, tracer, s_12_0);
        // N s_12_2: branch s_12_1 b15 b13
        if s_12_1 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var m:i64
        let s_13_0: i64 = fn_state.m;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: call D_read(s_13_1)
        let s_13_2: u64 = D_read(state, tracer, s_13_1);
        // D s_13_3: write-var ga#366145 <= s_13_2
        fn_state.ga_366145 = s_13_2;
        // N s_13_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var d:i64
        let s_14_0: i64 = fn_state.d;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: read-var ga#366145:u64
        let s_14_2: u64 = fn_state.ga_366145;
        // D s_14_3: call D_set(s_14_1, s_14_2)
        let s_14_3: () = D_set(state, tracer, s_14_1, s_14_2);
        // N s_14_4: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var n:i64
        let s_15_0: i64 = fn_state.n;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: call D_read(s_15_1)
        let s_15_2: u64 = D_read(state, tracer, s_15_1);
        // D s_15_3: write-var ga#366145 <= s_15_2
        fn_state.ga_366145 = s_15_2;
        // N s_15_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: return
        return;
    }
}
