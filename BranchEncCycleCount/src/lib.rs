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
use BRBCycleCountingEnabled::*;
use FirstBranchAfterProhibited::*;
use Ones::*;
use integer_subrange::*;
use BranchRawCycleCount::*;
use Zeros::*;
use fdiv_int::*;
use common::*;
pub fn BranchEncCycleCount<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_3247: (),
) -> ProductType1a93b8c16f53fb84 {
    #[derive(Default)]
    struct FunctionState {
        gs_3248: bool,
        E: u8,
        return_value: ProductType1a93b8c16f53fb84,
        M: u8,
        cc: i128,
        gs_3247: (),
    }
    let fn_state = FunctionState {
        gs_3247,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1a93b8c16f53fb84 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call BranchRawCycleCount(s_0_0)
        let s_0_1: i128 = BranchRawCycleCount(state, tracer, s_0_0);
        // D s_0_2: write-var cc <= s_0_1
        fn_state.cc = s_0_1;
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call BRBCycleCountingEnabled(s_0_3)
        let s_0_4: bool = BRBCycleCountingEnabled(state, tracer, s_0_3);
        // S s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b14 b1
        if s_0_5 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1a93b8c16f53fb84 {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call FirstBranchAfterProhibited(s_1_0)
        let s_1_1: bool = FirstBranchAfterProhibited(state, tracer, s_1_0);
        // D s_1_2: write-var gs#3248 <= s_1_1
        fn_state.gs_3248 = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1a93b8c16f53fb84 {
        // D s_2_0: read-var gs#3248:u8
        let s_2_0: bool = fn_state.gs_3248;
        // N s_2_1: branch s_2_0 b13 b3
        if s_2_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1a93b8c16f53fb84 {
        // C s_3_0: const #8s : i
        let s_3_0: i128 = 8;
        // C s_3_1: pow2 s_3_0
        let s_3_1: i128 = (s_3_0).pow(2);
        // C s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // C s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: read-var cc:i
        let s_3_4: i128 = fn_state.cc;
        // D s_3_5: cmp-lt s_3_4 s_3_3
        let s_3_5: bool = ((s_3_4) < (s_3_3));
        // N s_3_6: branch s_3_5 b12 b4
        if s_3_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1a93b8c16f53fb84 {
        // C s_4_0: const #20s : i
        let s_4_0: i128 = 20;
        // C s_4_1: pow2 s_4_0
        let s_4_1: i128 = (s_4_0).pow(2);
        // C s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // C s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: read-var cc:i
        let s_4_4: i128 = fn_state.cc;
        // D s_4_5: cmp-ge s_4_4 s_4_3
        let s_4_5: bool = ((s_4_4) >= (s_4_3));
        // N s_4_6: branch s_4_5 b11 b5
        if s_4_5 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1a93b8c16f53fb84 {
        // C s_5_0: const #1s : i
        let s_5_0: i128 = 1;
        // C s_5_1: const #5s : i
        let s_5_1: i128 = 5;
        // C s_5_2: const #0s : i
        let s_5_2: i128 = 0;
        // S s_5_3: call integer_subrange(s_5_0, s_5_1, s_5_2)
        let s_5_3: Bits = integer_subrange(state, tracer, s_5_0, s_5_1, s_5_2);
        // S s_5_4: cast reint s_5_3 -> u8
        let s_5_4: u8 = (s_5_3.value() as u8);
        // D s_5_5: write-var E <= s_5_4
        fn_state.E = s_5_4;
        // N s_5_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1a93b8c16f53fb84 {
        // C s_6_0: const #9s : i
        let s_6_0: i128 = 9;
        // C s_6_1: pow2 s_6_0
        let s_6_1: i128 = (s_6_0).pow(2);
        // C s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // C s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: read-var cc:i
        let s_6_4: i128 = fn_state.cc;
        // D s_6_5: cmp-ge s_6_4 s_6_3
        let s_6_5: bool = ((s_6_4) >= (s_6_3));
        // D s_6_6: not s_6_5
        let s_6_6: bool = !s_6_5;
        // N s_6_7: branch s_6_6 b8 b7
        if s_6_6 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1a93b8c16f53fb84 {
        // C s_7_0: const #1s : i
        let s_7_0: i128 = 1;
        // D s_7_1: read-var E:u8
        let s_7_1: u8 = fn_state.E;
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 6u16);
        // C s_7_3: cast cvt s_7_0 -> bv
        let s_7_3: Bits = Bits::new(s_7_0 as u128, 128);
        // D s_7_4: add s_7_2 s_7_3
        let s_7_4: Bits = (s_7_2 + s_7_3);
        // D s_7_5: cast reint s_7_4 -> u8
        let s_7_5: u8 = (s_7_4.value() as u8);
        // D s_7_6: write-var E <= s_7_5
        fn_state.E = s_7_5;
        // C s_7_7: const #2s : i
        let s_7_7: i128 = 2;
        // D s_7_8: read-var cc:i
        let s_7_8: i128 = fn_state.cc;
        // D s_7_9: call fdiv_int(s_7_8, s_7_7)
        let s_7_9: i128 = fdiv_int(state, tracer, s_7_8, s_7_7);
        // D s_7_10: write-var cc <= s_7_9
        fn_state.cc = s_7_9;
        // N s_7_11: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1a93b8c16f53fb84 {
        // D s_8_0: read-var cc:i
        let s_8_0: i128 = fn_state.cc;
        // C s_8_1: const #7s : i
        let s_8_1: i128 = 7;
        // C s_8_2: const #0s : i
        let s_8_2: i128 = 0;
        // D s_8_3: call integer_subrange(s_8_0, s_8_1, s_8_2)
        let s_8_3: Bits = integer_subrange(state, tracer, s_8_0, s_8_1, s_8_2);
        // D s_8_4: cast reint s_8_3 -> u8
        let s_8_4: u8 = (s_8_3.value() as u8);
        // D s_8_5: write-var M <= s_8_4
        fn_state.M = s_8_4;
        // N s_8_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1a93b8c16f53fb84 {
        // D s_9_0: read-var E:u8
        let s_9_0: u8 = fn_state.E;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 6u16);
        // D s_9_2: read-var M:u8
        let s_9_2: u8 = fn_state.M;
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 8u16);
        // D s_9_4: cast reint s_9_1 -> u128
        let s_9_4: u128 = (s_9_1.value() as u128);
        // D s_9_5: size-of s_9_1
        let s_9_5: u16 = s_9_1.length();
        // D s_9_6: cast reint s_9_3 -> u128
        let s_9_6: u128 = (s_9_3.value() as u128);
        // D s_9_7: size-of s_9_3
        let s_9_7: u16 = s_9_3.length();
        // D s_9_8: lsl s_9_4 s_9_7
        let s_9_8: u128 = s_9_4 << s_9_7;
        // D s_9_9: or s_9_8 s_9_6
        let s_9_9: u128 = ((s_9_8) | (s_9_6));
        // D s_9_10: add s_9_5 s_9_7
        let s_9_10: u16 = (s_9_5 + s_9_7);
        // D s_9_11: create-bits s_9_9 s_9_10
        let s_9_11: Bits = Bits::new(s_9_9, s_9_10);
        // D s_9_12: cast reint s_9_11 -> u14
        let s_9_12: u16 = (s_9_11.value() as u16);
        // C s_9_13: const #0u : u8
        let s_9_13: bool = false;
        // D s_9_14: create-product struct = ["s_9_13", "s_9_12"]
        let s_9_14: ProductType1a93b8c16f53fb84 = ProductType1a93b8c16f53fb84 {
            _0: s_9_13,
            _1: s_9_12,
        };
        // D s_9_15: write-var return_value <= s_9_14
        fn_state.return_value = s_9_14;
        // N s_9_16: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1a93b8c16f53fb84 {
        // D s_10_0: read-var return_value:struct
        let s_10_0: ProductType1a93b8c16f53fb84 = fn_state.return_value;
        // N s_10_1: return s_10_0
        return s_10_0;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1a93b8c16f53fb84 {
        // C s_11_0: const #6s : i
        let s_11_0: i128 = 6;
        // S s_11_1: call Ones(s_11_0)
        let s_11_1: Bits = Ones(state, tracer, s_11_0);
        // S s_11_2: cast reint s_11_1 -> u8
        let s_11_2: u8 = (s_11_1.value() as u8);
        // D s_11_3: write-var E <= s_11_2
        fn_state.E = s_11_2;
        // C s_11_4: const #8s : i
        let s_11_4: i128 = 8;
        // S s_11_5: call Ones(s_11_4)
        let s_11_5: Bits = Ones(state, tracer, s_11_4);
        // S s_11_6: cast reint s_11_5 -> u8
        let s_11_6: u8 = (s_11_5.value() as u8);
        // D s_11_7: write-var M <= s_11_6
        fn_state.M = s_11_6;
        // N s_11_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1a93b8c16f53fb84 {
        // C s_12_0: const #6s : i
        let s_12_0: i128 = 6;
        // S s_12_1: call Zeros(s_12_0)
        let s_12_1: Bits = Zeros(state, tracer, s_12_0);
        // S s_12_2: cast reint s_12_1 -> u8
        let s_12_2: u8 = (s_12_1.value() as u8);
        // D s_12_3: write-var E <= s_12_2
        fn_state.E = s_12_2;
        // C s_12_4: const #7s : i
        let s_12_4: i128 = 7;
        // C s_12_5: const #0s : i
        let s_12_5: i128 = 0;
        // D s_12_6: read-var cc:i
        let s_12_6: i128 = fn_state.cc;
        // D s_12_7: call integer_subrange(s_12_6, s_12_4, s_12_5)
        let s_12_7: Bits = integer_subrange(state, tracer, s_12_6, s_12_4, s_12_5);
        // D s_12_8: cast reint s_12_7 -> u8
        let s_12_8: u8 = (s_12_7.value() as u8);
        // D s_12_9: write-var M <= s_12_8
        fn_state.M = s_12_8;
        // N s_12_10: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1a93b8c16f53fb84 {
        // C s_13_0: const #14s : i
        let s_13_0: i128 = 14;
        // S s_13_1: call Zeros(s_13_0)
        let s_13_1: Bits = Zeros(state, tracer, s_13_0);
        // S s_13_2: cast reint s_13_1 -> u14
        let s_13_2: u16 = (s_13_1.value() as u16);
        // C s_13_3: const #1u : u8
        let s_13_3: bool = true;
        // D s_13_4: create-product struct = ["s_13_3", "s_13_2"]
        let s_13_4: ProductType1a93b8c16f53fb84 = ProductType1a93b8c16f53fb84 {
            _0: s_13_3,
            _1: s_13_2,
        };
        // D s_13_5: write-var return_value <= s_13_4
        fn_state.return_value = s_13_4;
        // N s_13_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1a93b8c16f53fb84 {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#3248 <= s_14_0
        fn_state.gs_3248 = s_14_0;
        // N s_14_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
