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
use neq_int::*;
use get_R::*;
use Zeros::*;
use common::*;
pub fn X_read<T: Tracer>(state: &mut State, tracer: &T, n: i128, width: i64) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        return_value: Bits,
        widthshadow_265: i64,
        gs_15459: bool,
        gs_15461: bool,
        gs_15460: bool,
        gs_15454: bool,
        n: i128,
        width: i64,
    }
    let fn_state = FunctionState {
        n,
        width,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_0_0: read-var width:i64
        let s_0_0: i64 = fn_state.width;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var widthshadow#265 <= s_0_2
        fn_state.widthshadow_265 = s_0_2;
        // C s_0_4: const #0s : i
        let s_0_4: i128 = 0;
        // D s_0_5: read-var n:i
        let s_0_5: i128 = fn_state.n;
        // D s_0_6: cmp-ge s_0_5 s_0_4
        let s_0_6: bool = ((s_0_5) >= (s_0_4));
        // N s_0_7: branch s_0_6 b15 b1
        if s_0_6 {
            return block_15(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#15454 <= s_1_0
        fn_state.gs_15454 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_2_0: read-var gs#15454:u8
        let s_2_0: bool = fn_state.gs_15454;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #8s : i
        let s_2_2: i128 = 8;
        // D s_2_3: read-var widthshadow#265:i64
        let s_2_3: i64 = fn_state.widthshadow_265;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: cmp-eq s_2_4 s_2_2
        let s_2_5: bool = ((s_2_4) == (s_2_2));
        // N s_2_6: branch s_2_5 b14 b3
        if s_2_5 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_3_0: const #16s : i
        let s_3_0: i128 = 16;
        // D s_3_1: read-var widthshadow#265:i64
        let s_3_1: i64 = fn_state.widthshadow_265;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) == (s_3_0));
        // N s_3_4: branch s_3_3 b13 b4
        if s_3_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_4_0: const #32s : i
        let s_4_0: i128 = 32;
        // D s_4_1: read-var widthshadow#265:i64
        let s_4_1: i64 = fn_state.widthshadow_265;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: cmp-eq s_4_2 s_4_0
        let s_4_3: bool = ((s_4_2) == (s_4_0));
        // N s_4_4: branch s_4_3 b12 b5
        if s_4_3 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_5_0: const #64s : i
        let s_5_0: i128 = 64;
        // D s_5_1: read-var widthshadow#265:i64
        let s_5_1: i64 = fn_state.widthshadow_265;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_0
        let s_5_3: bool = ((s_5_2) == (s_5_0));
        // D s_5_4: write-var gs#15459 <= s_5_3
        fn_state.gs_15459 = s_5_3;
        // N s_5_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_6_0: read-var gs#15459:u8
        let s_6_0: bool = fn_state.gs_15459;
        // D s_6_1: write-var gs#15460 <= s_6_0
        fn_state.gs_15460 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_7_0: read-var gs#15460:u8
        let s_7_0: bool = fn_state.gs_15460;
        // D s_7_1: write-var gs#15461 <= s_7_0
        fn_state.gs_15461 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_8_0: read-var gs#15461:u8
        let s_8_0: bool = fn_state.gs_15461;
        // N s_8_1: assert s_8_0
        let s_8_1: () = assert!(s_8_0);
        // C s_8_2: const #31s : i
        let s_8_2: i128 = 31;
        // D s_8_3: read-var n:i
        let s_8_3: i128 = fn_state.n;
        // D s_8_4: call neq_int(s_8_3, s_8_2)
        let s_8_4: bool = neq_int(state, tracer, s_8_3, s_8_2);
        // N s_8_5: branch s_8_4 b11 b9
        if s_8_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_9_0: read-var widthshadow#265:i64
        let s_9_0: i64 = fn_state.widthshadow_265;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: call Zeros(s_9_1)
        let s_9_2: Bits = Zeros(state, tracer, s_9_1);
        // D s_9_3: write-var return_value <= s_9_2
        fn_state.return_value = s_9_2;
        // N s_9_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_10_0: read-var return_value:bv
        let s_10_0: Bits = fn_state.return_value;
        // N s_10_1: return s_10_0
        return s_10_0;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_11_0: read-var n:i
        let s_11_0: i128 = fn_state.n;
        // D s_11_1: cast reint s_11_0 -> i64
        let s_11_1: i64 = (s_11_0 as i64);
        // D s_11_2: call get_R(s_11_1)
        let s_11_2: u64 = get_R(state, tracer, s_11_1);
        // C s_11_3: const #1s : i
        let s_11_3: i128 = 1;
        // D s_11_4: read-var widthshadow#265:i64
        let s_11_4: i64 = fn_state.widthshadow_265;
        // D s_11_5: cast zx s_11_4 -> i
        let s_11_5: i128 = (i128::try_from(s_11_4).unwrap());
        // D s_11_6: sub s_11_5 s_11_3
        let s_11_6: i128 = ((s_11_5) - (s_11_3));
        // D s_11_7: cast reint s_11_6 -> i64
        let s_11_7: i64 = (s_11_6 as i64);
        // C s_11_8: const #0s : i
        let s_11_8: i128 = 0;
        // D s_11_9: cast zx s_11_2 -> bv
        let s_11_9: Bits = Bits::new(s_11_2 as u128, 64u16);
        // D s_11_10: cast zx s_11_7 -> i
        let s_11_10: i128 = (i128::try_from(s_11_7).unwrap());
        // C s_11_11: const #1s : i64
        let s_11_11: i64 = 1;
        // C s_11_12: cast zx s_11_11 -> i
        let s_11_12: i128 = (i128::try_from(s_11_11).unwrap());
        // D s_11_13: sub s_11_10 s_11_8
        let s_11_13: i128 = ((s_11_10) - (s_11_8));
        // D s_11_14: add s_11_13 s_11_12
        let s_11_14: i128 = (s_11_13 + s_11_12);
        // D s_11_15: bit-extract s_11_9 s_11_8 s_11_14
        let s_11_15: Bits = (Bits::new(
            ((s_11_9) >> (s_11_8)).value(),
            u16::try_from(s_11_14).unwrap(),
        ));
        // D s_11_16: write-var return_value <= s_11_15
        fn_state.return_value = s_11_15;
        // N s_11_17: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#15459 <= s_12_0
        fn_state.gs_15459 = s_12_0;
        // N s_12_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#15460 <= s_13_0
        fn_state.gs_15460 = s_13_0;
        // N s_13_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#15461 <= s_14_0
        fn_state.gs_15461 = s_14_0;
        // N s_14_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_15_0: const #31s : i
        let s_15_0: i128 = 31;
        // D s_15_1: read-var n:i
        let s_15_1: i128 = fn_state.n;
        // D s_15_2: cmp-le s_15_1 s_15_0
        let s_15_2: bool = ((s_15_1) <= (s_15_0));
        // D s_15_3: write-var gs#15454 <= s_15_2
        fn_state.gs_15454 = s_15_2;
        // N s_15_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
