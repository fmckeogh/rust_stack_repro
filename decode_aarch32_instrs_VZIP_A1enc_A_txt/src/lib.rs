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
use ConditionPassed::*;
use execute_aarch32_instrs_VZIP_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VZIP_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    size: u8,
    Vd: u8,
    Q: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_323083: bool,
        gs_323090: bool,
        gs_323084: bool,
        gs_323089: bool,
        D: bool,
        size: u8,
        Vd: u8,
        Q: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        size,
        Vd,
        Q,
        M,
        Vm,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call ConditionPassed(s_0_0)
        let s_0_1: bool = ConditionPassed(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
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
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var size:u8
        let s_2_0: u8 = fn_state.size;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #3u : u8
        let s_2_2: u8 = 3;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b18 b3
        if s_2_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var Q:u8
        let s_3_0: bool = fn_state.Q;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // C s_3_2: const #0u : u8
        let s_3_2: bool = false;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b17 b4
        if s_3_4 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#323083 <= s_4_0
        fn_state.gs_323083 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#323083:u8
        let s_5_0: bool = fn_state.gs_323083;
        // D s_5_1: write-var gs#323084 <= s_5_0
        fn_state.gs_323084 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#323084:u8
        let s_6_0: bool = fn_state.gs_323084;
        // N s_6_1: branch s_6_0 b16 b7
        if s_6_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var Q:u8
        let s_7_0: bool = fn_state.Q;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 1u16);
        // C s_7_2: const #1u : u8
        let s_7_2: bool = true;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // N s_7_5: branch s_7_4 b12 b8
        if s_7_4 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#323090 <= s_8_0
        fn_state.gs_323090 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#323090:u8
        let s_9_0: bool = fn_state.gs_323090;
        // N s_9_1: branch s_9_0 b11 b10
        if s_9_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var Q:u8
        let s_10_0: bool = fn_state.Q;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #1u : u8
        let s_10_2: bool = true;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: read-var size:u8
        let s_10_5: u8 = fn_state.size;
        // D s_10_6: cast zx s_10_5 -> bv
        let s_10_6: Bits = Bits::new(s_10_5 as u128, 2u16);
        // D s_10_7: cast zx s_10_6 -> i
        let s_10_7: i128 = (s_10_6.value() as i128);
        // D s_10_8: cast reint s_10_7 -> i64
        let s_10_8: i64 = (s_10_7 as i64);
        // C s_10_9: const #8s : i64
        let s_10_9: i64 = 8;
        // D s_10_10: lsl s_10_9 s_10_8
        let s_10_10: i64 = s_10_9 << s_10_8;
        // D s_10_11: read-var D:u8
        let s_10_11: bool = fn_state.D;
        // D s_10_12: cast zx s_10_11 -> bv
        let s_10_12: Bits = Bits::new(s_10_11 as u128, 1u16);
        // D s_10_13: read-var Vd:u8
        let s_10_13: u8 = fn_state.Vd;
        // D s_10_14: cast zx s_10_13 -> bv
        let s_10_14: Bits = Bits::new(s_10_13 as u128, 4u16);
        // D s_10_15: cast reint s_10_12 -> u128
        let s_10_15: u128 = (s_10_12.value() as u128);
        // D s_10_16: size-of s_10_12
        let s_10_16: u16 = s_10_12.length();
        // D s_10_17: cast reint s_10_14 -> u128
        let s_10_17: u128 = (s_10_14.value() as u128);
        // D s_10_18: size-of s_10_14
        let s_10_18: u16 = s_10_14.length();
        // D s_10_19: lsl s_10_15 s_10_18
        let s_10_19: u128 = s_10_15 << s_10_18;
        // D s_10_20: or s_10_19 s_10_17
        let s_10_20: u128 = ((s_10_19) | (s_10_17));
        // D s_10_21: add s_10_16 s_10_18
        let s_10_21: u16 = (s_10_16 + s_10_18);
        // D s_10_22: create-bits s_10_20 s_10_21
        let s_10_22: Bits = Bits::new(s_10_20, s_10_21);
        // D s_10_23: cast reint s_10_22 -> u8
        let s_10_23: u8 = (s_10_22.value() as u8);
        // D s_10_24: cast zx s_10_23 -> bv
        let s_10_24: Bits = Bits::new(s_10_23 as u128, 5u16);
        // D s_10_25: cast zx s_10_24 -> i
        let s_10_25: i128 = (s_10_24.value() as i128);
        // D s_10_26: cast reint s_10_25 -> i64
        let s_10_26: i64 = (s_10_25 as i64);
        // D s_10_27: read-var M:u8
        let s_10_27: bool = fn_state.M;
        // D s_10_28: cast zx s_10_27 -> bv
        let s_10_28: Bits = Bits::new(s_10_27 as u128, 1u16);
        // D s_10_29: read-var Vm:u8
        let s_10_29: u8 = fn_state.Vm;
        // D s_10_30: cast zx s_10_29 -> bv
        let s_10_30: Bits = Bits::new(s_10_29 as u128, 4u16);
        // D s_10_31: cast reint s_10_28 -> u128
        let s_10_31: u128 = (s_10_28.value() as u128);
        // D s_10_32: size-of s_10_28
        let s_10_32: u16 = s_10_28.length();
        // D s_10_33: cast reint s_10_30 -> u128
        let s_10_33: u128 = (s_10_30.value() as u128);
        // D s_10_34: size-of s_10_30
        let s_10_34: u16 = s_10_30.length();
        // D s_10_35: lsl s_10_31 s_10_34
        let s_10_35: u128 = s_10_31 << s_10_34;
        // D s_10_36: or s_10_35 s_10_33
        let s_10_36: u128 = ((s_10_35) | (s_10_33));
        // D s_10_37: add s_10_32 s_10_34
        let s_10_37: u16 = (s_10_32 + s_10_34);
        // D s_10_38: create-bits s_10_36 s_10_37
        let s_10_38: Bits = Bits::new(s_10_36, s_10_37);
        // D s_10_39: cast reint s_10_38 -> u8
        let s_10_39: u8 = (s_10_38.value() as u8);
        // D s_10_40: cast zx s_10_39 -> bv
        let s_10_40: Bits = Bits::new(s_10_39 as u128, 5u16);
        // D s_10_41: cast zx s_10_40 -> i
        let s_10_41: i128 = (s_10_40.value() as i128);
        // D s_10_42: cast reint s_10_41 -> i64
        let s_10_42: i64 = (s_10_41 as i64);
        // D s_10_43: cast zx s_10_10 -> i
        let s_10_43: i128 = (i128::try_from(s_10_10).unwrap());
        // D s_10_44: cast reint s_10_43 -> i64
        let s_10_44: i64 = (s_10_43 as i64);
        // D s_10_45: call execute_aarch32_instrs_VZIP_Op_A_txt(s_10_26, s_10_44, s_10_42, s_10_4)
        let s_10_45: () = execute_aarch32_instrs_VZIP_Op_A_txt(
            state,
            tracer,
            s_10_26,
            s_10_44,
            s_10_42,
            s_10_4,
        );
        // N s_10_46: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: panic
        panic!("{:?}", ());
        // N s_11_1: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0s : i
        let s_12_0: i128 = 0;
        // D s_12_1: read-var Vd:u8
        let s_12_1: u8 = fn_state.Vd;
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 4u16);
        // C s_12_3: const #1u : u64
        let s_12_3: u64 = 1;
        // D s_12_4: bit-extract s_12_2 s_12_0 s_12_3
        let s_12_4: Bits = (Bits::new(
            ((s_12_2) >> (s_12_0)).value(),
            u16::try_from(s_12_3).unwrap(),
        ));
        // D s_12_5: cast reint s_12_4 -> u8
        let s_12_5: bool = ((s_12_4.value()) != 0);
        // C s_12_6: const #0s : i
        let s_12_6: i128 = 0;
        // C s_12_7: const #0u : u64
        let s_12_7: u64 = 0;
        // D s_12_8: cast zx s_12_5 -> u64
        let s_12_8: u64 = (s_12_5 as u64);
        // C s_12_9: const #1u : u64
        let s_12_9: u64 = 1;
        // D s_12_10: and s_12_8 s_12_9
        let s_12_10: u64 = ((s_12_8) & (s_12_9));
        // D s_12_11: cmp-eq s_12_10 s_12_9
        let s_12_11: bool = ((s_12_10) == (s_12_9));
        // D s_12_12: lsl s_12_8 s_12_6
        let s_12_12: u64 = s_12_8 << s_12_6;
        // D s_12_13: or s_12_7 s_12_12
        let s_12_13: u64 = ((s_12_7) | (s_12_12));
        // D s_12_14: cmpl s_12_12
        let s_12_14: u64 = !s_12_12;
        // D s_12_15: and s_12_7 s_12_14
        let s_12_15: u64 = ((s_12_7) & (s_12_14));
        // D s_12_16: select s_12_11 s_12_13 s_12_15
        let s_12_16: u64 = if s_12_11 { s_12_13 } else { s_12_15 };
        // D s_12_17: cast trunc s_12_16 -> u8
        let s_12_17: bool = ((s_12_16) != 0);
        // D s_12_18: cast zx s_12_17 -> bv
        let s_12_18: Bits = Bits::new(s_12_17 as u128, 1u16);
        // C s_12_19: const #1u : u8
        let s_12_19: bool = true;
        // C s_12_20: cast zx s_12_19 -> bv
        let s_12_20: Bits = Bits::new(s_12_19 as u128, 1u16);
        // D s_12_21: cmp-eq s_12_18 s_12_20
        let s_12_21: bool = ((s_12_18) == (s_12_20));
        // N s_12_22: branch s_12_21 b15 b13
        if s_12_21 {
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
        // C s_13_0: const #0s : i
        let s_13_0: i128 = 0;
        // D s_13_1: read-var Vm:u8
        let s_13_1: u8 = fn_state.Vm;
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 4u16);
        // C s_13_3: const #1u : u64
        let s_13_3: u64 = 1;
        // D s_13_4: bit-extract s_13_2 s_13_0 s_13_3
        let s_13_4: Bits = (Bits::new(
            ((s_13_2) >> (s_13_0)).value(),
            u16::try_from(s_13_3).unwrap(),
        ));
        // D s_13_5: cast reint s_13_4 -> u8
        let s_13_5: bool = ((s_13_4.value()) != 0);
        // C s_13_6: const #0s : i
        let s_13_6: i128 = 0;
        // C s_13_7: const #0u : u64
        let s_13_7: u64 = 0;
        // D s_13_8: cast zx s_13_5 -> u64
        let s_13_8: u64 = (s_13_5 as u64);
        // C s_13_9: const #1u : u64
        let s_13_9: u64 = 1;
        // D s_13_10: and s_13_8 s_13_9
        let s_13_10: u64 = ((s_13_8) & (s_13_9));
        // D s_13_11: cmp-eq s_13_10 s_13_9
        let s_13_11: bool = ((s_13_10) == (s_13_9));
        // D s_13_12: lsl s_13_8 s_13_6
        let s_13_12: u64 = s_13_8 << s_13_6;
        // D s_13_13: or s_13_7 s_13_12
        let s_13_13: u64 = ((s_13_7) | (s_13_12));
        // D s_13_14: cmpl s_13_12
        let s_13_14: u64 = !s_13_12;
        // D s_13_15: and s_13_7 s_13_14
        let s_13_15: u64 = ((s_13_7) & (s_13_14));
        // D s_13_16: select s_13_11 s_13_13 s_13_15
        let s_13_16: u64 = if s_13_11 { s_13_13 } else { s_13_15 };
        // D s_13_17: cast trunc s_13_16 -> u8
        let s_13_17: bool = ((s_13_16) != 0);
        // D s_13_18: cast zx s_13_17 -> bv
        let s_13_18: Bits = Bits::new(s_13_17 as u128, 1u16);
        // C s_13_19: const #1u : u8
        let s_13_19: bool = true;
        // C s_13_20: cast zx s_13_19 -> bv
        let s_13_20: Bits = Bits::new(s_13_19 as u128, 1u16);
        // D s_13_21: cmp-eq s_13_18 s_13_20
        let s_13_21: bool = ((s_13_18) == (s_13_20));
        // D s_13_22: write-var gs#323089 <= s_13_21
        fn_state.gs_323089 = s_13_21;
        // N s_13_23: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#323089:u8
        let s_14_0: bool = fn_state.gs_323089;
        // D s_14_1: write-var gs#323090 <= s_14_0
        fn_state.gs_323090 = s_14_0;
        // N s_14_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#323089 <= s_15_0
        fn_state.gs_323089 = s_15_0;
        // N s_15_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: panic
        panic!("{:?}", ());
        // N s_16_1: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var size:u8
        let s_17_0: u8 = fn_state.size;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 2u16);
        // C s_17_2: const #2u : u8
        let s_17_2: u8 = 2;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 2u16);
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // D s_17_5: write-var gs#323083 <= s_17_4
        fn_state.gs_323083 = s_17_4;
        // N s_17_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#323084 <= s_18_0
        fn_state.gs_323084 = s_18_0;
        // N s_18_2: jump b6
        return block_6(state, tracer, fn_state);
    }
}
