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
pub fn V_read<T: Tracer>(state: &mut State, tracer: &T, n: i128, width: i64) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        gs_15477: bool,
        gs_15469: bool,
        widthshadow_266: i64,
        gs_15476: bool,
        gs_15475: bool,
        gs_15478: bool,
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
        // D s_0_3: write-var widthshadow#266 <= s_0_2
        fn_state.widthshadow_266 = s_0_2;
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
        // D s_1_1: write-var gs#15469 <= s_1_0
        fn_state.gs_15469 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_2_0: read-var gs#15469:u8
        let s_2_0: bool = fn_state.gs_15469;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #8s : i
        let s_2_2: i128 = 8;
        // D s_2_3: read-var widthshadow#266:i64
        let s_2_3: i64 = fn_state.widthshadow_266;
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
        // D s_3_1: read-var widthshadow#266:i64
        let s_3_1: i64 = fn_state.widthshadow_266;
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
        // D s_4_1: read-var widthshadow#266:i64
        let s_4_1: i64 = fn_state.widthshadow_266;
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
        // D s_5_1: read-var widthshadow#266:i64
        let s_5_1: i64 = fn_state.widthshadow_266;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_0
        let s_5_3: bool = ((s_5_2) == (s_5_0));
        // N s_5_4: branch s_5_3 b11 b6
        if s_5_3 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_6_0: const #128s : i
        let s_6_0: i128 = 128;
        // D s_6_1: read-var widthshadow#266:i64
        let s_6_1: i64 = fn_state.widthshadow_266;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: cmp-eq s_6_2 s_6_0
        let s_6_3: bool = ((s_6_2) == (s_6_0));
        // D s_6_4: write-var gs#15475 <= s_6_3
        fn_state.gs_15475 = s_6_3;
        // N s_6_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_7_0: read-var gs#15475:u8
        let s_7_0: bool = fn_state.gs_15475;
        // D s_7_1: write-var gs#15476 <= s_7_0
        fn_state.gs_15476 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_8_0: read-var gs#15476:u8
        let s_8_0: bool = fn_state.gs_15476;
        // D s_8_1: write-var gs#15477 <= s_8_0
        fn_state.gs_15477 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_9_0: read-var gs#15477:u8
        let s_9_0: bool = fn_state.gs_15477;
        // D s_9_1: write-var gs#15478 <= s_9_0
        fn_state.gs_15478 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_10_0: read-var gs#15478:u8
        let s_10_0: bool = fn_state.gs_15478;
        // N s_10_1: assert s_10_0
        let s_10_1: () = assert!(s_10_0);
        // C s_10_2: const #1800u : u32
        let s_10_2: u32 = 1800;
        // D s_10_3: read-reg s_10_2:[u2048; 32]
        let s_10_3: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_10_2 as isize);
            tracer.read_register(s_10_2 as isize, value);
            value
        };
        // D s_10_4: read-var n:i
        let s_10_4: i128 = fn_state.n;
        // D s_10_5: read-element s_10_3[s_10_4]
        let s_10_5: u64 = s_10_3[(s_10_4) as usize];
        // C s_10_6: const #1s : i
        let s_10_6: i128 = 1;
        // D s_10_7: read-var widthshadow#266:i64
        let s_10_7: i64 = fn_state.widthshadow_266;
        // D s_10_8: cast zx s_10_7 -> i
        let s_10_8: i128 = (i128::try_from(s_10_7).unwrap());
        // D s_10_9: sub s_10_8 s_10_6
        let s_10_9: i128 = ((s_10_8) - (s_10_6));
        // D s_10_10: cast reint s_10_9 -> i64
        let s_10_10: i64 = (s_10_9 as i64);
        // C s_10_11: const #0s : i
        let s_10_11: i128 = 0;
        // D s_10_12: cast zx s_10_5 -> bv
        let s_10_12: Bits = Bits::new(s_10_5 as u128, 2048u16);
        // D s_10_13: cast zx s_10_10 -> i
        let s_10_13: i128 = (i128::try_from(s_10_10).unwrap());
        // C s_10_14: const #1s : i64
        let s_10_14: i64 = 1;
        // C s_10_15: cast zx s_10_14 -> i
        let s_10_15: i128 = (i128::try_from(s_10_14).unwrap());
        // D s_10_16: sub s_10_13 s_10_11
        let s_10_16: i128 = ((s_10_13) - (s_10_11));
        // D s_10_17: add s_10_16 s_10_15
        let s_10_17: i128 = (s_10_16 + s_10_15);
        // D s_10_18: bit-extract s_10_12 s_10_11 s_10_17
        let s_10_18: Bits = (Bits::new(
            ((s_10_12) >> (s_10_11)).value(),
            u16::try_from(s_10_17).unwrap(),
        ));
        // N s_10_19: return s_10_18
        return s_10_18;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var gs#15475 <= s_11_0
        fn_state.gs_15475 = s_11_0;
        // N s_11_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#15476 <= s_12_0
        fn_state.gs_15476 = s_12_0;
        // N s_12_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#15477 <= s_13_0
        fn_state.gs_15477 = s_13_0;
        // N s_13_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#15478 <= s_14_0
        fn_state.gs_15478 = s_14_0;
        // N s_14_2: jump b10
        return block_10(state, tracer, fn_state);
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
        // D s_15_3: write-var gs#15469 <= s_15_2
        fn_state.gs_15469 = s_15_2;
        // N s_15_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
