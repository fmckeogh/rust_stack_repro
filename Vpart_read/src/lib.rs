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
use u__id::*;
use V_read::*;
use common::*;
pub fn Vpart_read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i128,
    part: i128,
    width: i128,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        gs_15488: bool,
        widthshadow_267: i128,
        return_value: Bits,
        gs_15504: bool,
        gs_15508: bool,
        gs_15485: bool,
        gs_15502: bool,
        gs_15494: bool,
        gs_15506: bool,
        n: i128,
        part: i128,
        width: i128,
    }
    let fn_state = FunctionState {
        n,
        part,
        width,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_0_0: read-var width:i
        let s_0_0: i128 = fn_state.width;
        // D s_0_1: write-var widthshadow#267 <= s_0_0
        fn_state.widthshadow_267 = s_0_0;
        // C s_0_2: const #0s : i
        let s_0_2: i128 = 0;
        // D s_0_3: read-var n:i
        let s_0_3: i128 = fn_state.n;
        // D s_0_4: cmp-ge s_0_3 s_0_2
        let s_0_4: bool = ((s_0_3) >= (s_0_2));
        // N s_0_5: branch s_0_4 b24 b1
        if s_0_4 {
            return block_24(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#15485 <= s_1_0
        fn_state.gs_15485 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_2_0: read-var gs#15485:u8
        let s_2_0: bool = fn_state.gs_15485;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #0s : i
        let s_2_2: i128 = 0;
        // D s_2_3: read-var part:i
        let s_2_3: i128 = fn_state.part;
        // D s_2_4: cmp-eq s_2_3 s_2_2
        let s_2_4: bool = ((s_2_3) == (s_2_2));
        // N s_2_5: branch s_2_4 b23 b3
        if s_2_4 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_3_0: const #1s : i
        let s_3_0: i128 = 1;
        // D s_3_1: read-var part:i
        let s_3_1: i128 = fn_state.part;
        // D s_3_2: cmp-eq s_3_1 s_3_0
        let s_3_2: bool = ((s_3_1) == (s_3_0));
        // D s_3_3: write-var gs#15488 <= s_3_2
        fn_state.gs_15488 = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_4_0: read-var gs#15488:u8
        let s_4_0: bool = fn_state.gs_15488;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #0s : i
        let s_4_2: i128 = 0;
        // D s_4_3: read-var part:i
        let s_4_3: i128 = fn_state.part;
        // D s_4_4: cmp-eq s_4_3 s_4_2
        let s_4_4: bool = ((s_4_3) == (s_4_2));
        // N s_4_5: branch s_4_4 b10 b5
        if s_4_4 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_5_0: const #32s : i
        let s_5_0: i128 = 32;
        // D s_5_1: read-var widthshadow#267:i
        let s_5_1: i128 = fn_state.widthshadow_267;
        // D s_5_2: cmp-eq s_5_1 s_5_0
        let s_5_2: bool = ((s_5_1) == (s_5_0));
        // N s_5_3: branch s_5_2 b9 b6
        if s_5_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_6_0: const #64s : i
        let s_6_0: i128 = 64;
        // D s_6_1: read-var widthshadow#267:i
        let s_6_1: i128 = fn_state.widthshadow_267;
        // D s_6_2: cmp-eq s_6_1 s_6_0
        let s_6_2: bool = ((s_6_1) == (s_6_0));
        // D s_6_3: write-var gs#15494 <= s_6_2
        fn_state.gs_15494 = s_6_2;
        // N s_6_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_7_0: read-var gs#15494:u8
        let s_7_0: bool = fn_state.gs_15494;
        // N s_7_1: assert s_7_0
        let s_7_1: () = assert!(s_7_0);
        // C s_7_2: const #128s : i64
        let s_7_2: i64 = 128;
        // D s_7_3: read-var n:i
        let s_7_3: i128 = fn_state.n;
        // D s_7_4: call V_read(s_7_3, s_7_2)
        let s_7_4: Bits = V_read(state, tracer, s_7_3, s_7_2);
        // D s_7_5: cast reint s_7_4 -> u128
        let s_7_5: u128 = (s_7_4.value() as u128);
        // C s_7_6: const #2s : i
        let s_7_6: i128 = 2;
        // D s_7_7: read-var widthshadow#267:i
        let s_7_7: i128 = fn_state.widthshadow_267;
        // D s_7_8: mul s_7_7 s_7_6
        let s_7_8: i128 = ((s_7_7) * (s_7_6));
        // D s_7_9: cast reint s_7_8 -> i64
        let s_7_9: i64 = (s_7_8 as i64);
        // C s_7_10: const #1s : i
        let s_7_10: i128 = 1;
        // D s_7_11: cast zx s_7_9 -> i
        let s_7_11: i128 = (i128::try_from(s_7_9).unwrap());
        // D s_7_12: sub s_7_11 s_7_10
        let s_7_12: i128 = ((s_7_11) - (s_7_10));
        // D s_7_13: cast reint s_7_12 -> i64
        let s_7_13: i64 = (s_7_12 as i64);
        // D s_7_14: cast zx s_7_5 -> bv
        let s_7_14: Bits = Bits::new(s_7_5 as u128, 128u16);
        // D s_7_15: cast zx s_7_13 -> i
        let s_7_15: i128 = (i128::try_from(s_7_13).unwrap());
        // D s_7_16: read-var widthshadow#267:i
        let s_7_16: i128 = fn_state.widthshadow_267;
        // C s_7_17: const #1s : i64
        let s_7_17: i64 = 1;
        // C s_7_18: cast zx s_7_17 -> i
        let s_7_18: i128 = (i128::try_from(s_7_17).unwrap());
        // D s_7_19: sub s_7_15 s_7_16
        let s_7_19: i128 = ((s_7_15) - (s_7_16));
        // D s_7_20: add s_7_19 s_7_18
        let s_7_20: i128 = (s_7_19 + s_7_18);
        // D s_7_21: bit-extract s_7_14 s_7_16 s_7_20
        let s_7_21: Bits = (Bits::new(
            ((s_7_14) >> (s_7_16)).value(),
            u16::try_from(s_7_20).unwrap(),
        ));
        // D s_7_22: write-var return_value <= s_7_21
        fn_state.return_value = s_7_21;
        // N s_7_23: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_8_0: read-var return_value:bv
        let s_8_0: Bits = fn_state.return_value;
        // N s_8_1: return s_8_0
        return s_8_0;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#15494 <= s_9_0
        fn_state.gs_15494 = s_9_0;
        // N s_9_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_10_0: const #128s : i
        let s_10_0: i128 = 128;
        // D s_10_1: read-var widthshadow#267:i
        let s_10_1: i128 = fn_state.widthshadow_267;
        // D s_10_2: cmp-lt s_10_1 s_10_0
        let s_10_2: bool = ((s_10_1) < (s_10_0));
        // N s_10_3: assert s_10_2
        let s_10_3: () = assert!(s_10_2);
        // D s_10_4: read-var widthshadow#267:i
        let s_10_4: i128 = fn_state.widthshadow_267;
        // D s_10_5: call __id(s_10_4)
        let s_10_5: i128 = u__id(state, tracer, s_10_4);
        // D s_10_6: cast reint s_10_5 -> i64
        let s_10_6: i64 = (s_10_5 as i64);
        // C s_10_7: const #8s : i
        let s_10_7: i128 = 8;
        // D s_10_8: cast zx s_10_6 -> i
        let s_10_8: i128 = (i128::try_from(s_10_6).unwrap());
        // D s_10_9: cmp-eq s_10_8 s_10_7
        let s_10_9: bool = ((s_10_8) == (s_10_7));
        // N s_10_10: branch s_10_9 b22 b11
        if s_10_9 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_11_0: read-var widthshadow#267:i
        let s_11_0: i128 = fn_state.widthshadow_267;
        // D s_11_1: call __id(s_11_0)
        let s_11_1: i128 = u__id(state, tracer, s_11_0);
        // D s_11_2: cast reint s_11_1 -> i64
        let s_11_2: i64 = (s_11_1 as i64);
        // C s_11_3: const #16s : i
        let s_11_3: i128 = 16;
        // D s_11_4: cast zx s_11_2 -> i
        let s_11_4: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_5: cmp-eq s_11_4 s_11_3
        let s_11_5: bool = ((s_11_4) == (s_11_3));
        // D s_11_6: write-var gs#15502 <= s_11_5
        fn_state.gs_15502 = s_11_5;
        // N s_11_7: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_12_0: read-var gs#15502:u8
        let s_12_0: bool = fn_state.gs_15502;
        // N s_12_1: branch s_12_0 b21 b13
        if s_12_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_13_0: read-var widthshadow#267:i
        let s_13_0: i128 = fn_state.widthshadow_267;
        // D s_13_1: call __id(s_13_0)
        let s_13_1: i128 = u__id(state, tracer, s_13_0);
        // D s_13_2: cast reint s_13_1 -> i64
        let s_13_2: i64 = (s_13_1 as i64);
        // C s_13_3: const #32s : i
        let s_13_3: i128 = 32;
        // D s_13_4: cast zx s_13_2 -> i
        let s_13_4: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_5: cmp-eq s_13_4 s_13_3
        let s_13_5: bool = ((s_13_4) == (s_13_3));
        // D s_13_6: write-var gs#15504 <= s_13_5
        fn_state.gs_15504 = s_13_5;
        // N s_13_7: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_14_0: read-var gs#15504:u8
        let s_14_0: bool = fn_state.gs_15504;
        // N s_14_1: branch s_14_0 b20 b15
        if s_14_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_15_0: read-var widthshadow#267:i
        let s_15_0: i128 = fn_state.widthshadow_267;
        // D s_15_1: call __id(s_15_0)
        let s_15_1: i128 = u__id(state, tracer, s_15_0);
        // D s_15_2: cast reint s_15_1 -> i64
        let s_15_2: i64 = (s_15_1 as i64);
        // C s_15_3: const #64s : i
        let s_15_3: i128 = 64;
        // D s_15_4: cast zx s_15_2 -> i
        let s_15_4: i128 = (i128::try_from(s_15_2).unwrap());
        // D s_15_5: cmp-eq s_15_4 s_15_3
        let s_15_5: bool = ((s_15_4) == (s_15_3));
        // D s_15_6: write-var gs#15506 <= s_15_5
        fn_state.gs_15506 = s_15_5;
        // N s_15_7: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_16_0: read-var gs#15506:u8
        let s_16_0: bool = fn_state.gs_15506;
        // N s_16_1: branch s_16_0 b19 b17
        if s_16_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_17_0: read-var widthshadow#267:i
        let s_17_0: i128 = fn_state.widthshadow_267;
        // D s_17_1: call __id(s_17_0)
        let s_17_1: i128 = u__id(state, tracer, s_17_0);
        // D s_17_2: cast reint s_17_1 -> i64
        let s_17_2: i64 = (s_17_1 as i64);
        // C s_17_3: const #128s : i
        let s_17_3: i128 = 128;
        // D s_17_4: cast zx s_17_2 -> i
        let s_17_4: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_5: cmp-eq s_17_4 s_17_3
        let s_17_5: bool = ((s_17_4) == (s_17_3));
        // D s_17_6: write-var gs#15508 <= s_17_5
        fn_state.gs_15508 = s_17_5;
        // N s_17_7: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_18_0: read-var gs#15508:u8
        let s_18_0: bool = fn_state.gs_15508;
        // N s_18_1: assert s_18_0
        let s_18_1: () = assert!(s_18_0);
        // D s_18_2: read-var widthshadow#267:i
        let s_18_2: i128 = fn_state.widthshadow_267;
        // D s_18_3: cast reint s_18_2 -> i64
        let s_18_3: i64 = (s_18_2 as i64);
        // D s_18_4: read-var n:i
        let s_18_4: i128 = fn_state.n;
        // D s_18_5: call V_read(s_18_4, s_18_3)
        let s_18_5: Bits = V_read(state, tracer, s_18_4, s_18_3);
        // D s_18_6: write-var return_value <= s_18_5
        fn_state.return_value = s_18_5;
        // N s_18_7: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#15508 <= s_19_0
        fn_state.gs_15508 = s_19_0;
        // N s_19_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#15506 <= s_20_0
        fn_state.gs_15506 = s_20_0;
        // N s_20_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#15504 <= s_21_0
        fn_state.gs_15504 = s_21_0;
        // N s_21_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#15502 <= s_22_0
        fn_state.gs_15502 = s_22_0;
        // N s_22_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#15488 <= s_23_0
        fn_state.gs_15488 = s_23_0;
        // N s_23_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_24_0: const #31s : i
        let s_24_0: i128 = 31;
        // D s_24_1: read-var n:i
        let s_24_1: i128 = fn_state.n;
        // D s_24_2: cmp-le s_24_1 s_24_0
        let s_24_2: bool = ((s_24_1) <= (s_24_0));
        // D s_24_3: write-var gs#15485 <= s_24_2
        fn_state.gs_15485 = s_24_2;
        // N s_24_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
