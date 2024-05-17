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
use V_set::*;
use u__id::*;
use V_read::*;
use common::*;
pub fn Vpart_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i128,
    part: i128,
    width: i128,
    value_name: Bits,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_15535: bool,
        widthshadow_268: i128,
        gs_15529: bool,
        gs_15513: bool,
        gs_15533: bool,
        gs_15516: bool,
        gs_15531: bool,
        n: i128,
        part: i128,
        width: i128,
        value_name: Bits,
    }
    let fn_state = FunctionState {
        n,
        part,
        width,
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var width:i
        let s_0_0: i128 = fn_state.width;
        // D s_0_1: write-var widthshadow#268 <= s_0_0
        fn_state.widthshadow_268 = s_0_0;
        // C s_0_2: const #0s : i
        let s_0_2: i128 = 0;
        // D s_0_3: read-var n:i
        let s_0_3: i128 = fn_state.n;
        // D s_0_4: cmp-ge s_0_3 s_0_2
        let s_0_4: bool = ((s_0_3) >= (s_0_2));
        // N s_0_5: branch s_0_4 b20 b1
        if s_0_4 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#15513 <= s_1_0
        fn_state.gs_15513 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#15513:u8
        let s_2_0: bool = fn_state.gs_15513;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #0s : i
        let s_2_2: i128 = 0;
        // D s_2_3: read-var part:i
        let s_2_3: i128 = fn_state.part;
        // D s_2_4: cmp-eq s_2_3 s_2_2
        let s_2_4: bool = ((s_2_3) == (s_2_2));
        // N s_2_5: branch s_2_4 b19 b3
        if s_2_4 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #1s : i
        let s_3_0: i128 = 1;
        // D s_3_1: read-var part:i
        let s_3_1: i128 = fn_state.part;
        // D s_3_2: cmp-eq s_3_1 s_3_0
        let s_3_2: bool = ((s_3_1) == (s_3_0));
        // D s_3_3: write-var gs#15516 <= s_3_2
        fn_state.gs_15516 = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#15516:u8
        let s_4_0: bool = fn_state.gs_15516;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #0s : i
        let s_4_2: i128 = 0;
        // D s_4_3: read-var part:i
        let s_4_3: i128 = fn_state.part;
        // D s_4_4: cmp-eq s_4_3 s_4_2
        let s_4_4: bool = ((s_4_3) == (s_4_2));
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
        // C s_5_0: const #64s : i
        let s_5_0: i128 = 64;
        // D s_5_1: read-var widthshadow#268:i
        let s_5_1: i128 = fn_state.widthshadow_268;
        // D s_5_2: cmp-eq s_5_1 s_5_0
        let s_5_2: bool = ((s_5_1) == (s_5_0));
        // N s_5_3: assert s_5_2
        let s_5_3: () = assert!(s_5_2);
        // C s_5_4: const #64s : i64
        let s_5_4: i64 = 64;
        // D s_5_5: read-var n:i
        let s_5_5: i128 = fn_state.n;
        // D s_5_6: call V_read(s_5_5, s_5_4)
        let s_5_6: Bits = V_read(state, tracer, s_5_5, s_5_4);
        // D s_5_7: cast reint s_5_6 -> u64
        let s_5_7: u64 = (s_5_6.value() as u64);
        // C s_5_8: const #128s : i64
        let s_5_8: i64 = 128;
        // C s_5_9: const #0s : i
        let s_5_9: i128 = 0;
        // D s_5_10: read-var value_name:bv
        let s_5_10: Bits = fn_state.value_name;
        // C s_5_11: const #1s : i64
        let s_5_11: i64 = 1;
        // C s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (i128::try_from(s_5_11).unwrap());
        // C s_5_13: const #63s : i
        let s_5_13: i128 = 63;
        // C s_5_14: add s_5_13 s_5_12
        let s_5_14: i128 = (s_5_13 + s_5_12);
        // D s_5_15: bit-extract s_5_10 s_5_9 s_5_14
        let s_5_15: Bits = (Bits::new(
            ((s_5_10) >> (s_5_9)).value(),
            u16::try_from(s_5_14).unwrap(),
        ));
        // D s_5_16: cast reint s_5_15 -> u64
        let s_5_16: u64 = (s_5_15.value() as u64);
        // D s_5_17: cast zx s_5_16 -> bv
        let s_5_17: Bits = Bits::new(s_5_16 as u128, 64u16);
        // D s_5_18: cast zx s_5_7 -> bv
        let s_5_18: Bits = Bits::new(s_5_7 as u128, 64u16);
        // D s_5_19: cast reint s_5_17 -> u128
        let s_5_19: u128 = (s_5_17.value() as u128);
        // D s_5_20: size-of s_5_17
        let s_5_20: u16 = s_5_17.length();
        // D s_5_21: cast reint s_5_18 -> u128
        let s_5_21: u128 = (s_5_18.value() as u128);
        // D s_5_22: size-of s_5_18
        let s_5_22: u16 = s_5_18.length();
        // D s_5_23: lsl s_5_19 s_5_22
        let s_5_23: u128 = s_5_19 << s_5_22;
        // D s_5_24: or s_5_23 s_5_21
        let s_5_24: u128 = ((s_5_23) | (s_5_21));
        // D s_5_25: add s_5_20 s_5_22
        let s_5_25: u16 = (s_5_20 + s_5_22);
        // D s_5_26: create-bits s_5_24 s_5_25
        let s_5_26: Bits = Bits::new(s_5_24, s_5_25);
        // D s_5_27: cast reint s_5_26 -> u128
        let s_5_27: u128 = (s_5_26.value() as u128);
        // D s_5_28: cast zx s_5_27 -> bv
        let s_5_28: Bits = Bits::new(s_5_27 as u128, 128u16);
        // D s_5_29: read-var n:i
        let s_5_29: i128 = fn_state.n;
        // D s_5_30: call V_set(s_5_29, s_5_8, s_5_28)
        let s_5_30: () = V_set(state, tracer, s_5_29, s_5_8, s_5_28);
        // N s_5_31: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #128s : i
        let s_6_0: i128 = 128;
        // D s_6_1: read-var widthshadow#268:i
        let s_6_1: i128 = fn_state.widthshadow_268;
        // D s_6_2: cmp-lt s_6_1 s_6_0
        let s_6_2: bool = ((s_6_1) < (s_6_0));
        // N s_6_3: assert s_6_2
        let s_6_3: () = assert!(s_6_2);
        // D s_6_4: read-var widthshadow#268:i
        let s_6_4: i128 = fn_state.widthshadow_268;
        // D s_6_5: call __id(s_6_4)
        let s_6_5: i128 = u__id(state, tracer, s_6_4);
        // D s_6_6: cast reint s_6_5 -> i64
        let s_6_6: i64 = (s_6_5 as i64);
        // C s_6_7: const #8s : i
        let s_6_7: i128 = 8;
        // D s_6_8: cast zx s_6_6 -> i
        let s_6_8: i128 = (i128::try_from(s_6_6).unwrap());
        // D s_6_9: cmp-eq s_6_8 s_6_7
        let s_6_9: bool = ((s_6_8) == (s_6_7));
        // N s_6_10: branch s_6_9 b18 b7
        if s_6_9 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var widthshadow#268:i
        let s_7_0: i128 = fn_state.widthshadow_268;
        // D s_7_1: call __id(s_7_0)
        let s_7_1: i128 = u__id(state, tracer, s_7_0);
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // C s_7_3: const #16s : i
        let s_7_3: i128 = 16;
        // D s_7_4: cast zx s_7_2 -> i
        let s_7_4: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_5: cmp-eq s_7_4 s_7_3
        let s_7_5: bool = ((s_7_4) == (s_7_3));
        // D s_7_6: write-var gs#15529 <= s_7_5
        fn_state.gs_15529 = s_7_5;
        // N s_7_7: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#15529:u8
        let s_8_0: bool = fn_state.gs_15529;
        // N s_8_1: branch s_8_0 b17 b9
        if s_8_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var widthshadow#268:i
        let s_9_0: i128 = fn_state.widthshadow_268;
        // D s_9_1: call __id(s_9_0)
        let s_9_1: i128 = u__id(state, tracer, s_9_0);
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // C s_9_3: const #32s : i
        let s_9_3: i128 = 32;
        // D s_9_4: cast zx s_9_2 -> i
        let s_9_4: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_5: cmp-eq s_9_4 s_9_3
        let s_9_5: bool = ((s_9_4) == (s_9_3));
        // D s_9_6: write-var gs#15531 <= s_9_5
        fn_state.gs_15531 = s_9_5;
        // N s_9_7: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#15531:u8
        let s_10_0: bool = fn_state.gs_15531;
        // N s_10_1: branch s_10_0 b16 b11
        if s_10_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var widthshadow#268:i
        let s_11_0: i128 = fn_state.widthshadow_268;
        // D s_11_1: call __id(s_11_0)
        let s_11_1: i128 = u__id(state, tracer, s_11_0);
        // D s_11_2: cast reint s_11_1 -> i64
        let s_11_2: i64 = (s_11_1 as i64);
        // C s_11_3: const #64s : i
        let s_11_3: i128 = 64;
        // D s_11_4: cast zx s_11_2 -> i
        let s_11_4: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_5: cmp-eq s_11_4 s_11_3
        let s_11_5: bool = ((s_11_4) == (s_11_3));
        // D s_11_6: write-var gs#15533 <= s_11_5
        fn_state.gs_15533 = s_11_5;
        // N s_11_7: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#15533:u8
        let s_12_0: bool = fn_state.gs_15533;
        // N s_12_1: branch s_12_0 b15 b13
        if s_12_0 {
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
        // D s_13_0: read-var widthshadow#268:i
        let s_13_0: i128 = fn_state.widthshadow_268;
        // D s_13_1: call __id(s_13_0)
        let s_13_1: i128 = u__id(state, tracer, s_13_0);
        // D s_13_2: cast reint s_13_1 -> i64
        let s_13_2: i64 = (s_13_1 as i64);
        // C s_13_3: const #128s : i
        let s_13_3: i128 = 128;
        // D s_13_4: cast zx s_13_2 -> i
        let s_13_4: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_5: cmp-eq s_13_4 s_13_3
        let s_13_5: bool = ((s_13_4) == (s_13_3));
        // D s_13_6: write-var gs#15535 <= s_13_5
        fn_state.gs_15535 = s_13_5;
        // N s_13_7: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#15535:u8
        let s_14_0: bool = fn_state.gs_15535;
        // N s_14_1: assert s_14_0
        let s_14_1: () = assert!(s_14_0);
        // D s_14_2: read-var widthshadow#268:i
        let s_14_2: i128 = fn_state.widthshadow_268;
        // D s_14_3: cast reint s_14_2 -> i64
        let s_14_3: i64 = (s_14_2 as i64);
        // D s_14_4: read-var n:i
        let s_14_4: i128 = fn_state.n;
        // D s_14_5: read-var value_name:bv
        let s_14_5: Bits = fn_state.value_name;
        // D s_14_6: call V_set(s_14_4, s_14_3, s_14_5)
        let s_14_6: () = V_set(state, tracer, s_14_4, s_14_3, s_14_5);
        // N s_14_7: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#15535 <= s_15_0
        fn_state.gs_15535 = s_15_0;
        // N s_15_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#15533 <= s_16_0
        fn_state.gs_15533 = s_16_0;
        // N s_16_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#15531 <= s_17_0
        fn_state.gs_15531 = s_17_0;
        // N s_17_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#15529 <= s_18_0
        fn_state.gs_15529 = s_18_0;
        // N s_18_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#15516 <= s_19_0
        fn_state.gs_15516 = s_19_0;
        // N s_19_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #31s : i
        let s_20_0: i128 = 31;
        // D s_20_1: read-var n:i
        let s_20_1: i128 = fn_state.n;
        // D s_20_2: cmp-le s_20_1 s_20_0
        let s_20_2: bool = ((s_20_1) <= (s_20_0));
        // D s_20_3: write-var gs#15513 <= s_20_2
        fn_state.gs_15513 = s_20_2;
        // N s_20_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
